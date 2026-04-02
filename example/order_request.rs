use crate::data::{
    flags::{MarketSide, OrderStatus, TimeInForce},
    trading_pair::TradingPair,
};
use rust_decimal::Decimal;
use std::fmt;

#[derive(Debug)]
pub(crate) struct OrderCommon {
    order_id: String,
    symbol: String,
    side: MarketSide,
    quantity: Decimal,
    status: OrderStatus,
    filled_quantity: Decimal,
    avg_price: Option<Decimal>,
}

#[derive(Debug)]
pub struct LimitOrder {
    common: OrderCommon,
    price: Decimal,
    time_in_force: TimeInForce,
}

#[derive(Debug)]
pub struct MarketOrder {
    common: OrderCommon,
}

#[derive(Debug)]
pub struct OrderRequest {
    client_order_id: String,
    trading_pair: TradingPair,
    side: MarketSide,
    quantity: Decimal,
    kind: OrderKind,
    trigger: OrderTrigger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[repr(u8)]
pub enum OrderKind {
    Market,
    Limit { price: Decimal },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[repr(u8)]
pub enum OrderTrigger {
    None,
    Stop { stop_price: Decimal },
}

impl fmt::Display for OrderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderKind::Market => write!(f, "Market"),
            OrderKind::Limit { price } => write!(f, "Limit({})", price),
        }
    }
}

impl fmt::Display for OrderTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderTrigger::None => write!(f, "None"),
            OrderTrigger::Stop { stop_price } => write!(f, "Stop({})", stop_price),
        }
    }
}

impl OrderRequest {
    pub(crate) fn new(
        client_order_id: String,
        trading_pair: TradingPair,
        side: MarketSide,
        kind: OrderKind,
        trigger: OrderTrigger,
        quantity: Decimal,
    ) -> Self {
        Self {
            client_order_id,
            trading_pair,
            side,
            kind,
            trigger,
            quantity,
        }
    }
    pub fn client_order_id(&self) -> &str {
        &self.client_order_id
    }
    pub fn trading_pair(&self) -> &TradingPair {
        &self.trading_pair
    }
    pub fn side(&self) -> &MarketSide {
        &self.side
    }
    pub fn kind(&self) -> &OrderKind {
        &self.kind
    }
    pub fn trigger(&self) -> &OrderTrigger {
        &self.trigger
    }
    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}
