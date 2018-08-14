#include <iostream>
using namespace std;

int swap (int& x, int& y, int& z) {
    int t = x, max = x;
    if (y >= max) {
        max = y;
    }
    if (z >= max) {
        max = z;
    }
    x = z;
    z = y;
    y = t;
    return max;
}

int swap (int* x, int* y, int* z) {
    int t = *x, max = *x;
    if (*y >= max) {
        max = *y;
    }
    if (*z >= max) {
        max = *z;
    }
    *x = *z;
    *z = *y;
    *y = t;
    return max;
}

int main() {
    int a = 0, b = 0, c = 0, max = 0;
    cin >> a >> b >> c;
    max = swap (a, b, c);
    max = swap (&a, &b, &c);
    cout << max << " " << a << " " << b << " " << c << endl;
    
	return 0;
}
