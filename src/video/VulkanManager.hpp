#ifndef VULKAN_MANAGER_HPP
#define VULKAN_MANAGER_HPP

#include <lib.hpp>
#include <VkInstanceRefrence.hpp>

#include "GPUManager.hpp"

namespace Dragon {
    class VulkanManager {
        public:
            VulkanManager();

            bool createDebugInfo();
            bool setupDebug();

            ~VulkanManager();

            GPUManager* gm = new GPUManager();

    };
};

#endif