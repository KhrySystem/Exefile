#include "InputManager.hpp"

Dragon::InputManager::InputManager() {

}

void Dragon::InputManager::update() {
    glfwPollEvents();
}