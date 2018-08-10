#include <graphics.h>

void mainloop() {
    
    int x = 0;
    setcolor(EGERGB(0, 0xFF, 0));
    setfillcolor(EGERGB(0, 0, 0xFF));
     
    for (;is_run(); delay_fps(60)) {
        x = (x + 1) % 440;
//        setfillcolor(EGERGB(random(255), random(255), random(255)));
        cleardevice();
//        fillellipse(200, 200, 50, 30); 
        fillellipse(x + 100, 200, 100, 100); 

    }
}

int main (void) {

    setinitmode(INIT_ANIMATION);
    initgraph(640, 480);
    randomize();
    mainloop();
    closegraph();
    return 0; 
}
