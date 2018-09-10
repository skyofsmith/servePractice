#include <iostream>
#include <cstdlib>
#include "Screen2.h"
#include "MyRectangle.h"
using namespace std;

int main() {
    int width, height;
    
    cin >> width >> height;
    
    Screen screen (width, height);
    
    int leftX, leftY, rightX, rightY;
    
    cin >> leftX >> leftY >> rightX >> rightY;
    
    MyRectangle myRectangle1 (leftX, leftY, rightX, rightY, &screen);
    MyRectangle* myRectangles = new MyRectangle[2];
    myRectangles[1].setCoordinations(10, 300, 700, 500);
    myRectangles[1].setScreen(screen);
    myRectangle1.Draw();
    
    for (int i = 0; i < 2; i++) {
        myRectangles[i].setScreen(screen);
        (myRectangles+i) -> Draw();
    }
    delete [] myRectangles;
    #ifdef DEBUG
    std::cin.get();
    #endif
    return 0;
}
