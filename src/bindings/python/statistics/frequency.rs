#[cfg(feature = "python")]
use {
    crate::statistics::frequency,
    num_complex::Complex,
    pyo3::{pyclass, pymethods},
};

#[cfg(feature = "python")]
#[pyclass(name = "Frequency")]
pub struct PyFrequency;

#[cfg(feature = "python")]
#[pymethods]
impl PyFrequency {
    pub fn discrete_fourier_transform(&self, time_series_values: Vec<f64>) -> Vec<(f64, f64)> {
        frequency::discrete_fourier_transform(&time_series_values)
            .iter()
            .map(|c| (c.re, c.im))
            .collect()
    }
    pub fn inverse_discrete_fourier_transform(
        &self,
        frequency_domain_values: Vec<(f64, f64)>,
    ) -> Vec<f64> {
        let complex_values: Vec<Complex<f64>> = frequency_domain_values
            .iter()
            .map(|(re, im)| Complex::new(*re, *im))
            .collect();
        frequency::inverse_discrete_fourier_transform(&complex_values)
    }
    pub fn periodogram(&self, time_series_values: Vec<f64>, sampling_frequency: f64) -> Vec<f64> {
        frequency::periodogram(&time_series_values, sampling_frequency)
    }
    pub fn spectral_density(&self, time_series_values: Vec<f64>) -> Vec<f64> {
        frequency::spectral_density(&time_series_values)
    }
}
