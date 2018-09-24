#pragma once
#include "GeometricObject.h"

class Rectangle: public GeometricObject
{
    public:
        Rectangle();
        Rectangle(double width, double height);
        double getWidth();
        void setWidth(double);
        double getHeight();
        void setHeight(double);
        double getArea();
        double getPerimeter();
        
    private:
        double width_;
        double height_;
};
