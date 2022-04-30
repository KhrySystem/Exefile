#ifndef WINDOW_H
#define WINDOW_H

#include <dragon/lib.hpp>

DGAPI namespace Dragon {
    class dgWindow {
        public:
            dgWindow(int width, int height, std::string title, bool isFullscreen, bool isBorderless);

        private:
            int width, height;
            std::string title;
            bool isFullscreen, isBorderless;
    };
};

#endif