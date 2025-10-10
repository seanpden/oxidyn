use oxidyn::Stock;
use std::process;

#[test]
fn create_stock() {
    let stock = Stock::build("ID", "NAME", 1.0, 1.0, "UNITS").unwrap_or_else(|err| {
        eprintln!("Problem creating stock `ID`: {err}");
        process::exit(1);
    });
    dbg!(stock);
}
