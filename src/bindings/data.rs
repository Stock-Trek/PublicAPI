#[cfg(feature = "python")]
use pyo3::{exceptions::PyTypeError, prelude::*};
#[cfg(feature = "python")]
use rust_decimal::{prelude::ToPrimitive, Decimal};
#[cfg(feature = "python")]
use std::str::FromStr;

#[cfg(feature = "python")]
impl<'a, 'py> FromPyObject<'a, 'py> for crate::data::Real {
    type Error = PyErr;

    fn extract(ob: pyo3::Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(i) = ob.extract::<i32>() {
            return Ok(crate::data::Real::Int(i));
        }
        if let Ok(i) = ob.extract::<i64>() {
            return Ok(crate::data::Real::Long(i));
        }
        if let Ok(f) = ob.extract::<f32>() {
            return Ok(crate::data::Real::Float(f));
        }
        if let Ok(f) = ob.extract::<f64>() {
            return Ok(crate::data::Real::Double(f));
        }
        if let Ok(s) = ob.extract::<String>() {
            if let Ok(d) = Decimal::from_str(&s) {
                return Ok(crate::data::Real::Decimal(d));
            }
        }

        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected int, long, float, double, or decimal string",
        ))
    }
}

#[cfg(feature = "python")]
#[pyclass]
pub struct Datum {
    inner: crate::data::Datum,
}

#[cfg(feature = "python")]
#[pymethods]
impl Datum {
    #[new]
    pub fn new(timestamp: i64, value: &Bound<'_, PyAny>) -> PyResult<Self> {
        use crate::data::Real;

        let real = if let Ok(i) = value.extract::<i32>() {
            Real::Int(i)
        } else if let Ok(i) = value.extract::<i64>() {
            Real::Long(i)
        } else if let Ok(f) = value.extract::<f64>() {
            Real::Double(f)
        } else if let Ok(s) = value.extract::<String>() {
            if let Ok(d) = Decimal::from_str(&s) {
                Real::Decimal(d)
            } else {
                return Err(PyTypeError::new_err(
                    "Value must be a number or decimal string",
                ));
            }
        } else {
            return Err(PyTypeError::new_err(
                "Value must be a number (int, float) or decimal string",
            ));
        };

        Ok(Self {
            inner: crate::data::Datum::new(timestamp, real),
        })
    }

    pub fn timestamp(&self) -> i64 {
        self.inner.timestamp()
    }

    pub fn value(&self) -> f64 {
        match self.inner.value() {
            crate::data::Real::Double(f) => f,
            crate::data::Real::Float(f) => f as f64,
            crate::data::Real::Int(i) => i as f64,
            crate::data::Real::Long(l) => l as f64,
            crate::data::Real::Decimal(d) => d.to_f64().unwrap_or(0.0),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Datum(timestamp={}, value={})",
            self.timestamp(),
            self.value()
        )
    }
}

#[cfg(feature = "python")]
#[pyclass]
pub struct TimeSeries {
    inner: crate::data::TimeSeries,
}

#[cfg(feature = "python")]
#[pymethods]
impl TimeSeries {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: crate::data::TimeSeries::new(),
        }
    }

    #[staticmethod]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: crate::data::TimeSeries::with_capacity(capacity),
        }
    }

    pub fn add_datum(&mut self, datum: &Datum) {
        self.inner.add_datum(datum.inner.clone());
    }

    pub fn get_datum(&self, index: usize) -> Option<Datum> {
        self.inner
            .get_datum(index)
            .map(|d| Datum { inner: d.clone() })
    }

    pub fn data(&self) -> Vec<Datum> {
        self.inner
            .data()
            .iter()
            .map(|d| Datum { inner: d.clone() })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(feature = "python")]
pub fn create_module(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "data")?;
    module.add_class::<Datum>()?;
    module.add_class::<TimeSeries>()?;
    Ok(module)
}
