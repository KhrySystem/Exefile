#include "WindowManager.hpp"

Dragon::WindowManager::WindowManager() {
    glfwInit(); 
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);
}

bool Dragon::WindowManager::createWindow(int width, int height) {
    window = glfwCreateWindow(width, height, "Technomancy", nullptr, nullptr);

    if(window != GLFW_FALSE) {
        return true;
    }

    return false;
}

bool Dragon::WindowManager::update() {
    if(glfwWindowShouldClose(window)) {
        return false;
    }

    glfwPollEvents();

    return true;
}

double Dragon::WindowManager::getTime() {
    return time;
}

void Dragon::WindowManager::closeWindow() {
    glfwDestroyWindow(window);
}

Dragon::WindowManager::~WindowManager() {
    closeWindow();
    glfwTerminate();
}