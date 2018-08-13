#include <iostream>
using namespace std;

void printArea (double radius = 1);
//void printArea (double radius = 1) {
//    double area = radius * radius * 3.14159;
//    cout << "area is " << area << endl;
//}

int main(int argc, char** argv) {
    printArea();
    printArea(4);
	return 0;
}

void printArea (double radius) {
    double area = radius * radius * 3.14159;
    cout << "area is " << area << endl;
}
