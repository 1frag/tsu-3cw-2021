import asyncio
import datetime
import time
import statistics

import asyncpg
from matplotlib import pyplot as plt

import real_world_application as rwa

from benchmarks.postgres.v2.impl import Flight, pool, _pool, DSN

M, N = 7, 1000


def get_range():
    return range(1, N, 4)


async def get_flights(limit: int) -> list[Flight]:
    async with _pool.get().acquire() as conn:  # type: asyncpg.Connection
        rows = await conn.fetch("""
            SELECT
                flight_id,
                flight_no,
                scheduled_departure,
                scheduled_arrival,
                departure_airport,
                arrival_airport,
                status,
                aircraft_code,
                actual_departure,
                actual_arrival
            FROM flights
            LIMIT $1::int;
        """, limit)
        return [Flight(**row) for row in rows]


async def go(func):
    async with pool():
        g_times = []
        for n in get_range():
            times = []
            for _ in range(M):
                t0 = time.time()
                __ = await func(n)
                t1 = time.time()
                times.append((t1 - t0) * 1000)
            g_times.append(statistics.median(times))
        print(func, g_times, sep='\n')
        return [*get_range()], g_times


async def main():
    await rwa.configure(DSN, datetime.timezone.utc)

    fig, ax = plt.subplots()
    ffi = await go(rwa.get_flights)
    python = await go(get_flights)

    ax.plot(*ffi, label='ffi')
    ax.plot(*python, label='python')

    plt.xlabel("Количество запрашиваемых строк")
    plt.ylabel("Время (ms)")
    ax.legend()

    fig.savefig("result_v1.png")

    rest1 = [*map(lambda x: x[0] / x[1], zip(python[1], ffi[1]))]
    fig, ax = plt.subplots()
    ax.plot([*get_range()], rest1)
    ax.plot([*get_range()], [1 for _ in [*get_range()]], linestyle='dashed', color='gray')
    plt.xticks([*filter(lambda x: get_range().start <= x <= get_range().stop, plt.xticks()[0])])
    plt.xlabel("Количество запрашиваемых строк")
    plt.ylabel("python[i] / ffi[i]")
    fig.savefig("result_v2.png")

    rest1 = [*map(lambda x: x[0] - x[1], zip(python[1], ffi[1]))]

    fig, ax = plt.subplots()
    ax.plot([*get_range()], rest1)
    ax.plot([*get_range()], [0 for _ in [*get_range()]], linestyle='dashed', color='gray')
    plt.xticks([*filter(lambda x: get_range().start <= x <= get_range().stop, plt.xticks()[0])])
    plt.xlabel("Количество запрашиваемых строк")
    plt.ylabel("python[i] - ffi[i]")
    fig.savefig("result_v3.png")


if __name__ == '__main__':
    asyncio.run(main())
