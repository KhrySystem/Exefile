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
    vkEnumeratePhysicalDevices(Dragon::instance, &deviceCount, nullptr);

    if (deviceCount == 0) 
        std::cerr << "Potential RuntimeError due to no GPU detected" << std::endl;

    devices.resize(deviceCount);

    vkEnumeratePhysicalDevices(Dragon::instance, &deviceCount, devices.data());
}

VkPhysicalDevice Dragon::GPUManager::getBestDevice() {
    updateGPUList();

    return devices.at(0);
}

int* Dragon::GPUManager::getScores() {
    return scores.data();
}

Dragon::GPUManager::~GPUManager() {
    std::cerr << "Potential Resource Issue: GPUManager deleted." << std::endl;
}