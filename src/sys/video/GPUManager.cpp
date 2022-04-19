#include "GPUManager.hpp"
#include <VulkanManager.hpp>

Dragon::GPUManager::GPUManager() {
    VkPhysicalDevice device = VK_NULL_HANDLE;

    updateGPUList();
}

VkPhysicalDevice* Dragon::GPUManager::getDevices() {
    return devices.data();
}

void Dragon::GPUManager::updateGPUList() {
    uint32_t deviceCount = 0;
    vkEnumeratePhysicalDevices(VulkanManager::getInstance(), &deviceCount, nullptr);

    if (deviceCount == 0) 
        printf("Potential RuntimeError due to no GPU detected");

    devices.resize(deviceCount);

    vkEnumeratePhysicalDevices(VulkanManager::getInstance(), &deviceCount, devices.data());
}

VkPhysicalDevice Dragon::GPUManager::getBestDevice() {
    updateGPUList();

    for(VkPhysicalDevice d : devices) {
        VkPhysicalDeviceProperties properties;
        vkGetPhysicalDeviceProperties(d, &properties);

    }

    return devices.at(0);
}

int* Dragon::GPUManager::getScores() {
    return scores.data();
}

int Dragon::GPUManager::getBestScore() {
    return scores.at(0);
}

Dragon::GPUManager::~GPUManager() {
    std::cerr << "Potential Resource Issue: GPUManager deleted." << std::endl;
}