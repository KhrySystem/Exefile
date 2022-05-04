#include "dgVulkanInstance.hpp"

Dragon::dgVulkanInstance::dgVulkanInstance() {
	this->appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
	this->appInfo.pApplicationName = "Hello Triangle";
    this->appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    this->appInfo.pEngineName = "No Engine";
    this->appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
    this->appInfo.apiVersion = VK_API_VERSION_1_0;

	this->createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
	this->createInfo.pApplicationInfo = &this->appInfo;

	this->glfwExtensions = glfwGetRequiredInstanceExtensions(&this->glfwExtensionCount);

	this->createInfo.enabledExtensionCount = this->glfwExtensionCount;
	this->createInfo.ppEnabledExtensionNames = this->glfwExtensions;

	this->createInfo.enabledLayerCount = 0;

	switch(vkCreateInstance(&this->createInfo, nullptr, &this->instance)) {
		case VK_SUCCESS:
			printf("VkCreateInstance successful.\n");
			break;
		case VK_ERROR_OUT_OF_HOST_MEMORY:
			printf("VkCreateInstance failed due to VK_ERROR_OUT_OF_HOST_MEMORY.\n");
			break;
		case VK_ERROR_OUT_OF_DEVICE_MEMORY:
			printf("VkCreateInstance failed due to VK_ERROR_OUT_OF_DEVICE_MEMORY.\n");
			break;
		case VK_ERROR_INITIALIZATION_FAILED:
			printf("VkCreateInstance failed due to VK_ERROR_INITIALIZATION_FAILED.\n");
			break;
		case VK_ERROR_LAYER_NOT_PRESENT:
			printf("VkCreateInstance failed due to VK_ERROR_LAYER_NOT_PRESENT.\n");
			break;
		case VK_ERROR_EXTENSION_NOT_PRESENT:
			printf("VkCreateInstance failed due to VK_ERROR_EXTENSION_NOT_PRESENT.\n");
			break;
		case VK_ERROR_INCOMPATIBLE_DRIVER:
			printf("VkCreateInstance failed due to VK_ERROR_INCOMPATIBLE_DRIVER.\n");
			break;
		default:
			printf("Unknown VkCreateInstance result.\n");
	}

	vkEnumerateInstanceExtensionProperties(nullptr, &this->extensionCount, nullptr);
	this->extensions.resize(this->extensionCount);
	vkEnumerateInstanceExtensionProperties(nullptr, &this->extensionCount, extensions.data());

	printf("Availible Extensions:\n");
	
	for(const auto &extension : this->extensions) 
		printf("\t%s-%d\n", extension.extensionName, extension.specVersion);

	this->setupDebugMessenger();
}

void Dragon::dgVulkanInstance::setupDebugMessenger() {
	VkDebugUtilsMessengerCreateInfoEXT cInfo;
	populateDebugMessengerCreateInfo(cInfo);

	if (this->CreateDebugUtilsMessengerEXT(this->instance, &cInfo, nullptr, &this->debugMessenger) != VK_SUCCESS) {
		throw std::runtime_error("failed to set up debug messenger!");
	}
}

VkResult Dragon::dgVulkanInstance::CreateDebugUtilsMessengerEXT(VkInstance i, const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pDebugMessenger) {
	auto func = (PFN_vkCreateDebugUtilsMessengerEXT) vkGetInstanceProcAddr(i, "vkCreateDebugUtilsMessengerEXT");
    if (func != nullptr) {
        return func(i, pCreateInfo, pAllocator, pDebugMessenger);
    } else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
	}
}

VkBool32 Dragon::dgVulkanInstance::debugCallback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageType, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) {
	printf("Validation Layer: %s", pCallbackData->pMessage);
	return VK_FALSE;
}

void Dragon::dgVulkanInstance::populateDebugMessengerCreateInfo(VkDebugUtilsMessengerCreateInfoEXT &cInfo) {
	cInfo = {};
	cInfo.sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
	cInfo.messageSeverity = VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT;
	cInfo.messageType = VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;
	cInfo.pfnUserCallback = this->debugCallback;
}

VkResult Dragon::dgVulkanInstance::createDebugMessengerEXT(VkInstance i, const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pDebugMessenger) {
	auto func = (PFN_vkCreateDebugUtilsMessengerEXT) vkGetInstanceProcAddr(i, "vkCreateDebugUtilsMessengerEXT");
    if (func != nullptr) {
        return func(i, pCreateInfo, pAllocator, pDebugMessenger);
    } else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    }
}

void Dragon::dgVulkanInstance::destroyDebugUtilsMessengerEXT(VkInstance i, VkDebugUtilsMessengerEXT dMessenger, const VkAllocationCallbacks* pAllocator) {
	auto func = (PFN_vkDestroyDebugUtilsMessengerEXT) vkGetInstanceProcAddr(i, "vkDestroyDebugUtilsMessengerEXT");
    if (func != nullptr) {
        func(i, dMessenger, pAllocator);
    }
}

Dragon::dgVulkanInstance::~dgVulkanInstance() {
	vkDestroyInstance(this->instance, nullptr);
	this->destroyDebugUtilsMessengerEXT(this->instance, this->debugMessenger, nullptr);
}