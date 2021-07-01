import asyncio
import datetime
import statistics
import time

import asyncpg
import real_world_application as rwa
from matplotlib import pyplot as plt

from benchmarks.postgres.impl import DSN, Flight, FlightLight, _pool, pool

M, N = 100, 100
in_dir = '3/'.__add__


def get_range():
    return range(N + 1)


PY_MODEL = Flight


async def get_flights(limit: int) -> list[Flight]:
    async with _pool.get().acquire() as conn:  # type: asyncpg.Connection
        rows = await conn.fetch("""
            SELECT flight_id, flight_no, scheduled_departure, scheduled_arrival, departure_airport,
                   arrival_airport, status, aircraft_code, actual_departure, actual_arrival
            FROM flights
            LIMIT $1::int;
        """, limit)
        return [PY_MODEL(**row) for row in rows]


async def go(func):
    async with pool():
        await func(1)
        times = []
        for n in get_range():
            t0 = time.time()
            await asyncio.gather(*[func(n) for _ in range(M)])
            t1 = time.time()
            times.append((t1 - t0) * 1000)

        print(func, times, sep='\n')
        return [*get_range()], times


def stats(rest, title):
    print(title)
    print('median', statistics.median(rest), sep=' = ')
    print('max', max(rest), sep=' = ')
    print('min', min(rest), sep=' = ')


async def main():
    global PY_MODEL
    await rwa.configure(DSN, datetime.timezone.utc)

    fig, ax = plt.subplots()
    ffi = await go(rwa.get_flights)
    pydantic = await go(get_flights)
    PY_MODEL = FlightLight
    no_checks = await go(get_flights)

    ax.plot(*ffi, label='ffi')
    ax.plot(*pydantic, label='python pydantic')
    ax.plot(*no_checks, label='python no checks')

    plt.xlabel("Количество возвращаемых объектов")
    plt.ylabel("Время (ms)")
    ax.legend()

    fig.savefig(in_dir("result_v2.png"))

    rest1 = [*map(lambda x: x[0] / x[1], zip(pydantic[1], ffi[1]))]
    stats(rest1, 'ffi vs pydantic (разы)')
    rest1 = [*map(lambda x: x[0] / x[1], zip(no_checks[1], ffi[1]))]
    stats(rest1, '\nffi vs no_checks (разы)')

    rest2 = [*map(lambda x: x[0] - x[1], zip(no_checks[1], ffi[1]))]
    stats(rest2, '\nffi vs no_checks (разница)')
    rest2 = [*map(lambda x: x[0] - x[1], zip(pydantic[1], ffi[1]))]
    stats(rest2, '\nffi vs pydantic (разница)')


if __name__ == '__main__':
    asyncio.run(main())
