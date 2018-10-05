#include <iostream>
#include "Screen.h"

int main() {
	int width, height;
	Screen *screen1, *screen2;

	std::cin >> width >> height;

	screen1 = Screen::getInstance(width, height);
	screen2 = Screen::getInstance();
	std::cout << screen1->getWidth() << ' ' << screen1->getHeight() << std::endl;
	std::cout << screen2->getWidth() << ' ' << screen2->getHeight();

	#ifdef DEBUG
	std::cin.get();
	#endif

	return 0;
}
