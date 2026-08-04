use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
    util::serde_ordering,
    value::value::NumberValue,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};
use strum::Display;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    Compare {
        left: NumberValue,
        #[serde(with = "serde_ordering")]
        comparison: Ordering,
        right: NumberValue,
    },
    HasAccountInCex {
        cex_id: CexId,
    },
    Not {
        condition: Box<Condition>,
    },
    OwnsAsset {
        asset_id: AssetId,
    },
    OwnsAssetInCex {
        cex_id: CexId,
        asset_id: AssetId,
    },
    QuantityOf {
        quantity_of: QuantityOf,
        conditions: Vec<Condition>,
    },
    Signal {
        signal: SignalKey<bool>,
    },
}

impl Condition {
    pub fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        match self {
            Condition::Compare {
                left,
                comparison,
                right,
            } => {
                let left_value = left.number(c)?;
                let right_value = right.number(c)?;
                match left_value.partial_cmp(&right_value) {
                    Some(Ordering::Less) => Ok(comparison.is_le()),
                    Some(Ordering::Equal) => Ok(comparison.is_eq()),
                    Some(Ordering::Greater) => Ok(comparison.is_ge()),
                    None => Err(StockTrekError::Stats(StatsError::IncomparableValues {
                        left: left_value,
                        right: right_value,
                    })),
                }
            }
            Condition::HasAccountInCex { cex_id } => Ok(c.portfolio.has_account_in_cex(cex_id)),
            Condition::Not { condition } => {
                let test_result = condition.test(c)?;
                Ok(!test_result)
            }
            Condition::OwnsAsset { asset_id } => Ok(c.portfolio.owns_asset(asset_id)),
            Condition::OwnsAssetInCex { cex_id, asset_id } => {
                Ok(c.portfolio.owns_asset_in_cex(asset_id, cex_id))
            }
            Condition::QuantityOf {
                quantity_of,
                conditions,
            } => {
                if conditions.is_empty() {
                    let empty_result = match quantity_of {
                        QuantityOf::All => true,
                        QuantityOf::Partial => false,
                        QuantityOf::None => true,
                        QuantityOf::Empty => true,
                    };
                    return Ok(empty_result);
                }
                let mut true_count = 0;
                let mut false_count = 0;
                for condition in conditions {
                    if condition.test(c)? {
                        true_count += 1;
                    } else {
                        false_count += 1;
                    }
                }
                let quantity = match quantity_of {
                    QuantityOf::All => false_count == 0,
                    QuantityOf::Partial => (true_count > 0) && (false_count > 0),
                    QuantityOf::None => true_count == 0,
                    QuantityOf::Empty => (true_count == 0) && (false_count == 0),
                };
                Ok(quantity)
            }
            Condition::Signal { signal } => c.signals.read(signal),
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuantityOf {
    All,
    Partial,
    None,
    Empty,
}

pub struct ConditionFactory;

impl ConditionFactory {
    pub fn compare(
        &self,
        left: NumberValue,
        comparison: Ordering,
        right: NumberValue,
    ) -> Condition {
        Condition::Compare {
            left,
            comparison,
            right,
        }
    }
    pub fn has_account_in_cex(&self, cex_id: CexId) -> Condition {
        Condition::HasAccountInCex { cex_id }
    }
    pub fn not(&self, condition: Condition) -> Condition {
        Condition::Not {
            condition: Box::new(condition),
        }
    }
    pub fn owns_asset(&self, asset_id: AssetId) -> Condition {
        Condition::OwnsAsset { asset_id }
    }
    pub fn owns_asset_in_cex(&self, asset_id: AssetId, cex_id: CexId) -> Condition {
        Condition::OwnsAssetInCex { cex_id, asset_id }
    }
    pub fn quantity_of(&self, quantity_of: QuantityOf, conditions: Vec<Condition>) -> Condition {
        Condition::QuantityOf {
            quantity_of,
            conditions,
        }
    }
    pub fn signal(&self, flag: &SignalKey<bool>) -> Condition {
        Condition::Signal {
            signal: flag.clone(),
        }
    }
}
