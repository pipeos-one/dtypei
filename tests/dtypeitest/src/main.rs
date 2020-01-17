#[macro_use]
extern crate dtypei_derive;

use dtypei;
use mathi;

fn main() {
    eprintln!("typedinterface0 = {:?}", mathi::typedinterface());

    println!("sum 2 + 5 = {}", mathi::sum(2, 5));

    eprintln!("typedinterface1 = {:?}", mathi::typedinterface());
}
