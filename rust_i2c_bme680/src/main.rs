extern crate bme680;

// Note that you'll have to import your board crates types corresponding to
// Delay and I2cdev.

use bme680::*;
use embedded_hal::blocking::i2c;
use hal::*;
use std::result;
use std::time::Duration;
use std::thread::sleep;


fn main() -> result::Result<(), Error<<hal::I2cdev as i2c::Read>::Error, <hal::I2cdev as i2c::Write>::Error>> {
    // Initialize I2C device (e.g., /dev/i2c-1 on Raspberry Pi)
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut delayer = Delay {}; // Your Delay construction will look different, perhaps using Delay::new(..)
    let mut dev = Bme680::init(i2c, &mut delayer, I2CAddress::Primary)?;
    let settings = SettingsBuilder::new()
        .with_humidity_oversampling(OversamplingSetting::OS2x)
        .with_pressure_oversampling(OversamplingSetting::OS4x)
        .with_temperature_oversampling(OversamplingSetting::OS8x)
        .with_temperature_filter(IIRFilterSize::Size3)
        .with_gas_measurement(Duration::from_millis(1500), 320, 25)
        .with_run_gas(true)
        .build();
    dev.set_sensor_settings(&mut delayer, settings)?;
    let profile_duration = dev.get_profile_dur(&settings.0)?;

    // Read sensor data
    dev.set_sensor_mode(&mut delayer, PowerMode::ForcedMode)?;
    sleep(profile_duration);
    let (data, _state) = dev.get_sensor_data(&mut delayer)?;

    println!("Temperature {}°C", data.temperature_celsius());
    println!("Pressure {}hPa", data.pressure_hpa());
    println!("Humidity {}%", data.humidity_percent());
    println!("Gas Resistence {}Ω", data.gas_resistance_ohm());

    Ok(())
}
