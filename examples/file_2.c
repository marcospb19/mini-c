#include "minicio.h"

int mod(int a, int b) {
    return a - (a / b) * b;
}

int gcd(int a, int b) {
    if (b == 0) {
        return a;
    } else {
        return gcd(b, mod(a, b));
    }
}

int main() {
    int a = getint();
    int b = getint();
    int divisor = gcd(a, b);

    putint(divisor);
    putnewline();

    return 0;
}
