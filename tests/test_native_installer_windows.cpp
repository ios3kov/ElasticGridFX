#include "NativeInstaller.h"
#include "Payload.h"
#include <cassert>
#include <fstream>
#include <iostream>
#include <stdexcept>
namespace fs=std::filesystem;
using namespace fstr::installer;
template<class F> void refuses(F f){bool failed=false;try{f();}catch(const std::exception& e){failed=true;std::cout<<"Refused: "<<e.what()<<'\n';}assert(failed);}
void previous(const fs::path& p){fs::create_directories(p.parent_path());std::ofstream f(p,std::ios::binary);f<<"MZ com.elasticgrid.fx.warp ElasticGridBuildID=EGFX-222222222222222222222222";}
std::string bytes(const fs::path& p){std::ifstream f(p,std::ios::binary);return {std::istreambuf_iterator<char>(f),std::istreambuf_iterator<char>()};}
int fixtureMain(){
    std::cout<<std::unitbuf;
    wchar_t temp[MAX_PATH];assert(GetTempPathW(MAX_PATH,temp));GUID id{};assert(CoCreateGuid(&id)==S_OK);wchar_t guid[40];assert(StringFromGUID2(id,guid,40));fs::path root=fs::path(temp)/(std::wstring(L"egfx-native-installer-")+guid);fs::create_directory(root);
    auto make=[&](const wchar_t* name){std::wcout<<L"Fixture: "<<name<<std::endl;auto p=root/name;fs::create_directories(p/L"scan");return InstallEnvironment{p/L"scan/FSTR FX",p/L"Backups",{p/L"scan"},[]{}};};
    {
        auto e=make(L"fresh");assert(runNative(e,false).starts_with(L"Installed."));auto before=bytes(e.active/L"FSTR Stretch.aex");assert(runNative(e,false)==L"This version is already installed.");assert(runNative(e,true).starts_with(L"No previous version"));assert(bytes(e.active/L"FSTR Stretch.aex")==before);
    }
    {
        auto e=make(L"update");previous(e.active/L"FSTR Stretch.aex");auto before=bytes(e.active/L"FSTR Stretch.aex");assert(runNative(e,false).starts_with(L"Installed."));assert(bytes(e.active/L"FSTR Stretch.aex")!=before);assert(runNative(e,true).starts_with(L"Previous version restored"));assert(bytes(e.active/L"FSTR Stretch.aex")==before);assert(runNative(e,false).starts_with(L"Installed."));std::ofstream(e.active/L"FSTR Stretch.aex",std::ios::app)<<"tamper";refuses([&]{runNative(e,true);});
    }
    {
        auto e=make(L"interrupted-post-publication");previous(e.active/L"FSTR Stretch.aex");auto before=bytes(e.active/L"FSTR Stretch.aex");assert(runNative(e,false).starts_with(L"Installed."));auto tx=fs::directory_iterator(e.backups)->path();assert(fs::remove(tx/L"installed"));refuses([&]{runNative(e,false);});assert(runNative(e,true).starts_with(L"Previous version restored"));assert(bytes(e.active/L"FSTR Stretch.aex")==before);
    }
    {
        auto e=make(L"host");e.checkHosts=[]{throw std::runtime_error("running host");};refuses([&]{runNative(e,false);});assert(!fs::exists(e.active));
    }
    {
        auto e=make(L"duplicate");previous(e.scanRoots[0]/L"other.aex");refuses([&]{runNative(e,false);});assert(!fs::exists(e.active/L"FSTR Stretch.aex"));
    }
    {
        auto e=make(L"pending");fs::create_directories(e.backups/L"interrupted");refuses([&]{runNative(e,false);});assert(!fs::exists(e.active/L"FSTR Stretch.aex"));
    }
    {
        auto e=make(L"lock");assert(runNative(e,false).starts_with(L"Installed."));auto h=CreateFileW((e.active/L".fstr-install-lock-v1").c_str(),GENERIC_READ,0,nullptr,OPEN_EXISTING,0,nullptr);assert(h!=INVALID_HANDLE_VALUE);refuses([&]{runNative(e,false);});CloseHandle(h);
    }
    {
        auto e=make(L"reparse");auto outside=root/L"outside";fs::create_directory(outside);
        auto link=CreateSymbolicLinkW(e.active.c_str(),outside.c_str(),SYMBOLIC_LINK_FLAG_DIRECTORY|SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE);
        if(link){refuses([&]{runNative(e,false);});assert(fs::is_empty(outside));}
        else std::cout<<"Reparse fixture NOT RUN: symlink privilege unavailable\n";
    }
    std::cout<<"Native Windows installer fixture checks PASS; retained at "<<root<<'\n';return 0;
}
int main(){try{return fixtureMain();}catch(const std::exception& e){std::cerr<<"Native fixture FAIL: "<<e.what()<<std::endl;return 1;}}
