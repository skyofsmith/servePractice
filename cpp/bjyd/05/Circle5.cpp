#include "Circle5.h"

int Circle::numberOfObjects = 0;

Circle::Circle()
{
    radius = 1;
    numberOfObjects++;
}

Circle::Circle(double radius)
{
    this -> radius = radius;
    numberOfObjects++;
}

double Circle::getArea()
{
    return radius * radius * 3.14159;
}

double Circle::getRadius()
{
    return radius;
}

void Circle::setRadius(double radius)
{
    this -> radius = (radius >= 0) ? radius : 0;
}

int Circle::getNumberOfObjects()
{
    return numberOfObjects;
}
