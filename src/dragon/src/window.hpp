#ifndef WINDOW_H
#define WINDOW_H

#include <dragon/lib.hpp>

DGAPI struct DRAGON_WINDOW {
    GLFWwindow* window;
    bool isFullscreen, isBordered;
    int width, height;
    char* title;
} dgWindow;

DGAPI void setDefaultGLFWWindowHints(dgWindow* window) {

}

DGAPI dgWindow createWindow() {
    dgWindow window;
    window.width = 800;
    window.height = 600;
    window.window = glfwCreateWindow(window.width, window.height, window.title, NULL, NULL);
}

bool isWindowFullscreen(dgWindow* window) {
    return window->isFullscreen;
}


#endif