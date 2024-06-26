{% if microcontroller_family == "esp" -%}
#![feature(type_alias_impl_trait)]
{% else -%}
#![no_std]
#![no_main]
{% endif %}

mod vial;

use rmk::macros::rmk_keyboard;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

#[rmk_keyboard]
mod keyboard {}
