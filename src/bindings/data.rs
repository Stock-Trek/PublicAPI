#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use rust_decimal::Decimal;
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
pub fn add_submodule(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let data = PyModule::new(py, "data")?;
    data.add_class::<Datum>()?;
    data.add_class::<TimeSeries>()?;
    parent_module.add_submodule(&data)?;
    Ok(())
}
