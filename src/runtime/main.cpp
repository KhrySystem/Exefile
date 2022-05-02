#include <dragon/dragon10.hpp>

int main() {
    if(dgInit() != DG_TRUE) {
        printf("Dragon_Init unsuccessful. Exiting.\n");
        return 1;
    }

    Dragon::dgWindow* window = new Dragon::dgWindow(800, 600, "Technomancy", false, false);

    window->init();

    while(window->isAlive()) {

    }
    delete window;
    printf("Dragon_Init successful. Exiting.\n");
    
}