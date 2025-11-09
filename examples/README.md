# Oxidyn Examples

This directory contains example models demonstrating how oxidyn can be used to model cognitive systems.

## Running Examples

```bash
cargo run --example <example-name>
```

## Available Examples

### Working Memory (`simple_population.rs`)

A simple population model, mimicking the STELLA tutorial up to step 8 (seen [here](https://iseesystems.com/resources/help/v4/default.htm#03-BuildingModels/1.ModelBuildingTutorial.htm?Highlight=population)).

### Working Memory (`working_memory.rs`)

A simple model of working memory as a dynamic system, demonstrating:

- **Capacity limits**: 7 items constraint
- **Memory decay**: Items decay over time
- **Encoding**: New information enters working memory

This example shows that this tool can enable cognitive scientists to model working memory as a dynamic system.
The model successfully constructs working memory using stocks, flows, and constraints:

- **Stocks** represent cognitive states (e.g., items in memory, activation levels)
- **Constraints** represent cognitive limits (e.g., capacity)
- **Flows** represent cognitive processes (e.g., encoding, forgetting, attention)

### Working Memory (`stockarray_working_memory.rs`)

A model of working memory with `StockArray`s as a dynamic system.
More complex than `working_memory.rs`, and inspired by the limitations of the library at the time of writing that example.
This demonstrates everything shown there, as well as:

- **Primacy**: First items are better recalled
- **Recency**: Last items are better recalled

This model demonstrates the use of `StockArray`s as a useful feature for modeling logically grouped cognitive items:

- **StockArrays** represent grouped items (e.g. discrete in memory)
