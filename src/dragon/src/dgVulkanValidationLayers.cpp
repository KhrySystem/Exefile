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
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT successful.\n");
                #endif
                break;
            case VK_NOT_READY:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_NOT_READY\n");
                #endif
                break;
            case VK_TIMEOUT:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_TIMEOUT\n");
                #endif
                break;
            case VK_EVENT_SET:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_EVENT_SET\n");
                #endif
                break;
            case VK_EVENT_RESET:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_EVENT_RESET\n");
                #endif
                break;
            case VK_INCOMPLETE:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_INCOMPLETE\n");
                #endif
                break;
            case VK_SUBOPTIMAL_KHR:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_SUBOPTIMAL_KHR\n");
                #endif
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
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT improperly returned VK_PIPELINE_COMPILE_REQUIRED\n");
                #endif
                break;
            case VK_ERROR_OUT_OF_HOST_MEMORY:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_OUT_OF_HOST_MEMORY\n");
                #endif
                break;
            case VK_ERROR_OUT_OF_DEVICE_MEMORY:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_OUT_OF_DEVICE_MEMORY\n");
                #endif
                break;
            case VK_ERROR_INITIALIZATION_FAILED:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INITIALIZATION_FAILED\n");
                #endif
                break;
            case VK_ERROR_DEVICE_LOST:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_DEVICE_LOST\n");
                #endif
                break;
            case VK_ERROR_MEMORY_MAP_FAILED:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_MEMORY_MAP_FAILED\n");
                #endif
                break;
            case VK_ERROR_LAYER_NOT_PRESENT:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_LAYER_NOT_PRESENT\n");
                #endif
                break;
            case VK_ERROR_EXTENSION_NOT_PRESENT:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_EXTENSION_NOT_PRESENT\n");
                #endif
                break;
            case VK_ERROR_FEATURE_NOT_PRESENT:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FEATURE_NOT_PRESENT\n");
                #endif
                break;
            case VK_ERROR_INCOMPATIBLE_DRIVER:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INCOMPATIBLE_DRIVER\n");
                #endif
                break;
            case VK_ERROR_TOO_MANY_OBJECTS:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_TOO_MANY_OBJECTS\n");
                #endif
                break;
            case VK_ERROR_FORMAT_NOT_SUPPORTED:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FORMAT_NOT_SUPPORTED\n");
                #endif
                break;
            case VK_ERROR_FRAGMENTED_POOL:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FRAGMENTED_POOL\n");
                #endif
                break;
            case VK_ERROR_SURFACE_LOST_KHR:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_SURFACE_LOST_KHR\n");
                #endif
                break;
            case VK_ERROR_NATIVE_WINDOW_IN_USE_KHR:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_NATIVE_WINDOW_IN_USE_KHR\n");
                #endif
                break;
            case VK_ERROR_OUT_OF_DATE_KHR:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_OUT_OF_DATE_KHR\n");
                #endif
                break;
            case VK_ERROR_INCOMPATIBLE_DISPLAY_KHR:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INCOMPATIBLE_DISPLAY_KHR\n");
                #endif
                break;
            case VK_ERROR_INVALID_SHADER_NV:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INVALID_SHADER_NV\n");
                #endif
                break;
            case VK_ERROR_OUT_OF_POOL_MEMORY:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_OUT_OF_POOL_MEMORY\n");
                #endif
                break;
            case VK_ERROR_INVALID_EXTERNAL_HANDLE:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INVALID_EXTERNAL_HANDLE\n");
                #endif
                break;
            case VK_ERROR_FRAGMENTATION:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FRAGMENTATION\n");
                #endif
                break;
            case VK_ERROR_INVALID_DEVICE_ADDRESS_EXT:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_INVALID_DEVICE_ADDRESS_EXT\n");
                #endif
                break;
            case VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT failed due to VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT\n");
                #endif
                break;
            default:
                #ifdef DEBUG_ENABLED
                printf("CreateDebugUtilsMessengerEXT returned an invalid response.\n");
                #endif
                break;
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
    #ifdef DEBUG_ENABLED
    printf("Validation Layer: %s", pCallbackData->pMessage);
    #endif
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