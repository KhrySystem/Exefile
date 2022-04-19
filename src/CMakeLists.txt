cmake_minimum_required(VERSION 3.22)

set(PRINT_DEBUG 0)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_C_STANDARD 23)

set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)

project(Exefile)

find_package(Vulkan REQUIRED)
link_libraries(Vulkan::Vulkan)
link_libraries(Vulkan::Headers)

add_subdirectory(../lib/glfw ../bin/lib/glfw)
add_subdirectory(../lib/glm ../bin/lib/glm)

include_directories(../lib/glfw)
include_directories(../lib/glfw/include)
include_directories(../lib/glfw/src)

include_directories(../lib/glm/glm)

link_libraries(glfw)

include_directories(sys)
include_directories(sys/input)
include_directories(sys/model)
include_directories(sys/video)
include_directories(sys/include)

include_directories(run)

set(
	SYS_VIDEO_SRC
	sys/video/GPUManager.cpp 
	sys/video/VulkanManager.cpp
	sys/video/WindowManager.cpp 
)

set(
	SYS_MODEL_SRC
)

set(
	SYS_INPUT_SRC
	sys/input/InputManager.cpp
)

set(FILES ${SYS_VIDEO_SRC} ${SYS_MODEL_SRC} ${SYS_INPUT_SRC})

add_executable(${PROJECT_NAME} main/main.cpp ${FILES})