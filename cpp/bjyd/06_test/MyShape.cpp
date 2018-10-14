#include <iostream>
#include "MyShape.h"
using namespace std;

MyShape::MyShape():type_("myshape"){
  red_ = 255;
  green_ = 255;
  blue_ = 255;
}

MyShape::MyShape(Screen* screen):type_("myshape"){
  red_ = 255;
  green_ = 255;
  blue_ = 255;
  screen_ = screen;
}

MyShape::MyShape(int R, int G, int B, Screen* screen):type_("myshape"){
  red_ = R;
  green_ = G;
  blue_ = B;
  screen_ = screen;
}

int MyShape::getRed(){
  return red_;
}

int MyShape::getGreen(){
  return green_;
}

int MyShape::getBlue(){
  return blue_;
}

void MyShape::setColor(int R, int G, int B){
  red_ = R;
  green_ = G;
  blue_ = B;
}

void MyShape::setScreen(Screen& screen){
  screen_ = &screen;
}

void MyShape::Draw() {
  cout << "[" << screen_->getWidth() << "X" << screen_->getHeight() << "]" << type_ <<
  "(" << red_ << "," << green_ << "," << blue_ << ")" << endl;
}
