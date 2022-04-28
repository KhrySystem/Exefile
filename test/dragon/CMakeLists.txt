cmake_minimum_required(VERSION 3.22)

project(Dragon)

find_package(Vulkan REQUIRED)
link_libraries(Vulkan::Vulkan)
link_libraries(Vulkan::Headers)

add_subdirectory(lib/glfw)

include_directories(include)
include_directories(src)

set(
        INCLUDE_FILES
        include/dragon/dragon10.h
        include/dragon/lib.h
)

set(
        SRC_FILES
        src/window.h
)

set(
        FILES 
        ${INCLUDE_FILES}
        ${SRC_FILES}
)

add_library(${PROJECT_NAME} ${FILES})

set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE Cxx) 