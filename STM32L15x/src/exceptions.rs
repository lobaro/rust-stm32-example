#[doc(hidden)]
#[export_name = "_reset"]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        fn main();
    }
    main();
    loop {}
}

#[doc(hidden)]
#[export_name = "_nmi"]
pub unsafe extern "C" fn nmi() -> ! {
    loop {}
}

#[doc(hidden)]
#[export_name = "_hard_fault"]
pub unsafe extern "C" fn hard_fault() -> ! {
    loop {}
}

#[doc(hidden)]
#[export_name = "_memmanage_fault"]
pub unsafe extern "C" fn memmanage_fault() -> ! {
    loop {}
}

#[doc(hidden)]
#[export_name = "_bus_fault"]
pub unsafe extern "C" fn bus_fault() -> ! {
    loop {}
}

#[doc(hidden)]
#[export_name = "_usage_fault"]
pub unsafe extern "C" fn usage_fault() -> ! {
    loop {}
}

#[doc(hidden)]
#[export_name = "_svcall"]
pub unsafe extern "C" fn svcall() -> ! {
    loop {}
}

#[doc(hidden)]
#[export_name = "_debug"]
pub unsafe extern "C" fn debug() -> ! {
    loop {}
}

#[doc(hidden)]
#[export_name = "_pendsv"]
pub unsafe extern "C" fn pendsv() -> ! {
    loop {}
}

#[doc(hidden)]
#[export_name = "_systick"]
pub unsafe extern "C" fn systick() -> ! {
    loop {}
}

pub type Handler = unsafe extern "C" fn();

#[link_section = ".vector.exceptions"]
#[no_mangle]
pub static EXCEPTION_HANDLERS: [Option<Handler>; 15] = [
    Some(_reset),                  // Reset Handler
    Some(_nmi),                    // Non-maskable interrupt.
    Some(_hard_fault),             // All class of fault.
    Some(_memmanage_fault),        // Memory Management
    Some(_bus_fault),              // Pre-fetch fault, memory access fault.
    Some(_usage_fault),            // Undefined instruction or illegal state.
    None,
    None,
    None,
    None,
    Some(_svcall),                 // System service call via SWI instruction
    Some(_debug),                  // Debug
    None,
    Some(_pendsv),                 // Pendable request for system service
    Some(_systick),                // System tick timer
];


extern "C" {
    pub fn _reset();               // Reset Handler
pub fn _nmi();                 // Non-maskable interrupt.
pub fn _hard_fault();          // All class of fault.
pub fn _memmanage_fault();     // Memory Management
pub fn _bus_fault();           // Pre-fetch fault, memory access fault.
pub fn _usage_fault();         // Undefined instruction or illegal state.
pub fn _svcall();              // System service call via SWI instruction
pub fn _debug();               // Debug
pub fn _pendsv();              // Pendable request for system service
pub fn _systick();             // System tick timer
}
