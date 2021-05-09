use pyo3::{
    prelude::*,
    wrap_pyfunction,
};
use num_bigint::{BigInt, ToBigInt};

#[pyfunction]
fn next_prime(x: BigInt) -> BigInt {
    let zero = 0.to_bigint().unwrap();
    let one = 1.to_bigint().unwrap();
    let two = 2.to_bigint().unwrap();

    let mut i = &x + &one;
    loop {
        let _lim = &i.sqrt() + &one;
        let mut j = two.clone();
        while j <= _lim {
            if j == _lim { return i.clone(); }
            if &i % &j == zero { break; }
            j += &one;
        }
        i += &one;
    }
}

#[pymodule]
fn using_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(next_prime, m)?)?;
    Ok(())
}
