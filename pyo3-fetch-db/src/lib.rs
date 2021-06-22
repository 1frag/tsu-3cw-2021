use pyo3::{prelude::*, wrap_pyfunction, create_exception};
use log;
use pyo3_log;
use pyo3::exceptions::PyException;
use pyo3::class::iter::PyIterProtocol;
use std::iter::Iterator;
use pyo3_asyncio;
use tokio_postgres::{Error, NoTls};
use postgres_types::ToSql;
use std::collections::HashMap;

const CONFIG: &str = "host=localhost port=5438 user=postgres password=postgres dbname=demo";

async fn fetchall(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Vec<tokio_postgres::Row>, Error> {
    let (client, connection) =
        tokio_postgres::connect(CONFIG, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("connection error: {}", e);
        }
    });
    client.query(query, params).await
}

async fn rust_fetch_db(flight_id: i32) -> Result<Vec<Row>, Error> {
    let rows = fetchall(r"
        SELECT t.ticket_no,
               b.seat_no,
               t.passenger_name,
               t.contact_data
        FROM boarding_passes b
        LEFT JOIN tickets t ON b.ticket_no = t.ticket_no
        WHERE b.flight_id = $1::int;
    ", &[&flight_id]).await?;

    Ok(rows.iter().map(|r| {
        let contact_data: serde_json::Value = r.get(3);
        let map: HashMap<String, String> =
            serde_json::from_value(contact_data)
                .expect("Could not convert contact_data to hashmap");

        Row {
            ticket_no: r.get(0),
            seat_no: r.get(1),
            passenger_name: r.get(2),
            contact_data: map,
        }
    }).collect())
}

create_exception!(exceptions, QueryException, PyException);

#[pyfunction]
fn fetch_db(py: Python, flight_id: &PyAny) -> PyResult<PyObject> {
    let flight_id = flight_id.extract()?;

    pyo3_asyncio::tokio::into_coroutine(py, async move {
        match rust_fetch_db(flight_id).await {
            Ok(result) => Python::with_gil(|py| Ok(result.into_py(py))),
            Err(e) => Err(QueryException::new_err(e.to_string())),
        }
    })
}

#[pyfunction]
fn init(py: Python) -> PyResult<()> {
    pyo3_asyncio::try_init(py)?;
    pyo3_asyncio::tokio::init_multi_thread();
    Ok(())
}

#[pyclass]
struct Row {
    #[pyo3(get)] ticket_no: String,
    #[pyo3(get)] seat_no: String,
    #[pyo3(get)] passenger_name: String,
    #[pyo3(get)] contact_data: HashMap<String, String>,
}

#[pyproto]
impl PyIterProtocol for Row {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PyAny>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let fields: [(&str, PyObject); 4] = [
            ("ticket_no", slf.ticket_no.clone().into_py(py)),
            ("seat_no", slf.seat_no.clone().into_py(py)),
            ("passenger_name", slf.passenger_name.clone().into_py(py)),
            ("contact_data", slf.contact_data.clone().into_py(py))
        ];
        Ok(fields.to_object(py).getattr(py, "__iter__")?.call0(py)?)
    }
}

#[pymodule]
fn pyo3_fetch_db(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add("QueryException", py.get_type::<QueryException>())?;
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_db, m)?)?;
    m.add_class::<Row>()?;
    Ok(())
}
