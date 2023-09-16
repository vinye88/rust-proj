#include "my_dll.h"
#include <windows.h>
#include <iostream>

int add(int a, int b)
{
    INT test = a;
    INT test2 = b;
    std::cout << "\tanswer is " << test + test2 << std::endl;
    return test + test2;
}
