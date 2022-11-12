mod greet;

use greet::{english,french};
fn main() {
    english::greet();
    french::greet();
    println!("Hello, world!");
}
