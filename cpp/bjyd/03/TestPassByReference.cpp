#include <iostream>
using namespace std;

void swap (int &n1, int &n2) {
    cout << "\tInside the swap function" << endl;
    cout << "\t\tBefore swapping n1 is " << n1 <<
            " n2 is " << n2 << endl;
    // Swap n1 with n2
    int temp = n1;
    n1 = n2;
    n2 = temp;
    
    cout << "\t\tAfter swapping n1 is " << n1 <<
            " n2 is " << n2 << endl;
} 

int main() {
    int num1 = 1;
    int num2 = 2;
    
    cout << "Before invoking the swap functin , num1 is "
         << num1 << " and num2 is " << num2 << endl;
    
    // Invoking the swap function to attempt to swap two variables
    swap(num1, num2);
    
    cout << "After invoking the swap functin , num1 is "
         << num1 << " and num2 is " << num2 << endl;
    
	return 0;
}
