use oxidyn::{Flow, FlowFunction, Stock, SystemState};

#[test]
fn test_constant_flow_creation() {
    let id = "ID";
    let name = "NAME";
    let rate = 1.0;
    let units = "UNITS";

    let flow = Flow::constant(id, name, rate, units);

    assert_eq!(flow.id, id);
    assert_eq!(flow.name, name);
    assert_eq!(flow.units, units);
    assert!(flow.from_stock.is_none());
    assert!(flow.to_stock.is_none());

    match flow.rate_function {
        FlowFunction::Constant(r) => assert_eq!(r, rate),
        _ => panic!("Expected constant flow function"),
    }
}

#[test]
fn test_linear_flow_creation() {
    let id = "ID";
    let name = "NAME";
    let slope = 0.5;
    let intercept = 10.;
    let input_stock = "INPUT_STOCK";
    let units = "UNITS";

    let flow = Flow::linear(id, name, slope, intercept, input_stock, units);

    assert_eq!(flow.id, id);
    assert_eq!(flow.name, name);
    assert_eq!(flow.units, units);
    assert!(flow.from_stock.is_none());
    assert!(flow.to_stock.is_none());

    match flow.rate_function {
        FlowFunction::Linear {
            slope: s,
            intercept: i,
            input_stock: is,
        } => {
            assert_eq!(s, slope);
            assert_eq!(i, intercept);
            assert_eq!(is, input_stock);
        }
        _ => panic!("Expected linear flow function"),
    }
}

#[test]
fn test_flow_from_stock() {
    let flow = Flow::constant("flow1", "Flow 1", 5.0, "units").from_stock("source_stock");

    assert_eq!(flow.from_stock, Some("source_stock".to_string()));
    assert!(flow.to_stock.is_none());
}

#[test]
fn test_flow_to_stock() {
    let flow = Flow::constant("flow1", "Flow 1", 5.0, "units").to_stock("dest_stock");

    assert!(flow.from_stock.is_none());
    assert_eq!(flow.to_stock, Some("dest_stock".to_string()));
}

#[test]
fn test_constant_flow_calculate_rate() {
    let flow = Flow::constant("const_flow", "Constant Flow", 10.0, "units/time");
    let state = SystemState::new();

    let rate = flow.calculate_rate(&state);
    assert_eq!(rate, 10.0);
}

#[test]
fn test_linear_flow_calculate_rate() {
    let mut state = SystemState::new();
    let stock = Stock::build("pop", "Population", 100.0, "people").unwrap();
    state.stocks.insert("pop".to_string(), stock);

    let flow = Flow::linear("growth", "Growth Rate", 0.02, 5.0, "pop", "people/year");
    let rate = flow.calculate_rate(&state);

    // Rate = 0.02 * 100.0 + 5.0 = 7.0
    assert_eq!(rate, 7.0);
}

#[test]
fn test_linear_flow_with_missing_stock() {
    let state = SystemState::new(); // No stocks
    let flow = Flow::linear("growth", "Growth Rate", 0.02, 5.0, "missing_stock", "units");

    let rate = flow.calculate_rate(&state);
    // Should use 0.0 for missing stock: 0.02 * 0.0 + 5.0 = 5.0
    assert_eq!(rate, 5.0);
}

#[test]
fn test_flow_builder_chain() {
    let flow = Flow::linear("transfer", "Transfer", 1.0, 0.0, "source", "units/time")
        .from_stock("tank_a")
        .to_stock("tank_b");

    assert_eq!(flow.id, "transfer");
    assert_eq!(flow.from_stock, Some("tank_a".to_string()));
    assert_eq!(flow.to_stock, Some("tank_b".to_string()));
}
