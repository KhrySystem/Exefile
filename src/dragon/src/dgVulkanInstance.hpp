#ifndef DG_VULKAN_INSTANCE_HPP
#define DG_VULKAN_INSTANCE_HPP

#include <dragon/lib.hpp>

DGAPI namespace Dragon {
	class dgVulkanInstance {
		public:
			dgVulkanInstance();

			~dgVulkanInstance();
		private:
			VkApplicationInfo appInfo{};
			VkInstanceCreateInfo createInfo{};
			uint32_t glfwExtensionCount, extensionCount;
			const char** glfwExtensions;
			std::vector<VkExtensionProperties> extensions;
			VkInstance instance;
	};
};

#endif