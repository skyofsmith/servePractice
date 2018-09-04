#include <iostream>
#include "Circle2.h"
using namespace std;

void printCircle(Circle *c) {
    cout << "The Object's address is " << c << endl;
    cout << "The area of the circle of " <<
            c->getRadius() << " is " << c->getArea() << endl;
}
int main() {
    Circle myCircle(5.0);
    cout << "The Object's address is " << &myCircle << endl;
    printCircle(&myCircle);
    
	return 0;
}
