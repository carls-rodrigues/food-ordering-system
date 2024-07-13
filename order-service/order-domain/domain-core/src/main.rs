use std::str::FromStr;

use bigdecimal::BigDecimal;
use common::domain::value_object::{BaseId, CustomerId, Money, OrderId, OrderStatus, RestaurantId};
use order_domain_core::domain::{
    entity::{Order, OrderData, OrderItem, Product},
    value_object::{OrderItemId, StreetAddress, TrackingId},
};

fn main() {
    let id: OrderId<uuid::Uuid> = OrderId::new(uuid::Uuid::now_v7());
    let order_data = OrderData {
        id,
        customer_id: CustomerId::new(uuid::Uuid::now_v7()),
        restaurant_id: RestaurantId::new(uuid::Uuid::now_v7()),
        street_address: StreetAddress::new(
            uuid::Uuid::now_v7(),
            "123 Main St".to_string(),
            "122345".to_string(),
            "New York".to_string(),
        ),
        price: Money::new(BigDecimal::from_str("100").unwrap()),
        items: vec![OrderItem::from(
            OrderItemId::new(uuid::Uuid::now_v7()),
            OrderId::new(uuid::Uuid::now_v7()),
            Product::new(
                uuid::Uuid::now_v7(),
                "Product 1".to_string(),
                Money::new(BigDecimal::from_str("100").unwrap()),
            ),
            1,
            Money::new(BigDecimal::from_str("100").unwrap()),
            Money::new(BigDecimal::from_str("100").unwrap()),
        )],
        tracking_id: TrackingId::new(uuid::Uuid::now_v7()),
        order_status: OrderStatus::Pending,
        failure_messages: vec![],
    };
    let order = Order::from(order_data);
    println!("{:#?}", order)
}
