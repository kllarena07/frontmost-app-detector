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
