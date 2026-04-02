use std::collections::HashSet;

use rust_decimal::Decimal;

use crate::data::{flags::MarketSide, trading_pair::TradingPair};

#[derive(Debug)]
pub struct Position {
    pair: TradingPair,
    side: MarketSide,
    quantity: Decimal,
    quantity_signed: Decimal,
    entry_price: Decimal,
    realized_pnl: Decimal,
    unrealized_pnl: Decimal,
    trade_ids: HashSet<String>,
}

impl Position {
    pub fn new(
        pair: TradingPair,
        side: MarketSide,
        quantity: Decimal,
        entry_price: Decimal,
        realized_pnl: Decimal,
        unrealized_pnl: Decimal,
        trade_ids: HashSet<String>,
    ) -> Self {
        let quantity_signed = match side {
            MarketSide::Buy => quantity,
            MarketSide::Sell => -quantity,
        };
        Self {
            pair,
            side,
            quantity,
            quantity_signed,
            entry_price,
            realized_pnl,
            unrealized_pnl,
            trade_ids,
        }
    }
    pub fn pair(&self) -> &TradingPair {
        &self.pair
    }
    pub fn side(&self) -> MarketSide {
        self.side
    }
    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
    pub fn quantity_signed(&self) -> Decimal {
        self.quantity_signed
    }
    pub fn entry_price(&self) -> Decimal {
        self.entry_price
    }
    pub fn realized_pnl(&self) -> Decimal {
        self.realized_pnl
    }
    pub fn unrealized_pnl(&self) -> Decimal {
        self.unrealized_pnl
    }
    pub fn trade_ids(&self) -> &HashSet<String> {
        &self.trade_ids
    }
}
