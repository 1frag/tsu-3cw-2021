#include <math.h>

long long
next_prime(long long x)
{
    long long i, j, _lim;

    for (i = x + 1;;i++) {
        _lim = sqrt(i) + 1;
        for (j = 2; j <= _lim; j++) {
            if (j == _lim) return i;
            if (i % j == 0) break;
        }
    }
}
