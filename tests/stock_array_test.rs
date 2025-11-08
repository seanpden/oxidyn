use oxidyn::{Flow, Model, StockArray};

#[test]
fn test_stock_array_creation() {
    let arr = StockArray::new("test", "Test Array", 5, 1.0, "units");

    assert_eq!(arr.base_id, "test");
    assert_eq!(arr.size, 5);
    assert_eq!(arr.initial_values.len(), 5);
    assert!(arr.initial_values.iter().all(|&v| v == 1.0));
}

#[test]
fn test_stock_array_with_values() {
    let values = vec![1.0, 2.0, 3.0];
    let arr = StockArray::from_values("test", "Test", values.clone(), "units");

    assert_eq!(arr.size, 3);
    assert_eq!(arr.initial_values, values);
}

#[test]
fn test_stock_array_expand() {
    let arr = StockArray::new("mem", "Memory", 3, 0.5, "items");
    let stocks = arr.expand();

    assert_eq!(stocks.len(), 3);
    assert_eq!(stocks[0].id, "mem[0]");
    assert_eq!(stocks[1].id, "mem[1]");
    assert_eq!(stocks[2].id, "mem[2]");
    assert!(stocks.iter().all(|s| s.initial_value == 0.5));
}

#[test]
fn test_stock_array_with_constraints() {
    let arr = StockArray::new("test", "Test", 2, 5.0, "units")
        .with_min(0.0)
        .with_max(10.0);

    let stocks = arr.expand();
    assert_eq!(stocks[0].min_value, Some(0.0));
    assert_eq!(stocks[0].max_value, Some(10.0));
    assert_eq!(stocks[1].min_value, Some(0.0));
    assert_eq!(stocks[1].max_value, Some(10.0));
}

#[test]
fn test_stock_array_in_model() {
    let arr_size = 3;
    let mut model = Model::new("array_test");

    let arr = StockArray::new("memory", "Memory", arr_size, 1., "strength");
    model.add_stock_array(arr.clone());

    // add flows to each array element
    for i in 0..arr_size {
        model.add_flow(
            Flow::constant(&format!("decay_{}", i), "Decay", 0.1, "strength/sec")
                .from_stock(&arr.stock_id(i)),
        );
    }

    let result = model.simulate(5.0);

    // check that all stocks exist and have decayed
    for i in 0..3 {
        let stock_id = arr.stock_id(i);
        assert!(result.stock_values.contains_key(&stock_id));

        let values = result.stock_values.get(&stock_id).unwrap();
        let initial = values.first().unwrap();
        let final_val = values.last().unwrap();

        assert!(*initial > *final_val, "Stock {} should have decayed", i);
    }
}
