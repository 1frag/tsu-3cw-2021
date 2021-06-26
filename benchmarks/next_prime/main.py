import itertools
import time
import math

from matplotlib import pyplot as plt

import using_pyo3
import c_api_bindings

N, M = 20, 5
TEST_NUMBER = 7_177_162_611_713


def long_next_prime(x: int) -> int:
    i = x + 1
    while True:
        for j in range(2, int(math.sqrt(i) + 1)):
            if i % j == 0:
                break
        else:
            return i
        i += 1


def test_long_using_rust():
    for _ in range(M):
        using_pyo3.long_next_prime(TEST_NUMBER)


def test_short_using_rust():
    for _ in range(M):
        using_pyo3.short_next_prime(TEST_NUMBER)


def test_long_using_python():
    for _ in range(M):
        long_next_prime(TEST_NUMBER)


def test_short_using_c():
    for _ in range(M):
        c_api_bindings.short_next_prime(TEST_NUMBER)


FUNCS = {
    'long rust': test_long_using_rust,
    'short rust': test_short_using_rust,
    'long python': test_long_using_python,
    'short c': test_short_using_c,
}


def go():
    times = itertools.cycle(([], [], [], []))
    for _ in range(N):
        for f in FUNCS.values():
            t0 = time.time()
            f()
            t1 = time.time()
            next(times).append(t1 - t0)

    print('\n'.join(map(' = '.join, zip(map(str, FUNCS.keys()), map(str, times)))))
    return [*itertools.islice(map(lambda t: [[*range(1, N + 1)], t], times), 4)]


def main():
    fig, ax = plt.subplots()
    for label, res in zip(FUNCS, go()):
        ax.plot(*res, label=label)

    plt.xlabel("Номер попытки")
    plt.ylabel("Время (sec)")
    ax.legend()

    plt.xticks([*range(1, N + 1, 3)])

    fig.savefig("result_v2.png")


if __name__ == '__main__':
    main()
