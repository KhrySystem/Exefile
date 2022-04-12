#include "Dragon.hpp"

using namespace Dragon;

#ifdef NDEBUG
    const bool enableValidationLayers = false;
#else
    const bool enableValidationLayers = true;
#endif

int main() {
    DgInstance* instance = new DgInstance();

    instance->wm->createWindow();

    while(instance->update()) {
        
    }

    instance->wm->closeWindow();

    delete instance;
}