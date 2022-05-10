#ifndef DG_VULKAN_VALIDATION_LAYER_HPP
#define DG_VULKAN_VALIDATION_LAYER_HPP

#include <dragon/lib.hpp>

namespace Dragon {
    class dgVulkanValidationLayer {
        public:
            dgVulkanValidationLayer();

            void createDebugMessenger(VkInstance* instance);

			const char** getRequiredExtensions();

			VkResult createDebugMessengerEXT(VkInstance instance, const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pDebugMessenger);
			VkResult CreateDebugUtilsMessengerEXT(const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pDebugMessenger); 

			static VKAPI_ATTR VkBool32 VKAPI_CALL debugCallback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageType, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData);

			void populateDebugMessengerCreateInfo();
			void destroyDebugUtilsMessengerEXT(VkDebugUtilsMessengerEXT debugMessenger, const VkAllocationCallbacks* pAllocator);

            ~dgVulkanValidationLayer();

        private:
            VkInstance* pInstance;
			VkDebugUtilsMessengerEXT debugMessenger;
			std::vector<const char*> validationLayers = {
				"VK_LAYER_KHRONOS_validation"
			};
            VkDebugUtilsMessengerCreateInfoEXT createInfo;
    };
};

#endif
