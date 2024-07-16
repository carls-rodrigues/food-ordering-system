use async_trait::async_trait;
use order_domain_core::domain::entity::Restaurant;

#[async_trait]
pub trait RestaurantRepository {
    async fn find_restaurant_information(
        &self,
        restaurant: Restaurant,
    ) -> Result<Option<Restaurant>, String>;
}
