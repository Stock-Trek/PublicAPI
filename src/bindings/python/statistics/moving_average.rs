#[cfg(feature = "python")]
use {
    crate::statistics::moving_average,
    pyo3::{pyclass, pymethods},
};

#[cfg(feature = "python")]
#[pyclass(name = "MovingAverage")]
pub struct PyMovingAverage;

#[cfg(feature = "python")]
#[pymethods]
impl PyMovingAverage {
    pub fn exponential_moving_average(&self, time_series_values: Vec<f64>, alpha: f64) -> Vec<f64> {
        moving_average::exponential_moving_average(&time_series_values, alpha)
    }
    pub fn simple_moving_average(
        &self,
        time_series_values: Vec<f64>,
        window_size: usize,
    ) -> Vec<f64> {
        moving_average::simple_moving_average(&time_series_values, window_size)
    }
    pub fn weighted_moving_average(
        &self,
        time_series_values: Vec<f64>,
        weight_values: Vec<f64>,
    ) -> Vec<f64> {
        moving_average::weighted_moving_average(&time_series_values, &weight_values)
    }
}
