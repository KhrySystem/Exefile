#ifndef DRAGON_HPP
#define DRAGON_HPP

#define ENABLE_DEBUG

#include <lib.hpp>
#include <WindowManager.hpp>
#include <VulkanManager.hpp>

using namespace Dragon;

namespace Dragon {
    class DgInstance {
        public:
            DgInstance() {

            }
            
            ~DgInstance() {
                delete vm;
                delete wm;
            }

            Dragon::VulkanManager* vm = new VulkanManager;
            Dragon::WindowManager* wm = new WindowManager;
    };
};

#endif