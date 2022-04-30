#include <dragon/dragon10.hpp>

int main() {
    dgInit();
    if(DRAGON_INIT != DG_TRUE) 
        println("Dragon_Init unsuccessful. Exiting. ");
        return 1;

    println("Dragon_Init successful. Exiting.");
    return 0;
    
	
}
