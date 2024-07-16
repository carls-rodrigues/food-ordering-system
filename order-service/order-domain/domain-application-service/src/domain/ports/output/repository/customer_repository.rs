use async_trait::async_trait;
use order_domain_core::domain::entity::Customer;
use uuid::Uuid;
use order_domain_core::domain::exception::OrderDomainException;

#[async_trait]
pub trait CustomerRepository {
    async fn find_customer(&self, customer_id: &Uuid) -> Result<Option<Customer>, OrderDomainException>;
}
