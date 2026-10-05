#include "NativeInstaller.h"
#include "Payload.h"
#include "bridge/license_dialog_template.h"
#include <shellapi.h>
#include <exception>

namespace {
INT_PTR CALLBACK waitingDialog(HWND window,UINT message,WPARAM wparam,LPARAM){
    if(message==WM_CLOSE){EndDialog(window,IDCANCEL);return TRUE;}
    if(message==WM_COMMAND && HIWORD(wparam)==BN_CLICKED){
        auto id=LOWORD(wparam);if(id==IDOK || id==IDCANCEL){EndDialog(window,id);return TRUE;}
    }
    return FALSE;
}
bool continueAfterClosingHosts(HWND parent){
    elasticgrid::license_ui::DialogTemplate d;
    d.dword(0x80c800c0);d.dword(0);d.word(3);d.word(0);d.word(0);d.word(300);d.word(90);d.word(0);d.word(0);d.text(u"Close Adobe applications");d.word(9);d.text(u"Segoe UI");
    d.control(0x50000000,12,12,276,36,100,0x82,u"Close After Effects and other Adobe render applications, then click Continue. Your selected action will continue.");
    d.control(0x50010001,80,60,96,18,IDOK,0x80,u"Continue");d.control(0x50010000,188,60,96,18,IDCANCEL,0x80,u"Cancel");
    return DialogBoxIndirectParamW(GetModuleHandleW(nullptr),reinterpret_cast<const DLGTEMPLATE*>(d.words.data()),parent,waitingDialog,0)==IDOK;
}
INT_PTR CALLBACK dialog(HWND window,UINT message,WPARAM wparam,LPARAM){
    if(message==WM_CLOSE){EndDialog(window,0);return TRUE;}
    if(message!=WM_COMMAND || HIWORD(wparam)!=BN_CLICKED)return FALSE;
    auto id=LOWORD(wparam);if(id==IDCANCEL || id==3){EndDialog(window,0);return TRUE;}
    if(id!=1 && id!=101)return FALSE;
    EnableWindow(GetDlgItem(window,1),FALSE);EnableWindow(GetDlgItem(window,101),FALSE);
    SetCursor(LoadCursorW(nullptr,IDC_WAIT));
    try {
        auto environment=fstr::installer::systemEnvironment();
        for(;;){
            try {environment.checkHosts();break;}
            catch(const fstr::installer::HostsRunning&){
                SetCursor(LoadCursorW(nullptr,IDC_ARROW));
                if(!continueAfterClosingHosts(window)){EndDialog(window,0);return TRUE;}
                SetCursor(LoadCursorW(nullptr,IDC_WAIT));
            }
        }
        // Only the pre-transaction check retries; engine errors retain recovery.
        auto result=fstr::installer::runNative(environment,id==101);MessageBoxW(window,result.c_str(),L"FSTR Stretch",MB_OK);SetCursor(LoadCursorW(nullptr,IDC_ARROW));EndDialog(window,0);return TRUE;}
    catch(const std::exception& error){std::wstring text;for(const char* c=error.what();*c;++c)text.push_back(static_cast<wchar_t>(static_cast<unsigned char>(*c)));MessageBoxW(window,text.c_str(),L"Installation stopped",MB_OK|MB_ICONWARNING);}
    SetCursor(LoadCursorW(nullptr,IDC_ARROW));EnableWindow(GetDlgItem(window,1),TRUE);EnableWindow(GetDlgItem(window,101),TRUE);return TRUE;
}
}
int WINAPI wWinMain(HINSTANCE instance,HINSTANCE,PWSTR command,int){
    if(command && *command)return 2; // No test-root/payload/destination switches.
    if(!fstr::installer::elevated()){MessageBoxW(nullptr,L"Administrator authorization is required.",L"FSTR Stretch",MB_OK|MB_ICONWARNING);return 1;}
    auto com=CoInitializeEx(nullptr,COINIT_APARTMENTTHREADED);
    elasticgrid::license_ui::DialogTemplate d;d.dword(0x80c800c0);d.dword(0);d.word(4);d.word(0);d.word(0);d.word(350);d.word(185);d.word(0);d.word(0);d.text(u"FSTR Stretch Installer");d.word(9);d.text(u"Segoe UI");
    std::u16string version;for(const char* c=fstr::payload::version;*c;++c)version.push_back(static_cast<char16_t>(*c));
    std::u16string body=u"Version "+version+u"\r\n\r\nInstall saves your previous version. Restore brings it back. Close Adobe render applications first.\r\n\r\nIf you use a custom plug-in folder, remove older FSTR copies there first.\r\n\r\nBackups: ProgramData\\FSTR FX\\Backups";
    d.control(0x50000000,12,12,326,128,100,0x82,body);d.control(0x50010001,12,155,96,18,1,0x80,u"Install");d.control(0x50010000,127,155,96,18,101,0x80,u"Restore");d.control(0x50010000,242,155,96,18,IDCANCEL,0x80,u"Close");
    auto result=DialogBoxIndirectParamW(instance,reinterpret_cast<const DLGTEMPLATE*>(d.words.data()),nullptr,dialog,0);
    if(SUCCEEDED(com))CoUninitialize();
    return result==-1?1:0;
}
