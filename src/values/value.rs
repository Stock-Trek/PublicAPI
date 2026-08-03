use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
        value::ValueError,
    },
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId, tag::Tag};
use strum::Display;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    CexId(CexId),
    AssetId(AssetId),
    Flag(bool),
    Number(f64),
    CexIdSignal(SignalKey<CexId>),
    AssetIdSignal(SignalKey<AssetId>),
    FlagSignal(SignalKey<bool>),
    NumberSignal(SignalKey<f64>),
    ActiveOrders,
    ActiveOrdersWithTag(Tag),
    ActiveOrdersInCex(Box<Value>),
    ActiveOrdersInCexWithTag {
        cex_id_value: Box<Value>,
        tag: Tag,
    },
    AssetTotal(Box<Value>),
    AssetInCex {
        cex_id_value: Box<Value>,
        asset_id_value: Box<Value>,
    },
    AllocationForAssetTotal(Box<Value>),
    AllocationForAssetInCex {
        cex_id_value: Box<Value>,
        asset_id_value: Box<Value>,
    },
    Binary {
        left: Box<Value>,
        operator: BinaryOperator,
        right: Box<Value>,
    },
    Unary {
        number: Box<Value>,
        operator: UnaryOperator,
    },
}

impl Value {
    pub fn cex_id(&self, c: &ResolvedContext) -> StockTrekResult<CexId> {
        match self {
            Value::CexId(cex_id) => Ok(*cex_id),
            Value::CexIdSignal(key) => key.read(c),
            _ => err("CexId", self),
        }
    }

    pub fn asset_id(&self, c: &ResolvedContext) -> StockTrekResult<AssetId> {
        match self {
            Value::AssetId(asset_id) => Ok(*asset_id),
            Value::AssetIdSignal(key) => key.read(c),
            _ => err("AssetId", self),
        }
    }

    pub fn flag(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        match self {
            Value::Flag(flag) => Ok(*flag),
            Value::FlagSignal(key) => key.read(c),
            _ => err("Flag", self),
        }
    }

    pub fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        match self {
            Value::Number(number) => Ok(*number),
            Value::NumberSignal(key) => key.read(c),
            Value::ActiveOrders => Ok(c.portfolio.active_orders()),
            Value::ActiveOrdersWithTag(tag) => Ok(c.portfolio.active_orders_with_tag(tag)),
            Value::ActiveOrdersInCex(cex_id_value) => {
                let cex_id = cex_id_value.cex_id(c)?;
                Ok(c.portfolio.active_orders_in_cex(&cex_id))
            }
            Value::ActiveOrdersInCexWithTag { cex_id_value, tag } => {
                let cex_id = cex_id_value.cex_id(c)?;
                Ok(c.portfolio.active_orders_in_cex_with_tag(&cex_id, tag))
            }
            Value::AssetTotal(asset_id_value) => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_total(&asset_id))
            }
            Value::AssetInCex {
                cex_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_in_cex(&asset_id, &cex_id))
            }
            Value::AllocationForAssetTotal(asset_id_value) => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation.allocation_for_asset_total(&asset_id))
            }
            Value::AllocationForAssetInCex {
                cex_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation.allocation_for_asset_in_cex(&asset_id, &cex_id))
            }
            Value::Binary {
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
            Value::Unary { number, operator } => {
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
            _ => err("Number", self),
        }
    }

    fn type_name(&self) -> &'static str {
        match self {
            Value::CexId(_) => "CexId",
            Value::AssetId(_) => "AssetId",
            Value::Flag(_) => "Flag",
            Value::Number(_) => "Number",
            Value::CexIdSignal(_) => "CexIdSignal",
            Value::AssetIdSignal(_) => "AssetIdSignal",
            Value::FlagSignal(_) => "FlagSignal",
            Value::NumberSignal(_) => "NumberSignal",
            Value::ActiveOrders => "ActiveOrders",
            Value::ActiveOrdersWithTag(_) => "ActiveOrdersWithTag",
            Value::ActiveOrdersInCex(_) => "ActiveOrdersInCex",
            Value::ActiveOrdersInCexWithTag { .. } => "ActiveOrdersInCexWithTag",
            Value::AssetTotal(_) => "AssetTotal",
            Value::AssetInCex { .. } => "AssetInCex",
            Value::AllocationForAssetTotal(_) => "AllocationForAssetTotal",
            Value::AllocationForAssetInCex { .. } => "AllocationForAssetInCex",
            Value::Binary { .. } => "Binary",
            Value::Unary { .. } => "Unary",
        }
    }
}

impl From<CexId> for Value {
    fn from(value: CexId) -> Self {
        Value::CexId(value)
    }
}
impl From<AssetId> for Value {
    fn from(value: AssetId) -> Self {
        Value::AssetId(value)
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Flag(value)
    }
}
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl TryFrom<Value> for CexId {
    type Error = StockTrekError;
    fn try_from(value: Value) -> StockTrekResult<Self> {
        match value {
            Value::CexId(cex_id) => Ok(cex_id),
            other => err("CexId", &other),
        }
    }
}
impl TryFrom<Value> for AssetId {
    type Error = StockTrekError;
    fn try_from(value: Value) -> StockTrekResult<Self> {
        match value {
            Value::AssetId(asset_id) => Ok(asset_id),
            other => err("AssetId", &other),
        }
    }
}
impl TryFrom<Value> for bool {
    type Error = StockTrekError;
    fn try_from(value: Value) -> StockTrekResult<Self> {
        match value {
            Value::Flag(flag) => Ok(flag),
            other => err("Flag", &other),
        }
    }
}
impl TryFrom<Value> for f64 {
    type Error = StockTrekError;
    fn try_from(value: Value) -> StockTrekResult<Self> {
        match value {
            Value::Number(number) => Ok(number),
            other => err("Number", &other),
        }
    }
}

fn err<T>(expected: &str, found: &Value) -> StockTrekResult<T> {
    Err(StockTrekError::Value(ValueError::IncorrectType {
        expected: expected.to_string(),
        found: found.type_name().to_string(),
    }))
}
