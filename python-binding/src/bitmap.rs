use std::fmt;

use pyo3::{exceptions::PyValueError, prelude::*};
use roaring::bitmap::RoaringBitmap;

#[pyclass(name = "Bitmap")]
#[derive(Debug)]
pub struct PyBitmap {
    inner: RoaringBitmap,
}

#[pymethods]
impl PyBitmap {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(PyBitmap {
            inner: RoaringBitmap::new(),
        })
    }

    pub fn insert(&mut self, value: u32) -> PyResult<bool> {
        Ok(self.inner.insert(value))
    }

    pub fn append(&mut self, values: Vec<u32>) -> PyResult<u64> {
        self.inner
            .append(values)
            .map_err(|_| PyValueError::new_err("Failed to append values"))
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn contains(&self, value: u32) -> PyResult<bool> {
        Ok(self.inner.contains(value))
    }

    pub fn len(&self) -> PyResult<u64> {
        Ok(self.inner.len())
    }

    pub fn is_empty(&self) -> PyResult<bool> {
        Ok(self.inner.is_empty())
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

impl fmt::Display for PyBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
