#pragma once
#include <string>
#include "Screen.h"
using namespace std;

class MyShape{
private:
  int red_;
  int green_;
  int blue_;
  Screen* screen_;
  string type_;

public:
  MyShape();
  MyShape(Screen*);
  MyShape(int, int, int, Screen*);
  int getRed();
  int getGreen();
  int getBlue();
  void setColor(int, int, int);
  void setScreen(Screen&);
  void showShape();
};
