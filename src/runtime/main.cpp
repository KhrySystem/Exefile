#include "runtime_config.h"

#include <dragon/dragon10.hpp>

using namespace Dragon;

int main() {
    printf("Running\n");
    dgWindow* window = new dgWindow(800, 600, "Technomancy", false, false);

    window->init();

    while(window->isAlive()) {
        window->getEvents();
    }

    delete window;
}