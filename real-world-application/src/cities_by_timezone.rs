use pyo3::prelude::*;
use std::iter::Iterator;
use pyo3_asyncio;
use tokio_postgres::Error;

use crate::utils;
use py_rwa_macroses::{Iterable, sql};

#[pyclass]
#[derive(Iterable)]
pub struct CityByTimeZone {
    #[pyo3(get)] timezone: String,
    #[pyo3(get)] cities: Vec<String>,
    #[pyo3(get)] count: i64,
}

async fn _cities_by_timezone(search: String) -> Result<Vec<CityByTimeZone>, Error> {
    let rows = utils::fetch_all(sql!(r"
        SELECT
            timezone,
            ARRAY_AGG(city) as cities,
            COUNT(city)
        FROM airports
        WHERE timezone ILIKE '%' || $1::text || '%'
        GROUP BY timezone;
    "), &[&search]).await?;

    Ok(rows.iter().map(|r| {
        CityByTimeZone {
            timezone: r.get(0),
            cities: r.get(1),
            count: r.get(2),
        }
    }).collect())
}

#[pyfunction]
pub fn cities_by_timezone(py: Python, search: Option<String>) -> PyResult<PyObject> {
    let search = match search {
        Some(t) => t,
        None => "".to_string()
    };

    pyo3_asyncio::tokio::into_coroutine(py, async move {
        match _cities_by_timezone(search).await {
            Ok(result) => Python::with_gil(|py| Ok(result.into_py(py))),
            Err(e) => Err(utils::QueryException::new_err(e.to_string())),
        }
    })
}
