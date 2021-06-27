import asyncio
import contextlib
import os
import sys
import typing

import asyncpg
import asyncpg.transaction

DB_DSN = 'postgresql://postgres:postgres@0.0.0.0:5438/postgres'


@contextlib.asynccontextmanager
async def connection() -> typing.AsyncContextManager[asyncpg.Connection]:
    conn: asyncpg.Connection = await asyncpg.connect(dsn=DB_DSN)
    async with conn.transaction():  # type: asyncpg.transaction.Transaction
        yield conn
    await conn.close()


class Migration:
    _funcs = []

    def __init__(self, func):
        number = int(func.__name__[2:5])
        assert number == len(Migration._funcs)
        Migration._funcs.append(func)

    @classmethod
    async def execute(cls, start: typing.Optional[int], end: typing.Optional[int]):
        for func in cls._funcs[start:end]:
            await func()


@Migration
async def m_000_setup():
    url = "https://postgrespro.com/docs/postgrespro/13/demodb-bookings-installation"
    path_to_demo_small_en_zip = input(
        f"download and unpack sql script from `{url}` and write path to `demo-small-en-20170815.sql`: "
    )
    print(os.popen(f'psql {DB_DSN} -f {path_to_demo_small_en_zip}').read())


def main():
    argv = sys.argv[1:]
    if len(argv) == 2:
        start, end = map(lambda x: None if x == '-' else int(x), argv)
    elif len(argv) == 1:
        start = end = int(argv[0])
    else:
        raise NotImplementedError

    asyncio.run(Migration.execute(start, end))


if __name__ == '__main__':
    main()
