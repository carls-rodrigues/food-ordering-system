use std::sync::Arc;
use uuid::Uuid;
use common::domain::entity::BaseEntity;
use common::domain::value_object::BaseId;
use order_domain_core::domain::entity::{Order, Restaurant};
use order_domain_core::domain::exception::OrderDomainException;
use order_domain_core::domain::order_domain_service::OrderDomainService;
use crate::domain::dto::create::{CreateOrderCommand, CreateOrderResponse};
use crate::domain::mapper::OrderDataMapper;
use crate::domain::ports::output::repository::{CustomerRepository, OrderRepository, RestaurantRepository};

pub struct OrderCreateCommandHandler {
    order_domain_service: Arc<dyn OrderDomainService>,
    order_repository: Arc<dyn OrderRepository>,
    customer_repository: Arc<dyn CustomerRepository>,
    restaurant_repository: Arc<dyn RestaurantRepository>,
    order_data_mapper: Arc<OrderDataMapper>
}

impl OrderCreateCommandHandler {
    pub fn new(
        order_domain_service: impl OrderDomainService + 'static,
        order_repository: impl OrderRepository + 'static,
        customer_repository: impl CustomerRepository + 'static,
        restaurant_repository: impl RestaurantRepository + 'static,
        order_data_mapper: Arc<OrderDataMapper>
    ) -> Self {
        Self {
            order_domain_service: Arc::new(order_domain_service),
            order_repository: Arc::new(order_repository),
            customer_repository: Arc::new(customer_repository),
            restaurant_repository: Arc::new(restaurant_repository),
            order_data_mapper
        }
    }

    pub async fn create_order(&self, create_order_command: CreateOrderCommand) -> Result<CreateOrderResponse, OrderDomainException> {
        self.check_customer(create_order_command.customer_id()).await?;
        let restaurant = self.check_restaurant(&create_order_command).await?;
        let order = self.order_data_mapper.create_order_command_to_order(&create_order_command);
        let order_created_event = self.order_domain_service.validate_and_initiate_order(order.clone(), restaurant.clone())?;
        let order_result = self.save_order(order).await?;
        tracing::info!("Order created with order_id: {}", order_result.get_id().get_value());
        Ok(self.order_data_mapper.order_to_create_order_response(&order_result, None))

    }

    async fn check_customer(&self, customer_id: &Uuid) -> Result<(), OrderDomainException> {
        let customer = self.customer_repository.find_customer(customer_id).await?;
        if customer.is_none() {
            let message = format!("Could not find customer with customer_id: {}", customer_id);
            return Err(OrderDomainException::new(message, None));
        }
        Ok(())
    }

    async fn check_restaurant(&self, create_order_command: &CreateOrderCommand) -> Result<Restaurant, OrderDomainException> {
        let restaurant_mapped = self.order_data_mapper.create_order_command_to_restaurant(create_order_command);
        let restaurant = self.restaurant_repository.find_restaurant_information(restaurant_mapped).await?;
        if restaurant.is_none() {
            tracing::info!("Could not find restaurant with restaurant_id: {}", create_order_command.restaurant_id());
            let message = format!("Could not find restaurant with restaurant_id: {}", create_order_command.restaurant_id());
            return Err(OrderDomainException::new(message, None));
        }
        Ok(restaurant.unwrap())
    }
    async fn save_order(&self, order: Order) -> Result<Order, OrderDomainException> {
        let order_result = self.order_repository.save(order).await?;
        tracing::info!("Order created with order_id: {}", order_result.get_id().get_value());
        Ok(order_result)
    }
}