use crate::statistics::wavelet;

#[derive(Clone, Default)]
pub struct Wavelet;

impl Wavelet {
    pub fn continuous_wavelet_transform(
        &self,
        time_series_values: &[f64],
        scale_values: &[f64],
        wavelet_name: &str,
    ) -> Vec<Vec<f64>> {
        wavelet::continuous_wavelet_transform(time_series_values, scale_values, wavelet_name)
    }
    pub fn discrete_wavelet_transform(
        &self,
        time_series_values: &[f64],
        wavelet_name: &str,
    ) -> (Vec<f64>, Vec<f64>) {
        wavelet::discrete_wavelet_transform(time_series_values, wavelet_name)
    }
}

pub fn continuous_wavelet_transform(
    time_series_values: &[f64],
    scale_values: &[f64],
    wavelet_name: &str,
) -> Vec<Vec<f64>> {
    let n = time_series_values.len();
    let mut result = Vec::with_capacity(scale_values.len());
    for &scale in scale_values {
        let mut coeffs = Vec::with_capacity(n);
        for t in 0..n {
            let mut sum = 0.0;
            for (i, &x) in time_series_values.iter().enumerate() {
                let u = (i as f64 - t as f64) / scale;
                let psi = match wavelet_name {
                    "morlet" => {
                        let w0 = 5.0;
                        (-(u * u) / 2.0).exp() * (w0 * u).cos()
                    }
                    "mexican_hat" => (1.0 - u * u) * (-(u * u) / 2.0).exp(),
                    _ => 0.0,
                };
                sum += x * psi;
            }
            coeffs.push(sum / scale.sqrt());
        }
        result.push(coeffs);
    }
    result
}

pub fn discrete_wavelet_transform(
    time_series_values: &[f64],
    wavelet_name: &str,
) -> (Vec<f64>, Vec<f64>) {
    let n = time_series_values.len();
    if n < 2 {
        return (vec![], vec![]);
    }
    match wavelet_name {
        "haar" => {
            let mut approx = Vec::with_capacity(n / 2);
            let mut detail = Vec::with_capacity(n / 2);
            let norm = 1.0 / (2.0_f64).sqrt();
            for i in (0..n - 1).step_by(2) {
                let a = (time_series_values[i] + time_series_values[i + 1]) * norm;
                let d = (time_series_values[i] - time_series_values[i + 1]) * norm;
                approx.push(a);
                detail.push(d);
            }
            (approx, detail)
        }
        _ => (vec![], vec![]),
    }
}
