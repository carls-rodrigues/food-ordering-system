use std::collections::HashMap;

use super::{
    entity::{Order, Product, Restaurant},
    event::{OrderCancelledEvent, OrderCreatedEvent, OrderPaidEvent},
    exception::OrderDomainException,
    order_domain_service::OrderDomainService,
};
use common::domain::exception::DomainException;
use common::domain::{entity::BaseEntity, value_object::BaseId};

#[derive(Debug, Default)]
pub struct OrderDomainServiceImpl {}

impl OrderDomainServiceImpl {
    pub fn new() -> Self {
        Self {}
    }

    fn validate_restaurant(
        &self,
        restaurant: &Restaurant,
    ) -> Result<(), DomainException<OrderDomainException>> {
        if !restaurant.active() {
            let message = format!(
                "Restaurant with id {} is currently not active",
                restaurant.get_id().get_value()
            );
            return Err(DomainException::DomainError(OrderDomainException::new(
                message, None,
            )));
        }
        Ok(())
    }
    fn set_order_product_information(
        &self,
        order: &mut Order,
        restaurant: &Restaurant,
    ) -> Result<(), DomainException<OrderDomainException>> {
        let restaurant_products = restaurant
            .products()
            .iter()
            .map(|p| (p.get_id().get_value(), p.clone()))
            .collect::<HashMap<&uuid::Uuid, Product>>();

        for order_item in order.items_mut() {
            if let Some(restaurant_product) =
                restaurant_products.get(order_item.product().get_id().get_value())
            {
                order_item
                    .product_mut()
                    .update_with_confirmed_name_and_price(
                        restaurant_product.name(),
                        restaurant_product.price(),
                    );
            }
        }
        // order.items().iter().for_each(|order_item| {
        //     restaurant.products().iter().for_each(|restaurant_product| {
        //         let current_product = order_item.product();
        //         if current_product.eq(restaurant_product) {
        //             current_product.update_with_confirmed_name_and_price(
        //                 restaurant_product.name(),
        //                 restaurant_product.price(),
        //             );
        //         }
        //     })
        // });
        Ok(())
    }
}

impl OrderDomainService for OrderDomainServiceImpl {
    fn validate_and_initiate_order(
        &self,
        mut order: Order,
        restaurant: Restaurant,
    ) -> Result<OrderCreatedEvent, DomainException<OrderDomainException>> {
        self.validate_restaurant(&restaurant)?;
        self.set_order_product_information(&mut order, &restaurant)?;
        order.validate_order()?;
        order.initialize_order();
        tracing::info!("Order with id {} is initiated", order.get_id().get_value());
        Ok(OrderCreatedEvent::new(order))
    }

    fn pay_order(
        &self,
        order: &mut Order,
    ) -> Result<OrderPaidEvent, DomainException<OrderDomainException>> {
        order.pay()?;
        tracing::info!("Order with id {} is paid", order.get_id().get_value());
        Ok(OrderPaidEvent::new(order.clone()))
    }

    fn approve_order(
        &self,
        order: &mut Order,
    ) -> Result<(), DomainException<OrderDomainException>> {
        order.approve()?;
        tracing::info!("Order with id {} is approved", order.get_id().get_value());
        Ok(())
    }

    fn cancel_order_payment(
        &self,
        order: &mut Order,
        failure_messages: Vec<String>,
    ) -> Result<OrderCancelledEvent, DomainException<OrderDomainException>> {
        order.init_cancel(failure_messages)?;
        tracing::info!(
            "Order payment is cancelling for order id: {}",
            order.get_id().get_value()
        );
        Ok(OrderCancelledEvent::new(order.clone()))
    }

    fn cancel_order(
        &self,
        order: &mut Order,
        failure_messages: Vec<String>,
    ) -> Result<(), DomainException<OrderDomainException>> {
        order.cancel(failure_messages)?;
        tracing::info!("Order with id {} is cancelled", order.get_id().get_value());
        Ok(())
    }
}
