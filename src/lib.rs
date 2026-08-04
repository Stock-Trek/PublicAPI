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

pub use crate::actions::{
    Action, ActionFactory, ErrorCause, ErrorResponse, RecoverableAction, RecoveryPolicy,
    ResolvedAction,
};
pub use crate::algorithm::Algorithm;
pub use crate::allocations::{
    Allocation, AllocationFactory, Allocations, InMemoryAllocationBuilder,
};
pub use crate::cex::{combine_capabilities, HasRequiredCapabilities, OrderFactory};
pub use crate::commands::{Command, CommandFactory};
pub use crate::conditions::{Condition, ConditionFactory, QuantityOf};
pub use crate::error::{
    PortfolioError, StatsError, StockTrekError, StockTrekResult, ValueError, VerificationError,
};
pub use crate::market_data::{
    AlignedWindow, Market, MarketAlignedWindow, MarketCandle, MarketOhlcv, MarketOrderBook,
    MarketQuote, MarketRollingWindow, MarketTick, MarketTicks, Ohlcv, PriceQuantity,
    RollingWindow, TimedPriceQuantity, TimestampMillis,
};
pub use crate::portfolios::{Assets, InMemoryPortfolioBuilder, Portfolio, PortfolioFactory};
pub use crate::preferences::Preferences;
pub use crate::resolveable::Resolvable;
pub use crate::resolved_context::{EnqueueActionFn, ResolvedContext};
pub use crate::signal::{SignalKey, SignalKeyType, Signals, SignalValue};
pub use crate::signal_context::{
    CexMarketDataByBaseContext, CexMarketDataByQuoteContext, SignalContext,
};
pub use crate::strategy_context::StrategyContext;
pub use crate::values::{
    AllocationValuesFactory, AssetIdValue, BinaryOperator, CalculationValuesFactory, CexIdValue,
    FlagValue, LiteralValuesFactory, NumberValue, PortfolioValuesFactory, SignalValuesFactory,
    UnaryOperator,
};

pub mod prelude {
    pub use crate::{
        Action, ActionFactory, Algorithm, Allocation, AllocationFactory, Allocations, Assets,
        CexMarketDataByBaseContext, CexMarketDataByQuoteContext, Command, CommandFactory,
        Condition, ConditionFactory, EnqueueActionFn, ErrorCause, ErrorResponse,
        InMemoryAllocationBuilder, InMemoryPortfolioBuilder, OrderFactory, Portfolio,
        PortfolioFactory, Preferences, QuantityOf, RecoverableAction, RecoveryPolicy, Resolvable,
        ResolvedAction, ResolvedContext, SignalContext, SignalKey, SignalKeyType, Signals,
        SignalValue, StockTrekError, StockTrekResult, StrategyContext, VerificationError,
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
