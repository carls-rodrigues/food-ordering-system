use crate::domain::dto::message::RestaurantApprovalResponse;

pub trait RestaurantApprovalResponseMessageListener {
    fn order_approved(
        restaurant_approval_response: RestaurantApprovalResponse,
    ) -> Result<(), String>;
    fn order_reject(restaurant_approval_response: RestaurantApprovalResponse)
        -> Result<(), String>;
}
