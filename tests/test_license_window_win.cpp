#include <cassert>
#include <thread>
extern "C" bool eg_show_license_window(const char *) noexcept;
int main() {
    assert(!eg_show_license_window(nullptr));
    bool shown=true;
    std::thread worker([&] { shown=eg_show_license_window("0.9.4"); });
    worker.join();
    assert(!shown); // no UI from a thread without an owned active AE window
}
