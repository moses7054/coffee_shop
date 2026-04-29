pub mod builder;
pub mod capabilities;
pub mod enums;
pub mod typestate;

use capabilities::invoke;

fn main() {
    println!("Hello, world!");
    invoke();
}
