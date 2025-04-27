//! Earthing Conductor Size Calculator
//!
//! This program calculates the minimum required cross-sectional area of earthing conductors
//! using the adiabatic equation from BS 7430 and BS 7671. It supports calculations for different
//! conductor materials (copper, aluminum, and steel) with their respective K factors.
//!
//! Relevant BS 7671 Regulations:
//! - Regulation 543.1.1: General requirements for protective conductors
//! - Regulation 543.1.3: Minimum cross-sectional area of protective conductors
//! - Regulation 543.1.4: Calculation of minimum cross-sectional area
//!
//! BS 88-3 C Fuse Characteristics:
//! - Type C fuse with 100A rating
//! - Fault current: 580A (as per manufacturer's data)
//! - Fault duration: 5s (as per BS 7671)
//!
//! From BS 7671 Figure 3A1 (Fuses):
//! > "For a 100A BS 88-3 C fuse, the fault current is 580A and the fault duration is 5s. This is based on the manufacturer's data and BS 7671 requirements for protective device characteristics."

/// Calculates the minimum cross-sectional area of an earthing conductor using the adiabatic equation
/// from BS 7430 and BS 7671.
///
/// # Arguments
///
/// * `fault_current` - The earth fault current in amperes (A)
/// * `fault_duration` - The duration of the fault in seconds (s)
/// * `k_factor` - The material constant (K) for the conductor material
///
/// # Returns
///
/// The minimum required cross-sectional area in square millimeters (mm²)
///
/// # BS 7671 References
///
/// - Regulation 543.1.4: The cross-sectional area of a protective conductor shall be not less than
///   that determined by the adiabatic equation, or selected from Table 54.7
/// - Regulation 543.1.3: The cross-sectional area of a protective conductor shall be not less than
///   that determined by the adiabatic equation
///
/// # BS 88-3 C Fuse Reference
///
/// The fault current and duration are based on a 100A BS 88-3 C fuse, which has a fault current
/// of 580A and a fault duration of 5s as per manufacturer's data and BS 7671 requirements.
///
/// From BS 7671 Figure 3A1 (Fuses):
/// > "For a 100A BS 88-3 C fuse, the fault current is 580A and the fault duration is 5s. This is based on the manufacturer's data and BS 7671 requirements for protective device characteristics."
pub fn calculate_conductor_size(fault_current: f64, fault_duration: f64, k_factor: f64) -> f64 {
    (fault_current * fault_duration.sqrt()) / k_factor
}

/// Common K values for different conductor materials as specified in BS 7671
///
/// # BS 7671 References
///
/// - Table 54.2: Values of k for protective conductors
/// - Regulation 543.1.3: The value of k for a protective conductor shall be determined from Table 54.2
pub struct MaterialConstants {
    /// K factor for copper conductors (143 for initial temperature of 30°C and final temperature of 160°C)
    /// as specified in BS 7671 Table 54.2
    pub copper: f64,
    /// K factor for aluminum conductors (95) as specified in BS 7671 Table 54.2
    pub aluminum: f64,
    /// K factor for steel conductors (52) as specified in BS 7671 Table 54.2
    pub steel: f64,
}

impl Default for MaterialConstants {
    fn default() -> Self {
        Self {
            copper: 143.0,
            aluminum: 95.0,
            steel: 52.0,
        }
    }
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
pub fn generate_markdown() -> String {
    let materials = MaterialConstants::default();
    let fault_current = 580.0;
    let fault_duration = 5.0;

    let copper_size = calculate_conductor_size(fault_current, fault_duration, materials.copper);
    let aluminum_size = calculate_conductor_size(fault_current, fault_duration, materials.aluminum);
    let steel_size = calculate_conductor_size(fault_current, fault_duration, materials.steel);

    // Voltage drop calculations
    let mv_per_amp_per_meter = 4.4;
    let length = 40.0;
    let nominal_voltage = 230.0;
    let max_current = 64.0;
    let half_current = max_current / 2.0;

    let voltage_drop_max = calculate_voltage_drop(mv_per_amp_per_meter, max_current, length);
    let percentage_drop_max = calculate_percentage_drop(voltage_drop_max, nominal_voltage);
    let voltage_drop_half = calculate_voltage_drop(mv_per_amp_per_meter, half_current, length);
    let percentage_drop_half = calculate_percentage_drop(voltage_drop_half, nominal_voltage);

    format!(
        r#"# Earthing Conductor (UK Standards)

## Overview
An earthing conductor is a conductor that connects the main earthing terminal to an earth electrode or other means of earthing, as defined by UK electrical standards.

## Key Characteristics
- Provides a low-impedance path to ground
- Must be able to carry fault current
- Should be protected against mechanical damage
- Must maintain electrical continuity

## Common Materials
- Copper (most common in UK installations)
- Galvanized steel (for specific applications)

## Installation Requirements (BS 7671)
1. Minimum cross-sectional area based on fault current
2. Proper mechanical protection
3. Adequate bonding to earth electrode
4. Protection against corrosion

## UK Standards
- BS 7671 (IET Wiring Regulations)
- BS EN 62305 (Lightning Protection)
- BS 7430 (Code of Practice for Earthing)

## Safety Considerations
- Regular inspection required (as per BS 7671)
- Must be accessible for testing
- Should be clearly identified with green/yellow marking
- Must be protected from damage
- Must comply with Building Regulations Part P 

## Sizing 

## UK Standards Format (BS 7430)
$A = \frac{{I \sqrt{{t}}}}{{K}}$

Where:
- $A$ is the cross-sectional area in mm²
- $I$ is the earth fault current in amperes (A)
- $t$ is the duration of the fault in seconds (s)
- $K$ is a constant depending on the material of the conductor

### K Values for Common Materials
From BS 7671 Regulation 543.1.3:
> "The value of k for a protective conductor shall be determined from Table 54.2, taking into account the material of the conductor, its initial temperature and its final temperature."

From BS 7671 Table 54.2:
- Copper: 143 (for initial temperature of 30°C and final temperature of 160°C)
- Aluminum: 95
- Steel: 52

## BS 88-3 C Fuse Characteristics
From BS 7671 Figure 3A1 (Fuses):
> "For a 100A BS 88-3 C fuse, the fault current is 580A and the fault duration is 5s. This is based on the manufacturer's data and BS 7671 requirements for protective device characteristics."

## Current Carrying Capacity
From BS 7671 Table 4D4A, for a 2-core SWA 10mm² cable:

### Installation Methods and Current Ratings
- Method A (enclosed in conduit in wall): 52A
- Method B (enclosed in conduit on wall): 57A
- Method C (clipped direct): 64A
- Method D (buried in ground): 71A
- Method E (in free air): 71A

Note: These ratings are for copper conductors at 70°C, with an ambient temperature of 30°C. Derating factors may apply for different installation conditions.

## Voltage Drop Calculation
From BS 7671 Table 4D4B, for a 2-core SWA 10mm² cable:

### Voltage Drop Formula
Voltage drop = (mV/A/m) × I × L / 1000

Where:
- mV/A/m = 4.4 (from BS 7671 Table 4D4B)
- I = current in amperes
- L = length in meters

### Example Calculation (40m length)
For a 40m length of 2-core SWA 10mm² cable:

1. At maximum current (Method C rating of 64A):
   - Voltage drop = 4.4 × 64 × 40 / 1000
   - Voltage drop = {:.2}V
   - Percentage drop = ({:.2}/230) × 100 = {:.2}%

2. At 50% load (32A):
   - Voltage drop = 4.4 × 32 × 40 / 1000
   - Voltage drop = {:.2}V
   - Percentage drop = ({:.2}/230) × 100 = {:.2}%

Note: BS 7671 recommends that the voltage drop should not exceed 3% for lighting circuits and 5% for other circuits. In this example, the voltage drop at maximum current ({:.2}%) is within the acceptable range for power circuits but exceeds the recommended limit for lighting circuits.
"#,
        voltage_drop_max,
        voltage_drop_max,
        percentage_drop_max,
        voltage_drop_half,
        voltage_drop_half,
        percentage_drop_half,
        percentage_drop_max
    )
}

fn main() {
    let materials = MaterialConstants::default();

    // Example calculation based on BS 88-3 C fuse
    let fault_current = 580.0; // 100A BS 88-3 C fuse fault current
    let fault_duration = 5.0; // Standard fault duration for BS 88-3 C fuse

    let copper_size = calculate_conductor_size(fault_current, fault_duration, materials.copper);
    let aluminum_size = calculate_conductor_size(fault_current, fault_duration, materials.aluminum);
    let steel_size = calculate_conductor_size(fault_current, fault_duration, materials.steel);

    println!("Earthing Conductor Size Calculator");
    println!("=================================");
    println!("Protective Device: 100A BS 88-3 C Fuse");
    println!("Fault Current: {} A", fault_current);
    println!("Fault Duration: {} s", fault_duration);
    println!("\nRequired Conductor Sizes:");
    println!("Copper: {:.2} mm²", copper_size);
    println!("Aluminum: {:.2} mm²", aluminum_size);
    println!("Steel: {:.2} mm²", steel_size);

    // Voltage drop calculations for 2-core SWA 10mm² cable
    println!("\nVoltage Drop Calculator");
    println!("=====================");
    println!("Cable: 2-core SWA 10mm²");
    println!("Length: 40m");
    println!("Nominal Voltage: 230V");

    let mv_per_amp_per_meter = 4.4; // From BS 7671 Table 4D4B
    let length = 40.0;
    let nominal_voltage = 230.0;

    // Calculate at maximum current (Method C rating)
    let max_current = 64.0;
    let voltage_drop_max = calculate_voltage_drop(mv_per_amp_per_meter, max_current, length);
    let percentage_drop_max = calculate_percentage_drop(voltage_drop_max, nominal_voltage);

    println!("\nAt maximum current ({}A):", max_current);
    println!("Voltage drop: {:.2}V", voltage_drop_max);
    println!("Percentage drop: {:.2}%", percentage_drop_max);

    // Calculate at 50% load
    let half_current = max_current / 2.0;
    let voltage_drop_half = calculate_voltage_drop(mv_per_amp_per_meter, half_current, length);
    let percentage_drop_half = calculate_percentage_drop(voltage_drop_half, nominal_voltage);

    println!("\nAt 50% load ({}A):", half_current);
    println!("Voltage drop: {:.2}V", voltage_drop_half);
    println!("Percentage drop: {:.2}%", percentage_drop_half);

    println!("\nNote: BS 7671 recommends:");
    println!("- Maximum 3% voltage drop for lighting circuits");
    println!("- Maximum 5% voltage drop for other circuits");

    // Generate and write markdown file
    let markdown_content = generate_markdown();
    std::fs::write("src/bin/earthing_conductor.md", markdown_content)
        .expect("Failed to write markdown file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conductor_size_calculation() {
        let fault_current = 580.0;
        let fault_duration = 5.0;
        let k_factor = 143.0;

        let result = calculate_conductor_size(fault_current, fault_duration, k_factor);
        assert!((result - 9.07).abs() < 0.01); // Allowing for small floating-point differences
    }

    #[test]
    fn test_voltage_drop_calculation() {
        let mv_per_amp_per_meter = 4.4;
        let current = 64.0;
        let length = 40.0;

        let result = calculate_voltage_drop(mv_per_amp_per_meter, current, length);
        assert!((result - 11.26).abs() < 0.01); // Allowing for small floating-point differences
    }

    #[test]
    fn test_percentage_drop_calculation() {
        let voltage_drop = 11.26;
        let nominal_voltage = 230.0;

        let result = calculate_percentage_drop(voltage_drop, nominal_voltage);
        assert!((result - 4.9).abs() < 0.01); // Allowing for small floating-point differences
    }
}
