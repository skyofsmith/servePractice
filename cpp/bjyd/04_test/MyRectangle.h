#include "Screen2.h"

class MyRectangle {
private:
    int x1_;
    int y1_;
    int x2_;
    int y2_;
    Screen* screen_;
public:
    MyRectangle();
    MyRectangle(int, int, int, int, Screen*);
    void setCoordinations(int, int, int, int);
    void setScreen(Screen&);
    void Draw();
};
