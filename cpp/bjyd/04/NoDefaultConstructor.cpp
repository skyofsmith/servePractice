class Time
{
public:
    Time(int hour, int minute, int second)
    {
        
    }
private:
    int hour;
    int minute;
    int second;
};

class Action
{
public:
    Action(int hour, int minute, int second)
    {
        time = Time(hour, minute, second);
    }
private:
    Time time;
};

int main() {
	return 0;
}
