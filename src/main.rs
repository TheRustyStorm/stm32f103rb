#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate fun_stm32f103rb;
use fun_stm32f103rb::gpio::*;
use fun_stm32f103rb::rcc::*;
use fun_stm32f103rb::timer::*;

use core::ptr;
use cortex_m_rt::entry;

fn delay_ms(timer: &TIM6_7, n: u32) {
    timer.set(TIM6_7_REGISTER::ARR, n);
    timer.set(TIM6_7_REGISTER::CR1, 9);

    //*((self.BASE + offset) as *mut u32) = pattern;
    unsafe {
        let mut ready = false;
        while !ready {
            let old = ptr::read_volatile(0x40001010 as *mut u32);
            if (old & 1) == 1{
                ready = true;
            }
        }
    }
    timer.set(TIM6_7_REGISTER::SR, 0);
}

#[entry]
fn main() -> ! {
    let rcc = RCC::new();
    let gpioa = GPIO::new(GPIO_PORT::A);

    rcc.set(RCC_REGISTER::APB2ENR, 1 << 2);
    rcc.set(RCC_REGISTER::APB1ENR, 1 << 4);
    gpioa.set(GPIO_REGISTER::CRL, 1 << 21);

    let tim6 = TIM6_7::new(TIM6_7_TYPE::TIM6);

    tim6.set(TIM6_7_REGISTER::CR1, 1 << 3);
    tim6.set(TIM6_7_REGISTER::PSC, 7_999);

    // infinite loop; just so we don't leave this stack frame
    loop {
        gpioa.set_bit(GPIO_REGISTER::BSRR, 5);
        delay_ms(&tim6, 500);

        gpioa.set_bit(GPIO_REGISTER::BSRR, 16 + 5);
        delay_ms(&tim6, 500);
    }
}
