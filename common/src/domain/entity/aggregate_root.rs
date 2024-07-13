use super::BaseEntity;

pub trait AggregateRoot<ID: PartialEq>: BaseEntity<ID> {}

#[cfg(test)]
mod aggregate_root_tests {
    use crate::domain::entity::{AggregateRoot, BaseEntity};

    #[test]
    fn test_aggregate_root() {
        struct ConcreteAggregateRoot<ID> {
            id: ID,
        }
        impl ConcreteAggregateRoot<i32> {
            fn new(id: i32) -> Self {
                Self { id }
            }
        }
        impl<ID: PartialEq> BaseEntity<ID> for ConcreteAggregateRoot<ID> {
            fn get_id(&self) -> &ID {
                &self.id
            }

            fn set_id(&mut self, id: ID) {
                self.id = id;
            }
        }

        impl<ID: PartialEq> AggregateRoot<ID> for ConcreteAggregateRoot<ID> {}

        let id = 1;
        let aggregate_root = ConcreteAggregateRoot::new(id);
        assert_eq!(aggregate_root.get_id(), &id);
    }
}
