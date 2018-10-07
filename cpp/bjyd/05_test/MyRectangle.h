#pragma once
#include "Screen4.h"

class MyRectangle {
private:
    int x1_;
    int y1_;
    int x2_;
    int y2_;
    int red_;
    int green_;
    int blue_;
    Screen* screen_;
public:
    MyRectangle();
    MyRectangle(MyRectangle&);
    MyRectangle(int, int, int, int, Screen*);
    void setCoordinations(int, int, int, int);
    void setScreen(Screen&);
    void setColor(int, int, int);
    void Draw();
};
