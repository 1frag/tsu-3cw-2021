use pyo3::prelude::*;
use std::iter::Iterator;
use pyo3_asyncio;
use tokio_postgres::Error;

use crate::utils;
use py_rwa_macroses::{Iterable, sql};

#[pyclass]
#[derive(Iterable)]
pub struct Number {
    #[pyo3(get)] number: i32,
}

async fn _select_1() -> Result<Number, Error> {
    let row = utils::fetch_one(sql!("SELECT 1, 2;"), &[]).await?;

    Ok(Number { number: row.get(0) })
}

#[pyfunction]
pub fn select_1(py: Python) -> PyResult<PyObject> {
    pyo3_asyncio::tokio::into_coroutine(py, async move {
        match _select_1().await {
            Ok(result) => Python::with_gil(|py| Ok(result.into_py(py))),
            Err(e) => Err(utils::QueryException::new_err(e.to_string())),
        }
    })
}
