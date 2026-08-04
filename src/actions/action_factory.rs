use crate::{
    actions::{
        action::Action,
        recoverable_action::{RecoverableAction, RecoveryPolicy},
    },
    values::value::{AssetIdValue, CexIdValue, NumberValue},
};
use stock_trek_types::cex::order_request::OrderRequest;

pub struct ActionFactory;

impl ActionFactory {
    pub fn send_order_request(
        &self,
        cex_id_value: CexIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
        recovery_policy: RecoveryPolicy,
    ) -> RecoverableAction {
        RecoverableAction::new(
            Action::SendOrderRequest {
                cex_id_value,
                order_request: Box::new(order_request),
            },
            recovery_policy,
        )
    }
    // TODO
    // pub fn cancel_order(&self, cex_id_value: CexIdValue, order_id: OrderId, recovery_policy: RecoveryPolicy) -> RecoverableAction {
    //   RecoverableAction::new(
    //     Action::CancelOrder { cex_id_value, order_id },
    //     recovery_policy,
    //   )
    // }
}
