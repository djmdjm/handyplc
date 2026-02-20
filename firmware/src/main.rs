#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use stm32f4xx_hal as hal;
use stm32f4xx_hal::pac::rcc::cfgr::MCO2;

use core::cell::{Cell, RefCell};

use hal::gpio::{ErasedPin, Input, Output, PinState, PushPull, Speed};
use hal::pac;
use hal::pac::interrupt;
use hal::prelude::*;
use hal::rcc::Config;
use hal::timer::{CounterUs, Event};

mod fan;
mod morse;
mod probe;
mod simpletimer;
mod servo_reset;
use fan::FanControl;
use morse::Morse;
use probe::ProbeControl;
use servo_reset::ServoResetControl;
//use simpletimer::SimpleTimer;

static G_NOW: Mutex<Cell<i64>> = Mutex::new(Cell::new(0));
static G_TIM: Mutex<RefCell<Option<CounterUs<pac::TIM5>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM5() {
    // Take static reference to counter.
    static mut TIM: Option<CounterUs<pac::TIM5>> = None;
    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(None).unwrap())
    });

    let mut ms: i64 = 0;
    cortex_m::interrupt::free(|cs| {
        // Update monotonic millisecond counter.
        let now = G_NOW.borrow(cs);
        // Yes, this will fail after a few thousand centuries of uptime.
        // I'll fix it closer to then.
        ms = now.get() + 1;
        if ms == i64::MAX {
            panic!("timer tick overflow");
        }
        now.set(ms);
    });
    let _ = tim.wait();
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let mut rcc = dp
        .RCC
        .freeze(Config::hse(25.MHz()).hclk(25.MHz()).sysclk(100.MHz()));

    let _gpioa = dp.GPIOA.split(&mut rcc);
    let _gpiob = dp.GPIOB.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);
    let gpiod = dp.GPIOD.split(&mut rcc);
    let gpioe = dp.GPIOE.split(&mut rcc);
    let mut _delay = dp.TIM9.delay_us(&mut rcc);
    let mut timer = dp.TIM5.counter_us(&mut rcc);

    // Start the timer to give us a 1kHz clock interrupt.
    timer.start(1.millis()).unwrap();
    timer.listen(Event::Update);
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::TIM5);
    }

    let mut leds: [Option<ErasedPin<Output<PushPull>>>; 3] = [
        Some(gpioc.pc4.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpioc.pc5.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpioc.pc13.into_push_pull_output().speed(Speed::Low).erase()),
    ];

    let mut gp_outputs: [Option<ErasedPin<Output<PushPull>>>; 16] = [
        Some(gpiod.pd0.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd1.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd2.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd3.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd4.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd5.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd6.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd7.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd8.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd9.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd10.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd11.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd12.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd13.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd14.into_push_pull_output().speed(Speed::Low).erase()),
        Some(gpiod.pd15.into_push_pull_output().speed(Speed::Low).erase()),
    ];

    let mut inputs: [Option<ErasedPin<Input>>; 16] = [
        Some(gpioe.pe0.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe1.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe2.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe3.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe4.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe5.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe6.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe7.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe8.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe9.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe10.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe11.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe12.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe13.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe14.internal_pull_down(true).into_input().erase()),
        Some(gpioe.pe15.internal_pull_down(true).into_input().erase()),
    ];

    // Enable to expose sysclk on MCO2.
    if false {
        rcc.cfgr().modify(|_, w| w.mco2().variant(MCO2::Sysclk));
        let _pc9 = gpioc.pc9.into_alternate::<0>().set_speed(Speed::VeryHigh);
    }

    // Spindle fan control.
    let spindle_run = inputs[0].take().unwrap();
    let mut fan_run = gp_outputs[0].take().unwrap();
    let mut fan_control = FanControl::default();
    let mut fan_status_led = leds[0].take().unwrap();
    let mut fan_status_morse = Morse::default();

    // Wireless touch probe control.
    let probe_enable = inputs[1].take().unwrap();
    let probe_alarm = inputs[2].take().unwrap();
    let probe_lowbatt = inputs[3].take().unwrap();
    let mut probe_power = gp_outputs[1].take().unwrap();
    let mut probe_detect = gp_outputs[2].take().unwrap();
    let mut probe_control = ProbeControl::default();
    let mut probe_status_led = leds[1].take().unwrap();
    let mut probe_status_morse = Morse::default();

    // Servo reset signal control.
    let servo_reset_in = inputs[4].take().unwrap();
    let mut servo_reset_out = gp_outputs[3].take().unwrap();
    let mut servo_reset_control = ServoResetControl::default();

    let mut heartbeat = leds[2].take().unwrap();
    let mut now_ms: i64 = 0;
    loop {
        cortex_m::interrupt::free(|cs| {
            now_ms = G_NOW.borrow(cs).get();
        });
        heartbeat.set_state(PinState::from(((now_ms / 2000) & 1) == 0));
        // Fan control FSM.
        fan_control.update(spindle_run.is_high(), now_ms);
        fan_run.set_state(PinState::from(fan_control.fan_state()));
        fan_status_morse.set_char(fan_control.status_char());
        fan_status_morse.update(now_ms);
        fan_status_led.set_state(PinState::from(fan_status_morse.output()));

        // Probe control FSM.
        probe_control.update(
            probe_enable.is_high(),
            probe_alarm.is_high(),
            probe_lowbatt.is_high(),
            now_ms,
        );
        probe_power.set_state(PinState::from(probe_control.probe_power()));
        probe_detect.set_state(PinState::from(probe_control.probe_detect()));
        probe_status_morse.set_char(probe_control.status_char());
        probe_status_morse.update(now_ms);
        probe_status_led.set_state(PinState::from(probe_status_morse.output()));

        // Servo reset control FSM.
        servo_reset_control.update(servo_reset_in.is_high(), now_ms);
        servo_reset_out.set_state(PinState::from(servo_reset_control.reset_state()));
    }
}
