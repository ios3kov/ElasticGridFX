#import <AppKit/AppKit.h>

// DO_DIALOG UI entry only. No entitlement, credentials, network validation or
// project state is read/written here; store adapters are a separate milestone.
extern "C" bool eg_show_license_window(const char *version) {
    if (![NSThread isMainThread] || NSApp == nil || version == nullptr) return false;
    @autoreleasepool {
        @try {
            NSString *versionString = [NSString stringWithUTF8String:version];
            if (versionString == nil) return false;
            while (true) {
                NSAlert *alert = [[NSAlert alloc] init];
                alert.messageText = @"FSTR Stretch — License";
                alert.informativeText = @"Licensing is not connected yet. This development build works without activation.\n\nPlanned stores: aescripts and Plugin Play.";
                [alert addButtonWithTitle:@"Close"];
                [alert addButtonWithTitle:@"About"];
                [alert addButtonWithTitle:@"Support"];
                NSModalResponse response = [alert runModal];
                if (response == NSAlertSecondButtonReturn) {
                    NSAlert *about = [[NSAlert alloc] init];
                    about.messageText = @"FSTR Stretch";
                    about.informativeText = [NSString stringWithFormat:@"Version %@\n\nProfessional mesh deformation for Adobe After Effects\n\n© 2026 FSTR.tech. All rights reserved", versionString];
                    [about addButtonWithTitle:@"OK"];
                    [about runModal];
                } else if (response == NSAlertThirdButtonReturn) {
                    // Open only after an explicit Support click, never on startup
                    // or render. The URL contains no license/user/project data.
                    NSURL *url = [NSURL URLWithString:@"https://github.com/ios3kov/ElasticGridFX/issues"];
                    if (![[NSWorkspace sharedWorkspace] openURL:url]) {
                        NSAlert *error = [[NSAlert alloc] init];
                        error.messageText = @"Could not open support";
                        error.informativeText = @"Visit github.com/ios3kov/ElasticGridFX/issues in your browser.";
                        [error addButtonWithTitle:@"OK"];
                        [error runModal];
                    }
                } else {
                    return true;
                }
            }
        } @catch (NSException *) {
            // Do not let Objective-C exceptions cross the Rust/C ABI.
            return false;
        }
    }
}
