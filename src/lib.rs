//! Stock Trek time-series analysis.
//!
//! Core types for authoring trading algorithms that run on
//! [stock-trek.com](https://stock-trek.com): the [`Algorithm`] trait, the
//! signal and strategy contexts ([`SignalContext`], [`StrategyContext`]), and
//! the commands, actions, conditions, and values used to express trading
//! decisions ([`Command`], [`Action`], [`Condition`], [`Preferences`]).
//!
//! # Importing
//!
//! The most commonly used types are re-exported at the crate root and in the
//! [`prelude`] module, so consumers can import them either way:
//!
//! ```
//! use stock_trek::prelude::*;
//! // or
//! use stock_trek::{Algorithm, Command, Preferences, Signals};
//! ```

pub mod actions;
pub mod algorithm;
pub mod allocations;
pub mod cex;
pub mod commands;
pub mod conditions;
// pub mod dex;
pub mod error;
pub mod examples;
pub mod market_data;
pub mod portfolios;
pub mod preferences;
pub mod resolveable;
pub mod resolved_context;
pub mod signal;
pub mod signal_context;
pub mod strategy_context;
pub mod util;
pub mod values;

pub use actions::action::Action;
pub use actions::action_factory::ActionFactory;
pub use actions::recoverable_action::{
    ErrorCause, ErrorResponse, RecoverableAction, RecoveryPolicy,
};
pub use actions::resolved_action::ResolvedAction;
pub use algorithm::Algorithm;
pub use allocations::{Allocation, AllocationFactory, Allocations, InMemoryAllocationBuilder};
pub use cex::capability::{HasRequiredCapabilities, combine_capabilities};
pub use cex::order_factory::OrderFactory;
pub use commands::{Command, CommandFactory};
pub use conditions::{Condition, ConditionFactory, QuantityOf};
pub use error::portfolio::PortfolioError;
pub use error::result::{StockTrekError, StockTrekResult};
pub use error::stats::StatsError;
pub use error::value::ValueError;
pub use error::verification::VerificationError;
pub use market_data::aligned_window::AlignedWindow;
pub use market_data::market::Market;
pub use market_data::market_aligned_window::MarketAlignedWindow;
pub use market_data::market_candle::MarketCandle;
pub use market_data::market_ohlcv::MarketOhlcv;
pub use market_data::market_order_book::MarketOrderBook;
pub use market_data::market_quote::{MarketQuote, PriceQuantity, TimedPriceQuantity};
pub use market_data::market_rolling_window::{MarketRollingWindow, Ohlcv};
pub use market_data::market_tick::MarketTick;
pub use market_data::market_ticks::MarketTicks;
pub use market_data::rolling_window::RollingWindow;
pub use market_data::timestamp::TimestampMillis;
pub use portfolios::{Assets, InMemoryPortfolioBuilder, Portfolio, PortfolioFactory};
pub use preferences::Preferences;
pub use resolveable::Resolvable;
pub use resolved_context::{EnqueueActionFn, ResolvedContext};
pub use signal::key::{SignalKey, SignalKeyType};
pub use signal::signals::Signals;
pub use signal::value::SignalValue;
pub use signal_context::{CexMarketDataByBaseContext, CexMarketDataByQuoteContext, SignalContext};
pub use strategy_context::StrategyContext;
pub use values::binary_operator::BinaryOperator;
pub use values::unary_operator::UnaryOperator;
pub use values::value::{AssetIdValue, CexIdValue, FlagValue, NumberValue};
pub use values::values_factory::{
    AllocationValuesFactory, CalculationValuesFactory, LiteralValuesFactory,
    PortfolioValuesFactory, SignalValuesFactory,
};

pub mod prelude {
    pub use super::{
        Action, ActionFactory, Algorithm, Allocation, AllocationFactory, Allocations, Assets,
        CexMarketDataByBaseContext, CexMarketDataByQuoteContext, Command, CommandFactory,
        Condition, ConditionFactory, EnqueueActionFn, ErrorCause, ErrorResponse,
        InMemoryAllocationBuilder, InMemoryPortfolioBuilder, OrderFactory, Portfolio,
        PortfolioFactory, Preferences, QuantityOf, RecoverableAction, RecoveryPolicy, Resolvable,
        ResolvedAction, ResolvedContext, SignalContext, SignalKey, SignalKeyType, SignalValue,
        Signals, StockTrekError, StockTrekResult, StrategyContext, VerificationError,
    };

    pub use rust_decimal::RoundingStrategy;

    pub use stock_trek_types::cex::{
        activation::Activation,
        asset_id::AssetId,
        capability::CexCapability,
        cex_id::CexId,
        order_request::OrderRequest,
        order_response::OrderResponse,
        orders::single_order::SingleOrder,
        preferences::{CexPreferences, CexRoundingPreferences},
        price_basis::PriceBasis,
        pricing::Pricing,
        quantity::Quantity,
        side::Side,
        status::Status,
        tag::Tag,
        time_in_force::TimeInForce,
        trading_pair::TradingPair,
        trigger_direction::TriggerDirection,
        trigger_mode::TriggerMode,
    };

    pub use traitreg;
    pub use traitreg::register as register_algorithm;
}
