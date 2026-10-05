#import <AppKit/AppKit.h>
#include "NativeInstaller.h"
#include "Payload.h"
#include <iostream>
#include <unistd.h>

int main(int argc,const char* argv[]) {
    @autoreleasepool {
        if(argc==2 && (std::string(argv[1])=="--install" || std::string(argv[1])=="--restore")) {
            try {
                if(geteuid()!=0)throw std::runtime_error("Administrator authorization is required.");
                auto environment=fstr::installer::systemEnvironment();
                // Only pre-transaction host refusal is retryable. Later failures
                // retain the ordinary stopped/recovery path, never auto-retry.
                try {environment.checkHosts();}
                catch(const fstr::installer::HostsRunning& e){std::cerr<<e.what()<<'\n';return 75;}
                std::cout<<fstr::installer::runNative(environment,std::string(argv[1])=="--restore")<<'\n';return 0;
            } catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}
        }
        if(argc!=1)return 2; // No arbitrary destinations, test roots or payload input.
        [NSApplication sharedApplication];[NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];
        // This app uses NSAlert's modal loop rather than NSApplication::run.
        // Complete the launch lifecycle explicitly before presenting the UI.
        [NSApp finishLaunching];[NSApp activateIgnoringOtherApps:YES];
        for(;;){
            NSAlert* alert=[NSAlert new];alert.messageText=@"FSTR Stretch Installer";
            alert.informativeText=[NSString stringWithFormat:@"Version %s\n\nInstall saves your previous version. Restore brings it back. Close Adobe render applications first.\n\nIf you use a custom plug-in folder, remove older FSTR copies there first.\n\nBackups: /Library/Application Support/FSTR FX/Backups",fstr::payload::version];
            [alert addButtonWithTitle:@"Install"];[alert addButtonWithTitle:@"Restore"];[alert addButtonWithTitle:@"Close"];
            auto choice=[alert runModal];if(choice!=NSAlertFirstButtonReturn && choice!=NSAlertSecondButtonReturn)break;
            NSString* action=choice==NSAlertFirstButtonReturn?@"--install":@"--restore";
            // AppleScript's quoted form protects spaces, quotes and shell syntax
            // in the selected app's path. Only a fixed action is appended.
            NSString* executable=[NSBundle mainBundle].executablePath;
            if(!executable || [executable rangeOfCharacterFromSet:[NSCharacterSet controlCharacterSet]].location!=NSNotFound){
                NSAlert* stopped=[NSAlert new];stopped.messageText=@"Move the installer to a folder with a simple name.";[stopped runModal];continue;
            }
            NSString* literal=[executable stringByReplacingOccurrencesOfString:@"\\" withString:@"\\\\"];
            literal=[literal stringByReplacingOccurrencesOfString:@"\"" withString:@"\\\""];
            NSAppleScript* script=[[NSAppleScript alloc] initWithSource:[NSString stringWithFormat:@"do shell script ((quoted form of \"%@\") & \" %@\") with administrator privileges",literal,action]];
            NSDictionary* error=nullptr;NSAppleEventDescriptor* result=nullptr;
            for(;;){
                error=nullptr;result=[script executeAndReturnError:&error];
                if(result || [error[NSAppleScriptErrorNumber] intValue]!=75)break;
                NSAlert* waiting=[NSAlert new];waiting.messageText=@"Close Adobe applications";
                waiting.informativeText=@"Close After Effects and other Adobe render applications, then click Continue. Your selected action will continue.";
                [waiting addButtonWithTitle:@"Continue"];[waiting addButtonWithTitle:@"Cancel"];
                if([waiting runModal]!=NSAlertFirstButtonReturn)return 0;
            }
            if(!result && [error[NSAppleScriptErrorNumber] intValue]==-128)continue;
            NSAlert* outcome=[NSAlert new];outcome.messageText=result?@"Done":@"Installation stopped";
            outcome.informativeText=result.stringValue?:error[NSAppleScriptErrorMessage]?:@"No result received. Inspect Backups before trying again.";
            [outcome addButtonWithTitle:@"OK"];[outcome runModal];
            if(result)break; // Successful Install/Restore ends this session.
        }
    }
    return 0;
}
