use crate::domain::dto::create::{
    CreateOrderCommand, CreateOrderResponse, CreateOrderResponseBuilder, OrderAddress, OrderItemDTO,
};
use crate::domain::dto::track::{TrackOrderResponse, TrackOrderResponseBuilder};
use common::domain::value_object::{BaseId, CustomerId, Money, OrderId, OrderStatus, RestaurantId};
use order_domain_core::domain::entity::{
    Order, OrderBuilder, OrderItem, OrderItemBuilder, Product, Restaurant, RestaurantBuilder,
};
use order_domain_core::domain::value_object::{OrderItemId, StreetAddress, TrackingId};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct OrderDataMapper {}

impl OrderDataMapper {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_order_command_to_restaurant(
        &self,
        create_order_commander: &CreateOrderCommand,
    ) -> Restaurant {
        RestaurantBuilder::default()
            .id(RestaurantId::new(
                create_order_commander.restaurant_id().to_owned(),
            ))
            .products(self.create_products(create_order_commander.order_items().clone()))
            .build()
            .unwrap()
    }
    pub fn create_order_command_to_order(
        &self,
        create_order_commander: &CreateOrderCommand,
    ) -> Order {
        OrderBuilder::default()
            .id(OrderId::new(Uuid::now_v7()))
            .tracking_id(Some(TrackingId::new(Uuid::now_v7())))
            .order_status(None)
            .failure_messages(vec![])
            .customer_id(CustomerId::new(
                create_order_commander.customer_id().to_owned(),
            ))
            .restaurant_id(RestaurantId::new(
                create_order_commander.restaurant_id().to_owned(),
            ))
            .street_address(
                self.order_address_to_street_address(create_order_commander.order_address()),
            )
            .price(Money::new(create_order_commander.price().clone()))
            .items(self.order_items_to_order_item_entities(create_order_commander.order_items()))
            .build()
            .unwrap()
    }

    pub fn order_to_create_order_response(
        &self,
        order: &Order,
        message: Option<String>,
    ) -> CreateOrderResponse {
        CreateOrderResponseBuilder::default()
            .order_tracking_id(order.tracking_id().clone())
            .order_status(
                order
                    .order_status()
                    .clone()
                    .unwrap_or(OrderStatus::Pending)
                    .clone(),
            )
            .message(message)
            .build()
            .unwrap()
    }
    pub fn order_to_track_order_response(&self, order: &Order) -> TrackOrderResponse {
        TrackOrderResponseBuilder::default()
            .order_tracking_id(order.tracking_id().clone().unwrap())
            .order_status(
                order
                    .order_status()
                    .clone()
                    .unwrap_or(OrderStatus::Pending)
                    .clone(),
            )
            .failure_message(order.failure_messages().clone())
            .build()
            .unwrap()
    }
    fn order_items_to_order_item_entities(&self, order_items: &[OrderItemDTO]) -> Vec<OrderItem> {
        order_items
            .iter()
            .map(|order_item| {
                OrderItemBuilder::default()
                    .id(OrderItemId::new(Uuid::now_v7()))
                    .order_id(None)
                    .product(Product::new(order_item.product_id().clone(), None, None))
                    .price(Money::new(order_item.price().clone()))
                    .quantity(order_item.quantity().to_owned())
                    .subtotal(Money::new(order_item.subtotal().clone()))
                    .build()
                    .unwrap()
            })
            .collect()
    }
    fn order_address_to_street_address(&self, order_address: &OrderAddress) -> StreetAddress {
        StreetAddress::new(
            Uuid::now_v7(),
            order_address.street().clone(),
            order_address.postal_code().clone(),
            order_address.city().clone(),
        )
    }
    fn create_products(&self, order_items: Vec<OrderItemDTO>) -> Vec<Product> {
        order_items
            .into_iter()
            .map(|order_item| Product::new(order_item.product_id().clone(), None, None))
            .collect()
    }
}
