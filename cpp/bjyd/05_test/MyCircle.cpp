#include <iostream>
#include "MyCircle.h"

MyCircle::MyCircle()
{
    x_ = 200;
    y_ = 200;
    radius_ = 100;
    red_ = 255;
    green_ = 255;
    blue_ = 255;
    screen_ = NULL;
    std::cout << "mycircle" << std::endl;
}

MyCircle::MyCircle(int x, int y, int radius, Screen* pScreen)
{
    x_ = x;
    y_ = y;
    radius_ = radius;
    red_ = 255;
    green_ = 255;
    blue_ = 255;
    screen_ = pScreen;
    std::cout << "mycircle" << std::endl;
}

MyCircle::MyCircle(const MyCircle& myCircle)
{
    x_ = myCircle.x_;
    y_ = myCircle.y_;
    radius_ = myCircle.radius_;
    red_ = myCircle.red_;
    green_ = myCircle.green_;
    blue_ = myCircle.blue_;
    screen_ = myCircle.screen_;
    std::cout << "copy mycircle" << std::endl;
}

double MyCircle::getArea()
{
    return radius_ * radius_ * 3.14159;
}

int MyCircle::getRadius()
{
    return radius_;
}

void MyCircle::showScreen(){
  std::cout << screen_ -> getWidth() << " " << screen_ -> getHeight() << std::endl;
}

void MyCircle::setCenter(int x, int y) {
  x_ = x;
  y_ = y;
}

void MyCircle::setRadius(int radius)
{
    this -> radius_ = (radius >= 0) ? radius : 0;
}

void MyCircle::setColor(int R, int G, int B){
    red_ = R;
    green_ = G;
    blue_ = B;
}

void MyCircle::setScreen(Screen& screen){
  screen_ = &screen;
}

void MyCircle::Draw(){
    std::cout << x_ << " " << y_ << " " << radius_ << std::endl;
    std::cout << red_ << " " << green_ << " " << blue_ << std::endl;
}
