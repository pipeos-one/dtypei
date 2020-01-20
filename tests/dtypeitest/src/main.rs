use mathi;
use lib2;
use complex;

fn main() {
    eprintln!("typedinterface mathi = {:?}", mathi::typedinterface());
    println!("sum 2 + 5 = {}", mathi::sum(2, 5));
    println!("sub 8 - 2 = {}", mathi::sub(8, 2));

    eprintln!("typedinterface lib2 = {:?}", lib2::typedinterface());
    println!("sum 2 + 5 = {}", lib2::sum(2, 5));
    println!("sub 8 - 2 = {}", lib2::sub(8, 2));

    eprintln!("typedinterface complex = {:?}", complex::typedinterface());

    let mut table = complex::new_table();
    table = complex::sand(table);
    table = complex::paint(table, complex::Color::Black);
    table = complex::varnish(table);
    table = complex::polish(table);

    eprintln!("table = {:?}", table);

}
