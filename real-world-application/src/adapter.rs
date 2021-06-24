use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDelta};
use chrono::{Datelike, Timelike};
use tokio_postgres::Row;
use tokio_postgres::types::FromSql;

use crate::utils;

#[derive(Copy, Clone)]
pub struct Adaper<'p> {
    py: Python<'p>,
    row: Option<&'p Row>,
    idx: usize,
}

pub trait Adaptable {
    fn adaptable(row: Option<&Row>, idx: usize, py: Python) -> Self;
}

impl Adaper<'p> {
    pub fn new(py: Python<'p>) -> Self {
        Adaper { py, row: None, idx: 0 }
    }

    pub fn change_row(&mut self, row: Option<&'p Row>) -> Self {
        self.row = row;
        self.idx = 0;
        *self
    }

    pub fn to_f64(&mut self) -> f64 {
        let s: String = self.row.unwrap().get(self.idx);
        self.idx += 1;
        s.parse::<f64>().unwrap()
    }

    pub fn to_from_sql<T>(&mut self) -> T where T: FromSql<'p> {
        let s: T = self.row.unwrap().get(self.idx);
        self.idx += 1;
        s
    }

    pub fn to_date(&mut self) -> PyObject {
        let d: chrono::DateTime<chrono::offset::Utc> = self.row.unwrap().get(self.idx);
        self.idx += 1;
        PyDateTime::new(
            self.py,
            d.year(),
            d.month() as u8,
            d.day() as u8,
            d.hour() as u8,
            d.minute() as u8,
            d.second() as u8,
            d.timestamp_subsec_micros(),
            Some(&utils::get_config().utc_tz),
        ).unwrap().to_object(self.py)
    }

    pub fn to_time_delta(&mut self) -> PyObject {
        let d: f64 = self.row.unwrap().get(self.idx);
        self.idx += 1;
        PyDelta::new(
            self.py,
            0,
            d.floor() as i32,
            1_000_000 * ((d - d.floor()).ceil() as i32),
            true,
        ).unwrap().to_object(self.py)
    }

    pub fn to_string_(&mut self) -> String {
        let d: String = self.row.unwrap().get(self.idx);
        self.idx += 1;
        d
    }

    pub fn to_custom_type<T>(&mut self) -> T where T: Adaptable {
        let d = T::adaptable(self.row, self.idx, self.py);
        self.idx += 1;
        d
    }
}
