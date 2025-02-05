### Steps:
1. Connect the DS18B20 sensor to your Raspberry Pi or microcontroller.
2. Ensure the 1-Wire interface is enabled on your Raspberry Pi.

### Notes:
- Ensure the 1-Wire interface is enabled on your Raspberry Pi. You can enable it by adding `dtoverlay=w1-gpio` to `/boot/firmware/config.txt` and rebooting.
- The code assumes the sensor is connected to GPIO pin 4. Adjust the pin number if necessary.
- This example is for a Raspberry Pi, but it can be adapted for other platforms with GPIO and 1-Wire support.
