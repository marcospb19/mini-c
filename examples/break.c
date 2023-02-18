// Private: Check break
#include "minicio.h"

int main() {
    int ii;
    int jj;
    ii = 0;
    jj = 0;
    for (ii = 1; ii < 50; ii = ii + 1) {
        jj = jj + ii;
        if (ii == 20) {
            if (ii < 21) {
                break;
            }
            return 1;
        }
    }
    return 0;
}
