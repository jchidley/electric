# Earthing Conductor (UK Standards)

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
$A = \frac{I \sqrt{t}}{K}$

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
   - Voltage drop = 11.26V
   - Percentage drop = (11.26/230) × 100 = 4.9%

2. At 50% load (32A):
   - Voltage drop = 4.4 × 32 × 40 / 1000
   - Voltage drop = 5.63V
   - Percentage drop = (5.63/230) × 100 = 2.45%

Note: BS 7671 recommends that the voltage drop should not exceed 3% for lighting circuits and 5% for other circuits. In this example, the voltage drop at maximum current (4.9%) is within the acceptable range for power circuits but exceeds the recommended limit for lighting circuits.

