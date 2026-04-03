#[cfg(feature = "python")]
use {
    crate::bindings::python::schemas::{py_event::PyStockTrekEvent, py_signal::*},
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
pub fn create_module(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "schemas")?;
    module.add_class::<PyConfidencePercentageChanges>()?;
    module.add_class::<PyConfidenceProbabilities>()?;
    module.add_class::<PyConfidenceTimings>()?;
    module.add_class::<PyEvent>()?;
    module.add_class::<PyGenerator>()?;
    module.add_class::<PyHorizonConfidencesByMillis>()?;
    module.add_class::<PyInstrument>()?;
    module.add_class::<PyMarketContext>()?;
    module.add_class::<PyMarketRegime>()?;
    module.add_class::<PyMarketRegimeClassifications>()?;
    module.add_class::<PyMarketRegimeCycle>()?;
    module.add_class::<PyMarketRegimeTrend>()?;
    module.add_class::<PyMarketRegimeVolatility>()?;
    module.add_class::<PyMarketRegimeVolatilitySnapshot>()?;
    module.add_class::<PyMarketRegimeVolatilityTrend>()?;
    module.add_class::<PyMetadata>()?;
    module.add_class::<PyPrediction>()?;
    module.add_class::<PyPredictionRisk>()?;
    module.add_class::<PyPredictionRiskPercentageRisks>()?;
    module.add_class::<PyProvenance>()?;
    module.add_class::<PyRegimePersistence>()?;
    module.add_class::<PyStockTrekSignal>()?;
    module.add_class::<PyStockTrekEvent>()?;
    Ok(module)
}
