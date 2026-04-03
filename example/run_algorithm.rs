use crate::{
    algorithm::StockTrekAlgorithm,
    data::context::StockTrekContext,
    schemas::signal::{
        ConfidencePercentageChanges, HorizonConfidencesByMillis, Instrument, MarketContext,
        MarketRegime, MarketRegimeClassifications, MarketRegimeCycle, MarketRegimeTrend,
        MarketRegimeVolatility, MarketRegimeVolatilitySnapshot, MarketRegimeVolatilityTrend,
        Prediction, PredictionRisk, PredictionRiskPercentageRisks, RegimePersistence,
        StockTrekSignal,
    },
};

pub struct MyAlgo {}

#[traitreg::register]
impl StockTrekAlgorithm for MyAlgo {
    fn create_signal(&self, _context: StockTrekContext) -> StockTrekSignal {
        StockTrekSignal {
            instrument: Instrument {
                product: crate::schemas::signal::InstrumentProduct::Spot,
                base: "BTC".into(),
                quote: "USDT".into(),
            },
            market_context: MarketContext {
                market_regime: MarketRegime {
                    classifications: MarketRegimeClassifications {
                        confidence: 0.5,
                        dominant: "".to_string(),
                        top_alternatives: std::collections::HashMap::from([
                            ("dskfsd".into(), 0.21 + 0.0),
                            ("irewtnvc".into(), 0.17),
                            ("cfhwrehk".into(), 0.15),
                        ]),
                        unclassified: 0.2,
                    },
                    cycle: MarketRegimeCycle {
                        accumulation: 0.3,
                        distribution: 0.5,
                        markdown: 0.1,
                        markup: 0.2,
                        neutral: 0.8,
                    },
                    trend: MarketRegimeTrend {
                        bearish: 0.6,
                        bullish: 0.2,
                        sideways: 0.2,
                    },
                    volatility: MarketRegimeVolatility {
                        snapshot: MarketRegimeVolatilitySnapshot {
                            high: 0.1,
                            low: 0.9,
                        },
                        trend: MarketRegimeVolatilityTrend {
                            compression: 0.5,
                            expansion: 0.5,
                        },
                    },
                },
                regime_persistence: RegimePersistence {
                    regime_persistence_confidence: 0.7,
                    remaining_durations_millis: 483648732,
                },
            },
            prediction: Prediction {
                horizon_confidences_by_millis: HorizonConfidencesByMillis(
                    std::collections::HashMap::from([
                        ("vgfhgkfd".into(), 549357438),
                        ("cdiotkjr".into(), 549357438),
                    ]),
                ),
                optimal_horizon_millis: 1000,
                percentage_changes: ConfidencePercentageChanges {
                    p01: -10.2,
                    p05: -5.1,
                    p10: -0.4,
                    p25: 3.9,
                    p50: 10.2,
                    p75: 16.5,
                    p90: 19.4,
                    p95: 20.4,
                    p99: 20.8,
                },
                risk: PredictionRisk {
                    percentage_risks: PredictionRiskPercentageRisks {
                        cvar_95: -6.8,
                        cvar_99: -7.2,
                        max_drawdown_95: 25.3,
                        max_drawdown_99: 31.8,
                        var_95: -3.7,
                        var_99: -4.5,
                    },
                    risk_factors: std::collections::HashMap::from([
                        ("dvxvxvvodsgrg".into(), 0.63),
                        ("cnnfgvcxojtnn".into(), 0.41),
                    ]),
                },
                validity_duration_millis: 1_000_000,
            },
        }
    }
}
