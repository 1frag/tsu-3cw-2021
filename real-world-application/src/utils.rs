use pyo3::{prelude::*, create_exception};
use log;
use pyo3::exceptions::PyException;
use pyo3_asyncio;
use tokio_postgres::{Error, NoTls};
use postgres_types::ToSql;
use once_cell::sync::OnceCell;

#[derive(Clone)]
pub struct Config {
    pub postgres_dsn: String,
    pub utc_tz: PyObject,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

#[pyfunction]
pub fn configure(dsn: String, utc_tz: PyObject) -> bool {
    let obj = Config { postgres_dsn: dsn, utc_tz };
    match CONFIG.set(obj.clone()) {
        Err(_) => false,
        Ok(_) => true,
    }
}

pub fn get_config<'c>() -> &'c Config {
    CONFIG.get().expect("save Config first")
}

pub async fn fetch_all(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Vec<tokio_postgres::Row>, Error> {
    let dsn = get_config().postgres_dsn.as_str();
    let (client, connection) = tokio_postgres::connect(dsn, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("connection error: {}", e);
        }
    });
    client.query(query, params).await
}

pub async fn fetch_one(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<tokio_postgres::Row, Error> {
    let dsn = get_config().postgres_dsn.as_str();
    let (client, connection) = tokio_postgres::connect(dsn, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("connection error: {}", e);
        }
    });
    client.query_one(query, params).await
}

create_exception!(exceptions, QueryException, PyException);

#[pyfunction]
pub fn init(py: Python) -> PyResult<()> {
    pyo3_asyncio::try_init(py)?;
    pyo3_asyncio::tokio::init_multi_thread_once();
    Ok(())
}
