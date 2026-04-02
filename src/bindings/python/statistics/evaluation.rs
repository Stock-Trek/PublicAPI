#[cfg(feature = "python")]
use pyo3::{exceptions::PyRuntimeError, pyfunction, PyResult};

#[cfg(feature = "python")]
#[pyfunction]
pub fn mean_squared_error(true_values: Vec<f64>, predicted_values: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::evaluation::mean_squared_error(&true_values, &predicted_values)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn root_mean_squared_error(true_values: Vec<f64>, predicted_values: Vec<f64>) -> PyResult<f64> {
    let result =
        crate::statistics::evaluation::root_mean_squared_error(&true_values, &predicted_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn mean_absolute_error(true_values: Vec<f64>, predicted_values: Vec<f64>) -> PyResult<f64> {
    let result =
        crate::statistics::evaluation::mean_absolute_error(&true_values, &predicted_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn mean_absolute_percentage_error(
    true_values: Vec<f64>,
    predicted_values: Vec<f64>,
) -> PyResult<f64> {
    let result = crate::statistics::evaluation::mean_absolute_percentage_error(
        &true_values,
        &predicted_values,
    )
    .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn log_likelihood(model_parameters: Vec<f64>, observed_data: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::evaluation::log_likelihood(&model_parameters, &observed_data)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn akaike_information_criterion(
    log_likelihood_value: f64,
    number_of_parameters: usize,
) -> PyResult<f64> {
    let result = crate::statistics::evaluation::akaike_information_criterion(
        log_likelihood_value,
        number_of_parameters,
    )
    .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn bayesian_information_criterion(
    log_likelihood_value: f64,
    number_of_parameters: usize,
    number_of_observations: usize,
) -> PyResult<f64> {
    let result = crate::statistics::evaluation::bayesian_information_criterion(
        log_likelihood_value,
        number_of_parameters,
        number_of_observations,
    )
    .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}
