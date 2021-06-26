use pyo3::{prelude::*, wrap_pyfunction};
use tokio;

#[pyfunction]
fn rust_sleep(py: Python, n: u64) -> PyResult<PyObject> {
    pyo3_asyncio::tokio::into_coroutine(py, async move {
        tokio::time::sleep(std::time::Duration::from_secs(n)).await;
        Ok(Python::with_gil(|py| py.None()))
    })
}

#[pymodule]
fn pyo3_sleeps(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_asyncio::try_init(py)?;
    pyo3_asyncio::tokio::init_multi_thread_once();

    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;

    Ok(())
}
