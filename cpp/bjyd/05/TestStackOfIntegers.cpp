#include <iostream>
#include "StackOfIntegers.h"
using namespace std;

int main()
{
    StackOfInteger stack;
    
    for (int i = 0; i < 10; i++)
    {
        stack.push(i);
    }
    
    while(!stack.empty())
    {
        cour << stack.pop() << " ";
    }
    
	return 0;
}
