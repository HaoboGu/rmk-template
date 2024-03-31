#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod keymap;
mod vial;

use crate::keymap::{COL, NUM_LAYER, ROW};
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    flash::{Blocking, Flash},
    gpio::{AnyPin, Input, Output},
    peripherals::USB_OTG_HS,
    usb_otg::{Driver, InterruptHandler},
    Config,
};
use panic_probe as _;
use rmk::{
    config::{RmkConfig, VialConfig},
    initialize_keyboard_with_config_and_run,
};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

// TODO: USB interrupt binding varies according to the usage and chip.
// Check the example implementation of your microcontroller at: https://github.com/embassy-rs/embassy/blob/main/examples
bind_interrupts!(struct Irqs {
    OTG_HS => InterruptHandler<USB_OTG_HS>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Rmk start!");
    // RCC config
    let config = Config::default();

    // Initialize peripherals
    let p = embassy_stm32::init(config);

    // Usb config
    static EP_OUT_BUFFER: StaticCell<[u8; 1024]> = StaticCell::new();
    let mut usb_config = embassy_stm32::usb_otg::Config::default();
    usb_config.vbus_detection = false;
    let driver = Driver::new_fs(
        p.USB_OTG_HS,
        Irqs,
        p.PA12,
        p.PA11,
        &mut EP_OUT_BUFFER.init([0; 1024])[..],
        usb_config,
    );

    // Pin config
    // TODO: Use your own pin matrix config
    let (input_pins, output_pins) = config_matrix_pins_stm32!(peripherals: p, input: [PD9, PD8, PB13, PB12], output: [PE13, PE14, PE15]);

    // Use internal flash to emulate eeprom
    let f = Flash::new_blocking(p.FLASH);

    // RMK configurations
    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF);
    let keyboard_config = RmkConfig {
        vial_config,
        ..Default::default()
    };

    // Start serving
    initialize_keyboard_with_config_and_run::<
        Flash<'_, Blocking>,
        Driver<'_, USB_OTG_HS>,
        Input<'_, AnyPin>,
        Output<'_, AnyPin>,
        ROW,
        COL,
        NUM_LAYER,
    >(
        driver,
        input_pins,
        output_pins,
        Some(f),
        crate::keymap::KEYMAP,
        keyboard_config,
    )
    .await;
}
