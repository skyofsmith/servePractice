class Screen {
private:
    int width_;
    int height_;
    void exitWhenInvalidScreen();
public:
    Screen ();
    Screen (int, int);
    int getWidth();
    int getHeight();
    int setWidth(int);
    int setHeight(int);
};
