use crate::errors::{StatsError, StockTrekError};

pub fn mean(values: &[f64]) -> Result<f64, StockTrekError> {
    if values.is_empty() {
        return Err(StockTrekError::Stats(StatsError::EmptyInput));
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Ok(mean)
}

pub fn variance(values: &[f64], delta_degrees_of_freedom: usize) -> Result<f64, StockTrekError> {
    let n = values.len();
    if n == 0 {
        return Err(StockTrekError::Stats(StatsError::EmptyInput));
    }
    if n <= delta_degrees_of_freedom {
        return Err(StockTrekError::Stats(
            StatsError::InsufficientDegreesOfFreedom,
        ));
    }
    let mu = mean(values)?;
    let sum_sq: f64 = values
        .iter()
        .map(|x| {
            let d = x - mu;
            d * d
        })
        .sum();
    let variance = sum_sq / (n - delta_degrees_of_freedom) as f64;
    Ok(variance)
}

pub fn standard_deviation(
    values: &[f64],
    delta_degrees_of_freedom: usize,
) -> Result<f64, StockTrekError> {
    let standard_deviation_sq = variance(values, delta_degrees_of_freedom)?;
    let standard_deviation = standard_deviation_sq.sqrt();
    Ok(standard_deviation)
}

pub fn covariance(first: &[f64], second: &[f64]) -> Result<f64, StockTrekError> {
    let n = first.len();
    if n == 0 {
        return Err(StockTrekError::Stats(StatsError::EmptyInput));
    }
    if n != second.len() {
        return Err(StockTrekError::Stats(StatsError::MismatchedLengths));
    }
    let mean_x = mean(first)?;
    let mean_y = mean(second)?;
    let cov: f64 = first
        .iter()
        .zip(second.iter())
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum();
    let covariance = cov / n as f64;
    Ok(covariance)
}

pub fn correlation(first: &[f64], second: &[f64]) -> Result<f64, StockTrekError> {
    let cov = covariance(first, second)?;
    let std_x = standard_deviation(first, 0)?;
    let std_y = standard_deviation(second, 0)?;
    if std_x == 0.0 || std_y == 0.0 {
        return Err(StockTrekError::Stats(StatsError::ZeroVariance));
    }
    let correlation = cov / (std_x * std_y);
    Ok(correlation)
}

pub fn skewness(values: &[f64]) -> Result<f64, StockTrekError> {
    let n = values.len();
    if n == 0 {
        return Err(StockTrekError::Stats(StatsError::EmptyInput));
    }
    let mu = mean(values)?;
    let std = standard_deviation(values, 0)?;
    if std == 0.0 {
        return Err(StockTrekError::Stats(StatsError::ZeroVariance));
    }
    let m3: f64 = values
        .iter()
        .map(|x| {
            let z = (x - mu) / std;
            z * z * z
        })
        .sum::<f64>()
        / n as f64;
    Ok(m3)
}

pub fn kurtosis(values: &[f64]) -> Result<f64, StockTrekError> {
    let n = values.len();
    if n == 0 {
        return Err(StockTrekError::Stats(StatsError::EmptyInput));
    }
    let mu = mean(values)?;
    let std = standard_deviation(values, 0)?;
    if std == 0.0 {
        return Err(StockTrekError::Stats(StatsError::ZeroVariance));
    }
    let m4: f64 = values
        .iter()
        .map(|x| {
            let z = (x - mu) / std;
            z * z * z * z
        })
        .sum::<f64>()
        / n as f64;
    Ok(m4) // subtract 3.0 externally if you want excess kurtosis
}
