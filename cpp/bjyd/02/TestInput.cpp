#include "graphics.h"
int main () {
    initgraph(640, 400);
    
    char s[101];
    inputbox_getline("Input Box", "Please input a string", s, 100);
    outtextxy(10, 10, s);
    
    int x = getInteger();
    xyprintf(10, 30, "This is an integer: %d", x);
    
    double y = getDouble();
    xyprintf(10, 50, "This is an double: %f", y);
    
    getString(s, 100);
    outtextrect(10, 80, 100, 60, s);

    char c = getChar("Input a character");
    outtext(c);
    
    int coords[4];
    int* pc = getCoords(coords, 2);
    line(pc[0], pc[1], pc[2], pc[3]);
    
    ege::getch();
    closegraph();
    
    return 0;
}
