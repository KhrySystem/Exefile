#include <dragon/dragon10.hpp>

int main() {
    dgInit();
    if(DRAGON_INIT != DG_TRUE) 
        printf("Dragon_Init unsuccessful. Exiting. ");
        return 1;

    printf("Dragon_Init successful. Exiting.");
    return 0;
    
	
}