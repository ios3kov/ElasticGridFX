#pragma once
#define WIN32_LEAN_AND_MEAN
#define NOMINMAX
#ifndef UNICODE
#define UNICODE
#endif
#ifndef _UNICODE
#define _UNICODE
#endif
#include <windows.h>
#include <objbase.h>
#include <filesystem>
#include <functional>
#include <stdexcept>
#include <string>
#include <vector>
namespace fstr::installer {
class HostsRunning final : public std::runtime_error {
public:
    HostsRunning():std::runtime_error("Close After Effects and other Adobe render applications, then click Continue."){}
};
struct InstallEnvironment {
    std::filesystem::path active, backups;
    std::vector<std::filesystem::path> scanRoots;
    std::function<void()> checkHosts;
};
InstallEnvironment systemEnvironment();
std::wstring runNative(const InstallEnvironment&,bool restore);
bool elevated();
}
