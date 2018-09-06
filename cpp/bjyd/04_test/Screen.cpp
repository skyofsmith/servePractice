#include <iostream>
#include "Screen.h"

Screen::Screen () {
    width_ = 640;
    height_ = 480;
    std::cout << "screen" << std::endl;
}
Screen::Screen (int width, int height) {
    width_ = width;
    height_ = height;
    std::cout << "screen" << std::endl;
}
int Screen::getWidth() {
    return width_;
}
int Screen::getHeight() {
    return height_;
}
int Screen::setWidth(int width) {
    width_ = width;
    return width_;
}
int Screen::setHeight(int height) {
    height_ = height;
    return height_;
}
