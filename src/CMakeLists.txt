cmake_minimum_required(VERSION 3.22)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_C_STANDARD 23)

set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)

project(Exefile)

find_package(Vulkan REQUIRED)
link_libraries(Vulkan::Vulkan)
link_libraries(Vulkan::Headers)

add_subdirectory(../lib/glfw ../bin/lib)

include_directories(../lib/glfw)
include_directories(../lib/glfw/include)
include_directories(../lib/glfw/src)

link_libraries(glfw)

include_directories(video)
include_directories(/)

set(
    VIDEO_SRC 
    video/WindowManager.cpp
    video/VulkanManager.cpp
    video/GPUManager.cpp
)

set(FILES ${VIDEO_SRC})

add_executable(${PROJECT_NAME} main.cpp ${FILES})

