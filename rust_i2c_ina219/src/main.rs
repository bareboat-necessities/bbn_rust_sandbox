use linux_embedded_hal::{I2cdev, Delay};
use ina219::SyncIna219;
use ina219::address::Address;
use ina219::configuration::{Configuration, BusVoltageRange, Gain};
use ina219::calibration::{Calibration};
use ina219::measurements::{Measurement, CurrentRegister, PowerRegister};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the I2C device (e.g., /dev/i2c-1 on Raspberry Pi)
    let i2c = I2cdev::new("/dev/i2c-1")?;

    // Create a delay instance (used internally by the INA219 crate)
    let delay = Delay;

    // Initialize the INA219 sensor with default address (0x40)
    let mut ina219 = SyncIna219::new(i2c, Address::default(), delay);

    // Configure the INA219 sensor
    let calibration = Calibration::calibrate(
        BusVoltageRange::V16,  // Bus voltage range: 16V
        Gain::Div8,            // Gain: 1/8
        0.1,                   // Shunt resistor value in ohms (e.g., 0.1 ohm)
        3.2,                   // Maximum expected current in amps (e.g., 3.2A)
    )?;
    ina219.configure(Configuration::default().with_calibration(calibration))?;

    // Read and print the values
    loop {
        let measurement = ina219.read()?;
        println!(
            "Bus Voltage: {:.2} V, Shunt Voltage: {:.2} mV, Current: {:.2} mA, Power: {:.2} mW",
            measurement.bus_voltage,
            measurement.shunt_voltage * 1000.0,
            measurement.current * 1000.0,
            measurement.power * 1000.0,
        );

        // Wait for a second before the next reading
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
