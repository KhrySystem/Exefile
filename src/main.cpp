#include "Dragon.hpp"

using namespace Dragon;

#ifdef NDEBUG
    const bool enableValidationLayers = false;
#else
    const bool enableValidationLayers = true;
#endif

int main() {
    DgInstance* instance = new DgInstance();

    while(instance->update()) {
        
    }

    delete instance;
}