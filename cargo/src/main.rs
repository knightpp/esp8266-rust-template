#![no_std]
#![no_main]

use esp8266_hal::gpio::{Gpio0, Gpio2, Input, InterruptMode, Output, PullUp, PushPull};
use esp8266_hal::interrupt::{enable_interrupt, InterruptType};
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use esp_println::println;
use panic_halt as _;
use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};

static LED: CriticalSectionMutex<Option<Gpio2<Output<PushPull>>>> = CriticalSectionMutex::new(None);
static BUTTON: CriticalSectionMutex<Option<Gpio0<Input<PullUp>>>> = CriticalSectionMutex::new(None);

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();
    let led = pins.gpio2.into_push_pull_output();
    let mut button = pins.gpio0.into_pull_up_input();
    let (mut timer1, _) = dp.TIMER.timers();

    enable_interrupt(InterruptType::GPIO);
    button.set_interrupt_mode(InterruptMode::NegativeEdge);

    (&LED).lock(|led_locked| *led_locked = Some(led));
    (&BUTTON).lock(|btn| *btn = Some(button));

    loop {
        // NOTE: format args does not work!
        println!("main loop delaying for 1500 ms");
        timer1.delay_ms(1500u16);
    }
}

#[interrupt(gpio)]
fn button() {
    println!("button press interrupt");
    (&LED).lock(|led| led.as_mut().unwrap().toggle().unwrap());
    (&BUTTON).lock(|btn| btn.as_mut().unwrap().clear_interrupt())
}
