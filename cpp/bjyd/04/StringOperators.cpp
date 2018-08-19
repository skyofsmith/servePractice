#include <iostream>
using namespace std; 
int main() {
    string s1 = "ABC";
    string s2 = s1;
    for (int i = s2.size() - 1; i >= 0; i--) {
        cout << s2[i];
    }
    
    string s3 = s1 + "DEFG";
    cout << s3 << endl;
    
    s1 += "ABC";
    cout << s1 << endl;
    
    s1 = "ABC";
    s2 = "ABE";
    cout << (s1 == s2) << endl;
    cout << (s1 != s2) << endl;
    cout << (s1 > s2) << endl;
    cout << (s1 >= s2) << endl;
    cout << (s1 < s2) << endl;
    cout << (s1 <= s2) << endl;
	return 0;
}
