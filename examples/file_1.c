#include "minicio.h"

int max(int a, int b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

int main() {
    int a, b, maximum;

    a = getint();
    b = getint();
    maximum = max(a, b);

    putint(maximum);
    putnewline();

    return 0;
}
