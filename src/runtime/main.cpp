#include <dragon/dragon10.hpp>

int main() {
    dgInit();
    if(DRAGON_INIT != DG_TRUE) 
        print("Dragon_Init unsuccessful. Exiting. ");
        return 1;

    print("Dragon_Init successful. Exiting.");
    return 0;
    
	
}
