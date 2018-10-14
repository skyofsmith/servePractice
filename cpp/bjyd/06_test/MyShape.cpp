#include "MyShape.h"

MyShape::MyShape(){
  red_ = 255;
  green_ = 255;
  blue_ = 255;
}

MyShape::MyShape(Screen* screen){
  screen_ = screen;
}

MyShape::MyShape(int R, int G, int B, Screen* screen){
  red_ = 255;
  green_ = 255;
  blue_ = 255;
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

}
