cmake_minimum_required(VERSION 3.22)

set(DRAGON_BUILD_TYPE STATIC)
set(POLICY CMP0002 NEW)

project(Exefile VERSION 1.0.0)

include_directories(dragon/include)
include_directories(dragon/src)

include_directories(dragon/lib/Vulkan-Headers/include)

include_directories(dragon/lib/glfw/include)
include_directories(dragon/lib/glm)

add_subdirectory(dragon)

add_executable(${PROJECT_NAME} runtime/main.cpp)

target_link_libraries(${PROJECT_NAME} Dragon)