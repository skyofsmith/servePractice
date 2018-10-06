#include <iostream>
#include "Screen2.h"

int main() {
	int width, height;
	Screen *screen1, *screen2;
	std::cin >> width >> height;
	screen1 = Screen::getInstance(width, height);
	screen2 = Screen::getInstance();

	if (screen1 != screen2 ) {
		std::cout << "two instances" << std::endl;
	}

	std::cout << screen2->getWidth() << ' ' << screen2->getHeight() << std::endl;
	screen2->deleteInstance();
	screen1 = Screen::getInstance(2*width, 2*height);
	std::cout << screen1->getWidth() << ' ' << screen1->getHeight() << std::endl;
	screen1->deleteInstance();


	#ifdef DEBUG
	std::cin.get();
	#endif

	return 0;
}
