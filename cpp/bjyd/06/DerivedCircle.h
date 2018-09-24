#ifndef DERIVERDCIRCLE_H
#define DERIVERDCIRCLE_H
#include "GeometricObject.h"

class Circle : public GeometricObject
{
	public:
		Circle();
		Circle(double radius);
		double getRadius();
		void setRadius(double radius);
		double getArea();
		double getPerimeter();
		double getDiameter();

	private:
	    double radius_;
};

#endif
