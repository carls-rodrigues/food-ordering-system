#[derive(Debug, Clone, PartialEq)]
pub struct StreetAddress {
    id: uuid::Uuid,
    street: String,
    postal_code: String,
    city: String,
}

impl StreetAddress {
    pub fn new(id: uuid::Uuid, street: String, postal_code: String, city: String) -> Self {
        Self {
            id,
            street,
            postal_code,
            city,
        }
    }

    pub fn get_id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn get_street(&self) -> &String {
        &self.street
    }

    pub fn get_postal_code(&self) -> &String {
        &self.postal_code
    }

    pub fn get_city(&self) -> &String {
        &self.city
    }
}

#[cfg(test)]
mod street_address_tests {
    use super::*;

    #[test]
    fn test_street_address() {
        let id = uuid::Uuid::now_v7();
        let street = "street".to_string();
        let postal_code = "postal_code".to_string();
        let city = "city".to_string();
        let street_address =
            StreetAddress::new(id, street.clone(), postal_code.clone(), city.clone());
        assert_eq!(street_address.get_id(), &id);
        assert_eq!(street_address.get_street(), &street);
        assert_eq!(street_address.get_postal_code(), &postal_code);
        assert_eq!(street_address.get_city(), &city);
    }
}
