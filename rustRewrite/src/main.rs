#![no_std]
#![no_main]
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Executor;
use embassy_executor::Spawner;
use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::{bind_interrupts, interrupt};
use embassy_time::Timer;
use panic_probe as _;
use static_cell::StaticCell;

bind_interrupts!(
    pub struct Irqs{
        EXTI15_10 => exti::InterruptHandler<interrupt::typelevel::EXTI15_10>;
});

#[embassy_executor::task]
async fn led_button() {
    let p = embassy_stm32::init(Default::default());

    // let mut user_button = Input::new(p.PC13, Pull::Up);
    let mut user_button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up, Irqs);
    let mut led_green = Output::new(p.PB0, Level::High, Speed::Low);
    let mut led_yellow = Output::new(p.PE1, Level::High, Speed::Low);
    let mut led_red = Output::new(p.PB14, Level::High, Speed::Low);

    loop {
        user_button.wait_for_rising_edge().await;
        led_green.toggle();
        led_yellow.toggle();
        led_red.toggle();
    }
}

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // #[entry]
    // fn main() -> ! {
    // let p = embassy_stm32::init(Default::default());
    // info!("Hello World!");

    // let executor = EXECUTOR.init(Executor::new());

    // executor.run(|spawner| {
    //     spawner.spawn(unwrap!(led_button()));
    // })
    // loop {
    //     info!("led on!");
    //     led.set_high();pin, pull
    //     Timer::after_millis(1000).await;

    //     info!("led off!");
    //     led.set_low();
    //     Timer::after_millis(1000).await;
    // }
    let p = embassy_stm32::init(Default::default());

    // let mut user_button = Input::new(p.PC13, Pull::Up);
    let mut user_button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up, Irqs);
    let mut led_green = Output::new(p.PB0, Level::High, Speed::Low);
    let mut led_yellow = Output::new(p.PE1, Level::High, Speed::Low);
    let mut led_red = Output::new(p.PB14, Level::High, Speed::Low);

    loop {
        user_button.wait_for_rising_edge().await;
        led_green.toggle();
        led_yellow.toggle();
        led_red.toggle();
    }
}
