//! This example demonstrates how to use interrupts to read and write UART (serial)
//! data. We take advantage of global static Mutexes as buffers that can be accessed
//! from interrupt concept to read and write data as it's ready, allowing the CPU to
//! perform other tasks while waiting.

#![no_main]
#![no_std]

use core::cell::{Cell, RefCell};

use cortex_m::{
    interrupt::{self, free, Mutex},
    peripheral::NVIC,
};
use cortex_m_rt::entry;

use stm32_hal2::{
    clocks::Clocks,
    gpio::{AltFn, Edge, GpioA, GpioAPin, PinMode, PinNum},
    low_power, pac,
    usart::{Usart, UsartConfig, UsartDevice, UsartInterrupt},
};

const BUF_SIZE: usize = 10;

// Set up static global variables, for sharing state between interrupt contexts and the main loop.
// Initialize `UART` to `NONE`, since we need to declare the global before setting up the peripheral.
static UART: Mutex<RefCell<Option<Usart<pac::USART1>>>> = Mutex::new(RefCell::new(None));
static READ_BUF: Mutex<RefCell<[u8; BUF_SIZE]>> = Mutex::new(RefCell::new([0; BUF_SIZE]));
static READ_I: Mutex<Cell<usize>> = Mutex::new(Cell::new(0));

#[entry]
fn main() -> ! {
    // Set up CPU peripherals
    let mut cp = cortex_m::Peripherals::take().unwrap();
    // Set up microcontroller peripherals
    let mut dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();
    clock_cfg.setup(&mut dp.RCC, &mut dp.FLASH).unwrap();

    // Set up the GPIOA port.
    let mut gpioa = GpioA::new(dp.GPIOA, &mut dp.RCC);

    // Configure pins for UART, according to the user manual.
    let _uart_tx = gpioa.new_pin(PinNum::P9, PinMode::Alt(AltFn::Af7));
    let _uart_rx = gpioa.new_pin(PinNum::P10, PinMode::Alt(AltFn::Af7));

    // Set up the USART1 peripheral.
    let uart = Usart::new(
        dp.USART1,
        UsartDevice::One,
        9_600,
        UsartConfig::default(),
        &clock_cfg,
        &mut dp.RCC,
    );

    unsafe {
        // Unmask interrupt lines associated with the USART1.
        NVIC::unmask(interrupt::USART1);
    }

    free(|cs| {
        // Now that we've initialized the USART peripheral, make it global.
        UART.borrow(cs).replace(Some(uart));
    });

    loop {
        low_power::sleep_now(&mut SCB);
    }
}

#[interrupt]
/// Non-blocking USART read interrupt handler; read to a global buffer one byte
/// at a time as we receive them.
fn USART1() {
    free(|cs| {
        let mut u = UART.borrow(cs).borrow_mut();
        let uart = u.as_mut().unwrap();

        // Clear the interrupt flag, to prevent this ISR from repeatedly firing
        uart.clear_interrupt(UsartInterrupt::ReadNotEmpty);

        let mut buf = READ_BUF.borrow(cs).borrow_mut();

        let i = READ_I.borrow(cs);
        let i_val = i.get();
        if i_val == BUF_SIZE {
            // todo: signal end of read.
        }

        buf[i_val] = uart.read_one();
        i.set(i_val + 1);
    });
}
