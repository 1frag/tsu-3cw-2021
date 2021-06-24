use pyo3::prelude::*;
use pyo3::PyIterProtocol;
use std::iter::Iterator;
use pyo3_asyncio;
use tokio_postgres::{Error, Row};

use py_rwa_macroses::{EnumToPyObject, Iterable};
use crate::utils;
use crate::adapter::{Adaper, Adaptable};


#[derive(Clone)]
#[derive(EnumToPyObject)]
pub enum StatusFlight {
    Departed,
    Arrived,
    OnTime,
    Cancelled,
    Delayed,
    Scheduled,
}

#[pyclass]
#[derive(Iterable)]
pub struct Flight {
    #[pyo3(get)] flight_id: i32,
    #[pyo3(get)] flight_no: String,
    #[pyo3(get)] status: StatusFlight,
    #[pyo3(get)] duration: PyObject,
    #[pyo3(get)] actual_arrival: PyObject,
}

async fn _flight_by_min_duration(
    departure_airport: String,
    arrival_airport: String,
    target_daterange: String,
) -> Result<Flight, Error> {
    let row = utils::fetch_one(r"
        SELECT
            flight_id,
            flight_no,
            status,
            EXTRACT(EPOCH FROM scheduled_arrival - scheduled_departure) AS duration,
            actual_arrival
        FROM flights
        WHERE
            departure_airport = $1::text
            AND arrival_airport = $2::text
            AND DATERANGE(
                scheduled_departure::date, scheduled_arrival::date, '[]'
            ) && $3::text::daterange
        ORDER BY duration, scheduled_departure
        LIMIT 1
    ", &[&departure_airport, &arrival_airport, &target_daterange]).await?;

    let gil = Python::acquire_gil();
    let py = gil.python();
    let mut adapter = Adaper::new(py).change_row(Some(&row));

    Ok(
        Flight {
            flight_id: adapter.to_from_sql::<i32>(),
            flight_no: adapter.to_string_(),
            status: adapter.to_custom_type::<StatusFlight>(),
            duration: adapter.to_time_delta(),
            actual_arrival: adapter.to_date(),
        }
    )
}

#[pyfunction]
pub fn flight_by_min_duration(
    py: Python,
    departure_airport: String,
    arrival_airport: String,
    target_daterange: String,
) -> PyResult<PyObject> {
    pyo3_asyncio::tokio::into_coroutine(py, async move {
        match _flight_by_min_duration(departure_airport, arrival_airport, target_daterange).await {
            Ok(result) => Python::with_gil(|py| Ok(result.into_py(py))),
            Err(e) => Err(utils::QueryException::new_err(e.to_string())),
        }
    })
}
