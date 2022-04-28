#ifndef INIT_H
#define INIT_H

#include <dragon/lib.h>

#define DRAGON_INIT DG_FALSE 

DGAPI void dgInit() {
    if(DRAGON_INIT == DG_TRUE) return true;
	
}

#endif