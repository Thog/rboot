#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(underscore_const_names)]

#[macro_use]
extern crate enum_primitive;

#[macro_use]
extern crate log;

extern crate register;

#[macro_use]
extern crate static_assertions;

extern crate cortex_a;

use cortex_a::regs::*;

pub mod logger;

pub mod rt;
pub mod tegra210;

use core::fmt::Write;

use tegra210::board;
use tegra210::*;

use log::Level;

const APB: *const apb::AMBAPeripheralBus = 0x7000_0000 as *const apb::AMBAPeripheralBus;

entry!(main);

fn pinmux_init() {
    // clear clamping
    unsafe { (*APB).misc.pp.PINMUX_GLOBAL.set(0) };

    // configure GPIO
    for entry in board::p2371_2180::GPIO_CONFIG.iter() {
        entry.0.config(entry.1);
    }

    // configure PINGRP
    for entry in board::p2371_2180::PINGRP_CONFIG.iter() {
        entry.0.config(
            entry.1, entry.2, entry.3, entry.4, entry.5, entry.6, entry.7,
        );
    }

    // TODO: configure DRVCFG
}

extern "C" {
    static mut _sbss: u8;
    static mut _ebss: u8;
    static _stack_top: u8;
}

fn log_init() {
    let mut uart_a = &mut uart::UART::A;
    uart_a.init(115_200);
    logger::init(logger::Type::A, Level::Trace);
}

fn main() {
    pinmux_init();

    log_init();

    let mut uart_a = &mut uart::UART::A;
    write!(&mut uart_a, "Executing in EL: ");
    uart_a.put_u32(CurrentEL.read(CurrentEL::EL));
    uart_a.put_char(0xD);
    uart_a.put_char(0xA);

    //info!("Hello");
    // FIXME: core::fmt::Argument seems pretty broken??? Are we breaking the stack?!
    //core::fmt::write(&mut uart::UART::A, format_args!("example {:x} test {:x} words {:p}\r\n", 0xFF, 2, APB));
    //core::fmt::write(&mut uart::UART::A, format_args!("example {:x} test {:x} words {:p}\r\n", 0xFE, 5, APB));
    //core::fmt::write(&mut uart::UART::A, format_args!("example {:.1} test {:x} words {:p}\r\n", 0xFF, 0xDE, APB));
}
