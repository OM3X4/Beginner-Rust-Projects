use clap::Parser;

enum Unit {
    Length,
    Tempreature,
}

trait Conversion {
    fn convert(value: f64, from_unit: &str, to_unit: &str) -> f64;
}

impl Conversion for Unit {
    fn convert(value: f64, from_unit: &str, to_unit: &str) -> f64 {
        let mut length_units = std::collections::HashMap::new();
        length_units.insert("meter", 1.0);
        length_units.insert("kilometer", 1000.0);
        length_units.insert("centimeter", 0.01);
        length_units.insert("inch", 0.0254);
        length_units.insert("foot", 0.3048);

        let input_unit_value = length_units.get(from_unit).unwrap_or_else(|| panic!("Unsupported input unit"));
        let output_unit_value = length_units.get(to_unit).unwrap_or_else(|| panic!("Unsupported output unit"));

        let base_value = (value as f64) * input_unit_value;
        let converted = base_value / output_unit_value;

        converted
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    input: String,
    #[arg(long = "input_unit")]
    input_unit: String,
}

fn main() {
    let cli = Cli::parse();


    println!("{}" , cli.input_unit);
    let value: f64 = cli.input.parse().unwrap();
    let converted = Unit::convert(value, &cli.input_unit.trim(), "centimeter");
    println!("{}", converted);

}
