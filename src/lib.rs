mod action;
mod algorithm;
mod allocation;
mod cex;
mod commands;
mod conditions;
// pub mod dex;
mod error;
mod examples;
mod market_data;
mod portfolios;
mod preferences;
mod resolveable;
mod resolved_context;
mod signal;
mod signal_context;
mod strategy_context;
mod util;
mod value;

pub use algorithm::Algorithm;
pub use cex::order_factory::OrderFactory;
pub use commands::{Command, CommandFactory};
pub use conditions::{Condition, ConditionFactory, QuantityOf};
pub use portfolios::{InMemoryPortfolioBuilder, Portfolio, PortfolioFactory};
pub use preferences::Preferences;
pub use resolved_context::{EnqueueActionFn, ResolvedContext};
pub use rust_decimal::RoundingStrategy;
pub use strategy_context::StrategyContext;
pub use traitreg;
pub use traitreg::register as register_algorithm;

pub mod actions {
    pub use crate::action::action::Action;
    pub use crate::action::action_factory::ActionFactory;
    pub use crate::action::recoverable_action::{
        ActionErrorCause, ActionErrorResponse, RecoverableAction, RecoveryPolicy,
    };
    pub use crate::action::resolved_action::ResolvedAction;
}

pub mod allocations {
    pub use crate::allocation::{
        Allocation, AllocationFactory, Allocations, InMemoryAllocationBuilder,
    };
}

pub mod capabilities {
    pub use crate::cex::capability::{HasRequiredCapabilities, combine_capabilities};
}

pub mod errors {
    pub use crate::error::portfolio::PortfolioError;
    pub use crate::error::result::{StockTrekError, StockTrekResult};
    pub use crate::error::stats::StatsError;
    pub use crate::error::value::ValueError;
}

pub mod markets {
    pub use crate::market_data::aligned_window::AlignedWindow;
    pub use crate::market_data::market::Market;
    pub use crate::market_data::market::MarketBuilder;
    pub use crate::market_data::market_aligned_window::MarketAlignedWindow;
    pub use crate::market_data::market_candle::MarketCandle;
    pub use crate::market_data::market_ohlcv::MarketOhlcv;
    pub use crate::market_data::market_order_book::MarketOrderBook;
    pub use crate::market_data::market_quote::{MarketQuote, PriceQuantity, TimedPriceQuantity};
    pub use crate::market_data::market_rolling_window::{MarketRollingWindow, Ohlcv};
    pub use crate::market_data::market_tick::MarketTick;
    pub use crate::market_data::market_ticks::MarketTicks;
    pub use crate::market_data::rolling_window::RollingWindow;
    pub use crate::market_data::timestamp::TimestampMillis;
}

pub mod signals {
    pub use crate::signal::key::{SignalKey, SignalKeyType};
    pub use crate::signal::signals::Signals;
    pub use crate::signal::value::SignalValue;
    pub use crate::signal_context::SignalContext;
}

pub mod types {
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
}

pub mod values {
    pub use crate::value::binary_operator::BinaryOperator;
    pub use crate::value::unary_operator::UnaryOperator;
    pub use crate::value::value::{AssetIdValue, CexIdValue, FlagValue, NumberValue};
    pub use crate::value::values_factory::{
        AllocationValuesFactory, CalculationValuesFactory, LiteralValuesFactory,
        PortfolioValuesFactory, SignalValuesFactory,
    };
}

pub mod prelude {
    pub use super::{
        Algorithm, Command, Preferences, RoundingStrategy, StrategyContext,
        actions::{ActionErrorCause, ActionErrorResponse, RecoveryPolicy},
        register_algorithm,
        signals::{SignalContext, SignalKey, Signals},
        traitreg,
        types::{
            Activation, AssetId, CexId, CexPreferences, CexRoundingPreferences, Pricing, Quantity,
            Side, Tag,
        },
    };
}
