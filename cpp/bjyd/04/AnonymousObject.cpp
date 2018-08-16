#include <iostream>
using namespace std;

class Circle {
    public:
        double radius;
        
        Circle() {
            radius = 1;
        }
        
        Circle(double newRadius) {
            radius = newRadius;
        }
        
        double getArea() {
            return radius * radius * 3.14159;
        }
};

int main() {
    Circle circle1, circle2;
    circle1 = Circle();
    circle2 = Circle(5);
    
    cout << "Area is " << Circle().getArea() << endl;
    cout << "Area is " << Circle(5).getArea() << endl;
    
	return 0;
}
