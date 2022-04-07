#ifndef VULKAN_MANAGER_HPP
#define VULKAN_MANAGER_HPP

#ifndef GLFW_INCLUDE_VULKAN
    #define GLFW_INCLUDE_VULKAN
#endif

#include <GLFW/glfw3.h>
#include <vector>
#include <iostream>

namespace Dragon {
    class VulkanManager {
        public:
            VulkanManager();

            bool createDebugInfo();
            bool setupDebug();

            ~VulkanManager();

        private:
            VkInstance instance;
    };
};

#endif