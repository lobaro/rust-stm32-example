#![no_std]
#![no_main]

extern crate device;

// release profile: minimize the binary size of the application
// TODO: We want some custom panic handler that logs & resets
#[cfg(not(debug_assertions))]
extern crate panic_abort;
// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
//#[cfg(debug_assertions)]
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
#[cfg(debug_assertions)]
extern crate panic_semihosting;
// logs messages to the host stderr; requires a debugger

// requires nightly

use core::fmt::Write;
// extern crate panic_halt; // just stop the world
// extern crate panic_itm; // logs messages over ITM; requires ITM support
use core::ptr;

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use cortex_m_rt::{entry, ExceptionFrame};
use cortex_m_semihosting::hio::HStdout;
use cortex_m_semihosting::{debug, hio, hprintln};
use device::interrupt;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    // Does not break in QEMU (try on real HW?)
    trigger_hard_fault();

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    // this is configured for the LM3S6965 which has a default CPU clock of 12 MHz
    syst.set_reload(12_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    loop {
        // your code goes here
    }
}

#[exception]
fn DefaultHandler(irqn: i16) {
    // custom default handler
    // irqn is negative for Cortex-M exceptions
    // irqn is positive for device specific (line IRQ)
    panic!("Exception: {}", irqn);
}

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;
    static mut STDOUT: Option<HStdout> = None;

    *COUNT += 1;

    // Lazy initialization
    if STDOUT.is_none() {
        *STDOUT = hio::hstdout().ok();
    }

    if let Some(hstdout) = STDOUT.as_mut() {
        write!(hstdout, "{}", *COUNT).ok();
    }

    // IMPORTANT omit this `if` block if running on real hardware or your
    // debugger will end in an inconsistent state
    if *COUNT == 9 {
        // This will terminate the QEMU process
        panic!("9 is enough!");
    }
}

// TODO: Not working!!! (at least in QEMU)
fn trigger_hard_fault() {
    // read a nonexistent memory location
    unsafe {
        ptr::read_volatile(0x3FFF_FFFE as *const u32);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }

    loop {}
}

// Interrupt handler for the Timer2 interrupt
#[interrupt]
fn TIM2() {
    // ..
    // Clear reason for the generated interrupt request

    static mut COUNT: u32 = 0;

    // `COUNT` has type `&mut u32` and it's safe to use
    *COUNT += 1;
}
