use std::marker::PhantomData;

use crate::enums::SyrupFlavor;

use super::enums::{CoffeeType, Extra, Milk, OrderDetails, Size};

pub struct NeedsCoffee;

pub struct NeedsSize;

pub struct ReadyToBuild;

pub struct OrderBuilder<S> {
    coffee: Option<CoffeeType>,
    size: Option<Size>,
    milk: Milk,
    extras: Vec<Extra>,
    customer_name: String,
    _state: PhantomData<S>,
}

impl OrderBuilder<NeedsCoffee> {
    pub fn new(customer_name: impl Into<String>) -> Self {
        OrderBuilder {
            coffee: None,
            size: None,
            milk: Milk::None,
            extras: vec![],
            customer_name: customer_name.into(),
            _state: PhantomData,
        }
    }

    pub fn coffee(self, coffee: CoffeeType) -> OrderBuilder<NeedsSize> {
        OrderBuilder {
            coffee: Some(coffee),
            size: None,
            milk: self.milk,
            extras: self.extras,
            customer_name: self.customer_name,
            _state: PhantomData,
        }
    }
}

impl OrderBuilder<NeedsSize> {
    pub fn size(self, size: Size) -> OrderBuilder<ReadyToBuild> {
        OrderBuilder {
            coffee: self.coffee,
            size: Some(size),
            milk: self.milk,
            extras: self.extras,
            customer_name: self.customer_name,
            _state: PhantomData,
        }
    }
}

impl OrderBuilder<ReadyToBuild> {
    pub fn milk(mut self, milk: Milk) -> Self {
        self.milk = milk;
        self
    }

    pub fn extra(mut self, extra: Extra) -> Self {
        self.extras.push(extra);
        self
    }

    pub fn build(self) -> OrderDetails {
        OrderDetails {
            coffee: self.coffee.unwrap(),
            size: self.size.unwrap(),
            milk: self.milk,
            extras: self.extras,
            customer_name: self.customer_name,
        }
    }
}

pub fn demo_builder() {
    let sample = OrderBuilder::new("ron")
        .coffee(CoffeeType::Americano)
        .size(Size::Large)
        .extra(Extra::Decaf)
        .extra(Extra::Syrup(SyrupFlavor::Caramel))
        .build();
}
