use crate::statistics::decompose;

#[derive(Clone, Default)]
pub struct Decompose;

impl Decompose {
    pub fn loess_smooth(&self, values: &[f64], span: usize) -> Vec<f64> {
        decompose::loess_smooth(values, span)
    }
    pub fn seasonal_decompose(
        &self,
        time_series_values: &[f64],
        seasonal_period_length: usize,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        decompose::seasonal_decompose(time_series_values, seasonal_period_length)
    }
    pub fn seasonal_trend_decomposition_using_loess(
        &self,
        time_series_values: &[f64],
        seasonal_period_length: usize,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        decompose::seasonal_trend_decomposition_using_loess(
            time_series_values,
            seasonal_period_length,
        )
    }
}

pub fn seasonal_decompose(
    time_series_values: &[f64],
    seasonal_period_length: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = time_series_values.len();
    if n == 0 || seasonal_period_length == 0 || seasonal_period_length > n {
        return (vec![], vec![], vec![]);
    }
    // Trend via moving average
    let mut trend = vec![0.0; n];
    let half = seasonal_period_length / 2;
    for (i, trend_item) in trend.iter_mut().enumerate() {
        let start = i.saturating_sub(half);
        let end = usize::min(n, i + half + 1);
        let window = &time_series_values[start..end];
        *trend_item = window.iter().sum::<f64>() / window.len() as f64;
    }
    // Detrended
    let detrended: Vec<f64> = time_series_values
        .iter()
        .zip(trend.iter())
        .map(|(x, t)| x - t)
        .collect();
    // Seasonal component
    let mut seasonal = vec![0.0; n];
    let mut seasonal_means = vec![0.0; seasonal_period_length];
    let mut counts = vec![0usize; seasonal_period_length];
    for (i, detrended_item) in detrended.iter().enumerate() {
        let idx = i % seasonal_period_length;
        seasonal_means[idx] += detrended_item;
        counts[idx] += 1;
    }
    for i in 0..seasonal_period_length {
        if counts[i] > 0 {
            seasonal_means[i] /= counts[i] as f64;
        }
    }
    for i in 0..n {
        seasonal[i] = seasonal_means[i % seasonal_period_length];
    }
    // Residual
    let residual: Vec<f64> = time_series_values
        .iter()
        .zip(trend.iter())
        .zip(seasonal.iter())
        .map(|((x, t), s)| x - t - s)
        .collect();
    (trend, seasonal, residual)
}

pub fn seasonal_trend_decomposition_using_loess(
    time_series_values: &[f64],
    seasonal_period_length: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = time_series_values.len();
    if n == 0 || seasonal_period_length == 0 || seasonal_period_length > n {
        return (vec![], vec![], vec![]);
    }
    // Trend via LOESS
    let trend = decompose::loess_smooth(time_series_values, seasonal_period_length);
    // Detrended
    let detrended: Vec<f64> = time_series_values
        .iter()
        .zip(trend.iter())
        .map(|(x, t)| x - t)
        .collect();
    // Seasonal component (same approach as classical)
    let mut seasonal = vec![0.0; n];
    let mut seasonal_means = vec![0.0; seasonal_period_length];
    let mut counts = vec![0usize; seasonal_period_length];
    for (i, detrended_item) in detrended.iter().enumerate() {
        let idx = i % seasonal_period_length;
        seasonal_means[idx] += detrended_item;
        counts[idx] += 1;
    }
    for i in 0..seasonal_period_length {
        if counts[i] > 0 {
            seasonal_means[i] /= counts[i] as f64;
        }
    }
    for i in 0..n {
        seasonal[i] = seasonal_means[i % seasonal_period_length];
    }
    // Residual
    let residual: Vec<f64> = time_series_values
        .iter()
        .zip(trend.iter())
        .zip(seasonal.iter())
        .map(|((x, t), s)| x - t - s)
        .collect();
    (trend, seasonal, residual)
}

pub fn loess_smooth(values: &[f64], span: usize) -> Vec<f64> {
    let n = values.len();
    let mut result = vec![0.0; n];
    for i in 0..n {
        let start = i.saturating_sub(span);
        let end = usize::min(n, i + span + 1);
        let mut weighted_sum = 0.0;
        let mut weight_total = 0.0;
        for (j, j_value) in values.iter().enumerate().take(end).skip(start) {
            let dist = (i as isize - j as isize).abs() as f64;
            let w = (1.0 - (dist / span as f64).powi(3)).powi(3).max(0.0);
            weighted_sum += w * j_value;
            weight_total += w;
        }
        result[i] = if weight_total > 0.0 {
            weighted_sum / weight_total
        } else {
            values[i]
        };
    }
    result
}
