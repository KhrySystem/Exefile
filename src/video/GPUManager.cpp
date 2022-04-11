#include "GPUManager.hpp"

Dragon::GPUManager::GPUManager() {
    VkPhysicalDevice device = VK_NULL_HANDLE;

    uint32_t deviceCount = 0;
    vkEnumeratePhysicalDevices(Dragon::instance, &deviceCount, nullptr);

    if (deviceCount == 0) 
        std::cerr << "Potential RuntimeError due to no GPU detected" << std::endl;

    devices.resize(deviceCount);

    vkEnumeratePhysicalDevices(instance, &deviceCount, devices.data());
}

VkPhysicalDevice* Dragon::GPUManager::getDevices() {
    return devices.data();
}

int* Dragon::GPUManager::getScores() {
    return scores.data();
}