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

	this->createInfo.enabledLayerCount = 1;

	this->createInfo.ppEnabledLayerNames = this->validationLayers->getRequiredExtensions();

	switch(vkCreateInstance(&this->createInfo, nullptr, &this->instance)) {
		case VK_SUCCESS:
			#ifdef DEBUG_ENABLED
			printf("VkCreateInstance successful.\n");
			#endif
			break;
		case VK_ERROR_OUT_OF_HOST_MEMORY:
			#ifdef DEBUG_ENABLED
			printf("VkCreateInstance failed due to VK_ERROR_OUT_OF_HOST_MEMORY.\n");
			#endif
			break;
		case VK_ERROR_OUT_OF_DEVICE_MEMORY:
			#ifdef DEBUG_ENABLED
			printf("VkCreateInstance failed due to VK_ERROR_OUT_OF_DEVICE_MEMORY.\n");
			#endif
			break;
		case VK_ERROR_INITIALIZATION_FAILED:
			#ifdef DEBUG_ENABLED
			printf("VkCreateInstance failed due to VK_ERROR_INITIALIZATION_FAILED.\n");
			#endif
			break;
		case VK_ERROR_LAYER_NOT_PRESENT:
			#ifdef DEBUG_ENABLED
			printf("VkCreateInstance failed due to VK_ERROR_LAYER_NOT_PRESENT.\n");
			#endif
			break;
		case VK_ERROR_EXTENSION_NOT_PRESENT:
			#ifdef DEBUG_ENABLED
			printf("VkCreateInstance failed due to VK_ERROR_EXTENSION_NOT_PRESENT.\n");
			#endif
			break;
		case VK_ERROR_INCOMPATIBLE_DRIVER:
			#ifdef DEBUG_ENABLED
			printf("VkCreateInstance failed due to VK_ERROR_INCOMPATIBLE_DRIVER.\n");
			#endif
			break;
		default:
			#ifdef DEBUG_ENABLED
			printf("Unknown VkCreateInstance result.\n");
			#endif
			break;
	}

	vkEnumerateInstanceExtensionProperties(nullptr, &this->extensionCount, nullptr);
	this->extensions.resize(this->extensionCount);
	vkEnumerateInstanceExtensionProperties(nullptr, &this->extensionCount, extensions.data());
	
	#ifdef DEBUG_ENABLED
	printf("Availible Extensions:\n");
	
	for(const auto &extension : this->extensions) 
		printf("\t%s-Version:%d\n", extension.extensionName, extension.specVersion);
	#endif

	this->validationLayers = new Dragon::dgVulkanValidationLayer();
	this->validationLayers->createDebugMessenger(&this->instance);
}

Dragon::dgVulkanInstance::~dgVulkanInstance() {
	vkDestroyInstance(this->instance, nullptr);
	delete this->validationLayers;
}