use std::collections::HashMap;

use oxidyn::{Flow, Model, Stock};

#[test]
fn test_simple_constant() {
    let mut model = Model::new("my_model");

    model
        .add_stock(Stock::new("amount", "Amount", 0., "units"))
        .add_flow(Flow::constant("input", "Input", 2., "units").to_stock("amount"))
        .add_flow(Flow::constant("output", "Output", 1., "units").from_stock("amount"))
        .set_time_step(1.);

    let res = model.simulate(5.0);

    let expected_res_vec = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(res.time_series, expected_res_vec);

    let mut expected_stock_values = HashMap::new();
    expected_stock_values.insert("amount".to_string(), expected_res_vec);
    assert_eq!(res.stock_values, expected_stock_values);
}

#[test]
fn test_stella_population_example() {
    let mut model = Model::new("stella_model");

    model
        .add_stock(Stock::new("population", "Population", 25., "people"))
        .add_flow(
            Flow::linear(
                "births",
                "Birth Rate",
                0.05,
                0.,
                "population",
                "people/time",
            )
            .to_stock("population"),
        )
        .add_flow(
            Flow::linear(
                "deaths",
                "Death Rate",
                1. / 60.,
                0.,
                "population",
                "people/time",
            )
            .from_stock("population"),
        )
        .set_time_step(1.);

    let mut res = model.simulate(100.0);

    res.print_summary();
    res.print_detailed(&["population"]);
    // res.print_detailed();

    let expected_final_value = 664;
    let actual_final_value = res
        .stock_values
        .get_mut("population")
        .unwrap()
        .pop()
        .unwrap();

    // Might be some floating point issues for now, ignore and check for approx
    assert!(((expected_final_value as f64) - actual_final_value) < 1.);
}
