use async_trait::async_trait;
use common::domain::exception::DomainException;
use order_domain_core::domain::entity::Restaurant;
use order_domain_core::domain::exception::OrderDomainException;

#[async_trait]
pub trait RestaurantRepository {
    async fn find_restaurant_information(
        &self,
        restaurant: Restaurant,
    ) -> Result<Option<Restaurant>, DomainException<OrderDomainException>>;
}
