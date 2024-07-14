use common::domain::{
    entity::{AggregateRoot, BaseEntity},
    value_object::{BaseId, CustomerId, Money, OrderId, OrderStatus, RestaurantId},
};
use getset::{Getters, MutGetters};

use crate::domain::{
    exception::OrderDomainException,
    value_object::{OrderItemId, StreetAddress, TrackingId},
};

use super::OrderItem;

pub struct OrderData {
    pub id: OrderId<uuid::Uuid>,
    pub customer_id: CustomerId<uuid::Uuid>,
    pub restaurant_id: RestaurantId<uuid::Uuid>,
    pub street_address: StreetAddress,
    pub price: Money,
    pub items: Vec<OrderItem>,
    pub tracking_id: TrackingId<uuid::Uuid>,
    pub order_status: OrderStatus,
    pub failure_messages: Vec<String>,
}

#[derive(Debug, Getters, MutGetters, Clone)]
pub struct Order {
    id: OrderId<uuid::Uuid>,
    #[getset(get = "pub")]
    customer_id: CustomerId<uuid::Uuid>,
    #[getset(get = "pub")]
    restaurant_id: RestaurantId<uuid::Uuid>,
    #[getset(get = "pub")]
    street_address: StreetAddress,
    #[getset(get = "pub")]
    price: Money,
    #[getset(get = "pub", get_mut = "pub")]
    items: Vec<OrderItem>,
    #[getset(get = "pub")]
    tracking_id: TrackingId<uuid::Uuid>,
    #[getset(get = "pub")]
    order_status: OrderStatus,
    #[getset(get = "pub")]
    failure_messages: Vec<String>,
}

impl Order {
    pub fn new(order: OrderData) -> Self {
        Order::from(order)
    }
    pub fn from(order: OrderData) -> Self {
        Self {
            id: order.id,
            customer_id: order.customer_id,
            restaurant_id: order.restaurant_id,
            street_address: order.street_address,
            price: order.price,
            items: order.items,
            tracking_id: order.tracking_id,
            order_status: order.order_status,
            failure_messages: order.failure_messages,
        }
    }
    pub fn initialize_order(&mut self) {
        let id = OrderId::new(uuid::Uuid::now_v7());
        self.id = id;
        self.tracking_id = TrackingId::new(uuid::Uuid::now_v7());
        self.order_status = OrderStatus::Pending;
        self.initialize_order_items();
    }

    pub fn initialize_order_items(&mut self) {
        for item in self.items.iter_mut() {
            item.initialize_order_item(&self.id.clone(), OrderItemId::new(uuid::Uuid::now_v7()));
        }
    }

    pub fn pay(&mut self) -> Result<(), OrderDomainException> {
        if self.order_status != OrderStatus::Pending {
            return Err(OrderDomainException::new(
                "Order is not in correct state for payment operation!".to_string(),
                None,
            ));
        }
        self.order_status = OrderStatus::Paid;
        Ok(())
    }

    pub fn approve(&mut self) -> Result<(), OrderDomainException> {
        if self.order_status != OrderStatus::Paid {
            return Err(OrderDomainException::new(
                "Order is not in correct state for approval operation!".to_string(),
                None,
            ));
        }
        self.order_status = OrderStatus::Approved;
        Ok(())
    }

    pub fn init_cancel(
        &mut self,
        failure_message: Vec<String>,
    ) -> Result<(), OrderDomainException> {
        if self.order_status != OrderStatus::Paid {
            return Err(OrderDomainException::new(
                "Order is not in correct state for initCancel operation!".to_string(),
                None,
            ));
        }
        self.order_status = OrderStatus::Cancelling;
        self.update_failure_messages(failure_message);
        Ok(())
    }

    pub fn cancel(&mut self, failure_message: Vec<String>) -> Result<(), OrderDomainException> {
        if !(self.order_status == OrderStatus::Cancelling
            || self.order_status == OrderStatus::Pending)
        {
            return Err(OrderDomainException::new(
                "Order is not in correct state for cancel operation!".to_string(),
                None,
            ));
        }
        self.order_status = OrderStatus::Cancelled;
        self.update_failure_messages(failure_message);
        Ok(())
    }

    fn update_failure_messages(&mut self, failure_message: Vec<String>) {
        if !self.failure_messages().is_empty() && !failure_message.is_empty() {
            self.failure_messages = failure_message
                .iter()
                .filter(|message| !message.is_empty())
                .cloned()
                .collect();
            return;
        }
        if failure_message.is_empty() {
            self.failure_messages = failure_message;
        }
    }

    pub fn validate_order(&self) -> Result<(), OrderDomainException> {
        self.validate_order()?;
        self.validate_total_price()?;
        self.validate_items_price()?;
        Ok(())
    }
    fn validate_initalize_order(&self) -> Result<(), OrderDomainException> {
        if self.get_id().get_value().to_string().is_empty() {
            return Err(OrderDomainException::new(
                "Order is not in correct state for initialization!".to_string(),
                None,
            ));
        }
        Ok(())
    }
    fn validate_total_price(&self) -> Result<(), OrderDomainException> {
        if !self.price.is_greater_than_zero() {
            return Err(OrderDomainException::new(
                "Total price must be greater than zero!".to_string(),
                None,
            ));
        }
        Ok(())
    }
    fn validate_items_price(&self) -> Result<(), OrderDomainException> {
        for item in self.items.iter() {
            self.validate_item_price(item)?;
        }
        let order_items_total = self
            .items
            .iter()
            .map(|item| item.subtotal().clone())
            .reduce(|a, b| a.add(b.clone()));
        if let Some(total) = order_items_total {
            if !self.price.eq(&total) {
                let message = format!(
                    "Total price: {} is not equal to Order items total: {}!",
                    self.price.get_amount(),
                    total.get_amount()
                );
                return Err(OrderDomainException::new(message, None));
            }
        }
        Ok(())
    }
    fn validate_item_price(&self, order_item: &OrderItem) -> Result<(), OrderDomainException> {
        if !order_item.is_price_valid() {
            let message = format!(
                "Order item price: {} is not valid for the product {}.",
                order_item.price().get_amount(),
                order_item.product().get_id()
            );
            return Err(OrderDomainException::new(message, None));
        }
        Ok(())
    }
}

impl AggregateRoot<OrderId<uuid::Uuid>> for Order {}

impl BaseEntity<OrderId<uuid::Uuid>> for Order {
    fn get_id(&self) -> &OrderId<uuid::Uuid> {
        &self.id
    }

    fn set_id(&mut self, id: OrderId<uuid::Uuid>) {
        self.id = id;
    }
}
