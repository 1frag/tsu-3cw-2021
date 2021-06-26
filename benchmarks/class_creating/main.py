import time
import statistics

from matplotlib import pyplot as plt

import pyo3_class_creating

M = 15
N = 2000


class Item:
    def __init__(self, a: int, b: str, c: bool):
        self.a = a
        self.b = b
        self.c = c


def collect(n):
    items = [Item(
        a=i,
        b=str(i),
        c=(i & 1) == 1,
    ) for i in range(n)]
    return items


def go(func):
    g_times = []
    for n in range(1, N, 10):
        times = []
        for _ in range(M):
            t0 = time.time()
            __ = func(n)
            t1 = time.time()
            times.append((t1 - t0) * 1000)
        g_times.append(statistics.median(times))
    print(func, g_times, sep='\n')
    return [*range(1, N, 10)], g_times


def main():
    fig, ax = plt.subplots()
    ffi = go(pyo3_class_creating.collect)
    python = go(collect)

    ax.plot(*ffi, label='ffi')
    ax.plot(*python, label='python')

    plt.xlabel("Количество создаваемых экземпляров")
    plt.ylabel("Время (sec)")
    ax.legend()

    plt.xticks([1, *[*range(0, N + 1, 200)][1:]])

    fig.savefig("result_v2.png")


if __name__ == '__main__':
    main()
