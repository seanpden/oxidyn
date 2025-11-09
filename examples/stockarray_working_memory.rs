use oxidyn::{Flow, Model, StockArray};

fn main() {
    let mut model = Model::new("serial_position");
    let arr_size = 7;

    // strength of 7 positions in working memory
    // all items start at same value
    let memory_strength_arr =
        StockArray::new("strength", "Memory Strength", arr_size, 0.5, "strength")
            .with_min(0.0)
            .with_max(1.0);

    model.add_stock_array(memory_strength_arr.clone());

    // decay flows for each stock, decay at same rate
    for i in 0..arr_size {
        model.add_flow(
            Flow::linear(
                &format!("decay_{}", i),
                &format!("Decay [{}]", i),
                0.1,
                0.0,
                &memory_strength_arr.stock_id(i),
                "strength/sec",
            )
            .from_stock(&memory_strength_arr.stock_id(i)),
        );
    }

    // rehearsal flows
    for i in 0..arr_size {
        // rehearsal benefit decreases from position 0 to position 6,
        // implying that 0 is the first item rehearsed and 6 is the last item
        let rate = 0.1 - (i as f64) * 0.01;

        model.add_flow(
            Flow::constant(
                &format!("rehearsal_{}", i),
                &format!("Rehearsal [{}]", i),
                rate,
                "strength/sec",
            )
            .to_stock(&memory_strength_arr.stock_id(i)),
        );
    }

    // recency flows
    for i in 0..arr_size {
        // more recent (idx 5,6,etc) will be better remembered
        let distance_from_end = (arr_size - 1 - i) as f64;
        let rate = (0.05 - distance_from_end * 0.02).max(0.);
        model.add_flow(
            Flow::constant(
                &format!("recency_{}", i),
                &format!("Recency [{}]", i),
                rate,
                "strength/sec",
            )
            .to_stock(&memory_strength_arr.stock_id(i)),
        );
    }

    model.set_time_step(0.1);
    let result = model.simulate(10.0);

    result.print_summary();
    result.print_detailed(
        &(0..arr_size) // like list comprehension
            .map(|i| memory_strength_arr.stock_id(i))
            .collect::<Vec<_>>() // collect into a Vec<String>
            .iter() // create an iterator
            .map(|s| s.as_str()) // convert &String to &str
            .collect::<Vec<_>>(), // collect again into Vec<&str>
    );

    // Given this model, the result shows a "U" shaped result for the stock
    // values. This implies that more recently studied items and first items
    // in memory are better rememebered. The middling items aren't retained
    // as well.
}
