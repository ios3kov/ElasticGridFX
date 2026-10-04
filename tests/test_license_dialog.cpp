#include "bridge/license_dialog_template.h"
#include <cassert>
#include <string>

int main() {
    auto d = elasticgrid::license_ui::makeDialog();
    std::size_t offset = 0;
    auto word = [&]() { assert(offset < d.words.size()); return d.words[offset++]; };
    auto dword = [&]() { const auto low = word(); return std::uint32_t(low) | (std::uint32_t(word()) << 16); };
    auto text = [&]() { std::u16string s; for (auto c=word(); c; c=word()) s.push_back(static_cast<char16_t>(c)); return s; };
    const auto style=dword(); assert(style & 0x40); // DS_SETFONT
    assert(dword()==0); const auto count=word(); assert(count==4);
    word(); word(); const auto width=word(), height=word();
    assert(word()==0 && word()==0); assert(!text().empty());
    assert(word()>=8); assert(!text().empty());
    bool close=false, about=false, support=false, body=false;
    for (unsigned i=0; i<count; ++i) {
        if (offset%2) assert(word()==0);
        assert((offset*2)%4==0); // DLGITEMTEMPLATE DWORD alignment
        const auto controlStyle=dword(); assert(controlStyle & 0x40000000);
        assert(dword()==0);
        const auto x=word(),y=word(),w=word(),h=word(),id=word();
        assert(x+w<=width && y+h<=height && w>0 && h>0);
        assert(word()==0xffff); const auto klass=word(); const auto caption=text();
        assert(word()==0); // no creation data
        if (klass==0x82) { body=true; assert(caption.find(u"without activation")!=std::u16string::npos); }
        else {
            assert(klass==0x80 && (controlStyle & 0x10000)); // keyboard navigation
            if (id==1) { close=true; assert((controlStyle&0xf)==1 && caption==u"Close"); }
            else if (id==101) { about=true; assert(caption==u"About"); }
            else if (id==102) { support=true; assert(caption==u"Support"); }
            else assert(false);
        }
    }
    assert(offset==d.words.size() && close && about && support && body);
}
