#ifndef WINDOW_H
#define WINDOW_H

#include <dragon/lib.hpp>

DGAPI namespace Dragon {
    class dgWindow {
        public:
            dgWindow(int width, int height, std::string title, bool isFullscreen, bool isBorderless);

            bool init();
            bool setBorderless(bool isBorderless);
            bool setFullscreen(bool isFullscreen);
            void closeWindow();
            int isAlive();

            ~dgWindow();

        private:
            int width, height;
            std::string title;
            bool isFullscreen, isBorderless;
            GLFWwindow* window;

            static GLFWmonitor** monitors;
    };
};

#endif