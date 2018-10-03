#pragma once
#include "AbstractGeometricObject.h"

class Rectangle: public GeometricObject
{
public:
  Rectangle();
  Rectangle(double width, double height);
  Rectangle(double width, double height, string color, bool filled);
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
