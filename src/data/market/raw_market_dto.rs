use crate::data::market::{
    market_window::{AlignedWindow, RollingWindow},
    raw_market_candle::RawMarketCandle,
    raw_market_order_book::RawMarketOrderBook,
    raw_market_tick::RawMarketTick,
};
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawMarketDto {
    pub base_increment: Decimal,
    pub quote_increment: Decimal,
    pub ticks: Vec<RawMarketTick>,
    pub rolling: HashMap<RollingWindow, RawMarketCandle>,
    pub aligned: HashMap<AlignedWindow, Vec<RawMarketCandle>>,
    pub order_book: RawMarketOrderBook,
}
