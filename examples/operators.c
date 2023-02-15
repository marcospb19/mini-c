// Private: Check different operators
#include "minicio.h"

int main() {
    int a;
    a = 59;

    if (a == 59) {
        putint(1);
    } else {
        putint(-1);
    }

    if (-a == -59) {
        putint(2);
    } else {
        putint(-2);
    }

    if (a + 1 + 2 - 1 != 61) {
        putint(-3);
    } else {
        putint(3);
    }

    if (!(a * 10 * 10 / 20 == 59 * 5)) {
        putint(-4);
    } else {
        putint(4);
    }

    if (true && false && false) {
        putint(-5);
    } else {
        putint(5);
    }

    if (false || true || false) {
        putint(6);
    } else {
        putint(-6);
    }

    if (9 < 10 && 9 > 10 && 9 == 9) {
        putint(7);
    } else {
        putint(-7);
    }

    if (10 > 9 && 10 < 9 && 9 == 9) {
        putint(8);
    } else {
        putint(-8);
    }

    if (!(10 != 10)) {
        putint(9);
    } else {
        putint(-9);
    }

    return 0;
}
