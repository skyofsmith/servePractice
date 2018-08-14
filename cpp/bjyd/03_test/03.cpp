#include <iostream>
using namespace std;
void swap (int& x, int& y) {
    int t = x;
    x = y;
    y = t;
}

int main() {
    int a[] = {1, 2, 3, 4, 5};
    swap (a[1], a[3]);
    cout << a[0] << a[1] << a[2] << a[3];
	return 0;
}
