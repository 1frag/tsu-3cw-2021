#![feature(in_band_lifetimes)]

mod utils;
mod adapter;
mod cities_by_timezone;
mod get_bookings;
mod flight_by_min_duration;

use pyo3::prelude::*;
use pyo3::derive_utils::PyFunctionArguments;
use py_rwa_macroses::add_functions;

#[pymodule]
fn real_world_application(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    add_functions!(m,
        [init, configure] from utils,
        [cities_by_timezone] from cities_by_timezone,
        [get_bookings] from get_bookings,
        [flight_by_min_duration] from flight_by_min_duration,
    );

    m.add("QueryException", py.get_type::<utils::QueryException>())?;

    m.add_class::<cities_by_timezone::CityByTimeZone>()?;
    m.add_class::<get_bookings::Booking>()?;
    m.add_class::<flight_by_min_duration::Flight>()?;

    Ok(())
}
