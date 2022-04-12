#ifndef DRAGON_HPP
#define DRAGON_HPP

#define ENABLE_DEBUG

#include <lib.hpp>
#include <WindowManager.hpp>
#include <VulkanManager.hpp>
#include <InputManager.hpp>

using namespace Dragon;

namespace Dragon {
    class DgInstance {
        public:
            DgInstance() {

            }

            bool update() {
                im->update();
                return wm->update();
            }
            
            ~DgInstance() {
                delete im;
                delete vm;
                delete wm;
            }

            InputManager* im = new InputManager;
            VulkanManager* vm = new VulkanManager;
            WindowManager* wm = new WindowManager;
    };
};

#endif