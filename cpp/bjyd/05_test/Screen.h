#include <string>
using namespace std;

class Screen {
private:
    int width_;
    int height_;
    string enter_;
    string leave_;
    void exitWhenInvalidScreen();
    static Screen* instance_;

public:
    Screen (int, int);
    int getWidth();
    int getHeight();
    int setWidth(int);
    int setHeight(int);
    static Screen* getInstance(int width, int height);
    static Screen* getInstance();
};
