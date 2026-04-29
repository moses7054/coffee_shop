pub mod builder;
pub mod enums;
pub mod typestate;
pub mod capabilities;

use capabilities::invoke;




fn main() {
    println!("Hello, world!");
    invoke();
}
