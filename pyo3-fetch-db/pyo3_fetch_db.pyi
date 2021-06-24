from typing import List, Dict


class QueryException(Exception):
    pass


def init() -> None:
    """Initialize event loop of Rust"""


class Row:
    ticket_no: str = property(...)
    seat_no: str = property(...)
    passenger_name: str = property(...)
    contact_data: Dict[str, str] = property(...)


async def fetch_db(flight_id: int) -> List[Row]:
    """Returns list of people who are on flight :param flight_id"""


"""
План:
* Конструктор кода
    * модули
    * стейт
    * миграции
    * pyi
    * pydantic
* Брокеры сообщений
* Веб интеграции

типы данных:
* строки        ✓
* числа             (1)
* json(b)       ✓
* bool
* списки            (1)
* uuid
* timedelta                     (3)
* numeric               (2)
* datetime                      (3)
* optional                      (3)

проверить при увелечении строчек

(1) cities_by_timezone
select timezone, array_agg(city) as cities, count(city) from airports group by timezone;
(2) get_bookings
select * from bookings where total_amount between 10 and 15;
(3) flights_duration
select flight_id, flight_no, status, scheduled_arrival - scheduled_departure as duration, actual_arrival
from flights where departure_airport = 'DME'
and arrival_airport = 'TOF'
and daterange(scheduled_departure::date, scheduled_arrival::date, '[]')
&& '[2017-08-01, 2017-08-31]'::daterange
order by scheduled_departure;
(4)
добавить таблицу отзывов о полете


сериалайзинг?

"""
