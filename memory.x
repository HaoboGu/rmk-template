{% if microcontroller_family == "stm32" -%}
/* TODO: Update this link script for your microcontroller */
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K 
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
{% elsif microcontroller_family == "rp2040" -%}
MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    /* ### Boot loader */
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;
{% elsif chip == "nrf52832_xxAA" -%}
MEMORY
{
  /* These values correspond to the nRF52832 with Softdevices S132 7.3.0 */
  FLASH : ORIGIN = 0x26000,    LENGTH = 360K
  RAM : ORIGIN = 0x20007af8, LENGTH = 33K
}
{% elsif chip == "nrf52840_xxAA" -%34056}
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 256K */

  /* These values correspond to the nRF52840 with Softdevices S140 7.3.0 */
  FLASH : ORIGIN = 0x00027000, LENGTH = 868K
  RAM : ORIGIN = 0x20020000, LENGTH = 128K
}
{% elsif microcontroller_family == "nrf" -%}
/* TODO: Update this link script for your microcontroller */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 256K

  /* These values correspond to the NRF52840 with Softdevices S140 7.3.0 */
  /* FLASH : ORIGIN = 0x00027000, LENGTH = 868K
  RAM : ORIGIN = 0x20020000, LENGTH = 128K */
}
{% endif -%}