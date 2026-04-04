use crate::{context::StockTrekContext, signal::StockTrekSignal};

pub trait StockTrekAlgorithm {
    fn create_signal(&self, context: StockTrekContext) -> StockTrekSignal;
}
