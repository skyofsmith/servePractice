#include "Person1.h"

Person::Person(int id, int year, int month, int day)
{
    this -> id = id;
    birthDate = new Date(year, month, day);
}
Person::Person(Person &person)
{
    id = person.id;
    Date *p = person.getBirthDate();
    birthDate = new Date(*p);
}

int Person::getId()
{
    return id;
}

Date * Person::getBirthDate()
{
    return birthDate;
}
