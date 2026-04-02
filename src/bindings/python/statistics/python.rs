#[cfg(feature = "python")]
use {
    crate::bindings::python::statistics::{
        advanced, core, decompose, evaluation, exponential_smoothing, filters, frequency,
        hypothesis, moving_averages, time_series, transformations, wavelets,
    },
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
pub fn create_module(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "statistics")?;
    //
    module.add_function(wrap_pyfunction!(advanced::hurst_exponent, &module)?)?;
    module.add_function(wrap_pyfunction!(advanced::mutual_information, &module)?)?;
    module.add_function(wrap_pyfunction!(advanced::sample_entropy, &module)?)?;
    module.add_function(wrap_pyfunction!(advanced::shannon_entropy, &module)?)?;
    //
    module.add_function(wrap_pyfunction!(core::correlation, &module)?)?;
    module.add_function(wrap_pyfunction!(core::covariance, &module)?)?;
    module.add_function(wrap_pyfunction!(core::kurtosis, &module)?)?;
    module.add_function(wrap_pyfunction!(core::mean, &module)?)?;
    module.add_function(wrap_pyfunction!(core::skewness, &module)?)?;
    module.add_function(wrap_pyfunction!(core::standard_deviation, &module)?)?;
    module.add_function(wrap_pyfunction!(core::variance, &module)?)?;
    //
    module.add_function(wrap_pyfunction!(decompose::loess_smooth, &module)?)?;
    module.add_function(wrap_pyfunction!(decompose::seasonal_decompose, &module)?)?;
    module.add_function(wrap_pyfunction!(
        decompose::seasonal_trend_decomposition_using_loess,
        &module
    )?)?;
    //
    module.add_function(wrap_pyfunction!(
        evaluation::akaike_information_criterion,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(
        evaluation::bayesian_information_criterion,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(evaluation::log_likelihood, &module)?)?;
    module.add_function(wrap_pyfunction!(evaluation::mean_absolute_error, &module)?)?;
    module.add_function(wrap_pyfunction!(
        evaluation::mean_absolute_percentage_error,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(evaluation::mean_squared_error, &module)?)?;
    module.add_function(wrap_pyfunction!(
        evaluation::root_mean_squared_error,
        &module
    )?)?;
    //
    module.add_function(wrap_pyfunction!(
        exponential_smoothing::holt_linear_trend,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(
        exponential_smoothing::holt_winters,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(
        exponential_smoothing::simple_exponential_smoothing,
        &module
    )?)?;
    //
    module.add_function(wrap_pyfunction!(filters::hodrick_prescott_filter, &module)?)?;
    module.add_function(wrap_pyfunction!(filters::wiener_filter, &module)?)?;
    //
    module.add_function(wrap_pyfunction!(
        frequency::discrete_fourier_transform,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(
        frequency::inverse_discrete_fourier_transform,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(frequency::periodogram, &module)?)?;
    module.add_function(wrap_pyfunction!(frequency::spectral_density, &module)?)?;
    //
    module.add_function(wrap_pyfunction!(
        hypothesis::augmented_dickey_fuller,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(hypothesis::durbin_watson, &module)?)?;
    module.add_function(wrap_pyfunction!(hypothesis::jarque_bera, &module)?)?;
    module.add_function(wrap_pyfunction!(
        hypothesis::kwiatkowski_phillips_schmidt_shin,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(hypothesis::ljung_box, &module)?)?;
    //
    module.add_function(wrap_pyfunction!(
        moving_averages::exponential_moving_average,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(
        moving_averages::simple_moving_average,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(
        moving_averages::weighted_moving_average,
        &module
    )?)?;
    //
    module.add_function(wrap_pyfunction!(time_series::autocorrelation, &module)?)?;
    module.add_function(wrap_pyfunction!(time_series::autocovariance, &module)?)?;
    module.add_function(wrap_pyfunction!(time_series::cross_correlation, &module)?)?;
    module.add_function(wrap_pyfunction!(
        time_series::partial_autocorrelation,
        &module
    )?)?;
    //
    module.add_function(wrap_pyfunction!(transformations::box_cox, &module)?)?;
    module.add_function(wrap_pyfunction!(transformations::detrend_linear, &module)?)?;
    module.add_function(wrap_pyfunction!(transformations::difference, &module)?)?;
    module.add_function(wrap_pyfunction!(transformations::lag, &module)?)?;
    module.add_function(wrap_pyfunction!(transformations::logarithm, &module)?)?;
    module.add_function(wrap_pyfunction!(transformations::rolling_mean, &module)?)?;
    module.add_function(wrap_pyfunction!(
        transformations::rolling_standard_deviation,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(
        transformations::seasonal_difference,
        &module
    )?)?;
    //
    module.add_function(wrap_pyfunction!(
        wavelets::continuous_wavelet_transform,
        &module
    )?)?;
    module.add_function(wrap_pyfunction!(
        wavelets::discrete_wavelet_transform,
        &module
    )?)?;
    //
    Ok(module)
}
