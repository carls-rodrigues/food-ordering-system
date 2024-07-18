use crate::domain::value_object::OrderItemId;
use bigdecimal::BigDecimal;
use common::domain::value_object::BaseId;
use common::domain::{
    entity::BaseEntity,
    value_object::{Money, OrderId},
};
use derive_builder::Builder;
use getset::{Getters, MutGetters};
use uuid::Uuid;

use super::Product;

#[derive(Debug, Builder, Getters, MutGetters, Clone)]
pub struct OrderItem {
    id: OrderItemId<Uuid>,
    #[getset(get = "pub")]
    order_id: Option<OrderId<Uuid>>,
    #[getset(get = "pub", get_mut = "pub")]
    product: Product,
    #[getset(get = "pub")]
    quantity: i32,
    #[getset(get = "pub")]
    price: Money,
    #[getset(get = "pub")]
    subtotal: Money,
}

impl OrderItem {
    pub fn new(
        order_id: Option<OrderId<Uuid>>,
        product: Product,
        quantity: i32,
        price: Money,
        subtotal: Money,
    ) -> Self {
        Self::from(
            OrderItemId::new(Uuid::now_v7()),
            order_id,
            product,
            quantity,
            price,
            subtotal,
        )
    }
    pub fn from(
        id: OrderItemId<Uuid>,
        order_id: Option<OrderId<Uuid>>,
        product: Product,
        quantity: i32,
        price: Money,
        subtotal: Money,
    ) -> Self {
        Self {
            id,
            order_id,
            product,
            quantity,
            price,
            subtotal,
        }
    }
    pub fn initialize_order_item(
        &mut self,
        order_id: &OrderId<Uuid>,
        order_item_id: OrderItemId<Uuid>,
    ) {
        self.id = order_item_id;
        self.order_id = Some(order_id.clone());
    }
    pub fn is_price_valid(&self) -> bool {
        self.price.is_greater_than_zero()
            && self.price.eq(&self.product.price().clone().unwrap())
            && self
                .price()
                .multiply(Money::new(BigDecimal::from(self.quantity)))
                .eq(self.subtotal())
    }
}

impl BaseEntity<OrderItemId<Uuid>> for OrderItem {
    fn get_id(&self) -> &OrderItemId<Uuid> {
        &self.id
    }

    fn set_id(&mut self, id: OrderItemId<Uuid>) {
        self.id = id;
    }
}
