#pragma once
#include <cstdint>
#include <string_view>
#include <vector>

namespace elasticgrid::license_ui {
// Win32 standard dialog template wire format, independent of host ABI packing.
class DialogTemplate {
public:
    std::vector<std::uint16_t> words;
    void word(std::uint16_t value) { words.push_back(value); }
    void dword(std::uint32_t value) {
        word(static_cast<std::uint16_t>(value & 0xffff));
        word(static_cast<std::uint16_t>(value >> 16));
    }
    void text(std::u16string_view value) {
        for (char16_t c : value) word(static_cast<std::uint16_t>(c));
        word(0);
    }
    void align() { if (words.size() % 2) word(0); }
    void control(std::uint32_t style, std::uint16_t x, std::uint16_t y,
                 std::uint16_t width, std::uint16_t height, std::uint16_t id,
                 std::uint16_t classOrdinal, std::u16string_view caption) {
        align(); dword(style); dword(0);
        word(x); word(y); word(width); word(height); word(id);
        word(0xffff); word(classOrdinal); text(caption); word(0);
    }
};

inline DialogTemplate makeDialog() {
    DialogTemplate d;
    // WS_POPUP | WS_CAPTION | WS_SYSMENU | DS_MODALFRAME | DS_SETFONT.
    d.dword(0x80c800c0); d.dword(0); d.word(4);
    d.word(0); d.word(0); d.word(300); d.word(135);
    d.word(0); d.word(0); d.text(u"FSTR Stretch - License");
    d.word(9); d.text(u"Segoe UI");
    d.control(0x50000000, 12, 12, 276, 82, 100, 0x82,
        u"Licensing is not connected yet. This development build works without activation.\r\n\r\nPlanned stores: aescripts and Plugin Play.");
    // Close is the default; all three controls participate in keyboard navigation.
    d.control(0x50010001, 12, 105, 80, 18, 1, 0x80, u"Close");
    d.control(0x50010000, 110, 105, 80, 18, 101, 0x80, u"About");
    d.control(0x50010000, 208, 105, 80, 18, 102, 0x80, u"Support");
    return d;
}
} // namespace elasticgrid::license_ui
