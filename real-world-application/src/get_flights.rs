use pyo3::prelude::*;
use std::iter::Iterator;
use pyo3_asyncio;
use tokio_postgres::Error;

use crate::utils;
use py_rwa_macroses::{Iterable, sql};
use crate::adapter::Adaper;

#[pyclass]
#[derive(Iterable)]
pub struct Flight {
    #[pyo3(get)] flight_id: i32,
    #[pyo3(get)] flight_no: String,
    #[pyo3(get)] scheduled_departure: PyObject, // datetime
    #[pyo3(get)] scheduled_arrival: PyObject, // datetime
    #[pyo3(get)] departure_airport: String,
    #[pyo3(get)] arrival_airport: String,
    #[pyo3(get)] status: String,
    #[pyo3(get)] aircraft_code: String,
    #[pyo3(get)] actual_departure: PyObject, // Optional[datetime]
    #[pyo3(get)] actual_arrival: PyObject, // Optional[datetime]
}

async fn _get_flights(limit: i32) -> Result<Vec<Flight>, Error> {
    let rows = utils::fetch_all(sql!(r"
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
    "), &[&limit]).await?;

    let gil = Python::acquire_gil();
    let py = gil.python();
    let mut adapter = Adaper::new(py);

    Ok(rows.iter().map(|r| {
        adapter.change_row(Some(r));
        Flight {
            flight_id: adapter.next(),
            flight_no: adapter.next(),
            scheduled_departure: adapter.next_date(),
            scheduled_arrival: adapter.next_date(),
            departure_airport: adapter.next(),
            arrival_airport: adapter.next(),
            status: adapter.next(),
            aircraft_code: adapter.next(),
            actual_departure: adapter.next_date(),
            actual_arrival: adapter.next_date(),
        }
    }).collect())
}

#[pyfunction]
pub fn get_flights(py: Python, limit: i32) -> PyResult<PyObject> {
    pyo3_asyncio::tokio::into_coroutine(py, async move {
        match _get_flights(limit).await {
            Ok(result) => Python::with_gil(|py| Ok(result.into_py(py))),
            Err(e) => Err(utils::QueryException::new_err(e.to_string())),
        }
    })
}
