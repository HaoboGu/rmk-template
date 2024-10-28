{% if microcontroller_family == "esp" -%}
#![feature(type_alias_impl_trait)]
{% else -%}
#![no_std]
#![no_main]
{% endif %}

use rmk::macros::rmk_keyboard;

#[rmk_keyboard]
mod keyboard {}
