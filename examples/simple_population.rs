use oxidyn::{Flow, Model, Stock};

fn test_stella_population_example() {
    let mut model = Model::new("stella_model");

    model
        .add_stock(Stock::new("population", "Population", 25., "people").with_min(0.0))
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

    let res = model.simulate(100.0);

    res.print_summary();
    res.print_detailed(&["population"]);
}

fn main() {
    test_stella_population_example();
}
