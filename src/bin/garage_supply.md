# Garage Supply Cable Calculator

## Overview

This calculator provides voltage drop, current rating, power loss, and associated costs for SWA cables used in garage supplies, following BS 7671 standards and manufacturer's data.

## Cable Specifications

### 10mm² Cable

- Maximum Current Capacity: 71A (Reference Method D (buried))
- Voltage Drop per ampere per meter: 4.7 mV/A/m

#### Voltage Drop and Power Loss Calculations (45m length) 10mm² Cable

| Current | Voltage Drop | % Drop | Power Loss | Yearly Loss | Cosy Cost | Go Cost | Investment |
|---------|-------------|---------|------------|-------------|-----------|---------|------------|
| 32A | 6.77V | 2.94% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |
| 40A | 6.77V | 2.94% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |
| 50A | 6.77V | 2.94% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |

### 16mm² Cable

- Maximum Current Capacity: 91A (Reference Method D (buried))
- Voltage Drop per ampere per meter: 2.9 mV/A/m

#### Voltage Drop and Power Loss Calculations (45m length) 16mm² Cable

| Current | Voltage Drop | % Drop | Power Loss | Yearly Loss | Cosy Cost | Go Cost | Investment |
|---------|-------------|---------|------------|-------------|-----------|---------|------------|
| 32A | 6.77V | 2.94% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |
| 40A | 6.77V | 2.94% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |
| 50A | 6.77V | 2.94% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |

### 25mm² Cable

- Maximum Current Capacity: 116A (Reference Method D (buried))
- Voltage Drop per ampere per meter: 1.9 mV/A/m

#### Voltage Drop and Power Loss Calculations (45m length) 25mm² Cable

| Current | Voltage Drop | % Drop | Power Loss | Yearly Loss | Cosy Cost | Go Cost | Investment |
|---------|-------------|---------|------------|-------------|-----------|---------|------------|
| 32A | 6.77V | 2.94% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |
| 40A | 6.77V | 2.94% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |
| 50A | 6.77V | 2.94% | {:.2}W | {:.2}kWh | £{:.2} | £{:.2} | £{:.2} |

## BS 7671 Recommendations

- Maximum 3% voltage drop for lighting circuits
- Maximum 5% voltage drop for other circuits

## Notes

- All calculations are based on manufacturer's data
- Installation method is Reference Method D (buried)
- Cable length is 45 meters (default, can be changed with --length option)
- Nominal voltage is 230V
- Power loss is calculated as voltage drop × current
- Yearly power loss is calculated assuming 4 hours of daily usage
- Yearly power loss is converted to kWh (kilowatt-hours)
- Costs are calculated using:
  - Octopus Cosy: 14.14p per kWh
  - Octopus Go: 8.5p per kWh
- Investment column shows the cost difference between using a larger cable size
