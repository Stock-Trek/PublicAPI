#[cfg(feature = "python")]
use {
    crate::statistics::exponential_smoothing,
    pyo3::{pyclass, pymethods},
};

#[cfg(feature = "python")]
#[pyclass(name = "ExponentialSmoothing")]
pub struct PyExponentialSmoothing;

#[cfg(feature = "python")]
#[pymethods]
impl PyExponentialSmoothing {
    pub fn holt_linear_trend(
        &self,
        time_series_values: Vec<f64>,
        alpha: f64,
        beta: f64,
    ) -> Vec<f64> {
        exponential_smoothing::holt_linear_trend(&time_series_values, alpha, beta)
    }
    pub fn holt_winters(
        &self,
        time_series_values: Vec<f64>,
        alpha: f64,
        beta: f64,
        gamma: f64,
        season_len: usize,
        multiplicative: bool,
    ) -> Vec<f64> {
        exponential_smoothing::holt_winters(
            &time_series_values,
            alpha,
            beta,
            gamma,
            season_len,
            multiplicative,
        )
    }
    pub fn simple_exponential_smoothing(
        &self,
        time_series_values: Vec<f64>,
        alpha: f64,
    ) -> Vec<f64> {
        exponential_smoothing::simple_exponential_smoothing(&time_series_values, alpha)
    }
}
