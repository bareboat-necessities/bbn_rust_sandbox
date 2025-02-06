use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;
use linux_embedded_hal::I2cdev;
use std::thread;
use std::time::Duration;

const DHT12_I2C_ADDR: u16 = 0x5C; // Default I2C address for DHT12

fn main() {
    // Open the I2C device (usually /dev/i2c-1 on Raspberry Pi)
    let i2c_dev = LinuxI2CDevice::new("/dev/i2c-1", DHT12_I2C_ADDR).unwrap();

    // Wrap the I2C device in the `linux-embedded-hal` I2C interface
    let mut i2c = I2cdev::new(i2c_dev).unwrap();

    // Buffer to store the read data
    let mut buffer = [0u8; 5];

    loop {
        // Read 5 bytes from the DHT12 sensor
        if let Err(e) = i2c.read(&mut buffer) {
            eprintln!("Failed to read from DHT12: {}", e);
            continue;
        }

        // Calculate temperature and humidity from the raw data
        let humidity = buffer[0] as f32 + (buffer[1] as f32 * 0.1);
        let temperature = buffer[2] as f32 + (buffer[3] as f32 * 0.1);

        println!("Temperature: {:.1}°C, Humidity: {:.1}%", temperature, humidity);

        // Wait for 2 seconds before the next reading
        thread::sleep(Duration::from_secs(2));
    }
}
