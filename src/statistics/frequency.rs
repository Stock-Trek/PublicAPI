use num_complex::Complex;
use std::f64::consts::PI;

pub fn discrete_fourier_transform(time_series_values: &[f64]) -> Vec<Complex<f64>> {
    let n = time_series_values.len();
    let mut result = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for (t, &x) in time_series_values.iter().enumerate() {
            let angle = -2.0 * PI * (k as f64 * t as f64) / n as f64;
            let w = Complex::new(angle.cos(), angle.sin());
            sum += w * x;
        }
        result.push(sum);
    }
    result
}

pub fn inverse_discrete_fourier_transform(frequency_domain_values: &[Complex<f64>]) -> Vec<f64> {
    let n = frequency_domain_values.len();
    let mut result = Vec::with_capacity(n);
    for t in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for (k, &xk) in frequency_domain_values.iter().enumerate() {
            let angle = 2.0 * PI * (k as f64 * t as f64) / n as f64;
            let w = Complex::new(angle.cos(), angle.sin());
            sum += xk * w;
        }
        result.push((sum / n as f64).re);
    }
    result
}

pub fn periodogram(time_series_values: &[f64], sampling_frequency: f64) -> Vec<f64> {
    let n = time_series_values.len();
    let dft = discrete_fourier_transform(time_series_values);
    dft.iter()
        .map(|xk| (xk.norm_sqr() / n as f64) * sampling_frequency)
        .collect()
}

pub fn spectral_density(time_series_values: &[f64]) -> Vec<f64> {
    let n = time_series_values.len();
    let dft = discrete_fourier_transform(time_series_values);
    dft.iter().map(|xk| xk.norm_sqr() / n as f64).collect()
}
