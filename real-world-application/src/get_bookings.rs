use pyo3::prelude::*;
use std::iter::Iterator;
use pyo3_asyncio;
use tokio_postgres::Error;

use crate::utils;
use py_rwa_macroses::Iterable;
use crate::adapter::Adaper;

#[pyclass]
#[derive(Iterable)]
struct Booking {
    #[pyo3(get)] book_ref: String,
    #[pyo3(get)] book_date: PyObject,
    #[pyo3(get)] total_amount: f64,
}

async fn _get_bookings(lo: f64, hi: f64) -> Result<Vec<Booking>, Error> {
    let rows = utils::fetch_all(r"
        SELECT
            book_ref,
            book_date,
            total_amount::text
        FROM bookings
        WHERE total_amount BETWEEN $1::text::numeric AND $2::text::numeric
    ", &[&lo.to_string(), &hi.to_string()]).await?;

    let gil = Python::acquire_gil();
    let py = gil.python();
    let mut adapter = Adaper::new(py);

    Ok(rows.iter().map(|r| {
        adapter.change_row(Some(r));
        Booking {
            book_ref: adapter.to_string_(),
            book_date: adapter.to_date(),
            total_amount: adapter.to_f64(),
        }
    }).collect())
}

#[pyfunction]
pub fn get_bookings(py: Python, lo: f64, hi: f64) -> PyResult<PyObject> {
    pyo3_asyncio::tokio::into_coroutine(py, async move {
        match _get_bookings(lo, hi).await {
            Ok(result) => Python::with_gil(|py| Ok(result.into_py(py))),
            Err(e) => Err(utils::QueryException::new_err(e.to_string())),
        }
    })
}
