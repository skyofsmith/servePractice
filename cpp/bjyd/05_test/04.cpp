#include <iostream>
#include "Screen4.h"
#include "MyCircle.h"
#include "MyRectangle2.h"
using namespace std;

int main() {
	int width, height;
	cin >> width >> height;

	int leftX, leftY, rightX, rightY;
	cin >> leftX >> leftY >> rightX >> rightY;

	int centerX, centerY, radius;
	cin >> centerX >> centerY >> radius;

	Screen *screen = Screen::getInstance(width, height);

	MyRectangle myRectangle(leftX, leftY, rightX, rightY, screen);
	myRectangle.setColor(0, 0, 0xff);
	myRectangle.showScreen();
	myRectangle.Draw();

	// 构造圆形对象数组
	//// 第一个元素使用匿名对象（调用带参构造函数）初始化
	//// 第二个元素使用匿名对象（调用默认构造函数）初始化
	MyCircle myCircles[2] = { MyCircle(centerX, centerY, radius, screen) };

	// 设置对象数组中第二个元素的属性。注意访问成员函数的不同方法
	(myCircles + 1)->setCenter(centerX+10, centerY+20);
	myCircles[1].setRadius(radius+30);
	(*(myCircles+1)).setColor(0x00, 0x00, 0x00);
	myCircles[1].setScreen(*screen);
	for(int i=0; i<=1; i++) {
	myCircles[i].showScreen();
	(myCircles+i)->Draw();
	}

	// 调用拷贝构造函数以myCircles 数组中的第二个元素为模板创建新对象
	MyCircle yourCircle(myCircles[1]);

	yourCircle.showScreen();
	(&yourCircle)->Draw();

	screen->deleteInstance();

	#ifdef DEBUG
	std::cin.get();
	#endif
	return 0;
}
