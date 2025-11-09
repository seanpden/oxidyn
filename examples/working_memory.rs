use oxidyn::{Flow, Model, Stock};

fn main() {
    let mut model = Model::new("working_memory");

    // items held in working memory, 7 items (miller capacity)
    model.add_stock(
        Stock::new("items_in_memory", "Items in Working Memory", 0.0, "items")
            .with_min(0.0)
            .with_max(7.0),
    );

    // new items into working memory
    model.add_flow(
        Flow::constant("encoding", "Encoding Rate", 2.0, "items/sec").to_stock("items_in_memory"),
    );

    // forgetting items from working memory,
    // rate increases with more items (interference/decay)
    model.add_flow(
        Flow::linear(
            "forgetting",
            "Forgetting Rate",
            0.15, // slope: forgetting rate increases with load
            0.0,  // intercept: no forgetting when memory is empty
            "items_in_memory",
            "items/sec",
        )
        .from_stock("items_in_memory"),
    );

    model.set_time_step(0.1);
    let result = model.simulate(20.0);

    result.print_summary();
    result.print_detailed(&["items_in_memory"]);
}
