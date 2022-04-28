cmake_minimum_required(VERSION 3.22)

set(POLICY CMP0002 NEW)

set(CMAKE_Cxx_CREATE_STATIC_LIBRARY true)

project(TESTEXE VERSION 1.0.0)

include_directories(dragon/include)
include_directories(dragon/lib/glm)
include_directories(dragon/lib/glfw/include)
include_directories(dragon/src)

add_subdirectory(dragon)

link_libraries(dragon)

add_executable(${PROJECT_NAME} runtime/main.cpp)
