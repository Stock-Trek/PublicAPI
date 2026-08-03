use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    resolved_context::ResolvedContext,
    values::value::{
        AssetIdValue, AssetIdValueTrait, CexIdValue, CexIdValueTrait, FlagValue, FlagValueTrait,
        NumberValue, NumberValueTrait,
    },
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum SignalValue {
    CexId(CexId),
    AssetId(AssetId),
    Flag(bool),
    Number(f64),
}

impl From<AssetId> for SignalValue {
    fn from(value: AssetId) -> Self {
        SignalValue::AssetId(value)
    }
}
impl From<CexId> for SignalValue {
    fn from(value: CexId) -> Self {
        SignalValue::CexId(value)
    }
}
impl From<bool> for SignalValue {
    fn from(value: bool) -> Self {
        SignalValue::Flag(value)
    }
}
impl From<f64> for SignalValue {
    fn from(value: f64) -> Self {
        SignalValue::Number(value)
    }
}

impl TryFrom<SignalValue> for CexId {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::CexId(e) => Ok(e),
            SignalValue::AssetId(_) => err("CexId", "AssetId"),
            SignalValue::Flag(_) => err("CexId", "Flag"),
            SignalValue::Number(_) => err("CexId", "Number"),
        }
    }
}
impl TryFrom<SignalValue> for AssetId {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::CexId(_) => err("AssetId", "CexId"),
            SignalValue::AssetId(a) => Ok(a),
            SignalValue::Flag(_) => err("AssetId", "Flag"),
            SignalValue::Number(_) => err("AssetId", "Number"),
        }
    }
}
impl TryFrom<SignalValue> for bool {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::CexId(_) => err("Flag", "CexId"),
            SignalValue::AssetId(_) => err("Flag", "AssetId"),
            SignalValue::Flag(f) => Ok(f),
            SignalValue::Number(_) => err("Flag", "Number"),
        }
    }
}
impl TryFrom<SignalValue> for f64 {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::CexId(_) => err("Number", "CexId"),
            SignalValue::AssetId(_) => err("Number", "AssetId"),
            SignalValue::Flag(_) => err("Number", "Flag"),
            SignalValue::Number(n) => Ok(n),
        }
    }
}

#[typetag::serde]
impl CexIdValueTrait for SignalValue {
    fn clone_box(&self) -> CexIdValue {
        Box::new(self.clone())
    }
    fn cex_id(&self, _: &ResolvedContext) -> StockTrekResult<CexId> {
        CexId::try_from(self.clone())
    }
}

#[typetag::serde]
impl AssetIdValueTrait for SignalValue {
    fn clone_box(&self) -> AssetIdValue {
        Box::new(self.clone())
    }
    fn asset_id(&self, _: &ResolvedContext) -> StockTrekResult<AssetId> {
        AssetId::try_from(self.clone())
    }
}

#[typetag::serde]
impl FlagValueTrait for SignalValue {
    fn clone_box(&self) -> FlagValue {
        Box::new(self.clone())
    }
    fn flag(&self, _: &ResolvedContext) -> StockTrekResult<bool> {
        bool::try_from(self.clone())
    }
}

#[typetag::serde]
impl NumberValueTrait for SignalValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, _: &ResolvedContext) -> StockTrekResult<f64> {
        f64::try_from(self.clone())
    }
}

fn err<T>(expected: impl AsRef<str>, found: impl AsRef<str>) -> StockTrekResult<T> {
    Err(StockTrekError::Value(ValueError::IncorrectType {
        expected: expected.as_ref().to_string(),
        found: found.as_ref().to_string(),
    }))
}
