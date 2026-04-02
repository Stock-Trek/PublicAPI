pub fn simple_moving_average(time_series_values: &[f64], window_size: usize) -> Vec<f64> {
    time_series_values
        .windows(window_size)
        .map(|w| w.iter().sum::<f64>() / window_size as f64)
        .collect()
}

pub fn weighted_moving_average(time_series_values: &[f64], weight_values: &[f64]) -> Vec<f64> {
    let w_sum: f64 = weight_values.iter().sum();
    time_series_values
        .windows(weight_values.len())
        .map(|w| {
            w.iter()
                .zip(weight_values.iter())
                .map(|(x, w)| x * w)
                .sum::<f64>()
                / w_sum
        })
        .collect()
}

pub fn exponential_moving_average(time_series_values: &[f64], alpha: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(time_series_values.len());
    if let Some(&first) = time_series_values.first() {
        let mut prev = first;
        result.push(prev);

        for &x in &time_series_values[1..] {
            let ema = alpha * x + (1.0 - alpha) * prev;
            result.push(ema);
            prev = ema;
        }
    }
    result
}
