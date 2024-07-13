use std::ops::{Add, Div, Mul, Sub};

use bigdecimal::{BigDecimal, Signed};

#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    amount: BigDecimal,
}

impl Money {
    pub fn new(amount: BigDecimal) -> Self {
        Self {
            amount: amount.with_scale(2),
        }
    }

    pub fn is_greater_than_zero(&self) -> bool {
        self.amount.is_positive()
    }
    pub fn is_greater_than(&self, money: Money) -> bool {
        self.amount > money.amount && self.amount.is_positive()
    }
    pub fn add(&self, money: Money) -> Money {
        let scale = self
            .clone()
            .set_scale(self.amount.clone().add(money.get_amount()));
        Money::new(scale)
    }
    pub fn subtract(&self, money: Money) -> Money {
        let scale = self
            .clone()
            .set_scale(self.amount.clone().sub(money.get_amount()));
        Money::new(scale)
    }
    pub fn multiply(&self, money: Money) -> Money {
        let scale = self
            .clone()
            .set_scale(self.amount.clone().mul(money.get_amount()));
        Money::new(scale)
    }
    pub fn divide(&self, money: Money) -> Money {
        let scale = self
            .clone()
            .set_scale(self.amount.clone().div(money.get_amount()));
        Money::new(scale)
    }
    pub fn set_scale(&mut self, input: BigDecimal) -> BigDecimal {
        input.with_scale_round(2, bigdecimal::RoundingMode::HalfEven)
    }
    pub fn get_amount(&self) -> &BigDecimal {
        &self.amount
    }
}

#[cfg(test)]
mod money_tests {
    use std::str::FromStr;

    use super::*;
    use bigdecimal::FromPrimitive;

    #[test]
    fn test_money() {
        let amount = BigDecimal::from_f64(100f64).unwrap();
        let money = Money::new(amount.clone());
        assert_eq!(money.get_amount(), &amount);
    }
    #[test]
    fn test_is_greater_than_zero() {
        let amount = BigDecimal::from_f64(0.1).unwrap();
        let money = Money::new(amount.clone());
        assert!(money.is_greater_than_zero());
    }
    #[test]
    fn test_is_greater_than() {
        let amount = BigDecimal::from_f64(0.1).unwrap();
        let money = Money::new(amount.clone());
        let amount2 = BigDecimal::from_f64(0.2).unwrap();
        let money2 = Money::new(amount2.clone());
        assert!(money2.is_greater_than(money));
    }
    #[test]
    fn test_add() {
        let money = Money::new(BigDecimal::from_str("0.1").unwrap());
        let money2 = Money::new(BigDecimal::from_str("0.2").unwrap());
        let result = money.add(money2);
        assert_eq!(result.get_amount(), &BigDecimal::from_str("0.3").unwrap());
    }
    #[test]
    fn test_set_scale() {
        let mut money = Money::new(BigDecimal::from_str("0.1").unwrap());
        let with_scale = money.set_scale(BigDecimal::from_str("0.123456").unwrap());
        assert_eq!(with_scale, BigDecimal::from_str("0.12").unwrap());
    }
    #[test]
    fn test_subtract() {
        let money = Money::new(BigDecimal::from_str("0.2").unwrap());
        let money2 = Money::new(BigDecimal::from_str("0.1").unwrap());
        let result = money.subtract(money2);
        assert_eq!(result.get_amount(), &BigDecimal::from_str("0.1").unwrap());
    }
    #[test]
    fn test_multiply() {
        let money = Money::new(BigDecimal::from_str("0.2").unwrap());
        let money2 = Money::new(BigDecimal::from_str("0.2").unwrap());
        let result = money.multiply(money2);
        assert_eq!(result.get_amount(), &BigDecimal::from_str("0.04").unwrap());
    }
    #[test]
    fn test_divide() {
        let money = Money::new(BigDecimal::from_str("0.2").unwrap());
        let money2 = Money::new(BigDecimal::from_str("0.1").unwrap());
        let result = money.divide(money2);
        assert_eq!(result.get_amount(), &BigDecimal::from_str("2").unwrap());
    }
}
