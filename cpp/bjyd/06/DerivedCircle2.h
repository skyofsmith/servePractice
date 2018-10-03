#pragma once
#include "AbstractGeometricObject.h"

class Circle: public GeometricObject
{
public:
  Circle();
  Circle(double);
  Circle(double radius, string color, bool filled);
  double getRadius();
  void setRadius(double);
  double getArea();
  double getPerimeter();
  double getDiameter();

private:
  double radius_;
};
