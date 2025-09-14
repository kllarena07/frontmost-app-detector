use objc2::runtime::*;
use objc2::*;
use objc2_app_kit::*;
use objc2_foundation::*;

macro_rules! start_nsrunloop {
    () => {
        unsafe {
            let current_run_loop = NSRunLoop::currentRunLoop();
            current_run_loop.run();
        }
    };
}

struct FrontmostAppDetector;

type NotificationCallback = fn(&mut NSNotification);

impl FrontmostAppDetector {
    fn init(callback: NotificationCallback) {
        static mut CALLBACK: Option<NotificationCallback> = None;
        unsafe {
            CALLBACK = Some(callback);
        }

        let mut builder = ClassBuilder::new(c"AppObserver", NSObject::class())
            .expect("a class with name AppObserver likely already exists.");

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
                if let Some(callback) = CALLBACK {
                    callback(dereference_notif);
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

        let app_observer_class = builder.register();

        unsafe {
            let observer: *mut AnyObject = msg_send![app_observer_class, alloc];
            let observer: *mut AnyObject = msg_send![observer, init];

            let workspace = NSWorkspace::sharedWorkspace();
            let notification_center = workspace.notificationCenter();

            notification_center.addObserver_selector_name_object(
                &*(observer as *const AnyObject),
                sel!(applicationActivated:),
                Some(NSWorkspaceDidActivateApplicationNotification),
                None,
            );
        }
    }
}

fn main() {
    fn handle_app_change(notification: &mut NSNotification) {
        unsafe {
            let user_info = &*notification.userInfo().expect("User info returned None");
            let object = &*user_info
                .objectForKey(NSWorkspaceApplicationKey)
                .expect("Error getting NSWorkspaceApplicationKey Value");
            let key_value: &NSRunningApplication = object
                .downcast_ref::<NSRunningApplication>()
                .expect("Value is not an NSRunningApplication");
            println!(
                "Application activated: {:?}",
                key_value
                    .localizedName()
                    .expect("Failed to capture application localizedName")
            );
        }
    }

    FrontmostAppDetector::init(handle_app_change);

    println!("Monitoring application activations. Press Ctrl+C to stop.");
    start_nsrunloop!();
}
