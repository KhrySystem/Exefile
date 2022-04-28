#ifndef DRAGON_REQUIRED_LIBS_H
    #define DRAGON_REQUIRED_LIBS_H

    // Comment out this line for Dragon to have no debug at all
    #define DEBUG 

    #if defined(_WIN32) && defined(_BUILD_DLL)
        /* We are building Dragon as a Win32 DLL */
        #define DGAPI __declspec(dllexport)
    #elif defined(_WIN32) && defined(IMPORT_DLL)
        /* We are calling Dragon as a Win32 DLL */
        #define DGAPI __declspec(dllimport)
    #elif defined(__GNUC__) && defined(_BUILD_DLL)
        /* We are building Dragon as a shared / dynamic library */
        #define DGAPI __attribute__((visibility("default")))
    #else
        /* We are building or calling Dragon as a static library */
        #define DGAPI
    #endif

    #define GLM_SWIZZLE_XYZW
    #include <glm/glm.hpp>
    
    #include <vulkan/vulkan.hpp>

    #include <string>
    #include <vector>
    #include <iostream>
    #include <map>
    #include <stdexcept>
    #include <thread>
    #include <stdio.h>

    #ifdef DEBUG
        void* dgPrintf = &printf;
    #else
        typedef dgPrintf nullptr_t;
    #endif
    
    #include <glfw/glfw3.h>

    #define DG_FALSE 0
    #define DG_TRUE 1
#endif