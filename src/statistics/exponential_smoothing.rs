pub fn simple_exponential_smoothing(time_series_values: &[f64], alpha: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(time_series_values.len());
    if let Some(&first) = time_series_values.first() {
        let mut level = first;
        result.push(level);
        for &x in &time_series_values[1..] {
            level = alpha * x + (1.0 - alpha) * level;
            result.push(level);
        }
    }
    result
}

pub fn holt_linear_trend(time_series_values: &[f64], alpha: f64, beta: f64) -> Vec<f64> {
    let n = time_series_values.len();
    let mut result = Vec::with_capacity(n);
    if n == 0 {
        return result;
    }
    let mut level = time_series_values[0];
    let mut trend = if n > 1 {
        time_series_values[1] - time_series_values[0]
    } else {
        0.0
    };
    result.push(level);
    for &x in &time_series_values[1..] {
        let prev_level = level;
        level = alpha * x + (1.0 - alpha) * (level + trend);
        trend = beta * (level - prev_level) + (1.0 - beta) * trend;
        result.push(level + trend);
    }
    result
}

pub fn holt_winters(
    time_series_values: &[f64],
    alpha: f64,
    beta: f64,
    gamma: f64,
    season_len: usize,
    multiplicative: bool,
) -> Vec<f64> {
    let n = time_series_values.len();
    let mut result = Vec::with_capacity(n);
    if n == 0 || season_len == 0 || season_len > n {
        return result;
    }
    let mut seasonals = vec![1.0; season_len];
    if multiplicative {
        let season_avg: f64 =
            time_series_values[..season_len].iter().sum::<f64>() / season_len as f64;
        for i in 0..season_len {
            seasonals[i] = time_series_values[i] / season_avg;
        }
    } else {
        let season_avg: f64 =
            time_series_values[..season_len].iter().sum::<f64>() / season_len as f64;
        for i in 0..season_len {
            seasonals[i] = time_series_values[i] - season_avg;
        }
    }
    let mut level = time_series_values[0];
    let mut trend = if n > 1 {
        time_series_values[1] - time_series_values[0]
    } else {
        0.0
    };
    for i in 0..n {
        let season = seasonals[i % season_len];
        let x = time_series_values[i];
        let prev_level = level;
        if multiplicative {
            level = alpha * (x / season) + (1.0 - alpha) * (level + trend);
            trend = beta * (level - prev_level) + (1.0 - beta) * trend;
            seasonals[i % season_len] = gamma * (x / level) + (1.0 - gamma) * season;
            result.push((level + trend) * seasonals[i % season_len]);
        } else {
            level = alpha * (x - season) + (1.0 - alpha) * (level + trend);
            trend = beta * (level - prev_level) + (1.0 - beta) * trend;
            seasonals[i % season_len] = gamma * (x - level) + (1.0 - gamma) * season;
            result.push(level + trend + seasonals[i % season_len]);
        }
    }
    result
}
