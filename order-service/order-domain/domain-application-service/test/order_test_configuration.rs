mod order_application_service_test;

#[cfg(test)]
mod order_test_configuration {
    use std::str::FromStr;

    use async_trait::async_trait;
    use bigdecimal::BigDecimal;
    use common::domain::events::publisher::DomainEventPublisher;
    use common::domain::events::DomainEvent;
    use common::domain::exception::DomainException;
    use order_domain_application_service::domain::dto::create::{CreateOrderCommand, CreateOrderCommandBuilder, OrderAddressBuilder, OrderItemDTOBuilder};
    use order_domain_application_service::domain::mapper::OrderDataMapper;
    use order_domain_application_service::domain::ports::input::service::OrderApplicationService;
    use order_domain_application_service::domain::ports::output::repository::message::publisher::payment::{OrderCancelledPaymentRequestMessagePublisher, OrderCreatedPaymentRequestMessagePublisher};
    use order_domain_application_service::domain::ports::output::repository::message::publisher::restaurant_approval::OrderPaidRestaurantRequestMessagePublisher;
    use order_domain_application_service::domain::ports::output::repository::{CustomerRepository, OrderRepository, RestaurantRepository};
    use order_domain_core::domain::event::{OrderCancelledEvent, OrderCreatedEvent, OrderPaidEvent};
    use order_domain_core::domain::exception::OrderDomainException;
    use order_domain_core::domain::order_domain_service::OrderDomainService;
    use order_domain_core::domain::order_domain_service_impl::OrderDomainServiceImpl;
    use common::domain::value_object::{BaseId, CustomerId, Money, OrderId, OrderStatus, ProductId, RestaurantId};
    use order_domain_core::domain::entity::{Customer, Order, Product, Restaurant};
    use uuid::Uuid;

    use mockall::{mock, predicate::*};

    // const PRICE: BigDecimal = BigDecimal::from_str("100.00").unwrap();

    mock! {
        pub OrderRepository {}

        #[async_trait]
        impl OrderRepository for OrderRepository {
            async fn save(&self, order: order_domain_core::domain::entity::Order) -> Result<order_domain_core::domain::entity::Order, DomainException<OrderDomainException>>;
            async fn find_by_tracking_id(
                &self,
                tracking_id: order_domain_core::domain::value_object::TrackingId<uuid::Uuid> ,
            ) -> Result<Option<order_domain_core::domain::entity::Order>, DomainException<order_domain_core::domain::exception::OrderNotFoundException>>;
        }

    }
    mock! {
      pub CustomerRepository {}

      #[async_trait]
        impl CustomerRepository for CustomerRepository {
            async fn find_customer(
                &self,
                customer_id: &uuid::Uuid,
            ) -> Result<Option<Customer>, DomainException<OrderDomainException>>;
        }
    }
    mock! {
      pub RestaurantRepository {}

      #[async_trait]
        impl RestaurantRepository for RestaurantRepository {
            async fn find_restaurant_information(
                &self,
                restaurant: order_domain_core::domain::entity::Restaurant,
            ) -> Result<Option<order_domain_core::domain::entity::Restaurant>, DomainException<OrderDomainException>>;
        }
    }
    mock! {
        pub OrderCreatedPaymentRequestMessagePublisher {}

        #[async_trait]
        impl OrderCreatedPaymentRequestMessagePublisher for OrderCreatedPaymentRequestMessagePublisher {}

        #[async_trait]
        impl DomainEventPublisher<OrderCreatedEvent> for OrderCreatedPaymentRequestMessagePublisher{
            async fn publish(&self, event: &OrderCreatedEvent);
        }
        impl DomainEvent<OrderCreatedEvent> for OrderCreatedPaymentRequestMessagePublisher {}
    }
    mock! {
        pub OrderCancelledPaymentRequestMessagePublisher {}

        #[async_trait]
        impl OrderCancelledPaymentRequestMessagePublisher for OrderCancelledPaymentRequestMessagePublisher {}

        #[async_trait]
        impl DomainEventPublisher<OrderCancelledEvent> for OrderCancelledPaymentRequestMessagePublisher{
            async fn publish(&self, event: &OrderCancelledEvent);
        }
        impl DomainEvent<OrderCancelledEvent> for OrderCancelledPaymentRequestMessagePublisher {}
    }
    mock! {
        pub OrderPaidRestaurantRequestMessagePublisher {}

        #[async_trait]
        impl OrderPaidRestaurantRequestMessagePublisher for OrderPaidRestaurantRequestMessagePublisher {}

        #[async_trait]
        impl DomainEventPublisher<OrderPaidEvent> for OrderPaidRestaurantRequestMessagePublisher{
            async fn publish(&self, event: &OrderPaidEvent);
        }
        impl DomainEvent<OrderPaidEvent> for OrderPaidRestaurantRequestMessagePublisher {}
    }
    mock! {
        pub OrderApplicationService {}

        #[async_trait(?Send)]
        impl OrderApplicationService for OrderApplicationService {
            async fn create_order(
                &self,
                create_order_command: CreateOrderCommand,
            ) -> Result<order_domain_application_service::domain::dto::create::CreateOrderResponse, DomainException<OrderDomainException>>;
            async fn track_order(
                &self,
                track_order_query: order_domain_application_service::domain::dto::track::TrackOrderQuery,
            ) -> Result<order_domain_application_service::domain::dto::track::TrackOrderResponse, DomainException<order_domain_core::domain::exception::OrderNotFoundException>>;
        }
    }
    fn constants() -> (
        CustomerId<Uuid>,
        RestaurantId<Uuid>,
        OrderId<Uuid>,
        ProductId<Uuid>,
    ) {
        (
            CustomerId::new(Uuid::now_v7()),
            RestaurantId::new(Uuid::now_v7()),
            OrderId::new(Uuid::now_v7()),
            ProductId::new(Uuid::now_v7()),
        )
    }
    struct Output {
        mock_order_application_service: MockOrderApplicationService,
        mock_order_repository: MockOrderRepository,
        mock_customer_repository: MockCustomerRepository,
        mock_restaurant_repository: MockRestaurantRepository,
        mock_order_created_payment_request_message_publisher:
            MockOrderCreatedPaymentRequestMessagePublisher,
        mock_order_cancelled_payment_request_message_publisher:
            MockOrderCancelledPaymentRequestMessagePublisher,
        mock_order_paid_restaurant_request_message_publisher:
            MockOrderPaidRestaurantRequestMessagePublisher,
        order_data_mapper: OrderDataMapper,
        create_order_command: Option<CreateOrderCommand>,
        create_order_command_wrong_price: Option<CreateOrderCommand>,
        create_order_command_wrong_product_price: Option<CreateOrderCommand>,
        order: Order,
    }
    fn before_each() -> Output {
        let (customer_id, restaurant_id, _order_id, product_id) = constants();
        let order_data_mapper = OrderDataMapper::new();
        let create_order_command = Some(
            CreateOrderCommandBuilder::default()
                .customer_id(customer_id.get_value().to_owned())
                .restaurant_id(restaurant_id.get_value().to_owned())
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
                        .product_id(product_id.clone())
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("50.00").unwrap())
                        .build()
                        .unwrap(),
                    OrderItemDTOBuilder::default()
                        .product_id(product_id.clone())
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("150.00").unwrap())
                        .build()
                        .unwrap(),
                ])
                .build()
                .unwrap(),
        );
        let create_order_command_wrong_price = Some(
            CreateOrderCommandBuilder::default()
                .customer_id(customer_id.get_value().to_owned())
                .restaurant_id(restaurant_id.get_value().to_owned())
                .order_address(
                    OrderAddressBuilder::default()
                        .street("Street 1".to_string())
                        .postal_code("1000AB".to_string())
                        .city("Amsterdam".to_string())
                        .build()
                        .unwrap(),
                )
                .price(BigDecimal::from_str("250.00").unwrap())
                .order_items(vec![
                    OrderItemDTOBuilder::default()
                        .product_id(product_id.clone())
                        .quantity(1)
                        .price(BigDecimal::from_str("60.00").unwrap())
                        .subtotal(BigDecimal::from_str("60.00").unwrap())
                        .build()
                        .unwrap(),
                    OrderItemDTOBuilder::default()
                        .product_id(product_id.clone())
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("150.00").unwrap())
                        .build()
                        .unwrap(),
                ])
                .build()
                .unwrap(),
        );
        let create_order_command_wrong_product_price = Some(
            CreateOrderCommandBuilder::default()
                .customer_id(customer_id.get_value().to_owned())
                .restaurant_id(restaurant_id.get_value().to_owned())
                .order_address(
                    OrderAddressBuilder::default()
                        .street("Street 1".to_string())
                        .postal_code("1000AB".to_string())
                        .city("Amsterdam".to_string())
                        .build()
                        .unwrap(),
                )
                .price(BigDecimal::from_str("200.00").unwrap())
                .order_items(vec![
                    OrderItemDTOBuilder::default()
                        .product_id(product_id.clone())
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("50.00").unwrap())
                        .build()
                        .unwrap(),
                    OrderItemDTOBuilder::default()
                        .product_id(product_id.clone())
                        .quantity(1)
                        .price(BigDecimal::from_str("50.00").unwrap())
                        .subtotal(BigDecimal::from_str("50.00").unwrap())
                        .build()
                        .unwrap(),
                ])
                .build()
                .unwrap(),
        );
        let customer = Customer::new(customer_id.clone());
        let restaurant_response = Restaurant::new(
            vec![
                Product::new(
                    product_id.clone(),
                    Some("product-1".to_string()),
                    Some(Money::new(BigDecimal::from_str("50.00").unwrap())),
                ),
                Product::new(
                    product_id.clone(),
                    Some("product-2".to_string()),
                    Some(Money::new(BigDecimal::from_str("50.00").unwrap())),
                ),
            ],
            true,
        );
        let order =
            order_data_mapper.create_order_command_to_order(create_order_command.as_ref().unwrap());
        let cloned = order.clone();

        let mut output = Output {
            mock_order_application_service: MockOrderApplicationService::new(),
            mock_order_repository: MockOrderRepository::new(),
            mock_customer_repository: MockCustomerRepository::new(),
            mock_restaurant_repository: MockRestaurantRepository::new(),
            mock_order_created_payment_request_message_publisher:
                MockOrderCreatedPaymentRequestMessagePublisher::new(),
            mock_order_cancelled_payment_request_message_publisher:
                MockOrderCancelledPaymentRequestMessagePublisher::new(),
            mock_order_paid_restaurant_request_message_publisher:
                MockOrderPaidRestaurantRequestMessagePublisher::new(),
            order_data_mapper,
            create_order_command,
            create_order_command_wrong_price,
            create_order_command_wrong_product_price,
            order,
        };
        output
            .mock_customer_repository
            .expect_find_customer()
            .times(0)
            .returning(move |_| Ok(Some(customer.clone())));

        output
            .mock_restaurant_repository
            .expect_find_restaurant_information()
            .times(0)
            .returning(move |_| Ok(Some(restaurant_response.clone())));

        output
            .mock_order_repository
            .expect_save()
            .times(0)
            .returning(move |_| Ok(cloned.clone()));

        output
    }
    #[tokio::test]
    async fn test_create_order() {
        let mut output = before_each();
        output
            .mock_order_application_service
            .expect_create_order()
            .times(1)
            .returning(move |_| {
                Ok(output.order_data_mapper.order_to_create_order_response(
                    &output.order.clone(),
                    Some("Order created successfully".to_string()),
                ))
            });
        let create_order_response = output
            .mock_order_application_service
            .create_order(output.create_order_command.unwrap())
            .await
            .unwrap();

        assert_eq!(create_order_response.order_status(), &OrderStatus::Pending);
        assert_eq!(
            create_order_response.message(),
            &Some("Order created successfully".to_string())
        );
        assert_ne!(create_order_response.order_tracking_id(), &None);
    }

    #[tokio::test]
    async fn test_create_order_with_wrong_total_price() {
        let mut output = before_each();

        output
            .mock_order_application_service
            .expect_create_order()
            .times(1)
            .return_once(move |_| {
                Err(DomainException::DomainError(OrderDomainException::new(
                    "any_error".to_string(),
                    None,
                )))
            });

        let create_order_response = output
            .mock_order_application_service
            .create_order(output.create_order_command_wrong_price.unwrap())
            .await;

        assert!(create_order_response.is_err());
        assert_eq!(
            create_order_response.unwrap_err(),
            DomainException::DomainError(OrderDomainException::new("any_error".to_string(), None))
        );
    }

    #[tokio::test]
    async fn test_create_order_with_wrong_product_price() {
        let mut output = before_each();

        output
            .mock_order_application_service
            .expect_create_order()
            .times(1)
            .return_once(move |_| {
                Err(DomainException::DomainError(OrderDomainException::new(
                    "any_error".to_string(),
                    None,
                )))
            });

        let create_order_response = output
            .mock_order_application_service
            .create_order(output.create_order_command_wrong_product_price.unwrap())
            .await;

        assert!(create_order_response.is_err());
        assert_eq!(
            create_order_response.unwrap_err(),
            DomainException::DomainError(OrderDomainException::new("any_error".to_string(), None))
        );
    }
}
