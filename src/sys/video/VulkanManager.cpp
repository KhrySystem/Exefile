#include "VulkanManager.hpp"

using namespace Dragon;

Dragon::VulkanManager::VulkanManager() {
     VkApplicationInfo appInfo{};
    appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    appInfo.pApplicationName = "Hello Triangle";
    appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.pEngineName = "No Engine";
    appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.apiVersion = VK_API_VERSION_1_0;

    VkInstanceCreateInfo createInfo{};
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pApplicationInfo = &appInfo;

    uint32_t glfwExtensionCount = 0;
    const char** glfwExtensions;
    glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);

    createInfo.enabledExtensionCount = glfwExtensionCount;
    createInfo.ppEnabledExtensionNames = glfwExtensions;

    createInfo.enabledLayerCount = 0;

    switch(vkCreateInstance(&createInfo, nullptr, &instance)) {
        
        case VK_ERROR_OUT_OF_HOST_MEMORY:
            printf("VkCreateInstance failed due to VK_ERROR_OUT_OF_HOST_MEMORY");
            break;
        case VK_ERROR_OUT_OF_DEVICE_MEMORY:
            printf("VkCreateInstance failed due to VK_ERROR_OUT_OF_DEVICE_MEMORY");
            break;
        case VK_ERROR_INITIALIZATION_FAILED:
            printf("VkCreateInstance failed due to VK_ERROR_INITIALIZATION_FAILED");
            break; 
        case VK_ERROR_LAYER_NOT_PRESENT:
            printf("VkCreateInstance failed due to VK_ERROR_LAYER_NOT_PRESENT");
            break; 
        case VK_ERROR_EXTENSION_NOT_PRESENT:
            printf("VkCreateInstance failed due to VK_ERROR_EXTENSION_NOT_PRESENT");
            break; 
        case VK_ERROR_INCOMPATIBLE_DRIVER:
            printf("VkCreateInstance failed due to VK_ERROR_INCOMPATIBLE_DRIVER");
            break; 
        default:
            printf("VkCreateInstance VK_SUCCESS");
            break;
    }
    
    printf("\n");

    if(gm->getBestDevice() == VK_NULL_HANDLE)
        printf("No GPU handles found.");
    else 
        printf("GPU Detection successful with score %s", gm->getBestScore());
    printf("\n");
}

Dragon::VulkanManager::~VulkanManager() {
    vkDestroyInstance(instance, nullptr);
    delete gm;
}