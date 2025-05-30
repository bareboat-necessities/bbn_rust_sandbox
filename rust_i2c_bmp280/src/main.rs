use linux_embedded_hal::{Delay, I2cdev};
use bme280::i2c::BME280;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize I2C device (e.g., /dev/i2c-1 on Raspberry Pi)
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();

    // Initialize BMP280 sensor
    let mut bmp280 = BME280::new_primary(i2c_bus);

    // Initialize the sensor
    bmp280.init(&mut Delay).unwrap();

    loop {
        let measurements = bmp280.measure(&mut Delay).unwrap();

        // Print the pressure in Pascals (Pa)
        println!("Pressure: {:.2} Pa", measurements.pressure);

        // Wait for a second before reading again
        thread::sleep(Duration::from_secs(1));
    }
}
