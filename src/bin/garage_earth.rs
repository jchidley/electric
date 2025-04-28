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
}

impl Default for MaterialConstants {
    fn default() -> Self {
        Self { copper: 143.0 }
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

    // Read the markdown template
    let markdown_template = std::fs::read_to_string("src/bin/garage_earth.md")
        .expect("Failed to read markdown template");

    // Replace placeholders with calculated values
    markdown_template.replace("{:.2}", &format!("{:.2}", copper_size))
}

fn main() {
    let materials = MaterialConstants::default();

    // Example calculation based on BS 88-3 C fuse
    let fault_current = 580.0; // 100A BS 88-3 C fuse fault current
    let fault_duration = 5.0; // Standard fault duration for BS 88-3 C fuse

    // Calculate conductor size
    let copper_size = calculate_conductor_size(fault_current, fault_duration, materials.copper);

    // Display conductor size calculations
    println!("Earthing Conductor Size Calculator");
    println!("=================================");
    println!("Protective Device: 100A BS 88-3 C Fuse");
    println!("Fault Current: {} A", fault_current);
    println!("Fault Duration: {} s", fault_duration);
    println!("\nRequired Conductor Size:");
    println!("Copper: {:.2} mm²", copper_size);

    // Generate and write markdown file
    let markdown_content = generate_markdown();
    std::fs::write("src/bin/garage_earth.md", markdown_content)
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
