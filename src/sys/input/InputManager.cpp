#include "InputManager.hpp"

Dragon::InputManager::InputManager() {

}

void Dragon::InputManager::update() {
    glfwPollEvents();
}

bool Dragon::InputManager::mouseButtonPressed(int button) {
    return false;
}

bool Dragon::InputManager::keyPressed(int key) {
    return false;
}

bool Dragon::InputManager::buttonPressed(int button) {
    return false;
}

float Dragon::InputManager::joystickAxisDistance(int axis) {
    return 0.0f;
}

Dragon::InputManager::~InputManager() {

}