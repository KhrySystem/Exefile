cmake_minimum_required(VERSION 3.22)

set(POLICY CMP0002 NEW)

set(CMAKE_Cxx_CREATE_STATIC_LIBRARY true)

project(TESTEXE VERSION 1.0.0)

include_directories(dragon/include/dragon)

add_subdirectory(dragon)

add_executable(${PROJECT_NAME} main.cpp)
