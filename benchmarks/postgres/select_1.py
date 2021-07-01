import asyncio
import dataclasses
import datetime
import statistics
import time

import asyncpg
import real_world_application as rwa
from matplotlib import pyplot as plt

from benchmarks.postgres.impl import DSN, _pool, pool

N, M = 100, 25


# class Number(BaseModel):
#     number: int
@dataclasses.dataclass
class Number:
    number: int

def get_range():
    return range(1, N + 1)


async def select_1() -> Number:
    async with _pool.get().acquire() as conn:  # type: asyncpg.Connection
        return Number(number=await conn.fetchval("select 1"))


async def go(func):
    async with pool():
        await select_1()
        await rwa.select_1()
        g_times = []
        for _ in get_range():
            times = []
            for _ in get_range():
                t0 = time.time()
                __ = await asyncio.gather(*[func() for _ in range(1)])
                t1 = time.time()
                times.append((t1 - t0) * 1000)
            g_times.append(statistics.median(times))
        print(func, times, sep='\n')
        return [*get_range()], times


async def main():
    await rwa.configure(DSN, datetime.timezone.utc)

    fig, ax = plt.subplots()
    ffi = await go(rwa.select_1)
    python = await go(select_1)

    ax.plot(*ffi, label='ffi')
    ax.plot(*python, label='python')
    ffi_median, python_median = statistics.median(ffi[1]), statistics.median(python[1])
    ax.plot([*get_range()], [ffi_median for _ in get_range()], color='blue', linestyle='dashed')
    ax.plot([*get_range()], [python_median for _ in get_range()], color='orange', linestyle='dashed')

    plt.xlabel("Номер попытки")
    plt.ylabel("Время (ms)")
    ax.legend()

    fig.savefig("result_v4.png")


if __name__ == '__main__':
    asyncio.run(main())
