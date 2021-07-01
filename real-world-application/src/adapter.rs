use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDelta};
use chrono::{Datelike, Timelike};
use tokio_postgres::Row;
use tokio_postgres::types::FromSql;

use crate::utils;

#[derive(Copy, Clone)]
pub struct Adaper<'p, 'r> {
    py: Python<'p>,
    row: Option<&'r Row>,
    idx: usize,
}

impl Adaper<'p, 'r> {
    pub fn new(py: Python<'p>) -> Self {
        Adaper { py, row: None, idx: 0 }
    }

    pub fn change_row(&mut self, row: Option<&'r Row>) -> Self {
        self.row = row;
        self.idx = 0;
        *self
    }

    pub fn next<T>(&mut self) -> T where T: FromSql<'r> {
        self.idx += 1;
        self.row.unwrap().get(self.idx - 1)
    }

    pub fn next_f64(&mut self) -> f64 {
        let s: String = self.next();
        s.parse::<f64>().unwrap()
    }

    pub fn next_date(&mut self) -> PyObject {
        let t: Option<chrono::DateTime<chrono::offset::Utc>> = self.next();

        match t {
            None => self.py.None(),
            Some(d) => {
                PyDateTime::new(
                    self.py,
                    d.year(),
                    d.month() as u8,
                    d.day() as u8,
                    d.hour() as u8,
                    d.minute() as u8,
                    d.second() as u8,
                    d.timestamp_subsec_micros(),
                    Some(&utils::get_config().clone().utc_tz.unwrap()),
                ).unwrap().to_object(self.py)
            }
        }
    }

    pub fn next_timedelta(&mut self) -> PyObject {
        let d: f64 = self.next();
        PyDelta::new(
            self.py,
            0,
            d.floor() as i32,
            1_000_000 * ((d - d.floor()).ceil() as i32),
            true,
        ).unwrap().to_object(self.py)
    }
}
