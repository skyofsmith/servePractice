#include "Date.h"

class Person
{
public:
    Person(int id, int year, int monthm, int day);
    int getId();
    Date* getBirthDate();
private:
    int id;
    Date* birthDate;
};
