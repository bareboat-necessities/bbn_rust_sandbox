use ina219::address::Address;
use ina219::SyncIna219;
use linux_embedded_hal::I2cdev;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut ina = SyncIna219::new(i2c_bus, Address::from_byte(0x42));

    // Wait until a result is ready
    std::thread::sleep(ina.configuration().conversion_time().unwrap());

    println!("Bus Voltage: {}", ina.bus_voltage().unwrap());
    println!("Shunt Voltage: {}", ina.shunt_voltage().unwrap());

    Ok(())
}
