#include <string>
using namespace std;

class Screen {
private:
    int width_;
    int height_;
    string enter_;
    string leave_;
    void exitWhenInvalidScreen();
    static Screen* instance_ = 0;

public:
    Screen (int, int);
    int getWidth();
    int getHeight();
    int setWidth(int);
    int setHeight(int);
    static Screen* getInstance(int width = 640, int height = 480);
};
