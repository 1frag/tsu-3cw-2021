import time

import asyncpg
from fastapi import FastAPI
from fastapi_code_generator.parser import Request
from pydantic import BaseModel
import orjson as json
import pyo3_fetch_db

pyo3_fetch_db.init()

app = FastAPI()
CONFIG: str = "postgres://postgres:postgres@localhost:5438/demo"


class Row(BaseModel):
    ticket_no: str
    seat_no: str
    passenger_name: str
    contact_data: dict[str, str]


async def fetch_db(flight_id) -> list[Row]:
    conn: asyncpg.Connection = await asyncpg.connect(CONFIG)
    try:
        await conn.set_type_codec(
            'jsonb',
            encoder=lambda *a, **kw: json.dumps(*a, **kw).decode(),
            decoder=json.loads,
            schema='pg_catalog'
        )

        rows = await conn.fetch("""
            SELECT t.ticket_no,
                   b.seat_no,
                   t.passenger_name,
                   t.contact_data
            FROM boarding_passes b
            LEFT JOIN tickets t ON b.ticket_no = t.ticket_no
            WHERE b.flight_id = $1;
        """, flight_id)
        return [Row(**row) for row in rows]
    finally:
        await conn.close()


@app.middleware("http")
async def add_process_time_header(request: Request, call_next):
    start_time = time.time()
    response = await call_next(request)
    process_time = time.time() - start_time
    response.headers["X-Process-Time"] = str(process_time)
    return response


@app.get('/basic')
async def basic(flight_id: int) -> list[Row]:
    return await fetch_db(flight_id)


@app.get('/ffi')
async def fii(flight_id: int) -> list[pyo3_fetch_db.Row]:
    return await pyo3_fetch_db.fetch_db(flight_id)
