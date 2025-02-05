### Explanation:
1. **I2C Initialization**: The `I2cdev` struct from `linux-embedded-hal` is used to initialize the I2C device (e.g., `/dev/i2c-1` on a Raspberry Pi).
2. **INA219 Initialization**: The `INA219` struct is initialized with the I2C device and the default address (`0x40`).
3. **Calibration**: The sensor is calibrated using the `Calibration::calibrate` method, which takes the bus voltage range, gain, shunt resistor value, and maximum expected current as inputs.
4. **Reading Values**: The `ina219.read()` method reads the bus voltage, shunt voltage, current, and power values.
5. **Loop**: The program continuously reads and prints the values every second.

### Running the Code
1. Ensure your I2C interface is enabled on your device (e.g., Raspberry Pi).
2. Connect the INA219 sensor to the I2C pins (SDA and SCL).
3. Run the program using `cargo run`.

### Output
The program will print the bus voltage, shunt voltage, current, and power values in the following format:

```
Bus Voltage: 5.12 V, Shunt Voltage: 10.23 mV, Current: 102.30 mA, Power: 523.45 mW
```
