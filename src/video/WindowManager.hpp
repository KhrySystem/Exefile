#ifndef WINDOW_MANAGER_HPP
#define WINDOW_MANAGER_HPP

#include <GLFW/glfw3.h>

namespace Dragon {
    class WindowManager {
        public:
            WindowManager();

            bool createWindow();
            bool update();

            void closeWindow();

            ~WindowManager();

        private:
            GLFWwindow* window;
    };
};

#endif