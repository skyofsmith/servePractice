#include <iostream>
#include "Screen.h"
#include "MyShape.h"
using namespace std;

int main() {
	int width, height;
	cin >> width >> height;

	Screen *screen = Screen::getInstance(width, height);

	MyShape shape1(screen);
	MyShape* shape2 = new MyShape();
	shape2->setScreen(*screen);
	shape2->setColor(0, 0, 0xff);

	shape1.Draw();
	shape2->Draw();

	delete shape2;
	screen->deleteInstance();

	#ifdef DEBUG
	std::cin.get();
	#endif
	return 0;
}
