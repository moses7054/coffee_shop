

use super::builder::{OrderBuilder, ReadyToBuild};
use super::typestate::{Order, Customized, Paid, Pending, Preparing, Ready};


pub trait CanOrder {
    fn place_order(&self, builder: OrderBuilder<ReadyToBuild>) -> Order<Pending>;
}

pub trait CanCharge{
    fn charge(&self, order: Order<Customized>) -> Order<Paid>;
}

pub trait CanPrepare {
    fn start(&self, order: Order<Paid>) -> Order<Preparing>;
    fn finish(&self, order: Order<Preparing>) -> Order<Ready>;
}

pub struct Customer {
    pub name: String,
}

pub struct Cashier {
    pub name: String
}

pub struct Barista {
    pub name: String
}

pub struct Manager {
    pub name: String
}

impl CanOrder for Customer {
    fn place_order(&self, builder: OrderBuilder<ReadyToBuild>) -> Order<Pending> {
        println!("  [customer] {} placed an order", self.name);
        Order::new(builder)
    }
}


impl CanOrder for Manager {
    fn place_order(&self, builder: OrderBuilder<ReadyToBuild>) -> Order<Pending> {
        println!("  manager {} placed an order", self.name);
        Order::new(builder)
    }
}

impl CanCharge for Cashier {
    fn charge(&self, order: Order<Customized>) -> Order<Paid> {
        println!(" Cashier {} processed payment", self.name);
        order.pay()
    }
}


impl CanCharge for Manager {
    fn charge(&self, order: Order<Customized>) -> Order<Paid> {
        println!(" Manager {} processed payment", self.name);
        order.pay()
    }
}


impl CanPrepare for Barista {
    fn start(&self, order: Order<Paid>) -> Order<Preparing> {
        println!("  [barista] {} started preparing", self.name);
        order.start_preparing()
    }

    fn finish(&self, order: Order<Preparing>) -> Order<Ready> {
        println!("  [barista] {} finished the drink", self.name);
        order.finish()
    }
}

impl CanPrepare for Manager {
    fn start(&self, order: Order<Paid>) -> Order<Preparing> {
        println!("  [manager] {} jumped behind the bar", self.name);
        order.start_preparing()
    }

    fn finish(&self, order: Order<Preparing>) -> Order<Ready> {
        println!("  [manager] {} finished the drink", self.name);
        order.finish()
    }
}

pub fn run_bar_workflow<P: CanPrepare>(preparer: &P, order: Order<Paid>) -> Order<Ready> {
    let preparing = preparer.start(order);
    preparer.finish(preparing)
}

pub fn invoke() {
    use super::enums::CoffeeType;

    let customer = Customer {name: "glacier".to_string()};
    let cashier = Cashier {name: "cashier".to_string()};
    let barista = Barista {name: "barista".to_string()};

    let builder = OrderBuilder::new("glacier")
    .coffee(CoffeeType::Americano)
    .size(crate::enums::Size::Large)
    .milk(crate::enums::Milk::Almond);

    let pending = customer.place_order(builder);
    dbg!(&pending);
    let customized = pending.customize(crate::enums::Milk::Oat, vec![]);
    let paid = cashier.charge(customized);
    let ready = run_bar_workflow(&barista, paid);
    let served = ready.serve();
    println!(" receipt {}", served.receipt());
}