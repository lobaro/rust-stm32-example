# Rust STM32 Bootstrap example

The goal of this repo is to help everyone to get started with RustLang on a STM32 micro controller.
There is a lot of progress in the community but the initial setup is still painful.

Scope of the project:
* Linker file
* Relevant IO Mappings to configure and toggle IO-Pins
* Main function for a blinking LED

The codebase does not make any assumption about your eval board. We like to collect linker files and IO mappings for as many architectures as possible to enable everybody to get started with rust in the embedded world.

**If you have any problems running this code please file an git issue.**

Useful sources:
* http://shadetail.com/blog/rust-on-embedded-starting-up/
* http://www.acrawford.com/2017/03/09/rust-on-the-cortex-m3.html
* http://blog.japaric.io/quickstart/
* https://users.rust-lang.org/t/rust-for-embedded-development-where-we-are-and-whats-missing/10861

# Usage

To get started you need **rustup** from https://rustup.rs/

Please follow the instruction to install the "Rust Visual C++ prerequisites" carefully! 

Verify the installation using ``rustup show``

Add the nightly toolchain:

    rustup toolchain add nightly
    
Add rust sources:

    rustup component add rust-src
    
Use the nightly builds:
    
    rustup override set nightly

**Cross-compiling using Xargo**

Xargo is a tool to help with cross-compiling in Rust. Xargo is a wrapper around cargo that handles compiling Rust’s core libraries for your target.
 
 Install Xargo:
 
    cargo install xargo
    
To verify the installation run

    xargo --version
    
The output should show something like this:

    xargo 0.3.10
    cargo 0.25.0-nightly (e08f31018 2017-12-24)

Navigate to the subdir for your architecture and build the project with

    xargo build

## Useful commands

Get an object dump to have a look at the compiled result:

    arm-none-eabi-objdump -d target/thumbv7m-none-eabi/debug/STM32L15x

Convert the binary output file into hex format (might be easier to flash, depending on your tools)

    arm-none-eabi-objcopy -Oihex ./target/thumbv7m-none-eabi/debug/STM32L15x ./STM32L15x.hex

# Supported hardware
**STM32 Cortex-M3**

* STM32L151CB-A


# Contribute
We need help to collect linker files and memory mappings for as many Chips as possible. Any contribution is welcome.
