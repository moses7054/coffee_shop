use std::marker::PhantomData;

use super::enums::{Extra, Milk, OrderDetails};
use super::builder::{OrderBuilder, ReadyToBuild};


#[derive(Debug)]
pub struct Pending;

pub struct Customized;

pub struct Paid;

pub struct Preparing;

pub struct Ready;

pub struct Served;

#[derive(Debug)]
pub struct Order<S> {
    pub details: OrderDetails,
    _state: PhantomData<S>
}

fn transition<From, To>(order: Order<From>) -> Order<To> {
    Order { details: order.details, _state: PhantomData }
}

impl Order<Pending> {
    pub fn new(builder: OrderBuilder<ReadyToBuild>) -> Self {
        Order {details: builder.build(), _state: PhantomData }
    }

    pub fn customize(mut self, milk: Milk, extras: Vec<Extra>) -> Order<Customized> {
        self.details.milk = milk;
        self.details.extras = extras;

        transition(self)      
    }
}

impl Order<Customized> {
    pub fn pay(self) -> Order<Paid> {
        println!("ordered something {}", self.details.total_price_cents());
        transition(self)
    }
}

impl Order<Paid> {
    pub fn start_preparing(self) -> Order<Preparing> {
        println!("  [bar] Starting {:?} for {}", self.details.coffee, self.details.customer_name);
        transition(self)
    }
}


impl Order<Preparing> {
    pub fn finish(self) -> Order<Ready> {
        println!("  [bar] Ready: {:?} {:?}", self.details.size, self.details.coffee);
        transition(self)
    }
}

impl Order<Ready> {
    pub fn serve(self) -> Order<Served> {
        println!("  [counter] Served to {}!", self.details.customer_name);
        transition(self)
    }
}


impl Order<Served> {
    pub fn receipt(&self) -> String {
        format!(
            "Receipt — {} paid {}¢ for {:?} {:?}",
            self.details.customer_name,
            self.details.total_price_cents(),
            self.details.size,
            self.details.coffee,
        )
    }
}