use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Stock {
    pub id: String,
    pub name: String,
    pub initial_value: f64,
    pub current_value: f64,
    pub units: String,
    /// Minimum constraint, optional
    pub min_value: Option<f64>,
    /// Maximum constraint, optional
    pub max_value: Option<f64>,
}

impl Stock {
    pub fn new(id: &str, name: &str, initial_value: f64, units: &str) -> Self {
        Stock {
            id: id.to_string(),
            name: name.to_string(),
            initial_value,
            current_value: initial_value,
            units: units.to_string(),
            min_value: None,
            max_value: None,
        }
    }

    pub fn with_min(mut self, min: f64) -> Self {
        self.min_value = Some(min);
        self
    }
    pub fn with_max(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }
}

/// represents multiple related stocks.
///
/// useful for modeling discrete entities (like items in working memory)
/// where each position has its own state but follows similar dynamics.
#[derive(Debug, Clone)]
pub struct StockArray {
    pub base_id: String,
    pub name: String,
    pub size: usize,
    pub initial_values: Vec<f64>,
    pub units: String,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

impl StockArray {
    /// new stock array with uniform values
    pub fn new(base_id: &str, name: &str, size: usize, initial_value: f64, units: &str) -> Self {
        StockArray {
            base_id: base_id.to_string(),
            name: name.to_string(),
            size,
            initial_values: vec![initial_value; size],
            units: units.to_string(),
            min_value: None,
            max_value: None,
        }
    }

    /// new stock array with different values
    pub fn from_values(base_id: &str, name: &str, values: Vec<f64>, units: &str) -> Self {
        let size = values.len();
        StockArray {
            base_id: base_id.to_string(),
            name: name.to_string(),
            size,
            initial_values: values,
            units: units.to_string(),
            min_value: None,
            max_value: None,
        }
    }

    pub fn with_min(mut self, min: f64) -> Self {
        self.min_value = Some(min);
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }

    /// Converts the stock array into individual Stock instances.
    /// Each stock has an ID of the form "base_id[index]"
    pub fn expand(&self) -> Vec<Stock> {
        (0..self.size)
            .map(|i| Stock {
                id: format!("{}[{}]", self.base_id, i),
                name: format!("{} [{}]", self.name, i),
                initial_value: self.initial_values[i],
                current_value: self.initial_values[i],
                units: self.units.clone(),
                min_value: self.min_value,
                max_value: self.max_value,
            })
            .collect()
    }

    /// generates an indexed stock id for reference
    pub fn stock_id(&self, index: usize) -> String {
        format!("{}[{}]", self.base_id, index)
    }
}

/// A flow is the rate of change between stocks.
///
/// Flows can transfer from one stock to another, or they can be
/// sources (no from_stock) or sinks (no to_stock).
#[derive(Debug, Clone)]
pub struct Flow {
    pub id: String,
    pub name: String,
    /// Flow ID of the stock this flow takes from (None for sources)
    pub from_stock: Option<String>,
    /// Flow ID of the stock this flow adds to (None for sinks)
    pub to_stock: Option<String>,
    pub rate_function: FlowFunction,
    /// The units of this flow rate (e.g., "people/year", "dollars/month")
    pub units: String,
}

#[derive(Debug, Clone)]
pub enum FlowFunction {
    Constant(f64),

    /// A linear flow rate based on the value of an input stock.
    ///
    /// The flow rate calculation: `slope * input_stock_value + intercept`
    Linear {
        slope: f64,
        intercept: f64,
        /// The ID of the stock whose value is used as input
        input_stock: String,
    },
}

impl Flow {
    /// Creates a new flow with a constant rate function.
    pub fn constant(id: &str, name: &str, rate: f64, units: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            from_stock: None,
            to_stock: None,
            rate_function: FlowFunction::Constant(rate),
            units: units.to_string(),
        }
    }

    /// Creates a new flow with a linear rate function.
    ///
    /// The flow rate is calculated as: `slope * input_stock_value + intercept`
    pub fn linear(
        id: &str,
        name: &str,
        slope: f64,
        intercept: f64,
        input_stock: &str,
        units: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            from_stock: None,
            to_stock: None,
            rate_function: FlowFunction::Linear {
                slope,
                intercept,
                input_stock: input_stock.to_string(),
            },
            units: units.to_string(),
        }
    }

    pub fn from_stock(mut self, stock_id: &str) -> Self {
        self.from_stock = Some(stock_id.to_string());
        self
    }

    pub fn to_stock(mut self, stock_id: &str) -> Self {
        self.to_stock = Some(stock_id.to_string());
        self
    }

    /// Calculates the current flow rate given a system state.
    pub fn calculate_rate(&self, state: &SystemState) -> f64 {
        match &self.rate_function {
            FlowFunction::Constant(rate) => *rate,
            FlowFunction::Linear {
                slope,
                intercept,
                input_stock,
            } => {
                let input_value = state.get_stock_value(input_stock).unwrap_or(0.0);
                slope * input_value + intercept
            }
        }
    }
}

/// Represents the current state of the model.
#[derive(Debug, Clone, Default)]
pub struct SystemState {
    /// Stocks indexed by IDs
    pub stocks: HashMap<String, Stock>,
    /// Current sim time
    pub time: f64,
}

impl SystemState {
    pub fn new() -> Self {
        Self {
            stocks: HashMap::new(),
            time: 0.0,
        }
    }

    pub fn get_stock_names(&self) -> Vec<&str> {
        self.stocks.keys().map(|k| k.as_str()).collect()
    }

    /// Looks up the current value of a stock by ID.
    pub fn get_stock_value(&self, stock_id: &str) -> Option<f64> {
        self.stocks.get(stock_id).map(|stock| stock.current_value)
    }

    // Sets the stock value
    pub fn set_stock_value(&mut self, stock_id: &str, value: f64) {
        if let Some(stock) = self.stocks.get_mut(stock_id) {
            stock.current_value = value;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    pub name: String,
    pub state: SystemState,
    pub flows: HashMap<String, Flow>,
    pub time_step: f64,
}

impl Model {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: SystemState::new(),
            flows: HashMap::new(),
            time_step: 0.1,
        }
    }

    pub fn add_stock(&mut self, stock: Stock) -> &mut Self {
        self.state.stocks.insert(stock.id.clone(), stock);
        self
    }

    /// Adds a stock array to the model by expanding it into individual stocks.
    pub fn add_stock_array(&mut self, stock_array: StockArray) -> &mut Self {
        for stock in stock_array.expand() {
            self.state.stocks.insert(stock.id.clone(), stock);
        }
        self
    }

    pub fn add_flow(&mut self, flow: Flow) -> &mut Self {
        self.flows.insert(flow.id.clone(), flow);
        self
    }
    pub fn set_time_step(&mut self, dt: f64) -> &mut Self {
        self.time_step = dt;
        self
    }

    pub fn simulate(&mut self, duration: f64) -> SimulationResult {
        let mut result = SimulationResult::new();
        let end_time = self.state.time + duration;

        result.record_state(self.state.time, &self.state);
        while self.state.time < end_time {
            let snapshot = self.state.clone();

            let mut derivatives = HashMap::new();
            for stock_id in self.state.stocks.keys() {
                derivatives.insert(stock_id.clone(), 0.0);
            }

            for flow in self.flows.values() {
                let rate = flow.calculate_rate(&snapshot);

                if let Some(from_stock) = &flow.from_stock {
                    if let Some(derivative) = derivatives.get_mut(from_stock) {
                        *derivative -= rate;
                    }
                }
                if let Some(to_stock) = &flow.to_stock {
                    if let Some(derivative) = derivatives.get_mut(to_stock) {
                        *derivative += rate;
                    }
                }
            }

            for (stock_id, derivative) in derivatives {
                if let Some(stock) = self.state.stocks.get_mut(&stock_id) {
                    let mut new_value = stock.current_value + derivative * self.time_step;

                    if let Some(min) = stock.min_value {
                        new_value = new_value.max(min);
                    }

                    if let Some(max) = stock.max_value {
                        new_value = new_value.min(max);
                    }

                    stock.current_value = new_value;
                }
            }

            self.state.time += self.time_step;
            result.record_state(self.state.time, &self.state);
        }
        result
    }
}

#[derive(Debug, Default)]
pub struct SimulationResult {
    pub time_series: Vec<f64>,
    pub stock_values: HashMap<String, Vec<f64>>,
}

impl SimulationResult {
    pub fn new() -> Self {
        Self {
            time_series: Vec::new(),
            stock_values: HashMap::new(),
        }
    }

    pub fn record_state(&mut self, time: f64, state: &SystemState) {
        self.time_series.push(time);

        for (stock_id, stock) in &state.stocks {
            self.stock_values
                .entry(stock_id.clone())
                .or_default()
                .push(stock.current_value);
        }
    }

    pub fn print_summary(&self) {
        println!("Simulation Results Summary:");
        println!(
            "Duration: {:.2} time units",
            self.time_series.last().unwrap_or(&0.0)
        );
        println!("Time steps: {}", self.time_series.len());

        for (stock_id, values) in &self.stock_values {
            let initial = values.first().unwrap_or(&0.0);
            let final_val = values.last().unwrap_or(&0.0);
            println!("Stock '{}': {:.3} -> {:.3}", stock_id, initial, final_val);
        }
    }

    pub fn print_detailed(&self, stock_names: &[&str]) {
        println!("\nDetailed Results:");
        print!("{:>8}", "Time");
        for name in stock_names {
            print!("{:>12}", name);
        }
        println!();

        // Calculate and print dynamic separator
        let total_width = 8 + stock_names.len() * 12;
        println!("{}", "-".repeat(total_width));

        // Print data rows
        for (i, time) in self.time_series.iter().enumerate() {
            print!("{:8.2}", time);
            for stock_id in stock_names {
                if let Some(values) = self.stock_values.get(*stock_id) {
                    if i < values.len() {
                        print!("{:12.3}", values[i]);
                    } else {
                        print!("{:>12}", ""); // Empty cell if no data
                    }
                } else {
                    print!("{:>12}", ""); // Empty cell if stock not found
                }
            }
            println!();
        }
    }
}
