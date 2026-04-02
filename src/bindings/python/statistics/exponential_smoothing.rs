#[cfg(feature = "python")]
use pyo3::pyfunction;

#[cfg(feature = "python")]
#[pyfunction]
pub fn simple_exponential_smoothing(time_series_values: Vec<f64>, alpha: f64) -> Vec<f64> {
    crate::statistics::exponential_smoothing::simple_exponential_smoothing(
        &time_series_values,
        alpha,
    )
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn holt_linear_trend(time_series_values: Vec<f64>, alpha: f64, beta: f64) -> Vec<f64> {
    crate::statistics::exponential_smoothing::holt_linear_trend(&time_series_values, alpha, beta)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn holt_winters(
    time_series_values: Vec<f64>,
    alpha: f64,
    beta: f64,
    gamma: f64,
    season_len: usize,
    multiplicative: bool,
) -> Vec<f64> {
    crate::statistics::exponential_smoothing::holt_winters(
        &time_series_values,
        alpha,
        beta,
        gamma,
        season_len,
        multiplicative,
    )
}
