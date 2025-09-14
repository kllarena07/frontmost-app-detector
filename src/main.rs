mod frontmost;

use frontmost::FrontmostAppDetector;
use objc2_app_kit::NSRunningApplication;

fn main() {
    fn handle_app_change(ns_running_application: &NSRunningApplication) {
        unsafe {
            println!(
                "Application activated: {:?}",
                ns_running_application
                    .localizedName()
                    .expect("Failed to capture application localizedName")
            );
        }
    }

    FrontmostAppDetector::init(handle_app_change);

    println!("Monitoring application activations. Press Ctrl+C to stop.");
    start_nsrunloop!();
}
