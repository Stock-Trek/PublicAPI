use crate::{
    signal::key::SignalKey,
    values::value::{BinaryOperator, UnaryOperator, Value},
};
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId, tag::Tag};

pub struct AllocationValuesFactory;
pub struct PortfolioValuesFactory;
pub struct CalculationValuesFactory;
pub struct LiteralValuesFactory;
pub struct SignalValuesFactory;

impl AllocationValuesFactory {
    pub fn allocation_for_asset_in_cex(&self, cex_id_value: Value, asset_id_value: Value) -> Value {
        Value::AllocationForAssetInCex {
            cex_id_value: Box::new(cex_id_value),
            asset_id_value: Box::new(asset_id_value),
        }
    }
    pub fn asset_total(&self, asset_id_value: Value) -> Value {
        Value::AllocationForAssetTotal(Box::new(asset_id_value))
    }
}

impl PortfolioValuesFactory {
    pub fn asset_in_cex(&self, cex_id_value: Value, asset_id_value: Value) -> Value {
        Value::AssetInCex {
            cex_id_value: Box::new(cex_id_value),
            asset_id_value: Box::new(asset_id_value),
        }
    }
    pub fn asset_total(&self, asset_id_value: Value) -> Value {
        Value::AssetTotal(Box::new(asset_id_value))
    }
    pub fn active_orders_in_cex(&self, cex_id_value: Value) -> Value {
        Value::ActiveOrdersInCex(Box::new(cex_id_value))
    }
    pub fn active_orders_in_cex_with_tag(&self, cex_id_value: Value, tag: Tag) -> Value {
        Value::ActiveOrdersInCexWithTag {
            cex_id_value: Box::new(cex_id_value),
            tag,
        }
    }
    pub fn active_orders(&self) -> Value {
        Value::ActiveOrders
    }
    pub fn active_orders_with_tag(&self, tag: Tag) -> Value {
        Value::ActiveOrdersWithTag(tag)
    }
}

impl CalculationValuesFactory {
    pub fn binary(&self, left: Value, operator: BinaryOperator, right: Value) -> Value {
        Value::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
    pub fn unary(&self, number: Value, operator: UnaryOperator) -> Value {
        Value::Unary {
            number: Box::new(number),
            operator,
        }
    }
}

impl LiteralValuesFactory {
    pub fn cex_id(&self, literal: CexId) -> Value {
        Value::CexId(literal)
    }
    pub fn asset_id(&self, literal: AssetId) -> Value {
        Value::AssetId(literal)
    }
    pub fn flag(&self, literal: bool) -> Value {
        Value::Flag(literal)
    }
    pub fn number(&self, literal: f64) -> Value {
        Value::Number(literal)
    }
}

impl SignalValuesFactory {
    pub fn cex_id(&self, key: &SignalKey<CexId>) -> Value {
        Value::CexIdSignal(key.clone())
    }
    pub fn asset_id(&self, key: &SignalKey<AssetId>) -> Value {
        Value::AssetIdSignal(key.clone())
    }
    pub fn flag(&self, key: &SignalKey<bool>) -> Value {
        Value::FlagSignal(key.clone())
    }
    pub fn number(&self, key: &SignalKey<f64>) -> Value {
        Value::NumberSignal(key.clone())
    }
}
