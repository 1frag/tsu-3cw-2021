use pyo3::prelude::*;
use std::iter::Iterator;
use pyo3_asyncio;
use tokio_postgres::Error;

use crate::utils;
use py_rwa_macroses::{Iterable, sql};
use crate::adapter::Adaper;
use pyo3::types::{PyDict, PyType};

/*
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

#[pydantic(read_only = {flight_id, flight_no})]
pub struct _Flight {
    #[(gte = 1)] flight_id: i32,
    flight_no: String,
    #[(default_factory = @datetime_now, type = datetime.datetime)] created_at: PyObject,
    #[(type = datetime.datetime)] scheduled_departure: PyObject,
    #[(type = datetime.datetime)] scheduled_arrival: PyObject,
    #[(example = "TOF")] departure_airport: String,
    arrival_airport: String,
    #[(default = @get_random_status)] status: StatusFlight,
    aircraft_code: String,
    #[(type = Optional[datetime.datetime])] actual_departure: PyObject,
    #[(type = Optional[datetime.datetime])] actual_arrival: PyObject,
}*/

#[pyclass]
#[derive(Iterable)]
pub struct Flight2 {
    #[pyo3(get)] flight_id: i32,
    #[pyo3(get)] flight_no: String,
    #[pyo3(get, set)] scheduled_departure: PyObject,
    #[pyo3(get, set)] scheduled_arrival: PyObject,
    #[pyo3(get, set)] departure_airport: String,
    #[pyo3(get, set)] arrival_airport: String,
    #[pyo3(get, set)] status: String,
    #[pyo3(get, set)] aircraft_code: String,
    #[pyo3(get, set)] actual_departure: PyObject,
    #[pyo3(get, set)] actual_arrival: PyObject,
}

#[pymethods]
impl Flight2 {
    #[classmethod]
    fn pydantic(_cls: &PyType, py: Python) -> PyResult<PyObject> {
        let annotations = PyDict::new(py);
        annotations.set_item("flight_id", py.eval("int", None, None)?)?;

        let kwds = PyDict::new(py);
        kwds.set_item("__annotations__", annotations)?;
        kwds.set_item("flight_id", py.None())?;

        let pydantic = py.import("pydantic")?;
        let locals = PyDict::new(py);
        locals.set_item("kwds", kwds)?;
        locals.set_item("pydantic", pydantic)?;

        py.eval(
            format!("type('Flight', (pydantic.BaseModel,), kwds)").as_str(),
            None,
            Some(locals),
        )?.extract::<PyObject>()
    }
}

async fn _get_flights_v2(limit: i32) -> Result<Vec<Flight2>, Error> {
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
        Flight2 {
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
pub fn get_flights_v2(py: Python, limit: i32) -> PyResult<PyObject> {
    pyo3_asyncio::tokio::into_coroutine(py, async move {
        match _get_flights_v2(limit).await {
            Ok(result) => Python::with_gil(|py| Ok(result.into_py(py))),
            Err(e) => Err(utils::QueryException::new_err(e.to_string())),
        }
    })
}
