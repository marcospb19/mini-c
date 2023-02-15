// Test 3: Conditionals

#include "minicio.h"

bool isTen(int a) {
    if (a != 10)
        return false;
    return true;
}

int main() {
    int myScore, yourScore;
    myScore = 10;
    yourScore = -10;

    if (isTen(myScore) && isTen(yourScore)) {
        putint(1);
        return 1;
    }
    putint(0);
    return 0;
}
