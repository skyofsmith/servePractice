#include <iostream>
#include <iomanip>
using namespace std;

int main() {
    int a = 0;
    double b = 0.0;
    cin >> a >> b;
    int x = a / static_cast<int>(b);
    double y = static_cast<double>(a) / b;
    double z = static_cast<double>(a) / static_cast<double>(static_cast<int>(b));
    cout << std::fixed << std::setprecision(3) << x << " " << y << " " << z << endl;
	return 0;
}
