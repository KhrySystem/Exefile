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

	#ifdef DEBUG
		this->validationLayers = new Dragon::dgVulkanValidationLayer();
		this->validationLayers->createDebugMessenger(&this->instance);
	#endif
}

Dragon::dgVulkanInstance::~dgVulkanInstance() {
	vkDestroyInstance(this->instance, nullptr);
	#ifdef DEBUG
		delete this->validationLayers;
	#endif
}