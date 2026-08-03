use crate::values::value::Value;
use stock_trek_types::cex::{
    activation::Activation, order_request::OrderRequest, orders::single_order::SingleOrder,
    pricing::Pricing, quantity::Quantity, side::Side, tag::Tag,
};

pub struct OrderFactory;

impl OrderFactory {
    pub fn single(
        &self,
        base: Value,
        quote: Value,
        side: Side,
        activation: Activation<Value>,
        pricing: Pricing<Value>,
        quantity: Quantity<Value>,
        tag: Tag,
    ) -> OrderRequest<Value, Value> {
        OrderRequest::Single(SingleOrder {
            base,
            quote,
            side,
            activation,
            pricing,
            quantity,
            tag,
        })
    }
    // pub fn one_cancels_other(
    //     &self,
    //     primary: SingleOrderRaw,
    //     secondary: SingleOrderRaw,
    // ) -> OrderRequest<Value, Value> {
    //     OrderRequest::OneCancelsOther(OneCancelsOtherOrderGeneric { primary, secondary })
    // }
    // pub fn one_triggers_other(
    //     &self,
    //     primary: SingleOrderRaw,
    //     secondary: SingleOrderRaw,
    // ) -> OrderRequest<Value, Value> {
    //     OrderRequest::OneTriggersOther(OneTriggersOtherOrderGeneric { primary, secondary })
    // }
    // pub fn one_triggers_oco(
    //     &self,
    //     primary: SingleOrderRaw,
    //     oco_order: OneCancelsOtherOrderRaw,
    // ) -> OrderRequest<Value, Value> {
    //     OrderRequest::OneTriggersOco(OneTriggersOcoOrderGeneric { primary, oco_order })
    // }
}
