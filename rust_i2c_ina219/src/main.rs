use ina219::address::Address;
use ina219::configuration::{
    BusVoltageRange, Configuration, MeasuredSignals, OperatingMode, Reset, Resolution,
    ShuntVoltageRange,
};
use ina219::SyncIna219;
use linux_embedded_hal::I2cdev;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut ina = SyncIna219::new(i2c_bus, Address::from_byte(0x42).unwrap());

    ina.set_configuration(Configuration {
        // Be extra precise, but take some extra time
        bus_resolution: Resolution::Avg128,
        shunt_resolution: Resolution::Avg128,

        // We only care about low voltage bus and shunt, values larger are truncated to the max
        bus_voltage_range: BusVoltageRange::Fsr16v,
        shunt_voltage_range: ShuntVoltageRange::Fsr40mv,

        // Measure both signals continuously (default)
        operating_mode: OperatingMode::Continous(MeasuredSignals::ShutAndBusVoltage),

        // Do not perform a reset
        reset: Reset::Run,
    })?;
    
    /*
    // Wait until a result is ready
    std::thread::sleep(ina.configuration().unwrap().conversion_time().unwrap());

    println!("Bus Voltage: {}", ina.bus_voltage().unwrap());
    println!("Shunt Voltage: {}", ina.shunt_voltage().unwrap());
    */
    
    Ok(())
}
