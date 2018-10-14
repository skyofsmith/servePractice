#include <iostream>
#include "MyRectangle2.h"

MyRectangle::MyRectangle(){
    x1_ = 10;
    y1_ = 10;
    x2_ = 100;
    y2_ = 100;
    red_ = 255;
    green_ = 255;
    blue_ = 255;
    std::cout << "myrectangle" << std::endl;
}

MyRectangle::MyRectangle(MyRectangle& myRectangle) {
    x1_ = myRectangle.x1_;
    y1_ = myRectangle.y1_;
    x2_ = myRectangle.x2_;
    y2_ = myRectangle.y2_;
    red_ = myRectangle.red_;
    green_ = myRectangle.green_;
    blue_ = myRectangle.blue_;
    std::cout << "myrectangle" << std::endl;
}

MyRectangle::MyRectangle(int x1, int y1, int x2, int y2, Screen* screen){
    x1_ = x1;
    y1_ = y1;
    x2_ = x2;
    y2_ = y2;
    red_ = 255;
    green_ = 255;
    blue_ = 255;
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
void MyRectangle::setColor(int R, int G, int B){
  red_ = R;
  green_ = G;
  blue_ = B;
}
void MyRectangle::showScreen(){
  std::cout << screen_ -> getWidth() << " " << screen_ -> getHeight() << std::endl;
}
void MyRectangle::Draw() {
    std::cout << x1_ << " " << y1_ << " " << x2_ - x1_ << " " << y2_ - y1_ << std::endl;
    std::cout << red_ << " " << green_ << " " << blue_ << std::endl;
}
