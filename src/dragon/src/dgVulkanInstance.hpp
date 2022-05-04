#ifndef DG_VULKAN_INSTANCE_HPP
#define DG_VULKAN_INSTANCE_HPP

#include <dragon/lib.hpp>
#include "dgVulkanValidationLayers.hpp"

DGAPI namespace Dragon {
	class dgVulkanInstance {
		public:
			dgVulkanInstance();

			bool checkValidationLayerSupport();

			Dragon::dgVulkanValidationLayer* validationLayers;
			
			~dgVulkanInstance();


		private:
			VkApplicationInfo appInfo{};
			VkInstanceCreateInfo createInfo{};
			uint32_t glfwExtensionCount, extensionCount;
			const char** glfwExtensions;
			std::vector<VkExtensionProperties> extensions;
			std::vector<const char*> extensionNames;
			VkInstance instance;
	};
};

#endif