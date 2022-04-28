cmake_minimum_required(VERSION 3.22)

if(POLICY CMP0002)
    set(POLICY CMP0002 NEW)
endif()

project(TESTEXE VERSION 1.0.0)

add_subdirectory(dragon)

add_executable(${PROJECT_NAME} main.c)

include_directories(dragon/include)