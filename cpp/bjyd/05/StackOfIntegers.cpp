#pragma once

class StackOfIntegers
{
public:
    StackOfIntegers();
    bool empty();
    int peek();
    int push(int value);
    int pop();
    int getSize();
    
private:
    int elements[100];
    int size;
};
