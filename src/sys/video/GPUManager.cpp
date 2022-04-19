#include "GPUManager.hpp"

Dragon::GPUManager::GPUManager() {
    VkPhysicalDevice device = VK_NULL_HANDLE;

    updateGPUList();
}

VkPhysicalDevice* Dragon::GPUManager::getDevices() {
    return devices.data();
}

void Dragon::GPUManager::updateGPUList() {
    uint32_t deviceCount = 0;
    vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);

    if (deviceCount == 0) 
        std::cerr << "Potential RuntimeError due to no GPU detected" << std::endl;

    devices.resize(deviceCount);

    vkEnumeratePhysicalDevices(instance, &deviceCount, devices.data());
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