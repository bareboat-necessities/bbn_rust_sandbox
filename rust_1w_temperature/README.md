### Steps:
1. Connect the DS18B20 sensor to your Raspberry Pi or microcontroller.
2. Ensure the 1-Wire interface is enabled on your Raspberry Pi.
3. Use the `onewire` and `ds18b20` crates to read the temperature.

### Explanation:
1. **`OneWire::new(4)`**: Initializes the 1-Wire bus on GPIO pin 4 (BCM numbering). Adjust the pin number based on your setup.
2. **`one_wire_bus.devices(false)`**: Searches for connected devices on the 1-Wire bus.
3. **`Ds18b20::new(device_address)`**: Creates a DS18B20 instance for the detected device.
4. **`sensor.read_temperature(&mut one_wire_bus)`**: Reads the temperature from the sensor.
5. **`Resolution::Bits12`**: Sets the resolution of the temperature reading (9 to 12 bits). Higher resolution means more accurate readings but slower conversion times.

### Dependencies:
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
onewire = "0.4"
ds18b20 = "0.3"
```

### Notes:
- Ensure the 1-Wire interface is enabled on your Raspberry Pi. You can enable it by adding `dtoverlay=w1-gpio` to `/boot/firmware/config.txt` and rebooting.
- The code assumes the sensor is connected to GPIO pin 4. Adjust the pin number if necessary.
- This example is for a Raspberry Pi, but it can be adapted for other platforms with GPIO and 1-Wire support.
