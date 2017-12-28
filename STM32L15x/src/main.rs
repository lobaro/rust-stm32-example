#![feature(asm, lang_items, naked_functions)]
#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]

use hw::VolatileStruct;

extern crate compiler_builtins;

pub mod lang_items;
pub mod exceptions;
pub mod hw;

const PERIPH_BASE: u32 = 0x40000000;
const AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x20000;
const GPIOA_BASE: u32 = AHBPERIPH_BASE + 0x0000;
const RCC_BASE: u32 = AHBPERIPH_BASE + 0x3800;

fn set_led(gpio: &mut hw::GPIO, on: bool) {
    set_gpio(gpio, 1, on)
}

fn set_gpio(gpio: &mut hw::GPIO, pin: u8, state: bool) {
    if state {
        gpio.bssrl |= 1 << pin;
    } else {
        gpio.bssrh |= 1 << pin;
    }
}


#[no_mangle]
pub fn main() {
    let rcc = unsafe { hw::RCC::from_addr(RCC_BASE) };
    let gpio_a = unsafe { hw::GPIO::from_addr(GPIOA_BASE) };


    rcc.ahbenr.write(1);
    gpio_a.moder.write(4);


    loop {
        for _ in 0..2_00 {
            unsafe { asm!("nop") }
        }
        set_led(gpio_a, true);
        for _ in 0..2_00 {
            unsafe { asm!("nop") }
        }
        set_led(gpio_a, false);
    }
}



