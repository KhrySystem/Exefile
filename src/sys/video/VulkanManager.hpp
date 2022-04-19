#ifndef VULKAN_MANAGER_HPP
#define VULKAN_MANAGER_HPP

#include <lib.hpp>

#include "GPUManager.hpp"

namespace Dragon {
    class VulkanManager {
        public:
            VulkanManager();

            bool createDebugInfo();
            bool setupDebug();

            ~VulkanManager();

            static inline VkInstance getInstance() {
                return instance;
            }

            GPUManager* gm = new GPUManager();

            static inline VkInstance instance = VK_NULL_HANDLE;

    };
};

#endif