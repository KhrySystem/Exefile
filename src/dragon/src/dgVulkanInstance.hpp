#ifndef DG_VULKAN_INSTANCE_HPP
#define DG_VULKAN_INSTANCE_HPP

#include <dragon/lib.hpp>

DGAPI namespace Dragon {
	class dgVulkanInstance {
		public:
			dgVulkanInstance();

			bool checkValidationLayerSupport();
			std::vector<const char*> getRequiredExtensions();
			void setupDebugMessenger();
			VkResult createDebugMessengerEXT(VkInstance instance, const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, 
				const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pDebugMessenger);

			VkResult CreateDebugUtilsMessengerEXT(VkInstance instance, const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, 
				const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pDebugMessenger); 

			static VKAPI_ATTR VkBool32 VKAPI_CALL debugCallback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, 
				VkDebugUtilsMessageTypeFlagsEXT messageType, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData);

			void populateDebugMessengerCreateInfo(VkDebugUtilsMessengerCreateInfoEXT& createInfo);

			~dgVulkanInstance();

			void destroyDebugUtilsMessengerEXT(VkInstance instance, VkDebugUtilsMessengerEXT debugMessenger, const VkAllocationCallbacks* pAllocator);

		private:
			VkApplicationInfo appInfo{};
			VkInstanceCreateInfo createInfo{};
			uint32_t glfwExtensionCount, extensionCount;
			const char** glfwExtensions;
			std::vector<VkExtensionProperties> extensions;
			std::vector<const char*> extensionNames;
			std::vector<const char*> validationLayers = {
				"VK_LAYER_KHRONOS_validation"
			};
			VkInstance instance;
			VkDebugUtilsMessengerEXT debugMessenger;
	};
};

#endif