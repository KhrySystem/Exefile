#ifndef WINDOW_MANAGER_HPP
#define WINDOW_MANAGER_HPP

#include <lib.hpp>

namespace Dragon {
    class WindowManager {
        public:
            WindowManager();

            bool createWindow(int width, int height);
            bool update();

            double getTime();

            void closeWindow();

            ~WindowManager();

        private:
            GLFWwindow* window;

            double time;
    };
};

#endif