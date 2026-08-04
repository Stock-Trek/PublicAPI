use crate::{
    action::{recoverable_action::RecoveryPolicy, resolved_action::ResolvedAction},
    allocation::Allocation,
    error::result::StockTrekResult,
    portfolios::Portfolio,
    signal::signals::Signals,
};

pub struct ResolvedContext {
    pub enqueue_action: EnqueueActionFn,
    pub allocation: Allocation,
    pub portfolio: Portfolio,
    pub signals: Signals,
}

pub type EnqueueActionFn = Box<dyn Fn(&ResolvedAction, &RecoveryPolicy) -> StockTrekResult<()>>;
