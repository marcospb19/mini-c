// Private: Check if stmt stmt on if statement fails
#include "minicio.h"

int main() {
    int i;
    i = 0;

    if (i > 0)
        putint(i + 1);
    putint(i + 10);

    else putint(i + 10);

    return 0;
}
