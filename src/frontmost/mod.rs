use objc2::declare::ClassBuilder;
use objc2::runtime::{NSObject, Sel};
use objc2::{ClassType, msg_send, sel};
use objc2_app_kit::{
    NSRunningApplication, NSWorkspace, NSWorkspaceApplicationKey,
    NSWorkspaceDidActivateApplicationNotification,
};
use objc2_foundation::NSNotification;

#[macro_export]
macro_rules! start_nsrunloop {
    () => {
        unsafe {
            use objc2_foundation::NSRunLoop;
            let current_run_loop = NSRunLoop::currentRunLoop();
            current_run_loop.run();
        }
    };
}

pub struct FrontmostAppDetector;

impl FrontmostAppDetector {
    // initialize FrontmostAppDetector object with the `init()` function by
    // passing in the callback function that will be triggered upon switching the frontmost app
    pub fn init(callback: fn(&NSRunningApplication)) {
        static mut CALLBACK: Option<fn(&NSRunningApplication)> = None;
        unsafe {
            CALLBACK = Some(callback);
        }

        // this is the Observer object that we'll be using
        // using ClassBuilder since I wasn't able to figure out
        // how to register external methods with the `define_class!` macro
        let mut builder = ClassBuilder::new(c"AppObserver", NSObject::class())
            .expect("a class with name AppObserver likely already exists.");

        // defining the external methods
        unsafe extern "C" fn init(this: *mut NSObject, _sel: Sel) -> *mut NSObject {
            let this: *mut NSObject = msg_send![super(this, NSObject::class()), init];
            this
        }
        unsafe extern "C" fn application_activated(
            _this: *mut NSObject,
            _sel: Sel,
            notification: *mut NSNotification,
        ) {
            unsafe {
                let dereference_notif: &mut NSNotification = &mut *notification;
                let user_info = dereference_notif
                    .userInfo()
                    .expect("userInfo returned as None");
                let associated_object = user_info
                    .objectForKey(NSWorkspaceApplicationKey)
                    .expect("Failed to capture value for NSWorkspaceApplicationKey");
                let ns_running_app: &NSRunningApplication = associated_object
                    .downcast_ref::<NSRunningApplication>()
                    .expect("Failed to downcast ref associated object to an NSRunningApplication");
                if let Some(callback) = CALLBACK {
                    callback(ns_running_app);
                }
            }
        }

        unsafe {
            builder.add_method(
                sel!(init),
                init as unsafe extern "C" fn(*mut NSObject, Sel) -> *mut NSObject,
            );
            builder.add_method(
                sel!(applicationActivated:),
                application_activated
                    as unsafe extern "C" fn(*mut NSObject, Sel, *mut NSNotification),
            );
        }

        // register new AppObserver class to the Objective-C runtime
        let app_observer_class = builder.register();

        // add Observer to the notification center
        unsafe {
            let observer: *mut NSObject = msg_send![app_observer_class, alloc];
            let observer: *mut NSObject = msg_send![observer, init];

            let workspace = NSWorkspace::sharedWorkspace();
            let notification_center = workspace.notificationCenter();

            notification_center.addObserver_selector_name_object(
                &*(observer as *const NSObject),
                sel!(applicationActivated:),
                Some(NSWorkspaceDidActivateApplicationNotification),
                None,
            );
        }
    }
}
