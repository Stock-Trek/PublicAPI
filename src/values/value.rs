use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId, tag::Tag};
use strum::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CexIdValue {
    Literal { literal: CexId },
    Signal { signal: SignalKey<CexId> },
}

impl CexIdValue {
    pub fn cex_id(&self, c: &ResolvedContext) -> StockTrekResult<CexId> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AssetIdValue {
    Literal { literal: AssetId },
    Signal { signal: SignalKey<AssetId> },
}

impl AssetIdValue {
    pub fn asset_id(&self, c: &ResolvedContext) -> StockTrekResult<AssetId> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FlagValue {
    Literal { literal: bool },
    Signal { signal: SignalKey<bool> },
}

impl FlagValue {
    pub fn flag(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NumberValue {
    Literal {
        literal: f64,
    },
    Signal {
        signal: SignalKey<f64>,
    },
    ActiveOrders,
    ActiveOrdersInCex {
        cex_id_value: CexIdValue,
    },
    ActiveOrdersInCexWithTag {
        cex_id_value: CexIdValue,
        tag: Tag,
    },
    ActiveOrdersWithTag {
        tag: Tag,
    },
    AllocationForAssetInCex {
        cex_id_value: CexIdValue,
        asset_id_value: AssetIdValue,
    },
    AllocationForAssetTotal {
        asset_id_value: AssetIdValue,
    },
    AssetInCex {
        cex_id_value: CexIdValue,
        asset_id_value: AssetIdValue,
    },
    AssetTotal {
        asset_id_value: AssetIdValue,
    },
    BinaryCalculation {
        left: Box<NumberValue>,
        operator: BinaryOperator,
        right: Box<NumberValue>,
    },
    UnaryCalculation {
        number: Box<NumberValue>,
        operator: UnaryOperator,
    },
}

impl NumberValue {
    pub fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
            Self::ActiveOrders => Ok(c.portfolio.active_orders()),
            Self::ActiveOrdersInCex { cex_id_value } => {
                let cex_id = cex_id_value.cex_id(c)?;
                Ok(c.portfolio.active_orders_in_cex(&cex_id))
            }
            Self::ActiveOrdersInCexWithTag { cex_id_value, tag } => {
                let cex_id = cex_id_value.cex_id(c)?;
                Ok(c.portfolio.active_orders_in_cex_with_tag(&cex_id, tag))
            }
            Self::ActiveOrdersWithTag { tag } => Ok(c.portfolio.active_orders_with_tag(tag)),
            Self::AllocationForAssetInCex {
                cex_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation.allocation_for_asset_in_cex(&asset_id, &cex_id))
            }
            Self::AllocationForAssetTotal { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation.allocation_for_asset_total(&asset_id))
            }
            Self::AssetInCex {
                cex_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_in_cex(&asset_id, &cex_id))
            }
            Self::AssetTotal { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_total(&asset_id))
            }
            Self::BinaryCalculation {
                left,
                operator,
                right,
            } => {
                let left_value = left.number(c)?;
                let right_value = right.number(c)?;
                let calculation_result = match operator {
                    BinaryOperator::Add => left_value + right_value,
                    BinaryOperator::Atan2 => left_value.atan2(right_value),
                    BinaryOperator::Div => {
                        if right_value == 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DivisionByZero {
                                function: "Div",
                                detail: "divisor = 0 would produce +/- infinity".to_string(),
                            }));
                        }
                        left_value / right_value
                    }
                    BinaryOperator::Pow => {
                        if left_value < 0.0 && right_value.fract() != 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Pow",
                                message: format!(
                                    "base {} < 0 with fractional exponent {} would produce a complex number",
                                    left_value, right_value
                                ),
                            }));
                        }
                        left_value.powf(right_value)
                    }
                    BinaryOperator::Log => {
                        if left_value == 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Log",
                                message: "argument = 0 is undefined".to_string(),
                            }));
                        }
                        if right_value == 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Log",
                                message: "base = 0 is undefined".to_string(),
                            }));
                        }
                        if right_value == 1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Log",
                                message: "base = 1 is undefined".to_string(),
                            }));
                        }
                        if left_value < 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Log",
                                message: format!(
                                    "argument {} < 0 would produce a complex number",
                                    left_value
                                ),
                            }));
                        }
                        if right_value < 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Log",
                                message: format!(
                                    "base {} < 0 would produce a complex number",
                                    right_value
                                ),
                            }));
                        }
                        left_value.log(right_value)
                    }
                    BinaryOperator::Mod => {
                        if right_value == 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DivisionByZero {
                                function: "Mod",
                                detail: "divisor = 0 would produce +/- infinity".to_string(),
                            }));
                        }
                        left_value % right_value
                    }
                    BinaryOperator::Mul => left_value * right_value,
                    BinaryOperator::Sub => left_value - right_value,
                };
                Ok(calculation_result)
            }
            Self::UnaryCalculation { number, operator } => {
                let value = number.number(c)?;
                let calculation_result = match operator {
                    UnaryOperator::Abs => value.abs(),
                    UnaryOperator::Acos => {
                        if value < -1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Acos",
                                message: format!("value {} outside [-1, 1]", value),
                            }));
                        }
                        if value > 1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Acos",
                                message: format!("value {} outside [-1, 1]", value),
                            }));
                        }
                        value.acos()
                    }
                    UnaryOperator::Acosh => {
                        if value < 1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Acosh",
                                message: format!("value {} < 1", value),
                            }));
                        }
                        value.acosh()
                    }
                    UnaryOperator::Asin => {
                        if value < -1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Asin",
                                message: format!("value {} outside [-1, 1]", value),
                            }));
                        }
                        if value > 1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Asin",
                                message: format!("value {} outside [-1, 1]", value),
                            }));
                        }
                        value.asin()
                    }
                    UnaryOperator::Asinh => value.asinh(),
                    UnaryOperator::Atan => value.atan(),
                    UnaryOperator::Atanh => {
                        if value == -1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Atanh",
                                message: "value = -1 produces negative infinity".to_string(),
                            }));
                        }
                        if value == 1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Atanh",
                                message: "value = 1 produces positive infinity".to_string(),
                            }));
                        }
                        if value < -1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Atanh",
                                message: format!("value {} outside [-1, 1]", value),
                            }));
                        }
                        if value > 1.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Atanh",
                                message: format!("value {} outside [-1, 1]", value),
                            }));
                        }
                        value.atanh()
                    }
                    UnaryOperator::Ceil => value.ceil(),
                    UnaryOperator::Cos => value.cos(),
                    UnaryOperator::Cosh => value.cosh(),
                    UnaryOperator::Exp => value.exp(),
                    UnaryOperator::Exp2 => value.exp2(),
                    UnaryOperator::Floor => value.floor(),
                    UnaryOperator::Frac => value.fract(),
                    UnaryOperator::Log10 => {
                        if value == 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Log10",
                                message: "value = 0 is undefined".to_string(),
                            }));
                        }
                        if value < 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Log10",
                                message: format!(
                                    "value {} < 0 would produce a complex number",
                                    value
                                ),
                            }));
                        }
                        value.log10()
                    }
                    UnaryOperator::Log2 => {
                        if value == 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Log2",
                                message: "value = 0 is undefined".to_string(),
                            }));
                        }
                        if value < 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Log2",
                                message: format!(
                                    "value {} < 0 would produce a complex number",
                                    value
                                ),
                            }));
                        }
                        value.log2()
                    }
                    UnaryOperator::LogE => {
                        if value == 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "LogE",
                                message: "value = 0 is undefined".to_string(),
                            }));
                        }
                        if value < 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "LogE",
                                message: format!(
                                    "value {} < 0 would produce a complex number",
                                    value
                                ),
                            }));
                        }
                        value.ln()
                    }
                    UnaryOperator::Neg => -value,
                    UnaryOperator::RoundAway0 => value.round(),
                    UnaryOperator::RoundToEven => value.round_ties_even(),
                    UnaryOperator::Sin => value.sin(),
                    UnaryOperator::Sinh => value.sinh(),
                    UnaryOperator::Sqrt => {
                        if value < 0.0 {
                            return Err(StockTrekError::Stats(StatsError::DomainError {
                                function: "Sqrt",
                                message: format!(
                                    "value {} < 0 would produce a complex number",
                                    value
                                ),
                            }));
                        }
                        value.sqrt()
                    }
                    UnaryOperator::Tan => value.tan(),
                    UnaryOperator::Tanh => value.tanh(),
                    UnaryOperator::Trunc => value.trunc(),
                };
                Ok(calculation_result)
            }
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Log,
    Atan2,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnaryOperator {
    Abs,
    Neg,
    Floor,
    Ceil,
    RoundAway0,
    RoundToEven,
    Trunc,
    Frac,
    Sqrt,
    Exp,
    Exp2,
    Log2,
    LogE,
    Log10,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
}
