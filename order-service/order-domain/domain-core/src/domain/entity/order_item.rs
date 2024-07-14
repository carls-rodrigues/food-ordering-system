use bigdecimal::BigDecimal;
use common::domain::{
    entity::BaseEntity,
    value_object::{Money, OrderId},
};
use getset::{Getters, MutGetters};

use crate::domain::value_object::OrderItemId;

use super::Product;

#[derive(Debug, Getters, MutGetters, Clone)]
pub struct OrderItem {
    id: OrderItemId<uuid::Uuid>,
    #[getset(get = "pub")]
    order_id: OrderId<uuid::Uuid>,
    #[getset(get = "pub", get_mut = "pub")]
    product: Product<uuid::Uuid>,
    #[getset(get = "pub")]
    quantity: i32,
    #[getset(get = "pub")]
    price: Money,
    #[getset(get = "pub")]
    subtotal: Money,
}

impl OrderItem {
    pub fn from(
        id: OrderItemId<uuid::Uuid>,
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
    pub fn initialize_order_item(
        &mut self,
        order_id: &OrderId<uuid::Uuid>,
        order_item_id: OrderItemId<uuid::Uuid>,
    ) {
        self.id = order_item_id;
        self.order_id = order_id.clone();
    }
    pub fn is_price_valid(&self) -> bool {
        self.price.is_greater_than_zero()
            && self.price.eq(self.product.price())
            && self
                .price()
                .multiply(Money::new(BigDecimal::from(self.quantity)))
                .eq(self.subtotal())
    }
}

impl BaseEntity<OrderItemId<uuid::Uuid>> for OrderItem {
    fn get_id(&self) -> &OrderItemId<uuid::Uuid> {
        &self.id
    }

    fn set_id(&mut self, id: OrderItemId<uuid::Uuid>) {
        self.id = id;
    }
}
