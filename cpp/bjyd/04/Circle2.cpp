#include "Circle2.h"

Circle::Circle() {
    radius_ = 1;
}

Circle::Circle(double newRadius) {
    radius_ = newRadius;
}

double Circle::getArea() {
    return radius_ * radius_ * 3.14159;
}

double Circle::getRadius(){
    return radius_;
}

void Circle::setRadius(double newRadius){
    radius_ = (newRadius >= 0) ? newRadius : 0;
}
