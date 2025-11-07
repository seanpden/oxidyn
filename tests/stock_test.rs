use oxidyn::Stock;

#[test]
fn test_stock_creation() {
    let id = "ID";
    let name = "NAME";
    let initial_value = 1.0;
    let current_value = 1.0;
    let units = "UNITS";

    let stock = Stock::new(id, name, initial_value, units);

    assert_eq!(stock.id, id);
    assert_eq!(stock.name, name);
    assert_eq!(stock.initial_value, initial_value);
    assert_eq!(stock.current_value, current_value);
    assert_eq!(stock.units, units);
}
