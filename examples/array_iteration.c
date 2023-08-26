// Private: Check while loop
#include "minicio.h"

int main() {
    int J, K;
    int A[100], B[100];
    int i;
    int j;
    // test nested loop
    for (i = 0; i < 100; i = i + 1) {
        for (j = 0; j < 100; j = j + 1) {
            A[i] = i + j;
        }
    }
    J = 0;
    while (J < 100) {
        K = 0;
        while (K < 100) {
            K = K + 1;
        }
        if (B[J] == A[J]) {
            break;
        }
        J = J + 1;
    }

    putint(B[0]);
    putint(B[1]);
    putint(B[2]);
    putint(B[3]);
    putint(B[4]);

    return 0;
}
