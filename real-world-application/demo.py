import asyncio

import real_world_application as rwa
import datetime
from pprint import pprint


def demo(title, obj, *, isend=False):
    print('-' * 20, title, '-' * 20)
    pprint(obj)
    if not isend:
        print('\n' * 3)


async def main():
    await rwa.configure(
        dsn="postgresql://postgres:postgres@0.0.0.0:5438/demo",
        utc_tz=datetime.timezone.utc,
    )

    demo('cities_by_timezone', [dict(row) for row in await rwa.cities_by_timezone()])
    demo('get_bookings', [dict(row) for row in await rwa.get_bookings(1000, 5000)][:8])
    demo('flight_by_min_duration', dict(await rwa.flight_by_min_duration('DME', 'TOF', '[2000-01-01, 2020-01-01]')))
    demo('get_flights', [dict(row) for row in await rwa.get_flights(8)])
    demo('select_1', await rwa.select_1(), isend=True)


if __name__ == '__main__':
    asyncio.run(main())
