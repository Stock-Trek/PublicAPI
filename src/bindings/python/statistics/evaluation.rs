#[cfg(feature = "python")]
use {
    crate::statistics::evaluation,
    pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult},
};

#[cfg(feature = "python")]
#[pyclass(name = "Evaluation")]
pub struct PyEvaluation;

#[cfg(feature = "python")]
#[pymethods]
impl PyEvaluation {
    pub fn akaike_information_criterion(
        &self,
        log_likelihood_value: f64,
        number_of_parameters: usize,
    ) -> PyResult<f64> {
        let result =
            evaluation::akaike_information_criterion(log_likelihood_value, number_of_parameters)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
    pub fn bayesian_information_criterion(
        &self,
        log_likelihood_value: f64,
        number_of_parameters: usize,
        number_of_observations: usize,
    ) -> PyResult<f64> {
        let result = evaluation::bayesian_information_criterion(
            log_likelihood_value,
            number_of_parameters,
            number_of_observations,
        )
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
    pub fn log_likelihood(
        &self,
        model_parameters: Vec<f64>,
        observed_data: Vec<f64>,
    ) -> PyResult<f64> {
        let result = evaluation::log_likelihood(&model_parameters, &observed_data)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
    pub fn mean_absolute_error(
        &self,
        true_values: Vec<f64>,
        predicted_values: Vec<f64>,
    ) -> PyResult<f64> {
        let result = evaluation::mean_absolute_error(&true_values, &predicted_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
    pub fn mean_absolute_percentage_error(
        &self,
        true_values: Vec<f64>,
        predicted_values: Vec<f64>,
    ) -> PyResult<f64> {
        let result = evaluation::mean_absolute_percentage_error(&true_values, &predicted_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
    pub fn mean_squared_error(
        &self,
        true_values: Vec<f64>,
        predicted_values: Vec<f64>,
    ) -> PyResult<f64> {
        let result = evaluation::mean_squared_error(&true_values, &predicted_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
    pub fn root_mean_squared_error(
        &self,
        true_values: Vec<f64>,
        predicted_values: Vec<f64>,
    ) -> PyResult<f64> {
        let result = evaluation::root_mean_squared_error(&true_values, &predicted_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
}
