### Explanation:
1. **Dependencies**:
   - `linux-embedded-hal`: Provides I2C communication for Linux-based systems.
   - `bmp280`: A Rust driver for the BMP280 sensor.

2. **Code**:
   - The I2C device is initialized using `/dev/i2c-1`, which is the default I2C bus on many Linux systems (e.g., Raspberry Pi).
   - The BMP280 sensor is initialized using the `BMP280::new_primary` function.
   - The sensor is initialized with `bmp280.init()`.
   - In the loop, the pressure is read using `bmp280.read_pressure()` and printed in Pascals (Pa).
   - The program waits for 1 second before reading the pressure again.

### Running the Code:
1. Ensure your BMP280 sensor is properly connected to your I2C bus.
2. Run the program using `cargo run`.

### Output:
The program will print the pressure in Pascals (Pa) every second, like this:
```
Pressure: 101325.00 Pa
Pressure: 101324.50 Pa
...
```
