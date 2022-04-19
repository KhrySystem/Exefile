#ifndef LIB_HPP
    //#include <config.h>
    #define DEBUG 0
    #define LIB_HPP
    #ifndef GLFW_INCUDE_VULKAN
        #define GLFW_INCLUDE_VULKAN
    #endif
    #include <GLFW/glfw3.h>
    #if (DEBUG == 0)
        #include <stdio.h>
    #else
        inline _cdecl printf(const char* a) {
            return;
        }
    #endif
    #include <string>
    #include <vector>
    #include <iostream>
    #include <map>
    #include <stdexcept>
    #include <thread>
    #ifndef GLM_SWIZZLE_XYZW
        #define GLM_SWIZZLE_XYZW
    #endif
    #include <glm.hpp>
#endif