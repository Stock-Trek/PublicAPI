use crate::{
    errors::{StatsError, StockTrekError},
    statistics,
};

pub fn lag(values: &[f64], lag: usize) -> Result<Vec<Option<f64>>, StockTrekError> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        if i < lag {
            result.push(None);
        } else {
            result.push(Some(values[i - lag]));
        }
    }
    Ok(result)
}

pub fn difference(values: &[f64], order: usize) -> Result<Vec<f64>, StockTrekError> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    if order == 0 {
        return Ok(values.to_vec());
    }
    if order >= n {
        return Err(StatsError::InsufficientDegreesOfFreedom.into());
    }
    let mut result = values.to_vec();
    for _ in 0..order {
        result = result.windows(2).map(|w| w[1] - w[0]).collect();
    }
    Ok(result)
}

pub fn seasonal_difference(values: &[f64], period: usize) -> Result<Vec<f64>, StockTrekError> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    if period == 0 || period >= n {
        return Err(StatsError::InvalidLag.into());
    }
    let result: Vec<f64> = (period..n)
        .map(|i| values[i] - values[i - period])
        .collect();
    Ok(result)
}

pub fn logarithm(values: &[f64]) -> Result<Vec<f64>, StockTrekError> {
    if values.is_empty() {
        return Err(StatsError::EmptyInput.into());
    }
    let mut result = Vec::with_capacity(values.len());
    for &x in values {
        if x <= 0.0 {
            return Err(StockTrekError::Stats(StatsError::DomainError {
                message: "log undefined for non-positive values".into(),
            }));
        }
        result.push(x.ln());
    }
    Ok(result)
}

pub fn box_cox(values: &[f64], lambda: f64) -> Result<Vec<f64>, StockTrekError> {
    if values.is_empty() {
        return Err(StatsError::EmptyInput.into());
    }
    let mut result = Vec::with_capacity(values.len());
    for &x in values {
        if x <= 0.0 {
            return Err(StockTrekError::Stats(StatsError::DomainError {
                message: "Box-Cox requires positive values".into(),
            }));
        }
        if lambda == 0.0 {
            result.push(x.ln());
        } else {
            result.push((x.powf(lambda) - 1.0) / lambda);
        }
    }
    Ok(result)
}

pub fn detrend_linear(values: &[f64]) -> Result<Vec<f64>, StockTrekError> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let mean_x = statistics::core::mean(&x)?;
    let mean_y = statistics::core::mean(values)?;
    let numerator: f64 = x
        .iter()
        .zip(values.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum();
    let denominator: f64 = x
        .iter()
        .map(|xi| {
            let d = xi - mean_x;
            d * d
        })
        .sum();
    if denominator == 0.0 {
        return Err(StatsError::ZeroVariance.into());
    }
    let slope = numerator / denominator;
    let intercept = mean_y - slope * mean_x;
    let detrended: Vec<f64> = x
        .iter()
        .zip(values.iter())
        .map(|(xi, yi)| yi - (slope * xi + intercept))
        .collect();
    Ok(detrended)
}

pub fn rolling_mean(values: &[f64], window: usize) -> Result<Vec<f64>, StockTrekError> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    if window == 0 || window > n {
        return Err(StatsError::InvalidLag.into());
    }
    let result: Vec<f64> = values
        .windows(window)
        .map(|w| w.iter().sum::<f64>() / window as f64)
        .collect();
    Ok(result)
}

pub fn rolling_standard_deviation(
    values: &[f64],
    window: usize,
) -> Result<Vec<f64>, StockTrekError> {
    let n = values.len();
    if n == 0 {
        return Err(StatsError::EmptyInput.into());
    }
    if window == 0 || window > n {
        return Err(StatsError::InvalidLag.into());
    }
    let mut result = Vec::with_capacity(n - window + 1);
    for w in values.windows(window) {
        let var = statistics::core::variance(w, 0)?;
        result.push(var.sqrt());
    }
    Ok(result)
}
