#include "Dragon.hpp"

using namespace Dragon;

#ifdef NDEBUG
    const bool enableValidationLayers = false;
#else
    const bool enableValidationLayers = true;
#endif

int main() {
    WindowManager* wm = new WindowManager();
    VulkanManager* vm = new VulkanManager();
    wm->createWindow();

    while(wm->update()) {

    }

    delete wm;
    delete vm;
}