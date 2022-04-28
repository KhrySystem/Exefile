cmake_minimum_required(VERSION 3.22)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_C_STANDARD 23)

project(Dragon)

include(findVulkan)

add_subdirectory(lib/glfw)
add_subdirectory(lib/glm)

include_directories(include)
include_directories(src)

include_directories(lib/glfw/include)
include_directories(lib/glm/glm)
include_directories(${Vulkan_INCLUDE_DIR})
include_directories(${Vulkan_LIBRARY})

link_libraries(${Vulkan_LIBRARIES})

set(
        INCLUDE_FILES
        include/dragon/dragon10.hpp
        include/dragon/lib.hpp
)

set(
        SRC_FILES
        src/init.hpp
        src/window.hpp
)

set(
        FILES 
        ${INCLUDE_FILES}
        ${SRC_FILES}
)

add_library(${PROJECT_NAME} STATIC ${FILES})

set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE Cxx) 