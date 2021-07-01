import asyncio
import statistics
import time

from matplotlib import pyplot as plt

USE_LIGHT = False


def get_range():
    if USE_LIGHT:
        return range(1, 201, 2)
    else:
        return range(1000, 5001, 100)


async def go(func):
    times = []
    for n in get_range():
        t0 = time.time()
        await asyncio.gather(*[func(1) for _ in range(n)])
        t1 = time.time()
        times.append((t1 - t0 - 1) * 1000)
    print(func, times, sep='\n')
    return [*get_range()], times


async def main():
    import pyo3_sleeps
    fig, ax = plt.subplots()
    ffi = await go(pyo3_sleeps.rust_sleep)
    python = await go(asyncio.sleep)

    ax.plot(*ffi, label='ffi')
    ax.plot(*python, label='python')

    plt.xlabel("Количество ожидаемых функций")
    plt.ylabel("Дополнительное время (ms)")
    ax.legend()

    fig.savefig("3/v1.png")

    rest1 = [*map(lambda x: x[0] / x[1], zip(ffi[1], python[1]))]
    print('max_diff', max(map(lambda x: x[0] - x[1], zip(ffi[1], python[1]))))
    m = statistics.median(rest1)

    fig, ax = plt.subplots()
    ax.plot([*get_range()], rest1)
    ax.plot([*get_range()], [1 for _ in [*get_range()]], linestyle='dashed', color='gray')
    ax.plot([*get_range()], [m for _ in [*get_range()]], linestyle='dashed', color='gray')
    plt.yticks([*plt.yticks()[0], round(m, 3)])
    plt.xticks([*filter(lambda x: get_range().start <= x <= get_range().stop, plt.xticks()[0])])
    plt.xlabel("Количество ожидаемых функций")
    plt.ylabel("ffi[i] / python[i]")
    fig.savefig("3/v2.png")


if __name__ == '__main__':
    asyncio.run(main())
