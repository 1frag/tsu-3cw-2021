import asyncio

import pyo3_fetch_db


async def main():
    pyo3_fetch_db.init()
    lst = await pyo3_fetch_db.fetch_db(flight_id=28935)
    print(lst[0].seat_no)


if __name__ == '__main__':
    asyncio.run(main())
