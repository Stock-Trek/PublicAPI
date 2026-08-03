use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
    values::{binary_calculation::BinaryOperator, unary_calculation::UnaryOperator},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId, tag::Tag};

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
                operator.calculate(left_value, right_value)
            }
            Self::UnaryCalculation { number, operator } => {
                let value = number.number(c)?;
                operator.calculate(value)
            }
        }
    }
}
