cmake_minimum_required(VERSION 3.22)

project(Dragon)

find_package(Vulkan REQUIRED)
link_libraries(Vulkan::Vulkan)
link_libraries(Vulkan::Headers)

add_subdirectory(lib/glfw)
add_subdirectory(lib/glm)

include_directories(include)
include_directories(src)

include_directories(lib/glfw/include)
include_directories(lib/glm)

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

add_library(${PROJECT_NAME} ${FILES})

set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE Cxx) 