cmake_minimum_required(VERSION 3.22)

project(Dragon)

set(DRAGON_BUILD_TYPE "${DRAGON_BUILD_TYPE}" CACHE STRING "Dragon Build <STATIC | SHARED | OBJECT> Library Type.")

set(GLFW_LIBRARY_TYPE ${DRAGON_BUILD_TYPE})

add_subdirectory(lib/glfw)
add_subdirectory(lib/glm)
add_subdirectory(lib/SPIRV-Headers)
add_subdirectory(lib/SPIRV-Tools)
add_subdirectory(lib/glslang)
add_subdirectory(lib/Vulkan-Headers)
add_subdirectory(lib/Vulkan-ExtensionLayer)
add_subdirectory(lib/Vulkan-Loader)
add_subdirectory(lib/robin-hood-hashing)
add_subdirectory(lib/Vulkan-ValidationLayers)

set(
    GENERAL_FILES
    include/dragon/lib.hpp
    include/dragon/dragon10.hpp
    src/dgWindow.hpp
    src/dgWindow.cpp
    src/dgVulkanInstance.hpp
    src/dgVulkanInstance.cpp
)

add_library(${PROJECT_NAME}
    ${DRAGON_BUILD_TYPE}
        ${GENERAL_FILES}
)

set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE Cxx)

install(
    TARGETS ${PROJECT_NAME}
        RUNTIME DESTINATION "${CMAKE_INSTALL_BINDIR}"
        ARCHIVE DESTINATION "${CMAKE_INSTALL_LIBDIR}"
        LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}"
)

target_include_directories(${PROJECT_NAME} INTERFACE lib/Vulkan-Headers/include)

target_link_libraries(${PROJECT_NAME} glfw)
target_link_libraries(${PROJECT_NAME} glm)
target_link_libraries(${PROJECT_NAME} SPIRV)
target_link_libraries(${PROJECT_NAME} SPVRemapper)
target_link_libraries(${PROJECT_NAME} OGLCompiler)
target_link_libraries(${PROJECT_NAME} HLSL)
target_link_libraries(${PROJECT_NAME} robin_hood)
target_link_libraries(${PROJECT_NAME} SPIRV-Headers)
target_link_libraries(${PROJECT_NAME} SPIRV-Tools)
target_link_libraries(${PROJECT_NAME} Vulkan::Headers)
target_link_libraries(${PROJECT_NAME} VkLayer_khronos_synchronization2)
target_link_libraries(${PROJECT_NAME} Vulkan::Vulkan)
target_link_libraries(${PROJECT_NAME} VkLayer_khronos_validation)