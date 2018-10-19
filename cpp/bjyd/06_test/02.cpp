#include <iostream>
#include "Screen.h"
#include "MyShape.h"
#include "MyRectangle.h"
#include "MyCircle.h"

template <typename T>
struct Helper
{
    typedef char SmallType;
    typedef int LargeType;

    template <typename U>
    static char Test(U(*)[1]);
    template <typename U>
    static int Test(...);

    const static bool Result = sizeof(Test<T>(NULL)) == sizeof(LargeType);
};

int main() {
    int width, height;
    cin >> width >> height;

    Screen *screen = Screen::getInstance(width, height);
    
    if (!Helper<MyShape>::Result) cout << endl;
    MyShape *s1, *s2;
    
    s1 = new MyRectangle();
    s1->setScreen(*screen);
    s1->setColor(0, 0, 0xff);
    
    s2 = new MyCircle(100,110,50, screen);
    
    s1->Draw();
    s2->Draw();
    
    delete s1, s2;
    screen->deleteInstance();
    
    #ifdef DEBUG
    std::cin.get();
    #endif
    return 0;
}
