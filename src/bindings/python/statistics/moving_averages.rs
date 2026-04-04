#[cfg(feature = "python")]
use pyo3::pyfunction;

#[cfg(feature = "python")]
#[pyfunction]
pub fn simple_moving_average(time_series_values: Vec<f64>, window_size: usize) -> Vec<f64> {
    crate::statistics::moving_average::simple_moving_average(&time_series_values, window_size)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn weighted_moving_average(time_series_values: Vec<f64>, weight_values: Vec<f64>) -> Vec<f64> {
    crate::statistics::moving_average::weighted_moving_average(&time_series_values, &weight_values)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn exponential_moving_average(time_series_values: Vec<f64>, alpha: f64) -> Vec<f64> {
    crate::statistics::moving_average::exponential_moving_average(&time_series_values, alpha)
}
