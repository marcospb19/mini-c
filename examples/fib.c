// Private 3: Fibonacci
#include "minicio.h"

int i, result;

int fib(int t) {
    if (t < 1) {
        return 0;
    }
    if (t == 1) {
        return 1;
    }
    return fib(t - 1) + fib(t - 2);
}

int main() {
    i = 10;
    result = fib(i);
    putint(result);
    return 0;
}
