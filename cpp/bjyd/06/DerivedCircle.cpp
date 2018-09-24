#include "DerivedCircle.h"

Circle::Circle()
{
    radius_ = 1;
}

Circle::Circle(double radius)
{
    radius_ = radius;
}

double Circle::getRadius()
{
    return radius_;
}

void Circle::setRadius(double radius)
{
    radius_ = (radius >= 0) ? radius : 0;
}

double Circle::getArea()
{
    return radius_ * radius_ * 3.14159;
}

double Circle::getPerimeter()
{
    return 2 * radius_ * 3.14159;
}

double Circle::getDiameter()
{
    return 2 * radius_;
}
