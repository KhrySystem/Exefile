#ifndef INPUT_MANAGER_HPP
#define INPUT_MANAGER_HPP

#include <lib.hpp>
#include "Keyboard.hpp"
#include "Mouse.hpp"
#include "XboxController.hpp"

namespace Dragon {
    class InputManager {
        public:
            InputManager();

            void update();

            bool mouseButtonPressed(int button);
            bool keyPressed(int key);
            bool buttonPressed(int button);
            float joystickAxisDistance(int joystick);

            ~InputManager();
        private:
            Dragon::Input::Keyboard* k;
            Dragon::Input::Mouse* m;
            Dragon::Input::XboxController* xc;
    };
};

#endif