#include <iostream>
using namespace std;

int add (int x, int y = 10) {
    return x + y;
}

int add (int x) {
    return x + 100;
}

int main() {
    add(1);//[Error] call of overloaded 'add(int)' is ambiguous
    cin.get();
	return 0;
}
