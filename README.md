# Frontmost App Detector for MacOS

This repository hosts code that dynamically captures the currently focused application. Although it's plastered everywhere, I'll say it once again: **this works on MacOS only**.

## Motivations

I wrote this code for the purpose of integrating it into one of my other projects called [Discord RPC Everything](https://github.com/kllarena07/discord-rpc-everything) since I couldn't find anything else reliable online. So, I decided that I'd also share and open source it while I'm at it.

This code was adapted from some Objective-C code that I generated using LLMs (since I have no experience using Objective-C) using [`objc2`](https://docs.rs/objc2/latest/objc2/index.html) Rust bindings. It also uses the [`objc2_foundation`](https://docs.rs/objc2-foundation/latest/objc2_foundation/) and [`objc2_app_kit`](https://docs.rs/objc2-app-kit/0.3.1/objc2_app_kit/) frameworks 

## Why You Can't Just Use the Applications Crate

One might assume that you could simply use the [`applications`](https://crates.io/crates/applications) crate by placing `ctx.get_frontmost_application().unwrap()` inside a loop and be done with it. However, this approach does not work as expected in practice.

Why? Because macOS uses an event-driven architecture, and a blocking loop prevents the main [`NSRunLoop`](https://developer.apple.com/documentation/Foundation/RunLoop?language=objc) from processing events. The [`NSRunLoop`](https://developer.apple.com/documentation/Foundation/RunLoop?language=objc) powers macOS's asynchronous notification system, so blocking it disrupts normal event handling.

To work with this architecture, the code instead creates an observer (such as an NSWorkspace notification observer) that triggers when the user switches focus to a different application, specifically utilizing the `NSWorkspaceDidActivateApplicationNotification` notification. This allows your code to respond to application changes without blocking the run loop.

## How to Use
1. Bring the `frontmost` module and crate into scope
```
mod frontmost;
use frontmost::FrontmostAppDetector;
```
2. Create the callback function that will be used when the observer detects that the user has changed apps
```
use objc2_app_kit::NSRunningApplication;

fn handle_app_change(ns_running_application: &NSRunningApplication) {
    unsafe {
        let frontmost_app_name = ns_running_application
            .localizedName()
            .expect("Failed to capture application localizedName");
        println!("Application activated: {}", frontmost_app_name);
    }
}
```
3. Initialize a `FrontmostAppDetector` singleton by calling the `init` function and pass your callback function into it
```
FrontmostAppDetector::init(handle_app_change);
```
5. Start the event loop using the `start_nsrunloop!()` macro

## Original Objective-C Code
```
#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>

@interface AppObserver : NSObject
@end

@implementation AppObserver

- (void)applicationActivated:(NSNotification *)notification {
    NSRunningApplication *app = notification.userInfo[NSWorkspaceApplicationKey];
    NSLog(@"Activated: %@", app.localizedName);
}

@end

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        AppObserver *observer = [[AppObserver alloc] init];
        [[[NSWorkspace sharedWorkspace] notificationCenter]
            addObserver:observer
               selector:@selector(applicationActivated:)
                   name:NSWorkspaceDidActivateApplicationNotification
                 object:nil];

        // Start the run loop
        [[NSRunLoop currentRunLoop] run];
    }
    return 0;
}
```
