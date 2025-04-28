//! Garage Supply Cable Calculator
//!
//! This program calculates voltage drops and current ratings for SWA cables
//! used in garage supplies, following BS 7671 standards and manufacturer's data.

use clap::Parser;

/// Command line arguments for the garage supply calculator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the cable in meters
    #[arg(short, long, default_value_t = 45.0)]
    length: f64,
}

/// Calculates the voltage drop for a cable using the formula from BS 7671
///
/// # Arguments
///
/// * `mv_per_amp_per_meter` - The voltage drop per ampere per meter (mV/A/m)
/// * `current` - The current in amperes (A)
/// * `length` - The length of the cable in meters (m)
///
/// # Returns
///
/// The voltage drop in volts (V)
pub fn calculate_voltage_drop(mv_per_amp_per_meter: f64, current: f64, length: f64) -> f64 {
    (mv_per_amp_per_meter * current * length) / 1000.0
}

/// Calculates the percentage voltage drop
///
/// # Arguments
///
/// * `voltage_drop` - The voltage drop in volts (V)
/// * `nominal_voltage` - The nominal voltage in volts (V)
///
/// # Returns
///
/// The percentage voltage drop
pub fn calculate_percentage_drop(voltage_drop: f64, nominal_voltage: f64) -> f64 {
    (voltage_drop / nominal_voltage) * 100.0
}

/// Generates the markdown content with all calculations
pub fn generate_markdown(length: f64) -> String {
    // Voltage drop calculations for different cable sizes
    let mv_per_amp_per_meter_10mm = 4.7; // From manufacturer's data
    let mv_per_amp_per_meter_16mm = 2.9; // From manufacturer's data
    let mv_per_amp_per_meter_25mm = 1.9; // From manufacturer's data
    let nominal_voltage = 230.0;

    // Specific current values to calculate
    let current_32a = 32.0;
    let current_40a = 40.0;
    let current_50a = 50.0;

    // Calculate voltage drops for 10mm² cable
    let voltage_drop_32a_10mm =
        calculate_voltage_drop(mv_per_amp_per_meter_10mm, current_32a, length);
    let percentage_drop_32a_10mm =
        calculate_percentage_drop(voltage_drop_32a_10mm, nominal_voltage);
    let voltage_drop_40a_10mm =
        calculate_voltage_drop(mv_per_amp_per_meter_10mm, current_40a, length);
    let percentage_drop_40a_10mm =
        calculate_percentage_drop(voltage_drop_40a_10mm, nominal_voltage);
    let voltage_drop_50a_10mm =
        calculate_voltage_drop(mv_per_amp_per_meter_10mm, current_50a, length);
    let percentage_drop_50a_10mm =
        calculate_percentage_drop(voltage_drop_50a_10mm, nominal_voltage);

    // Calculate voltage drops for 16mm² cable
    let voltage_drop_32a_16mm =
        calculate_voltage_drop(mv_per_amp_per_meter_16mm, current_32a, length);
    let percentage_drop_32a_16mm =
        calculate_percentage_drop(voltage_drop_32a_16mm, nominal_voltage);
    let voltage_drop_40a_16mm =
        calculate_voltage_drop(mv_per_amp_per_meter_16mm, current_40a, length);
    let percentage_drop_40a_16mm =
        calculate_percentage_drop(voltage_drop_40a_16mm, nominal_voltage);
    let voltage_drop_50a_16mm =
        calculate_voltage_drop(mv_per_amp_per_meter_16mm, current_50a, length);
    let percentage_drop_50a_16mm =
        calculate_percentage_drop(voltage_drop_50a_16mm, nominal_voltage);

    // Calculate voltage drops for 25mm² cable
    let voltage_drop_32a_25mm =
        calculate_voltage_drop(mv_per_amp_per_meter_25mm, current_32a, length);
    let percentage_drop_32a_25mm =
        calculate_percentage_drop(voltage_drop_32a_25mm, nominal_voltage);
    let voltage_drop_40a_25mm =
        calculate_voltage_drop(mv_per_amp_per_meter_25mm, current_40a, length);
    let percentage_drop_40a_25mm =
        calculate_percentage_drop(voltage_drop_40a_25mm, nominal_voltage);
    let voltage_drop_50a_25mm =
        calculate_voltage_drop(mv_per_amp_per_meter_25mm, current_50a, length);
    let percentage_drop_50a_25mm =
        calculate_percentage_drop(voltage_drop_50a_25mm, nominal_voltage);

    // Read the markdown template
    let markdown_template = std::fs::read_to_string("src/bin/garage_supply.md")
        .expect("Failed to read markdown template");

    // Replace placeholders with calculated values
    markdown_template
        .replace("{:.2}V", &format!("{:.2}V", voltage_drop_32a_10mm))
        .replace("{:.2}%", &format!("{:.2}%", percentage_drop_32a_10mm))
        .replace("{:.2}V", &format!("{:.2}V", voltage_drop_40a_10mm))
        .replace("{:.2}%", &format!("{:.2}%", percentage_drop_40a_10mm))
        .replace("{:.2}V", &format!("{:.2}V", voltage_drop_50a_10mm))
        .replace("{:.2}%", &format!("{:.2}%", percentage_drop_50a_10mm))
        .replace("{:.2}V", &format!("{:.2}V", voltage_drop_32a_16mm))
        .replace("{:.2}%", &format!("{:.2}%", percentage_drop_32a_16mm))
        .replace("{:.2}V", &format!("{:.2}V", voltage_drop_40a_16mm))
        .replace("{:.2}%", &format!("{:.2}%", percentage_drop_40a_16mm))
        .replace("{:.2}V", &format!("{:.2}V", voltage_drop_50a_16mm))
        .replace("{:.2}%", &format!("{:.2}%", percentage_drop_50a_16mm))
        .replace("{:.2}V", &format!("{:.2}V", voltage_drop_32a_25mm))
        .replace("{:.2}%", &format!("{:.2}%", percentage_drop_32a_25mm))
        .replace("{:.2}V", &format!("{:.2}V", voltage_drop_40a_25mm))
        .replace("{:.2}%", &format!("{:.2}%", percentage_drop_40a_25mm))
        .replace("{:.2}V", &format!("{:.2}V", voltage_drop_50a_25mm))
        .replace("{:.2}%", &format!("{:.2}%", percentage_drop_50a_25mm))
}

fn main() {
    let args = Args::parse();

    // Voltage drop calculations for different cable sizes
    println!("Garage Supply Cable Calculator");
    println!("=============================");
    println!("Cable Length: {}m", args.length);
    println!("Nominal Voltage: 230V");

    // Specific current values to calculate
    let current_32a = 32.0;
    let current_40a = 40.0;
    let current_50a = 50.0;

    // 10mm² cable calculations
    let mv_per_amp_per_meter_10mm = 4.7;
    println!("\n10mm² Cable (mV/A/m = 4.7):");
    println!("Maximum Current Capacity: 71A (Reference Method D (buried))");
    println!("\n| Current | Voltage Drop | % Drop | Power Loss | Yearly Loss | Cosy Cost | Go Cost | Investment |");
    println!("|---------|-------------|---------|------------|-------------|-----------|---------|------------|");
    for current in &[current_32a, current_40a, current_50a] {
        let voltage_drop = calculate_voltage_drop(mv_per_amp_per_meter_10mm, *current, args.length);
        let percentage_drop = calculate_percentage_drop(voltage_drop, 230.0);
        let power_loss = voltage_drop * *current;
        let yearly_power_loss = power_loss * 4.0 * 365.0 / 1000.0; // Convert to kWh
        let cosy_cost = yearly_power_loss * 0.1414; // 14.14p per kWh
        let go_cost = yearly_power_loss * 0.085; // 8.5p per kWh
        let investment = 0.0; // Base case - no investment needed
        println!(
            "| {}A | {:.2}V | {:.2}% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |",
            current,
            voltage_drop,
            percentage_drop,
            power_loss,
            yearly_power_loss,
            cosy_cost,
            go_cost,
            investment
        );
    }

    // 16mm² cable calculations
    let mv_per_amp_per_meter_16mm = 2.9;
    println!("\n16mm² Cable (mV/A/m = 2.9):");
    println!("Maximum Current Capacity: 91A (Reference Method D (buried))");
    println!("\n| Current | Voltage Drop | % Drop | Power Loss | Yearly Loss | Cosy Cost | Go Cost | Investment |");
    println!("|---------|-------------|---------|------------|-------------|-----------|---------|------------|");
    for current in &[current_32a, current_40a, current_50a] {
        let voltage_drop = calculate_voltage_drop(mv_per_amp_per_meter_16mm, *current, args.length);
        let percentage_drop = calculate_percentage_drop(voltage_drop, 230.0);
        let power_loss = voltage_drop * *current;
        let yearly_power_loss = power_loss * 4.0 * 365.0 / 1000.0; // Convert to kWh
        let cosy_cost = yearly_power_loss * 0.1414; // 14.14p per kWh
        let go_cost = yearly_power_loss * 0.085; // 8.5p per kWh
                                                 // Calculate investment as the difference in yearly costs between 10mm² and 16mm²
        let voltage_drop_10mm =
            calculate_voltage_drop(mv_per_amp_per_meter_10mm, *current, args.length);
        let power_loss_10mm = voltage_drop_10mm * *current;
        let yearly_power_loss_10mm = power_loss_10mm * 4.0 * 365.0 / 1000.0;
        let cosy_cost_10mm = yearly_power_loss_10mm * 0.1414;
        let investment = cosy_cost_10mm - cosy_cost; // Cost savings from upgrading
        println!(
            "| {}A | {:.2}V | {:.2}% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |",
            current,
            voltage_drop,
            percentage_drop,
            power_loss,
            yearly_power_loss,
            cosy_cost,
            go_cost,
            investment
        );
    }

    // 25mm² cable calculations
    let mv_per_amp_per_meter_25mm = 1.9;
    println!("\n25mm² Cable (mV/A/m = 1.9):");
    println!("Maximum Current Capacity: 116A (Reference Method D (buried))");
    println!("\n| Current | Voltage Drop | % Drop | Power Loss | Yearly Loss | Cosy Cost | Go Cost | Investment |");
    println!("|---------|-------------|---------|------------|-------------|-----------|---------|------------|");
    for current in &[current_32a, current_40a, current_50a] {
        let voltage_drop = calculate_voltage_drop(mv_per_amp_per_meter_25mm, *current, args.length);
        let percentage_drop = calculate_percentage_drop(voltage_drop, 230.0);
        let power_loss = voltage_drop * *current;
        let yearly_power_loss = power_loss * 4.0 * 365.0 / 1000.0; // Convert to kWh
        let cosy_cost = yearly_power_loss * 0.1414; // 14.14p per kWh
        let go_cost = yearly_power_loss * 0.085; // 8.5p per kWh
                                                 // Calculate investment as the difference in yearly costs between 10mm² and 25mm²
        let voltage_drop_10mm =
            calculate_voltage_drop(mv_per_amp_per_meter_10mm, *current, args.length);
        let power_loss_10mm = voltage_drop_10mm * *current;
        let yearly_power_loss_10mm = power_loss_10mm * 4.0 * 365.0 / 1000.0;
        let cosy_cost_10mm = yearly_power_loss_10mm * 0.1414;
        let investment = cosy_cost_10mm - cosy_cost; // Cost savings from upgrading
        println!(
            "| {}A | {:.2}V | {:.2}% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |",
            current,
            voltage_drop,
            percentage_drop,
            power_loss,
            yearly_power_loss,
            cosy_cost,
            go_cost,
            investment
        );
    }

    println!("\nNote: BS 7671 recommends:");
    println!("- Maximum 3% voltage drop for lighting circuits");
    println!("- Maximum 5% voltage drop for other circuits");

    // Generate and write markdown file
    let markdown_content = generate_markdown(args.length);
    std::fs::write("src/bin/garage_supply.md", markdown_content)
        .expect("Failed to write markdown file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voltage_drop_calculation() {
        let mv_per_amp_per_meter = 4.7;
        let current = 32.0;
        let length = 45.0;

        let result = calculate_voltage_drop(mv_per_amp_per_meter, current, length);
        assert!((result - 6.77).abs() < 0.01); // Allowing for small floating-point differences
    }

    #[test]
    fn test_percentage_drop_calculation() {
        let voltage_drop = 6.77;
        let nominal_voltage = 230.0;

        let result = calculate_percentage_drop(voltage_drop, nominal_voltage);
        assert!((result - 2.94).abs() < 0.01); // Allowing for small floating-point differences
    }
}
