#pragma once
#include "Screen4.h"

class MyCircle
{
public:
    MyCircle();
    MyCircle(int, int, int, Screen*);
    MyCircle(const MyCircle& mycircle);
    double getArea();
    int getRadius();
    void showScreen();
    void setRadius(int);
    void setCenter(int, int);
    void setColor(int, int, int);
    void setScreen(Screen&);
    void Draw();

private:
    int x_;
    int y_;
    int radius_;
    int red_;
    int green_;
    int blue_;
    Screen* screen_;
};
