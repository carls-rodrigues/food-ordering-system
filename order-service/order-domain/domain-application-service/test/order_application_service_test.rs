use bigdecimal::BigDecimal;
use common::domain::value_object::{BaseId, CustomerId, Money, ProductId};
use getset::Getters;
use order_domain_application_service::domain::dto::create::{
    CreateOrderCommand, CreateOrderCommandBuilder, OrderAddress, OrderAddressBuilder,
    OrderItemDTOBuilder,
};
use order_domain_application_service::domain::mapper::OrderDataMapper;
use order_domain_application_service::domain::ports::input::service::OrderApplicationService;
use order_domain_application_service::domain::ports::output::repository::{
    CustomerRepository, OrderRepository, RestaurantRepository,
};
use order_domain_core::domain::entity::{Customer, Product, Restaurant};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Getters)]
pub struct OrderApplicationServiceTest {
    #[getset(get = "pub")]
    order_data_mapper: OrderDataMapper,
    #[getset(get = "pub")]
    order_application_service: Box<dyn OrderApplicationService>,
    #[getset(get = "pub")]
    order_repository: Box<dyn OrderRepository>,
    #[getset(get = "pub")]
    customer_repository: Box<dyn CustomerRepository>,
    #[getset(get = "pub")]
    restaurant_repository: Box<dyn RestaurantRepository>,
    #[getset(get = "pub")]
    create_order_command: Option<CreateOrderCommand>,
    create_order_command_wrong_price: Option<CreateOrderCommand>,
    create_order_command_wrong_product_price: Option<CreateOrderCommand>,
    #[getset(get = "pub")]
    customer_id: Uuid,
    #[getset(get = "pub")]
    restaurant_id: Uuid,
    #[getset(get = "pub")]
    order_id: Uuid,
    #[getset(get = "pub")]
    product_id: Uuid,
    #[getset(get = "pub")]
    price: BigDecimal,
}

impl OrderApplicationServiceTest {
    pub fn new(
        order_data_mapper: OrderDataMapper,
        order_application_service: impl OrderApplicationService + 'static,
        order_repository: impl OrderRepository + 'static,
        customer_repository: impl CustomerRepository + 'static,
        restaurant_repository: impl RestaurantRepository + 'static,
    ) -> Self {
        Self {
            order_data_mapper,
            order_application_service: Box::new(order_application_service),
            order_repository: Box::new(order_repository),
            customer_repository: Box::new(customer_repository),
            restaurant_repository: Box::new(restaurant_repository),
            create_order_command: None,
            create_order_command_wrong_price: None,
            create_order_command_wrong_product_price: None,
            customer_id: Uuid::now_v7(),
            restaurant_id: Uuid::now_v7(),
            order_id: Uuid::now_v7(),
            product_id: Uuid::now_v7(),
            price: BigDecimal::from_str("200.00").unwrap(),
        }
    }
    async fn before_all(&mut self) {
        self.create_order_command = Some(
            CreateOrderCommandBuilder::default()
                .customer_id(self.customer_id)
                .restaurant_id(self.restaurant_id)
                .order_address(
                    OrderAddressBuilder::default()
                        .street("Street 1".to_string())
                        .postal_code("1000AB".to_string())
                        .city("Paris".to_string())
                        .build()
                        .unwrap(),
                )
                .price(BigDecimal::from_str("250.00").unwrap())
                .order_items(vec![
                    OrderItemDTOBuilder::default()
                        .product_id(ProductId::new(self.product_id))
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("50.00").unwrap())
                        .build()
                        .unwrap(),
                    OrderItemDTOBuilder::default()
                        .product_id(ProductId::new(self.product_id))
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("150.00").unwrap())
                        .build()
                        .unwrap(),
                ])
                .build()
                .unwrap(),
        );
        self.create_order_command_wrong_price = Some(
            CreateOrderCommandBuilder::default()
                .customer_id(self.customer_id)
                .restaurant_id(self.restaurant_id)
                .order_address(
                    OrderAddressBuilder::default()
                        .street("Street 1".to_string())
                        .postal_code("1000AB".to_string())
                        .city("Amsterdam".to_string())
                        .build()
                        .unwrap(),
                )
                .price(BigDecimal::from_str("210.00").unwrap())
                .order_items(vec![
                    OrderItemDTOBuilder::default()
                        .product_id(ProductId::new(self.product_id))
                        .quantity(1)
                        .price(BigDecimal::from_str("60.00").unwrap())
                        .subtotal(BigDecimal::from_str("60.00").unwrap())
                        .build()
                        .unwrap(),
                    OrderItemDTOBuilder::default()
                        .product_id(ProductId::new(self.product_id))
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("150.00").unwrap())
                        .build()
                        .unwrap(),
                ])
                .build()
                .unwrap(),
        );
        self.create_order_command_wrong_product_price = Some(
            CreateOrderCommandBuilder::default()
                .customer_id(self.customer_id.clone())
                .restaurant_id(self.restaurant_id.clone())
                .order_address(
                    OrderAddressBuilder::default()
                        .street("Street 1".to_string())
                        .postal_code("1000AB".to_string())
                        .city("Amsterdam".to_string())
                        .build()
                        .unwrap(),
                )
                .price(self.price.clone())
                .order_items(vec![
                    OrderItemDTOBuilder::default()
                        .product_id(ProductId::new(self.product_id.clone()))
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("50.00").unwrap())
                        .build()
                        .unwrap(),
                    OrderItemDTOBuilder::default()
                        .product_id(ProductId::new(self.product_id.clone()))
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("50.00").unwrap())
                        .build()
                        .unwrap(),
                ])
                .build()
                .unwrap(),
        );
        let customer = Customer::new(CustomerId::new(self.customer_id));
        let restaurant = Restaurant::new(
            vec![
                Product::new(
                    ProductId::<Uuid>::new(self.product_id),
                    Some("product-1".to_string()),
                    Some(Money::new(BigDecimal::from_str("50.00").unwrap())),
                ),
                Product::new(
                    ProductId::<Uuid>::new(self.product_id),
                    Some("product-2".to_string()),
                    Some(Money::new(BigDecimal::from_str("50.00").unwrap())),
                ),
            ],
            true,
        );
        let order = self
            .order_data_mapper
            .create_order_command_to_order(self.create_order_command().as_ref().unwrap());

        let customer = self
            .customer_repository
            .find_customer(&self.customer_id)
            .await;
    }
}
