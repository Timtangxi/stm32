//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

fn delay(x: i64){
    let mut number: i64 = x;
    while number != 0 {
        number -= 1;
    }
}

#[entry]
fn main() -> ! {
    hprintln!("Hello, worldsss!").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    let mut count_num: i32 = 0;
    loop {
        hprintln!("Hello, world!{}",count_num).unwrap();
        delay(72000000);  // 1s
        count_num += 1;
    }
}
