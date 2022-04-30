cmake_minimum_required(VERSION 3.22)

project(Dragon)

add_subdirectory(lib/glfw)
add_subdirectory(lib/glm)
add_subdirectory(lib/SPIRV-Headers)
add_subdirectory(lib/SPIRV-Tools)
add_subdirectory(lib/Vulkan-Headers)
add_subdirectory(lib/Vulkan-Loader)
add_subdirectory(lib/robin-hood-hashing)
add_subdirectory(lib/Vulkan-ValidationLayers)

set(
    FILES
    include/dragon/lib.hpp
    include/dragon/dragon10.hpp
    src/window.hpp
    src/init.hpp
)

add_library(${PROJECT_NAME} STATIC ${FILES})

set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE Cxx)