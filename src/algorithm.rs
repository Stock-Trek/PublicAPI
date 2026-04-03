use crate::{data::context::StockTrekContext, schemas::signal::StockTrekSignal};

pub trait StockTrekAlgorithm {
    fn create_signal(&self, context: StockTrekContext) -> StockTrekSignal;
}
