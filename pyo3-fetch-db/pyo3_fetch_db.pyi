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
* строки
* числа
* json(b)
* bool
* списки
* uuid
* timedelta
* numeric
"""
