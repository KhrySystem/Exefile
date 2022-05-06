#include "dgVulkanValidationLayers.hpp"

Dragon::dgVulkanValidationLayer::dgVulkanValidationLayer() {
    #ifdef DEBUG
        printf("dgValidationDebug enabled.\n");
    #else
        printf("dgValidationDebug diaabled\n");
    #endif
}

void Dragon::dgVulkanValidationLayer::createDebugMessenger(VkInstance* instance) {
    this->pInstance = instance;
    this->populateDebugMessengerCreateInfo();

    switch(this->CreateDebugUtilsMessengerEXT(&this->createInfo, nullptr, &this->debugMessenger)) {
            case VK_SUCCESS:
                printf("CreateDebugUtilsMessengerEXT successful.\n");
                break;
            case VK_NOT_READY:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_NOT_READY\n");
                break;
            case VK_TIMEOUT:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_TIMEOUT\n");
                break;
            case VK_EVENT_SET:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_EVENT_SET\n");
                break;
            case VK_EVENT_RESET:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_EVENT_RESET\n");
                break;
            case VK_INCOMPLETE:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_INCOMPLETE\n");
                break;
            case VK_SUBOPTIMAL_KHR:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_SUBOPTIMAL_KHR\n");
                break;
            case VK_THREAD_IDLE_KHR:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_THREAD_IDLE_KHR\n");
                break;
            case VK_THREAD_DONE_KHR:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_THREAD_DONE_KHR\n");
                break;
            case VK_OPERATION_DEFERRED_KHR:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_OPERATION_DEFERRED_KHR\n");
                break;
            case VK_OPERATION_NOT_DEFERRED_KHR:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_OPERATION_NOT_DEFERRED_KHR\n");
                break;
            case VK_PIPELINE_COMPILE_REQUIRED:
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_PIPELINE_COMPILE_REQUIRED\n");
                break;
            case VK_ERROR_OUT_OF_HOST_MEMORY:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_OUT_OF_HOST_MEMORY\n");
                break;
            case VK_ERROR_OUT_OF_DEVICE_MEMORY:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_OUT_OF_DEVICE_MEMORY\n");
                break;
            case VK_ERROR_INITIALIZATION_FAILED:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INITIALIZATION_FAILED\n");
                break;
            case VK_ERROR_DEVICE_LOST:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_DEVICE_LOST\n");
                break;
            case VK_ERROR_MEMORY_MAP_FAILED:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_MEMORY_MAP_FAILED\n");
                break;
            case VK_ERROR_LAYER_NOT_PRESENT:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_LAYER_NOT_PRESENT\n");
                break;
            case VK_ERROR_EXTENSION_NOT_PRESENT:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_EXTENSION_NOT_PRESENT\n");
                break;
            case VK_ERROR_FEATURE_NOT_PRESENT:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FEATURE_NOT_PRESENT\n");
                break;
            case VK_ERROR_INCOMPATIBLE_DRIVER:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INCOMPATIBLE_DRIVER\n");
                break;
            case VK_ERROR_TOO_MANY_OBJECTS:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_TOO_MANY_OBJECTS\n");
                break;
            case VK_ERROR_FORMAT_NOT_SUPPORTED:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FORMAT_NOT_SUPPORTED\n");
                break;
            case VK_ERROR_FRAGMENTED_POOL:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FRAGMENTED_POOL\n");
                break;
            case VK_ERROR_SURFACE_LOST_KHR:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_SURFACE_LOST_KHR\n");
                break;
            case VK_ERROR_NATIVE_WINDOW_IN_USE_KHR:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_NATIVE_WINDOW_IN_USE_KHR\n");
                break;
            case VK_ERROR_OUT_OF_DATE_KHR:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_OUT_OF_DATE_KHR\n");
                break;
            case VK_ERROR_INCOMPATIBLE_DISPLAY_KHR:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INCOMPATIBLE_DISPLAY_KHR\n");
                break;
            case VK_ERROR_INVALID_SHADER_NV:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INVALID_SHADER_NV\n");
                break;
            case VK_ERROR_OUT_OF_POOL_MEMORY:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_OUT_OF_POOL_MEMORY\n");
                break;
            case VK_ERROR_INVALID_EXTERNAL_HANDLE:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INVALID_EXTERNAL_HANDLE\n");
                break;
            case VK_ERROR_FRAGMENTATION:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FRAGMENTATION\n");
                break;
            case VK_ERROR_INVALID_DEVICE_ADDRESS_EXT:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INVALID_DEVICE_ADDRESS_EXT\n");
                break;
            case VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT:
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT\n");
                break;
            default:
                printf("CreateDebugUtilsMessengerEXT returned an invalid response.\n");
        }
}

void Dragon::dgVulkanValidationLayer::populateDebugMessengerCreateInfo() {
    this->createInfo = {};
	this->createInfo.sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
	this->createInfo.messageSeverity = VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT;
	this->createInfo.messageType = VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;
	this->createInfo.pfnUserCallback = this->debugCallback;
}

const char** Dragon::dgVulkanValidationLayer::getRequiredExtensions() {
    printf(" init");

    return this->validationLayers.data();
}

VkBool32 Dragon::dgVulkanValidationLayer::debugCallback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageType, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) {
    printf("Validation Layer: %s", pCallbackData->pMessage);
	return VK_FALSE;
}

VkResult Dragon::dgVulkanValidationLayer::CreateDebugUtilsMessengerEXT(const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pDebugMessenger) {
    auto func = (PFN_vkCreateDebugUtilsMessengerEXT) vkGetInstanceProcAddr(*this->pInstance, "vkCreateDebugUtilsMessengerEXT");
    if (func != nullptr) {
        return func(*this->pInstance, pCreateInfo, pAllocator, pDebugMessenger);
    } else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
	}
}

void Dragon::dgVulkanValidationLayer::destroyDebugUtilsMessengerEXT(VkDebugUtilsMessengerEXT dMessenger, const VkAllocationCallbacks* pAllocator) {
    auto func = (PFN_vkDestroyDebugUtilsMessengerEXT) vkGetInstanceProcAddr(*this->pInstance, "vkDestroyDebugUtilsMessengerEXT");
    if (func != nullptr) {
        func(*this->pInstance, dMessenger, pAllocator);
    }
}

Dragon::dgVulkanValidationLayer::~dgVulkanValidationLayer() {
    this->destroyDebugUtilsMessengerEXT(this->debugMessenger, nullptr);
}