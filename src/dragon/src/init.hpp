#ifndef INIT_H
#define INIT_H

#include <dragon/lib.hpp>

#define DRAGON_INIT DG_FALSE 
#define VULKAN_INIT DG_FALSE

DGAPI void initVulkan() {

}

DGAPI void dgInit() {
    if(DRAGON_INIT == DG_TRUE) return;

    #undef DRAGON_INIT
    #define DRAGON_INIT DG_TRUE
	
}

#endif