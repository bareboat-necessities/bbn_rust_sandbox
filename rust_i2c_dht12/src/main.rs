use dht_embedded::{Dht22, DhtSensor, NoopInterruptControl};
use gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::{CdevPin, Delay};
use std::{thread::sleep, time::Duration};

fn main() -> anyhow::Result<()> {
    let mut gpiochip = Chip::new("/dev/gpiochip0")?;
    let line = gpiochip.get_line(17)?;
    let handle = line.request(LineRequestFlags::INPUT | LineRequestFlags::OUTPUT, 1, "dht-sensor")?;
    let pin = CdevPin::new(handle)?;
    let mut sensor = Dht22::new(NoopInterruptControl, Delay, pin);

    loop {
        match sensor.read() {
            Ok(reading) => println!("{}°C, {}% RH", reading.temperature(), reading.humidity()),
            Err(e) => eprintln!("Error: {}", e),
        }

        sleep(Duration::from_millis(2100));
    }
}
