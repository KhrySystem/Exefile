#include <Dragon.hpp>

int main() {
	printf("Running Main\n");
	printf("creating WindowManager Instance\n");
	Dragon::WindowManager* windowManager = new Dragon::WindowManager();

	printf("Creating Window\n");
	windowManager->createWindow(800, 600);

	printf("Main Loop\n");
	while(windowManager->update()) {
		
	}

	printf("Closing Window\n");
	windowManager->closeWindow();
}