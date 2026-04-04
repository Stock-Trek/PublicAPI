#[cfg(feature = "python")]
use {crate::signal::*, pyo3::prelude::*, std::collections::HashMap};

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyConfidencePercentageChanges {
    #[pyo3(get, set)]
    pub p01: f64,
    #[pyo3(get, set)]
    pub p05: f64,
    #[pyo3(get, set)]
    pub p10: f64,
    #[pyo3(get, set)]
    pub p25: f64,
    #[pyo3(get, set)]
    pub p50: f64,
    #[pyo3(get, set)]
    pub p75: f64,
    #[pyo3(get, set)]
    pub p90: f64,
    #[pyo3(get, set)]
    pub p95: f64,
    #[pyo3(get, set)]
    pub p99: f64,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyConfidencePercentageChanges {
    #[new]
    fn new(
        p01: f64,
        p05: f64,
        p10: f64,
        p25: f64,
        p50: f64,
        p75: f64,
        p90: f64,
        p95: f64,
        p99: f64,
    ) -> Self {
        Self {
            p01,
            p05,
            p10,
            p25,
            p50,
            p75,
            p90,
            p95,
            p99,
        }
    }
}

#[cfg(feature = "python")]
impl From<ConfidencePercentageChanges> for PyConfidencePercentageChanges {
    fn from(c: ConfidencePercentageChanges) -> Self {
        Self {
            p01: c.p01,
            p05: c.p05,
            p10: c.p10,
            p25: c.p25,
            p50: c.p50,
            p75: c.p75,
            p90: c.p90,
            p95: c.p95,
            p99: c.p99,
        }
    }
}

#[cfg(feature = "python")]
impl From<PyConfidencePercentageChanges> for ConfidencePercentageChanges {
    fn from(py_c: PyConfidencePercentageChanges) -> Self {
        Self {
            p01: py_c.p01,
            p05: py_c.p05,
            p10: py_c.p10,
            p25: py_c.p25,
            p50: py_c.p50,
            p75: py_c.p75,
            p90: py_c.p90,
            p95: py_c.p95,
            p99: py_c.p99,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyConfidenceProbabilities {
    #[pyo3(get, set)]
    pub p01: f64,
    #[pyo3(get, set)]
    pub p05: f64,
    #[pyo3(get, set)]
    pub p10: f64,
    #[pyo3(get, set)]
    pub p25: f64,
    #[pyo3(get, set)]
    pub p50: f64,
    #[pyo3(get, set)]
    pub p75: f64,
    #[pyo3(get, set)]
    pub p90: f64,
    #[pyo3(get, set)]
    pub p95: f64,
    #[pyo3(get, set)]
    pub p99: f64,
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyConfidenceTimings {
    #[pyo3(get, set)]
    pub p01: u64,
    #[pyo3(get, set)]
    pub p05: u64,
    #[pyo3(get, set)]
    pub p10: u64,
    #[pyo3(get, set)]
    pub p25: u64,
    #[pyo3(get, set)]
    pub p50: u64,
    #[pyo3(get, set)]
    pub p75: u64,
    #[pyo3(get, set)]
    pub p90: u64,
    #[pyo3(get, set)]
    pub p95: u64,
    #[pyo3(get, set)]
    pub p99: u64,
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyEvent {
    #[pyo3(get, set)]
    pub event_id: String,
    #[pyo3(get, set)]
    pub generated_timestamp_ms: u64,
    #[pyo3(get, set)]
    pub generated_timezone: String,
}

#[cfg(feature = "python")]
impl From<Event> for PyEvent {
    fn from(e: Event) -> Self {
        Self {
            event_id: e.event_id.to_string(),
            generated_timestamp_ms: e.generated_timestamp_ms,
            generated_timezone: e.generated_timezone,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyGenerator {
    #[pyo3(get, set)]
    pub creator: String,
    #[pyo3(get, set)]
    pub generator_id: String,
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub version: String,
}

#[cfg(feature = "python")]
impl From<Generator> for PyGenerator {
    fn from(g: Generator) -> Self {
        Self {
            creator: g.creator,
            generator_id: g.generator_id.to_string(),
            name: g.name,
            version: g.version,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyHorizonConfidencesByMillis {
    #[pyo3(get, set)]
    pub confidences: HashMap<String, u64>,
}

#[cfg(feature = "python")]
impl From<HorizonConfidencesByMillis> for PyHorizonConfidencesByMillis {
    fn from(h: HorizonConfidencesByMillis) -> Self {
        Self { confidences: h.0 }
    }
}

#[cfg(feature = "python")]
impl From<PyHorizonConfidencesByMillis> for HorizonConfidencesByMillis {
    fn from(py_h: PyHorizonConfidencesByMillis) -> Self {
        Self(py_h.confidences)
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyInstrument {
    #[pyo3(get, set)]
    pub base: String,
    #[pyo3(get, set)]
    pub product: String,
    #[pyo3(get, set)]
    pub quote: String,
}

#[cfg(feature = "python")]
impl From<Instrument> for PyInstrument {
    fn from(i: Instrument) -> Self {
        Self {
            base: i.base,
            product: i.product.to_string(),
            quote: i.quote,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyMarketContext {
    #[pyo3(get, set)]
    pub market_regime: PyMarketRegime,
    #[pyo3(get, set)]
    pub regime_persistence: PyRegimePersistence,
}

#[cfg(feature = "python")]
impl From<MarketContext> for PyMarketContext {
    fn from(mc: MarketContext) -> Self {
        Self {
            market_regime: mc.market_regime.into(),
            regime_persistence: mc.regime_persistence.into(),
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyMarketRegime {
    #[pyo3(get, set)]
    pub classifications: PyMarketRegimeClassifications,
    #[pyo3(get, set)]
    pub cycle: PyMarketRegimeCycle,
    #[pyo3(get, set)]
    pub trend: PyMarketRegimeTrend,
    #[pyo3(get, set)]
    pub volatility: PyMarketRegimeVolatility,
}

#[cfg(feature = "python")]
impl From<MarketRegime> for PyMarketRegime {
    fn from(mr: MarketRegime) -> Self {
        Self {
            classifications: mr.classifications.into(),
            cycle: mr.cycle.into(),
            trend: mr.trend.into(),
            volatility: mr.volatility.into(),
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyMarketRegimeClassifications {
    #[pyo3(get, set)]
    pub confidence: f64,
    #[pyo3(get, set)]
    pub dominant: String,
    #[pyo3(get, set)]
    pub top_alternatives: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub unclassified: f64,
}

#[cfg(feature = "python")]
impl From<MarketRegimeClassifications> for PyMarketRegimeClassifications {
    fn from(mrc: MarketRegimeClassifications) -> Self {
        Self {
            confidence: mrc.confidence,
            dominant: mrc.dominant,
            top_alternatives: mrc.top_alternatives,
            unclassified: mrc.unclassified,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyMarketRegimeCycle {
    #[pyo3(get, set)]
    pub accumulation: f64,
    #[pyo3(get, set)]
    pub distribution: f64,
    #[pyo3(get, set)]
    pub markdown: f64,
    #[pyo3(get, set)]
    pub markup: f64,
    #[pyo3(get, set)]
    pub neutral: f64,
}

#[cfg(feature = "python")]
impl From<MarketRegimeCycle> for PyMarketRegimeCycle {
    fn from(mrc: MarketRegimeCycle) -> Self {
        Self {
            accumulation: mrc.accumulation,
            distribution: mrc.distribution,
            markdown: mrc.markdown,
            markup: mrc.markup,
            neutral: mrc.neutral,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyMarketRegimeTrend {
    #[pyo3(get, set)]
    pub bearish: f64,
    #[pyo3(get, set)]
    pub bullish: f64,
    #[pyo3(get, set)]
    pub sideways: f64,
}

#[cfg(feature = "python")]
impl From<MarketRegimeTrend> for PyMarketRegimeTrend {
    fn from(mrt: MarketRegimeTrend) -> Self {
        Self {
            bearish: mrt.bearish,
            bullish: mrt.bullish,
            sideways: mrt.sideways,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyMarketRegimeVolatility {
    #[pyo3(get, set)]
    pub snapshot: PyMarketRegimeVolatilitySnapshot,
    #[pyo3(get, set)]
    pub trend: PyMarketRegimeVolatilityTrend,
}

#[cfg(feature = "python")]
impl From<MarketRegimeVolatility> for PyMarketRegimeVolatility {
    fn from(mrv: MarketRegimeVolatility) -> Self {
        Self {
            snapshot: mrv.snapshot.into(),
            trend: mrv.trend.into(),
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyMarketRegimeVolatilitySnapshot {
    #[pyo3(get, set)]
    pub high: f64,
    #[pyo3(get, set)]
    pub low: f64,
}

#[cfg(feature = "python")]
impl From<MarketRegimeVolatilitySnapshot> for PyMarketRegimeVolatilitySnapshot {
    fn from(mrvs: MarketRegimeVolatilitySnapshot) -> Self {
        Self {
            high: mrvs.high,
            low: mrvs.low,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyMarketRegimeVolatilityTrend {
    #[pyo3(get, set)]
    pub compression: f64,
    #[pyo3(get, set)]
    pub expansion: f64,
}

#[cfg(feature = "python")]
impl From<MarketRegimeVolatilityTrend> for PyMarketRegimeVolatilityTrend {
    fn from(mrvt: MarketRegimeVolatilityTrend) -> Self {
        Self {
            compression: mrvt.compression,
            expansion: mrvt.expansion,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyMetadata {
    #[pyo3(get, set)]
    pub event: PyEvent,
    #[pyo3(get, set)]
    pub generator: PyGenerator,
    #[pyo3(get, set)]
    pub provenance: PyProvenance,
    #[pyo3(get, set)]
    pub schema: String,
}

#[cfg(feature = "python")]
impl From<Metadata> for PyMetadata {
    fn from(m: Metadata) -> Self {
        Self {
            event: m.event.into(),
            generator: m.generator.into(),
            provenance: m.provenance.into(),
            schema: m.schema,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyPrediction {
    #[pyo3(get, set)]
    pub horizon_confidences_by_millis: PyHorizonConfidencesByMillis,
    #[pyo3(get, set)]
    pub optimal_horizon_millis: u64,
    #[pyo3(get, set)]
    pub percentage_changes: PyConfidencePercentageChanges,
    #[pyo3(get, set)]
    pub risk: PyPredictionRisk,
    #[pyo3(get, set)]
    pub validity_duration_millis: u64,
}

#[cfg(feature = "python")]
impl From<Prediction> for PyPrediction {
    fn from(p: Prediction) -> Self {
        Self {
            horizon_confidences_by_millis: p.horizon_confidences_by_millis.into(),
            optimal_horizon_millis: p.optimal_horizon_millis,
            percentage_changes: p.percentage_changes.into(),
            risk: p.risk.into(),
            validity_duration_millis: p.validity_duration_millis,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyPredictionRisk {
    #[pyo3(get, set)]
    pub percentage_risks: PyPredictionRiskPercentageRisks,
    #[pyo3(get, set)]
    pub risk_factors: HashMap<String, f64>,
}

#[cfg(feature = "python")]
impl From<PredictionRisk> for PyPredictionRisk {
    fn from(pr: PredictionRisk) -> Self {
        Self {
            percentage_risks: pr.percentage_risks.into(),
            risk_factors: pr.risk_factors,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyPredictionRiskPercentageRisks {
    #[pyo3(get, set)]
    pub cvar_95: f64,
    #[pyo3(get, set)]
    pub cvar_99: f64,
    #[pyo3(get, set)]
    pub max_drawdown_95: f64,
    #[pyo3(get, set)]
    pub max_drawdown_99: f64,
    #[pyo3(get, set)]
    pub var_95: f64,
    #[pyo3(get, set)]
    pub var_99: f64,
}

#[cfg(feature = "python")]
impl From<PredictionRiskPercentageRisks> for PyPredictionRiskPercentageRisks {
    fn from(prpr: PredictionRiskPercentageRisks) -> Self {
        Self {
            cvar_95: prpr.cvar_95,
            cvar_99: prpr.cvar_99,
            max_drawdown_95: prpr.max_drawdown_95,
            max_drawdown_99: prpr.max_drawdown_99,
            var_95: prpr.var_95,
            var_99: prpr.var_99,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyProvenance {
    #[pyo3(get, set)]
    pub description: String,
    #[pyo3(get, set)]
    pub methodology: String,
    #[pyo3(get, set)]
    pub references: Vec<String>,
}

#[cfg(feature = "python")]
impl From<Provenance> for PyProvenance {
    fn from(p: Provenance) -> Self {
        Self {
            description: p.description,
            methodology: p.methodology,
            references: p.references,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyRegimePersistence {
    #[pyo3(get, set)]
    pub regime_persistence_confidence: f64,
    #[pyo3(get, set)]
    pub remaining_durations_millis: u64,
}

#[cfg(feature = "python")]
impl From<RegimePersistence> for PyRegimePersistence {
    fn from(rp: RegimePersistence) -> Self {
        Self {
            regime_persistence_confidence: rp.regime_persistence_confidence,
            remaining_durations_millis: rp.remaining_durations_millis,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyStockTrekSignal {
    #[pyo3(get, set)]
    pub instrument: PyInstrument,
    #[pyo3(get, set)]
    pub market_context: PyMarketContext,
    #[pyo3(get, set)]
    pub prediction: PyPrediction,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyStockTrekSignal {
    #[new]
    fn new(
        instrument: PyInstrument,
        market_context: PyMarketContext,
        prediction: PyPrediction,
    ) -> Self {
        Self {
            instrument,
            market_context,
            prediction,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "StockTrekSignal(instrument={}, market_context=..., prediction=...)",
            self.instrument.base
        )
    }
}

#[cfg(feature = "python")]
impl From<StockTrekSignal> for PyStockTrekSignal {
    fn from(s: StockTrekSignal) -> Self {
        Self {
            instrument: s.instrument.into(),
            market_context: s.market_context.into(),
            prediction: s.prediction.into(),
        }
    }
}
