#define WIN32_LEAN_AND_MEAN
#define NOMINMAX
#include <windows.h>
#include <shellapi.h>
#include <objbase.h>
#include <string>
#include "license_dialog_template.h"

namespace {
struct Context { std::wstring about; };

INT_PTR CALLBACK dialogProc(HWND window, UINT message, WPARAM wparam, LPARAM lparam) {
    if (message == WM_INITDIALOG) {
        SetWindowLongPtrW(window, DWLP_USER, lparam);
        // Center on the owning AE window, never on a foreign application's window.
        RECT own{}, box{};
        if (GetWindowRect(GetParent(window), &own) && GetWindowRect(window, &box)) {
            SetWindowPos(window, nullptr,
                own.left + ((own.right-own.left)-(box.right-box.left))/2,
                own.top + ((own.bottom-own.top)-(box.bottom-box.top))/2,
                0, 0, SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
        }
        return TRUE;
    }
    if (message == WM_CLOSE) { EndDialog(window, IDOK); return TRUE; }
    if (message != WM_COMMAND || HIWORD(wparam) != BN_CLICKED) return FALSE;
    auto *context = reinterpret_cast<Context *>(GetWindowLongPtrW(window, DWLP_USER));
    switch (LOWORD(wparam)) {
        case IDOK: case IDCANCEL: EndDialog(window, IDOK); return TRUE;
        case 101:
            if (context) MessageBoxW(window, context->about.c_str(), L"FSTR Stretch", MB_OK);
            return TRUE;
        case 102: {
            // Explicit user click only; fixed URL contains no project/license data.
            const HRESULT com = CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED);
            auto result = ShellExecuteW(window, L"open",
                L"https://github.com/ios3kov/ElasticGridFX/issues", nullptr, nullptr, SW_SHOWNORMAL);
            if (SUCCEEDED(com)) CoUninitialize();
            if (reinterpret_cast<INT_PTR>(result) <= 32) {
                MessageBoxW(window,
                    L"Visit github.com/ios3kov/ElasticGridFX/issues in your browser.",
                    L"Could not open support", MB_OK | MB_ICONWARNING);
            }
            return TRUE;
        }
        default: return FALSE;
    }
}
}

extern "C" bool eg_show_license_window(const char *version) noexcept {
    try {
        if (!version) return false;
        // GetActiveWindow is thread-local. Refuse render workers/no owned UI.
        HWND owner = GetActiveWindow();
        DWORD process = 0;
        if (!owner || GetWindowThreadProcessId(owner, &process) != GetCurrentThreadId() ||
            process != GetCurrentProcessId()) return false;
        std::wstring wideVersion;
        for (std::size_t i=0; i<32; ++i) {
            unsigned char c = static_cast<unsigned char>(version[i]);
            if (!c) break;
            if (c > 127 || i == 31) return false;
            wideVersion.push_back(static_cast<wchar_t>(c));
        }
        if (wideVersion.empty()) return false;
        Context context{L"Version " + wideVersion +
            L"\r\n\r\nProfessional mesh deformation for Adobe After Effects"
            L"\r\n\r\n\u00a9 2026 FSTR.tech. All rights reserved"};
        auto dialog = elasticgrid::license_ui::makeDialog();
        static_assert(sizeof(WORD) == sizeof(std::uint16_t));
        // std::allocator storage is suitably aligned; each child record is DWORD aligned.
        auto result = DialogBoxIndirectParamW(GetModuleHandleW(nullptr),
            reinterpret_cast<const DLGTEMPLATE *>(dialog.words.data()), owner,
            dialogProc, reinterpret_cast<LPARAM>(&context));
        return result == IDOK;
    } catch (...) {
        // Never unwind a C++ exception across the Rust/C boundary.
        return false;
    }
}

// Called from supervised Point/UI callbacks only, never from render workers.
extern "C" bool eg_loupe_button_down() {
    DWORD owner = 0;
    const HWND foreground = GetForegroundWindow();
    if (!foreground || !GetWindowThreadProcessId(foreground, &owner) ||
        owner != GetCurrentProcessId()) return false;
    return (GetAsyncKeyState(VK_LBUTTON) & 0x8000) != 0;
}

// No ShowCursor counter: transparent image applies only to this UI cursor.
extern "C" bool eg_set_loupe_cursor(bool hidden) {
    if (!hidden) { SetCursor(LoadCursorW(nullptr, MAKEINTRESOURCEW(32512))); return true; }
    DWORD owner = 0;
    const HWND foreground = GetForegroundWindow();
    if (!foreground || GetWindowThreadProcessId(foreground, &owner) != GetCurrentThreadId() ||
        owner != GetCurrentProcessId()) return false;
    static HCURSOR blank = []() {
        BYTE mask[128]; BYTE pixels[128] = {};
        for (auto &b : mask) b = 0xff;
        return CreateCursor(GetModuleHandleW(nullptr), 0, 0, 32, 32, mask, pixels);
    }();
    if (!blank) return false;
    SetCursor(blank);
    return true;
}
