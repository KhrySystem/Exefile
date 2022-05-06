cmake_minimum_required(VERSION 3.22)

set(POLICY CMP0002 NEW)

set(CMAKE_Cxx_CREATE_STATIC_LIBRARY true)

project(Exefile VERSION 1.0.0)

include_directories(dragon/include)
include_directories(dragon/src)

include_directories(dragon/lib/Vulkan-Headers/include)

include_directories(dragon/lib/glfw/include)
include_directories(dragon/lib/glm)

add_subdirectory(dragon SHARED)

add_executable(${PROJECT_NAME} runtime/main.cpp)

target_link_libraries(${PROJECT_NAME} Dragon)