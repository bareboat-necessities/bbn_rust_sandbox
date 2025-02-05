### Steps:
1. Connect the DS18B20 sensor to your Raspberry Pi or microcontroller.
2. Ensure the 1-Wire interface is enabled on your Raspberry Pi.

### Notes:
- Ensure the 1-Wire interface is enabled on your Raspberry Pi. You can enable it by adding `dtoverlay=w1-gpio` to `/boot/firmware/config.txt` and rebooting.

### See:

https://github.com/awendland/rpi-ds18b20-rust
