#include <iostream>

/* run this program using the console pauser or add your own getch, system("pause") or input loop */

int main(int argc, char** argv) {
	int count = 5;
	for (int i = 0; i < count; i++) {
		int j = 0;
		char str[i+1];
		do {
			str[j] = '*';
		} while (j++ < i);
		str[j] = '\0';
		std::cout << str << std::endl;
	}
	return 0;
}
