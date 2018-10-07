#include <iostream>
#include <cstdlib>
#include "Screen.h"

Screen* instance_ = NULL;
Screen::Screen (int width, int height) {
    width_ = width;
    height_ = height;
    enter_ = "enter screen";
    leave_ = "leave screen";
    std::cout << enter_ << std::endl;
    exitWhenInvalidScreen();
}
void Screen::exitWhenInvalidScreen()
{
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
    width_ = width;
    return width_;
}
int Screen::setHeight(int height) {
    height_ = height;
    return height_;
}

Screen* Screen::getInstance(int width, int height)
{
  if (instance_ == NULL) {
    instance_ = new Screen(width, height);
  }
  return instance_;
}

Screen* Screen::getInstance()
{
  if (instance_ == NULL) {
    instance_ = new Screen(640, 480);
  }
  return instance_;
}
