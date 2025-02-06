use linux_embedded_hal::I2cdev;
use std::thread;
use std::time::Duration;

const DHT12_I2C_ADDR: u16 = 0x5C; // Default I2C address for DHT12

fn main() {
    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();

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
