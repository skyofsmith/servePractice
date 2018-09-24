#ifndef GEOMETRICOBJECT_H
#define GEOMETRICOBJECT_H
#include <string>
using namespace std;

class GeometricObject
{
    public:
        GeometricObject();
        GeometricObject(string, bool);
        string getColor();
        void setColor(string);
        bool isFilled();
        void setFilled(bool);
        string toString();

    private:
        string color_;
        bool filled_;
};
#endif
