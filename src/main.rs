#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_rtt_target as _;

mod local_lcd;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use crate::local_lcd::LcdHardware;
    use const_format::{formatcp, str_repeat};
    use core::cmp;
    use core::fmt::Write;
    use hal::gpio::*;
    use hal::pac::TIM3;
    use hal::prelude::*;
    use lcd::*;
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal as hal;
    use systick_monotonic::{ExtU64, Systick};

    type MyLcd = LcdHardware<TIM3>;

    const _RIDDLER: &str = "It's 40 kids to a room. At 12 you're already a \
    drophead, numbing the pain. You wake up to rats, eating your fingers. And \
    every winter one of the babies dies, because it's just so cold.";
    const MY_STR: &str = formatcp!("{}{}", str_repeat!(" ", 16), _RIDDLER);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        display: Display<MyLcd>,
        idx: usize,
    }

    #[monotonic(binds = SysTick, default = true)]
    type Tonic = Systick<1000>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        let gpioa = ctx.device.GPIOA.split();
        let gpiob = ctx.device.GPIOB.split();

        let mono = Systick::new(ctx.core.SYST, 48_000_000);

        let mut display = Display::new(LcdHardware {
            delay: ctx.device.TIM3.delay_us(&clocks),
            rs: gpiob.pb8.into_push_pull_output().into(),
            rw: gpiob.pb9.into_push_pull_output().into(),
            e: gpiob.pb5.into_push_pull_output().into(),
            data4: gpioa.pa5.into_push_pull_output().into(),
            data5: gpioa.pa6.into_push_pull_output().into(),
            data6: gpioa.pa7.into_push_pull_output().into(),
            data7: gpioa.pa8.into_push_pull_output().into(),
        });
        display.init(FunctionLine::Line2, FunctionDots::Dots5x8);
        display.display(
            DisplayMode::DisplayOn,
            DisplayCursor::CursorOff,
            DisplayBlink::BlinkOff,
        );
        display.entry_mode(
            EntryModeDirection::EntryRight,
            EntryModeShift::NoShift,
        );
        let idx = 0;

        run_display::spawn().ok();

        (Shared {}, Local { display, idx }, init::Monotonics(mono))
    }

    #[task(local = [display, idx], priority = 4)]
    fn run_display(ctx: run_display::Context) {
        let idx = *ctx.local.idx;
        ctx.local.display.position(0, 0);
        write!(
            ctx.local.display,
            "{: <16}",
            &MY_STR[idx..cmp::min(idx + 16, MY_STR.len())]
        )
        .unwrap();
        if idx >= MY_STR.len() {
            *ctx.local.idx = 0;
        } else {
            *ctx.local.idx += 1;
        }
        run_display::spawn_after(200u64.millis()).ok();
    }
}
