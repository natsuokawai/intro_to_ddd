use std::fmt;
use std::ops::Add;

fn main() {
    let jpy = Money::new(100, Currency::JPY);
    let usd = Money::new(1, Currency::USD);
    println!("{}", jpy);
    println!("{}", usd);
}

#[derive(PartialEq)]
enum Currency {
    JPY,
    USD,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Currency::JPY => write!(f, "JPY"),
            Currency::USD => write!(f, "USD"),
        }
    }
}

struct Money {
    amount: u32,
    currency: Currency,
}

impl Money {
    fn new(amount: u32, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            panic!("cannot add different currency.")
        }

        Self {
            amount: self.amount + other.amount,
            currency: self.currency,
        }
    }
}
