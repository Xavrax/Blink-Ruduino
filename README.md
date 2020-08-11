# Blink-Ruduino

Simple blinking program for arduino uno written in rust.

# Build & Upload

```bash
# build program
cargo build -Z build-std=core --target avr-atmega328p.json --release

# make .hex file
avr-objcopy -j .text -j .data -O ihex avr-test.elf project.hex

# uploud to arduino
/usr/bin/avrdude -C/etc/avrdude.conf -v -patmega328p -cusbasp -Pusb -U flash:w:project.hex
```
