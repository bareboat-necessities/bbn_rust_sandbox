use ds18b20_2::{Ds18b20, Resolution};
use one_wire_bus_2::{OneWire};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the 1-Wire bus on GPIO pin 4 (BCM numbering)
    let mut one_wire_bus = OneWire::new(4).unwrap();

    /*
    // Search for connected devices on the bus
    let mut devices = one_wire_bus.devices(false, &mut Delay)?;

    // Check if any DS18B20 sensor is found
    if let Some(device_address) = devices.next() {
        println!("Found DS18B20 sensor: {:?}", device_address);

        // Create a DS18B20 instance
        let sensor = Ds18b20::new(device_address)?;

        // Set the resolution (optional, default is 12-bit)
        sensor.set_resolution(&mut one_wire_bus, Resolution::Bits12)?;

        // Read the temperature
        let temperature = sensor.read_temperature(&mut one_wire_bus)?;
        println!("Temperature: {:.2}°C", temperature);
    } else {
        println!("No DS18B20 sensor found!");
    }
    */

    Ok(())
}
