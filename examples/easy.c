// Private 1: Simple assignments
#include "minicio.h"

bool alpha, beta;
int delta, epsilon;

void main() {
    alpha = true;
    beta = alpha;

    delta = -1;
    epsilon = delta + 1;

    putint(delta);
    putnewline();
    putint(epsilon);

    return;
}
