#include "dgWindow.hpp"

Dragon::dgWindow::dgWindow(int width, int height, std::string title, bool isFullscreen, bool isBorderless) {
    if(!glfwInit()) 
        printf("GLFW Initialization failed.\n");
    else
        printf("GLFW Initialization successful.\n");
    this->width = width;
    this->height = height;
    this->title = title;
    this->isFullscreen = isFullscreen;
    this->isBorderless = isBorderless;
    this->instance = new Dragon::dgVulkanInstance();

    int monitorcount;
    Dragon::monitors = glfwGetMonitors(&monitorcount);
    if(monitorcount == 0) 
        printf("Error with glfwGetMonitors\n");
    else
        printf("glfwGetMonitors successful\n");
}

bool Dragon::dgWindow::init() {
    if(this->isFullscreen && this->isBorderless)
        return false;
    else if(this->isFullscreen && !this->isBorderless)
        this->window = glfwCreateWindow(
            this->width, this->height,
            this->title.c_str(),
            Dragon::monitors[0],
            NULL
        );
    else if(!this->isFullscreen && this->isBorderless)
        return false;
    else if(!this->isFullscreen && !this->isBorderless)
        this->window = glfwCreateWindow(
            this->width, this->height,
            this->title.c_str(),
            NULL,
            NULL
        );
    else
        return false;

    if(this->window == NULL) 
        return false;
    return true;
}

bool Dragon::dgWindow::setBorderless(bool bordered) {
    this->isBorderless = bordered;
    return this->init();
}

bool Dragon::dgWindow::setFullscreen(bool fullscreen) {
    this->isFullscreen = fullscreen;
    return this->init();
}

void Dragon::dgWindow::closeWindow() {
    glfwSetWindowShouldClose(this->window, GLFW_TRUE);
}

int Dragon::dgWindow::isAlive() {
    return !glfwWindowShouldClose(this->window);
}

void Dragon::dgWindow::getEvents() {
	glfwPollEvents();
}

Dragon::dgWindow::~dgWindow() {
    delete this->instance;
    this->closeWindow();
    glfwTerminate();
}