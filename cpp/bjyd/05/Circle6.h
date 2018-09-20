#pragma once

class Circle
{
public:
    Circle();
    Circle(double);
    ~Circle();
    double getArea();
    double getRadius();
    void setRadius(double);
    static int getNumberOfObjects();    //static function

private:
    double radius;
    static int numberOfObjects;         //static variable
};

