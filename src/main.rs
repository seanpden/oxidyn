use std::process;

#[derive(Debug, Clone)]
pub struct Stock {
    pub id: String,
    pub name: String,
    pub initial_value: f64,
    pub current_value: f64,
    pub units: String,
}

impl Stock {
    pub fn build(
        id: &str,
        name: &str,
        initial_value: f64,
        current_value: f64,
        units: &str,
    ) -> Result<Stock, &'static str> {
        Ok(Stock {
            id: id.to_string(),
            name: name.to_string(),
            initial_value,
            current_value,
            units: units.to_string(),
        })
    }
}

fn main() {
    let stock = Stock::build("ID", "NAME", 1.0, 1.0, "UNITS").unwrap_or_else(|err| {
        eprintln!("Problem creating stock `ID`: {err}");
        process::exit(1);
    });
    dbg!(stock);
}
