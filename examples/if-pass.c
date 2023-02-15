// Private: Check if stmt on if statement passes
#include "minicio.h"

int main() {
    int i;
    i = 0;

    if (i > 0)
        putint(i + 1);

    else
        putint(i + 10);

    return 0;
}
