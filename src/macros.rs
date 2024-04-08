{% if microcontroller_family == "rp2040" -%}
macro_rules! config_matrix_pins_rp {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Output::new(AnyPin::from($p.$out_pin), embassy_rp::gpio::Level::Low)), +];
            let input_pins = [$(Input::new(AnyPin::from($p.$in_pin), embassy_rp::gpio::Pull::Down)), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}
{% elsif microcontroller_family == "stm32" -%}
macro_rules! config_matrix_pins_stm32 {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Output::new($p.$out_pin, embassy_stm32::gpio::Level::Low, embassy_stm32::gpio::Speed::VeryHigh).degrade()), +];
            let input_pins = [$(Input::new($p.$in_pin, embassy_stm32::gpio::Pull::Down).degrade()), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}
{% elsif microcontroller_family == "nrf" -%}
macro_rules! config_matrix_pins_nrf {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Output::new(AnyPin::from($p.$out_pin), embassy_nrf::gpio::Level::Low, embassy_nrf::gpio::OutputDrive::Standard)), +];
            let input_pins = [$(Input::new(AnyPin::from($p.$in_pin), embassy_nrf::gpio::Pull::Down)), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}
{% elsif microcontroller_family == "esp" -%}
macro_rules! config_matrix_pins_esp {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(PinDriver::output($p.pins.$out_pin.downgrade_output()).unwrap()), +];
            let input_pins = [$(PinDriver::input($p.pins.$in_pin.downgrade_input()).unwrap()), +];
            output_pins.iter_mut().for_each(|p| {
                let _ = p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}
{% endif -%}
