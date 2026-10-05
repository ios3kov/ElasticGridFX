#pragma once
#include <filesystem>
#include <functional>
#include <stdexcept>
#include <string>
#include <vector>
namespace fstr::installer {
// Trusted native configuration. Shipping UI exposes actions only, not roots or
// verifier switches. Disposable test executables supply their own configuration.
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
std::string runNative(const InstallEnvironment&, bool restore);
}
