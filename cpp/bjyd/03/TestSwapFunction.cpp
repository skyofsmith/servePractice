#include <iostream>
using namespace std;

void swap1 (int x, int y) {
    int t;
    t = x; x = y; y = t;
}

void swap2 (int* x, int* y) {
    int t;
    t = *x; *x = *y; *y = t;
}

void swap3 (int& x, int& y) {
    int t;
    t = x; x = y; y = t;
}

void print_a_b (char* status, int a, int b) {
    cout << status << ": a=" << a <<
            " b=" << b << endl;
}

int main() {
    
    int a, b;
    char* before = "Before\0";
    char* after = "After\0";
          
    a = 5, b = 10;
    print_a_b(before, a, b);
    swap1(a, b);
    print_a_b(after, a, b);
    
    a = 5, b = 10;
    print_a_b(before, a, b);
    swap2(&a, &b);
    print_a_b(after, a, b);
    
    a = 5, b = 10;
    print_a_b(before, a, b);
    swap3(a, b);
    print_a_b(after, a, b);
    
    
	return 0;
}
