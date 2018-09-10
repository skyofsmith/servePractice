#include "MyRectangle.h"
#include <iostream>

MyRectangle::MyRectangle(){
    x1_ = 0;
    y1_ = 0;
    x2_ = 0;
    y2_ = 0;
    std::cout << "myrectangle" << std::endl;
}
MyRectangle::MyRectangle(int x1, int y1, int x2, int y2, Screen* screen){
    x1_ = x1;
    y1_ = y1;
    x2_ = x2;
    y2_ = y2;
    screen_ = screen;
    std::cout << "myrectangle" << std::endl;
}

void MyRectangle::setCoordinations(int x1, int y1, int x2, int y2){
    x1_ = x1;
    y1_ = y1;
    x2_ = x2;
    y2_ = y2;
}
void MyRectangle::setScreen(Screen& screen){
    screen_ = &screen;
}
void MyRectangle::Draw() {
    if (x1_ <= 0 || y1_ <= 0 || x2_ >= screen_->getWidth() || y2_ >= screen_->getHeight()) {
       std::cout << "invalid myrectangle" << std::endl;
    } else {
        std::cout << x1_ << " " << y1_ << " " << x2_ - x1_ << " " << y2_ - y1_ << std::endl;
    }
}
