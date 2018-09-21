#include "Date1.h"

class Person
{
public:
    Person(int id, int year, int monthm, int day);
    Person(Person&);
    int getId();
    Date* getBirthDate();
private:
    int id;
    Date* birthDate;
};
