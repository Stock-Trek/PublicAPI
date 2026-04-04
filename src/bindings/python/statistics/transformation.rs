#[cfg(feature = "python")]
use {
    crate::statistics::transformation,
    pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult},
};

#[cfg(feature = "python")]
#[pyclass(name = "Transformation")]
pub struct PyTransformation;

#[cfg(feature = "python")]
#[pymethods]
impl PyTransformation {
    pub fn box_cox(&self, values: Vec<f64>, lambda: f64) -> PyResult<Vec<f64>> {
        transformation::box_cox(&values, lambda).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn detrend_linear(&self, values: Vec<f64>) -> PyResult<Vec<f64>> {
        transformation::detrend_linear(&values).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn difference(&self, values: Vec<f64>, order: usize) -> PyResult<Vec<f64>> {
        transformation::difference(&values, order)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn lag(&self, values: Vec<f64>, lag: usize) -> PyResult<Vec<Option<f64>>> {
        transformation::lag(&values, lag).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn logarithm(&self, values: Vec<f64>) -> PyResult<Vec<f64>> {
        transformation::logarithm(&values).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn rolling_mean(&self, values: Vec<f64>, window: usize) -> PyResult<Vec<f64>> {
        transformation::rolling_mean(&values, window)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn rolling_standard_deviation(
        &self,
        values: Vec<f64>,
        window: usize,
    ) -> PyResult<Vec<f64>> {
        transformation::rolling_standard_deviation(&values, window)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn seasonal_difference(&self, values: Vec<f64>, period: usize) -> PyResult<Vec<f64>> {
        transformation::seasonal_difference(&values, period)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}
