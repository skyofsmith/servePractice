#include <iostream>
#include "MyRectangle.h"
#include "Screen3.h"

int main() {
  int width, height;
  int leftX, leftY, rightX, rightY;
  Screen *screen;

  cin >> width >> height;
  cin >> leftX >> leftY >> rightX >> rightY;

  screen = Screen::getInstance(width, height);
  MyRectangle myRectangle(leftX, leftY, rightX, rightY, screen);
  myRectangle.setColor(0, 0, 0xff);
  myRectangle.Draw();

  screen->deleteInstance();

  #ifdef DEBUG
  std::cin.get();
  #endif
  return 0;
}
