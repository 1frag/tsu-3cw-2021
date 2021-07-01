import datetime
import enum
from contextlib import asynccontextmanager
from contextvars import ContextVar
from typing import Optional

import asyncpg
from pydantic import BaseModel, Field

DSN = 'postgresql://postgres:postgres@0.0.0.0:5438/demo'
_pool: ContextVar[Optional[asyncpg.Pool]] = ContextVar('_pool')


@asynccontextmanager
async def pool():
    async with asyncpg.create_pool(dsn=DSN) as __pool:
        yield _pool.set(__pool).var.get()


class Status(str, enum.Enum):
    Departed = 'Departed'
    Arrived = 'Arrived'
    OnTime = 'On Time'
    Cancelled = 'Cancelled'
    Delayed = 'Delayed'
    Scheduled = 'Scheduled'


class Flight(BaseModel):
    flight_id: int = Field(..., read_only=True)
    flight_no: str
    scheduled_departure: datetime.datetime
    scheduled_arrival: datetime.datetime
    departure_airport: str = Field(..., example='TOF')
    arrival_airport: str
    status: Status
    aircraft_code: str
    actual_departure: Optional[datetime.datetime]
    actual_arrival: Optional[datetime.datetime]


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


class FlightLight:
    def __init__(self, flight_id: int, flight_no: str, scheduled_departure: datetime.datetime,
                 scheduled_arrival: datetime.datetime, departure_airport: str, arrival_airport: str, status: str,
                 aircraft_code: str, actual_departure: Optional[datetime.datetime],
                 actual_arrival: Optional[datetime.datetime]):
        self.flight_id = flight_id
        self.flight_no = flight_no
        self.scheduled_departure = scheduled_departure
        self.scheduled_arrival = scheduled_arrival
        self.departure_airport = departure_airport
        self.arrival_airport = arrival_airport
        self.status = status
        self.aircraft_code = aircraft_code
        self.actual_departure = actual_departure
        self.actual_arrival = actual_arrival
