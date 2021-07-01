use pyo3::{prelude::*, create_exception};
use pyo3::exceptions::PyException;
use pyo3_asyncio;
use tokio_postgres::{Error, NoTls};
use postgres_types::ToSql;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use once_cell::sync::OnceCell;
use std::str::FromStr;

#[derive(Clone)]
pub struct Config {
    pub postgres_config: Option<tokio_postgres::config::Config>,
    pub utc_tz: Option<PyObject>,
    pub pool: Option<Pool<PostgresConnectionManager<NoTls>>>,
}

static mut CONFIG: OnceCell<Config> = OnceCell::new();

pub fn get_config<'c>() -> &'c Config {
    unsafe { CONFIG.get() }.expect("save Config first")
}

pub async fn fetch_all(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Vec<tokio_postgres::Row>, Error> {
    let pool = get_config().pool.clone().unwrap();
    let connection = pool.get().await.unwrap();
    connection.query(query, params).await
}

pub async fn fetch_one(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<tokio_postgres::Row, Error> {
    let pool = get_config().pool.clone().unwrap();
    let connection = pool.get().await.unwrap();
    connection.query_one(query, params).await
}

create_exception!(exceptions, QueryException, PyException);
create_exception!(exceptions, ConfigException, PyException);

async fn init_postgres_pool(dsn: String) {
    let postgres_config = tokio_postgres::config::Config::from_str(&dsn).unwrap();
    let pg_mgr = PostgresConnectionManager::new(
        postgres_config.clone(),
        tokio_postgres::NoTls,
    );

    let pool = match Pool::builder().build(pg_mgr).await {
        Ok(pool) => pool,
        Err(e) => panic!("builder error: {:?}", e),
    };

    let mut py_config = unsafe { CONFIG.get_mut() }.unwrap();
    py_config.postgres_config = Some(postgres_config);
    py_config.pool = Some(pool);
}

#[pyfunction]
pub fn configure(py: Python, dsn: String, utc_tz: PyObject) -> PyResult<PyObject> {
    unsafe {
        if CONFIG.set(Config {
            postgres_config: None,
            utc_tz: Some(utc_tz),
            pool: None,
        }).is_err() {
            return Err(ConfigException::new_err(()));
        }
    }
    pyo3_asyncio::try_init(py)?;
    pyo3_asyncio::tokio::init_current_thread_once();

    pyo3_asyncio::tokio::into_coroutine(py, async move {
        init_postgres_pool(dsn).await;
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(py.None())
    })
}
