#ifndef INIT_H
#define INIT_H

#include <dragon/lib.hpp>

DGAPI void dgTerminate() {
    glfwTerminate();
}

DGAPI int dgInit() {
    if(!glfwInit()) {
        dgTerminate();
        return DG_FALSE;
    }

    return DG_TRUE;
	
}

#endif