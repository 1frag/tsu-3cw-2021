use pyo3::{
    prelude::*,
    wrap_pyfunction,
};
use num_bigint::BigInt;

#[pyclass]
struct Item {
    #[pyo3(get, set)] a: BigInt,
    #[pyo3(get, set)] b: String,
    #[pyo3(get, set)] c: bool,
}

#[pyfunction]
fn collect(n: i64) -> Vec<Item> {
    (0..n).map(|i| {
        Item {
            a: BigInt::from(i),
            b: i.to_string(),
            c: (i & 1) == 1,
        }
    }).collect()
}

#[pymodule]
fn pyo3_class_creating(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Item>()?;
    m.add_function(wrap_pyfunction!(collect, m)?)?;
    Ok(())
}
