use objc2::runtime::*;
use objc2::*;
use objc2_app_kit::*;
use objc2_foundation::*;

fn register_class() -> &'static AnyClass {
    let mut builder = ClassBuilder::new(c"AppObserver", NSObject::class())
        .expect("a class with name AppObserver likely already exists.");

    unsafe extern "C" fn init(this: *mut AnyObject, _sel: Sel) -> *mut AnyObject {
        let this: *mut AnyObject = msg_send![super(this, NSObject::class()), init];
        this
    }
    unsafe {
        builder.add_method(
            sel!(init),
            init as unsafe extern "C" fn(*mut AnyObject, Sel) -> *mut AnyObject,
        );
    }

    unsafe extern "C" fn application_activated(
        _this: *mut AnyObject,
        _sel: Sel,
        notification: *mut NSNotification,
    ) {
        unsafe {
            let dereference_notif: &mut NSNotification = &mut *notification;
            let user_info = &*dereference_notif
                .userInfo()
                .expect("User info returned None");
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
    unsafe {
        builder.add_method(
            sel!(applicationActivated:),
            application_activated as unsafe extern "C" fn(*mut AnyObject, Sel, *mut NSNotification),
        );
    }

    builder.register()
}

fn main() {
    unsafe {
        let app_observer_class = register_class();
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

        println!("Monitoring application activations. Press Ctrl+C to stop.");
        let current_run_loop = NSRunLoop::currentRunLoop();
        current_run_loop.run();
    }
}
