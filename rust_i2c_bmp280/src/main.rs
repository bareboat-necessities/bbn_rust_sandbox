use bmp280::BMP280;
use linux_embedded_hal::{Delay, I2cdev};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize I2C device (e.g., /dev/i2c-1 on Raspberry Pi)
    let i2c = I2cdev::new("/dev/i2c-1")?;

    // Initialize BMP280 sensor
    let mut bmp280 = BMP280::new_primary(i2c, Delay);

    // Initialize the sensor
    bmp280.init()?;

    loop {
        // Read pressure data
        let pressure = bmp280.read_pressure()?;

        // Print the pressure in Pascals (Pa)
        println!("Pressure: {:.2} Pa", pressure);

        // Wait for a second before reading again
        thread::sleep(Duration::from_secs(1));
    }
}
