#ifndef WINDOW_MANAGER_HPP
#define WINDOW_MANAGER_HPP

#include <lib.hpp>

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