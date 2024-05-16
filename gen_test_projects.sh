#!/usr/bin/expect

# Test stm32
spawn cargo generate --path rmk-template
expect "*Project Name*"
send "stm32h7\n"
expect "*Choose your microcontroller family*"
send "\n"
expect "*Choose your microcontroller's target*"
send "\n"
expect "*Enter your MCU model*"
send "stm32h7b0vb\n"
expect eof

# Test rp2040
spawn cargo generate --path rmk-template
expect "*Project Name*"
send "rp\n"
expect "*Choose your microcontroller family*"
send "\033\[B\n"
expect eof

# Test nrf52840 + usb
spawn cargo generate --path rmk-template
expect "*Project Name*"
send "nrf_usb\n"
expect "*Choose your microcontroller family*"
send "\033\[B\033\[B\n"
expect "*Use BLE(wireless) or USB*"
send "\033\[B\n"
expect "*Choose your nrf MCU model*"
send "\n"
expect eof

# Test nrf52840 + ble
spawn cargo generate --path rmk-template
expect "*Project Name*"
send "nrf52840_ble\n"
expect "*Choose your microcontroller family*"
send "\033\[B\033\[B\n"
expect "*Use BLE(wireless) or USB*"
send "\n"
expect "*Choose your nrf MCU model*"
send "\n"
expect eof

# Test nrf52832 + ble
spawn cargo generate --path rmk-template
expect "*Project Name*"
send "nrf52832_ble\n"
expect "*Choose your microcontroller family*"
send "\033\[B\033\[B\n"
expect "*Use BLE(wireless) or USB*"
send "\n"
expect "*Choose your nrf MCU model*"
send "\033\[B\n"
expect eof

# Test esp32c3
spawn cargo generate --path rmk-template
expect "*Project Name*"
send "esp32c3\n"
expect "*Choose your microcontroller family*"
send "\033\[A\n"
expect "*Use BLE(wireless) or USB*"
send "\n"
expect "*Choose your esp MCU model*"
send "\n"
expect eof

# Test esp32s3
spawn cargo generate --path rmk-template
expect "*Project Name*"
send "esp32s3\n"
expect "*Choose your microcontroller family*"
send "\033\[A\n"
expect "*Use BLE(wireless) or USB*"
send "\n"
expect "*Choose your esp MCU model*"
send "\033\[B\n"
expect eof