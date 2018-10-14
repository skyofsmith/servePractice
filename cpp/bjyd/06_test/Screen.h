#pragma once 
#include <string>
using namespace std;

class Screen {
private:
    int width_;
    int height_;
    string enter_;
    string leave_;
    void exitWhenInvalidScreen();

public:
    Screen (int, int);
    ~Screen();
    int getWidth();
    int getHeight();
    int setWidth(int);
    int setHeight(int);
    static Screen* getInstance(int width, int height);
    static Screen* getInstance();
    void deleteInstance();
};
