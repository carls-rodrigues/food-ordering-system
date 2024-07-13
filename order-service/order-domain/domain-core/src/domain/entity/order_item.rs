use common::domain::{
    entity::BaseEntity,
    value_object::{Money, OrderId},
};
use getset::Getters;

use super::Product;

#[derive(Debug, Getters)]
pub struct OrderItem<ID> {
    id: ID,
    #[getset(get = "pub")]
    order_id: OrderId<uuid::Uuid>,
    #[getset(get = "pub")]
    product: Product<uuid::Uuid>,
    #[getset(get = "pub")]
    quantity: i32,
    #[getset(get = "pub")]
    price: Money,
    #[getset(get = "pub")]
    subtotal: Money,
}

impl<ID> OrderItem<ID> {
    pub fn new(
        id: ID,
        order_id: OrderId<uuid::Uuid>,
        product: Product<uuid::Uuid>,
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
}

impl<ID: PartialEq> BaseEntity<ID> for OrderItem<ID> {
    fn get_id(&self) -> &ID {
        &self.id
    }

    fn set_id(&mut self, id: ID) {
        self.id = id;
    }
}
