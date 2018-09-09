#include <iostream>
#include <cstdlib>
#include "Screen2.h"

Screen::Screen () {
    width_ = 640;
    height_ = 480;
    exitWhenInvalidScreen();
    std::cout << "screen" << std::endl;
}
Screen::Screen (int width, int height) {
    width_ = width;
    height_ = height;
    exitWhenInvalidScreen();
    std::cout << "screen" << std::endl;
}
void Screen::exitWhenInvalidScreen() {
    if (width_ <= 0 || width_ > 1000 || height_ <= 0 || height_ > 1000) {
        std::cout << "invalid screen size" << std::endl;
        exit(0);
    }
}
int Screen::getWidth() {
    return width_;
}
int Screen::getHeight() {
    return height_;
}
int Screen::setWidth(int width) {
    exitWhenInvalidScreen();
    width_ = width;
    return width_;
}
int Screen::setHeight(int height) {
    exitWhenInvalidScreen();
    height_ = height;
    return height_;
}
