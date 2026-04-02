#[cfg(feature = "python")]
use {num_complex::Complex, pyo3::pyfunction};

#[cfg(feature = "python")]
#[pyfunction]
pub fn discrete_fourier_transform(time_series_values: Vec<f64>) -> Vec<(f64, f64)> {
    crate::statistics::frequency::discrete_fourier_transform(&time_series_values)
        .iter()
        .map(|c| (c.re, c.im))
        .collect()
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn inverse_discrete_fourier_transform(frequency_domain_values: Vec<(f64, f64)>) -> Vec<f64> {
    let complex_values: Vec<Complex<f64>> = frequency_domain_values
        .iter()
        .map(|(re, im)| Complex::new(*re, *im))
        .collect();
    crate::statistics::frequency::inverse_discrete_fourier_transform(&complex_values)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn periodogram(time_series_values: Vec<f64>, sampling_frequency: f64) -> Vec<f64> {
    crate::statistics::frequency::periodogram(&time_series_values, sampling_frequency)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn spectral_density(time_series_values: Vec<f64>) -> Vec<f64> {
    crate::statistics::frequency::spectral_density(&time_series_values)
}
