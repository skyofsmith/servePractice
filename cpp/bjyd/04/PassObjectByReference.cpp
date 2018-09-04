#include <iostream>
#include "Circle2.h"
using namespace std;

void printCircle (Circle &c) {
//    cout << "The Object myCircle's address is " << &c << endl;
    cout << "The area of the circle of " <<
            c.getRadius() << " is " << c.getArea() << endl;
}
int main() {
    Circle myCircle(5.0);
//    cout << "The Object myCircle's address is " << &myCircle << endl;
    printCircle(myCircle);
    
	return 0;
}

/*
  两个对象的地址相同， 是同一个对象 
*/
