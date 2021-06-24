#![feature(in_band_lifetimes)]

mod utils;
mod adapter;
mod cities_by_timezone;
mod get_bookings;
mod flight_by_min_duration;

use pyo3::prelude::*;
use pyo3::derive_utils::PyFunctionArguments;

#[pymodule]
fn real_world_application(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add("QueryException", py.get_type::<utils::QueryException>())?;

    m.add_function(utils::__pyo3_get_function_init(PyFunctionArguments::PyModule(m))?)?;
    m.add_function(utils::__pyo3_get_function_configure(PyFunctionArguments::PyModule(m))?)?;
    m.add_function(cities_by_timezone::__pyo3_get_function_cities_by_timezone(PyFunctionArguments::PyModule(m))?)?;
    m.add_function(get_bookings::__pyo3_get_function_get_bookings(PyFunctionArguments::PyModule(m))?)?;
    m.add_function(flight_by_min_duration::__pyo3_get_function_flight_by_min_duration(PyFunctionArguments::PyModule(m))?)?;

    m.add_class::<flight_by_min_duration::Flight>()?;
    Ok(())
}
