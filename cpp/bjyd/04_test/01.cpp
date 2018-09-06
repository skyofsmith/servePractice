#include <iostream>
#include "Screen.h"

int main() {
    int width, height;
    std::cin >> width >> height;
    Screen screen1 (width, height);
    Screen screen2;
    screen2.setWidth(800);
    screen2.setHeight(600);
    std::cout << screen1.getWidth() << ' ' << screen1.getHeight() << std::endl;
    std::cout << screen2.getWidth() << ' ' << screen2.getHeight();
    #ifdef DEBUG
    std::cin.get();
    #endif
    return 0;
}
