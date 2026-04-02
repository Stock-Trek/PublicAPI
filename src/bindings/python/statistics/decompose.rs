#[cfg(feature = "python")]
use pyo3::pyfunction;

#[cfg(feature = "python")]
#[pyfunction]
pub fn seasonal_decompose(
    time_series_values: Vec<f64>,
    seasonal_period_length: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    crate::statistics::decompose::seasonal_decompose(&time_series_values, seasonal_period_length)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn loess_smooth(values: Vec<f64>, span: usize) -> Vec<f64> {
    crate::statistics::decompose::loess_smooth(&values, span)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn seasonal_trend_decomposition_using_loess(
    time_series_values: Vec<f64>,
    seasonal_period_length: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    crate::statistics::decompose::seasonal_trend_decomposition_using_loess(
        &time_series_values,
        seasonal_period_length,
    )
}
