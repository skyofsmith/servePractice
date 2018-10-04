#include "Rectangle2.h"

Rectangle::Rectangle()
{
    width_ = 1;
    height_ = 1;
}

Rectangle::Rectangle(double width, double height)
{
    width_ = width;
    height_ = height;
}

double Rectangle::getWidth()
{
    return width_;
}

void Rectangle::setWidth(double width)
{
    width_ = (width >= 0) ? width : 0;
}

double Rectangle::getHeight()
{
    return height_;
}

void Rectangle::setHeight(double height)
{
    height_ = (height >= 0) ? height : 0;
}

double Rectangle::getArea()
{
    return width_ * height_;
}

double Rectangle::getPerimeter()
{
    return 2 * (width_ + height_);
}
