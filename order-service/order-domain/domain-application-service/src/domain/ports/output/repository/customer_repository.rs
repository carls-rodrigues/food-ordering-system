use async_trait::async_trait;
use common::domain::exception::DomainException;
use order_domain_core::domain::entity::Customer;
use order_domain_core::domain::exception::OrderDomainException;
use uuid::Uuid;

#[async_trait]
pub trait CustomerRepository {
    async fn find_customer(
        &self,
        customer_id: &Uuid,
    ) -> Result<Option<Customer>, DomainException<OrderDomainException>>;
}
