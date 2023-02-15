#include "minicio.h"

int N;
int a[20], c[400];

void markcolumn(int x, int y, int mark) {
    int i;
    for (i = 0; i < N; i = i + 1)
        c[i * N + y] = c[i * N + y] + mark;
}

void markdiag(int x, int y, int mark) {
    int i;
    for (i = 0; i < N; i = i + 1) {
        int x1;
        int y1;
        x1 = i;
        y1 = y + (i - x);
        if (y1 >= 0 && y1 < N)
            c[x1 * N + y1] = c[x1 * N + y1] + mark;
        y1 = y - (i - x);
        if (y1 >= 0 && y1 < N)
            c[x1 * N + y1] = c[x1 * N + y1] + mark;
    }
}

void search(int k) {
    int i;
    if (k == N) {
        for (i = 0; i < N; i = i + 1)
            putint(a[i]);
        putnewline();
        return;
    }
    for (i = 0; i < N; i = i + 1)
        if (c[k * N + i] == 0) {
            markcolumn(k, i, 1);
            markdiag(k, i, 1);
            a[k] = i;
            search(k + 1);
            markdiag(k, i, -1);
            markcolumn(k, i, -1);
        }
}

int main() {
    int i;
    N = getint();
    for (i = 0; i < N * N; i = i + 1)
        c[i] = 0;
    search(0);
    return 0;
}
