#!/bin/bash
# $1 — это путь к бинарнику, который передаст Cargo

# 1. Извлекаем HEX из ELF
avr-objcopy -O ihex -R .eeprom "$1" main.hex

# 2. Шьем через avrdude
avrdude -q -V -p atmega328p -c arduino -b 115200 -P /dev/ttyUSB0 -D -U flash:w:main.hex:i
