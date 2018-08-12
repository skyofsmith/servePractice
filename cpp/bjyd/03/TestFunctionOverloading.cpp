#include <iostream>
using namespace std;

int max(int num1, int num2) {
    if (num1 > num2) {
        return num1;
    } else {
        return num2;
    }
}

double max(double num1, double num2) {
    if (num1 > num2) {
        return num1;
    } else {
        return num2;
    }
}

double max(double num1, double num2, double num3) {
    return max(max(num1, num2), num3);
}

int maxNumber(int num1, double num2) {
    if (num1 > num2) {
        return num1;
    } else {
        return num2;
    }
}

double maxNumber(double num1, int num2) {
    if (num1 > num2) {
        return num1;
    } else {
        return num2;
    }
}


int main() {
    cout << "The maxinum between 3 and 4 is " << max(3, 4) << endl;
    cout << "The maxinum between 3.0 and 5.4 is " << max(3.0, 5.4) << endl;
    cout << "The maxinum between 3.0 and 5.4 and 10.14 is " 
         << max(3.9, 5.4, 10.14) << endl;
//    cout << maxNumber(1, 2) << endl;//error
	return 0;
}
