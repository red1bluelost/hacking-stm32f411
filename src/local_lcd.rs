use stm32f4xx_hal as hal;
use hal::hal::digital::v2::OutputPin;
use hal::prelude::*;
use hal::timer::DelayUs;
use hal::timer::Instance;

pub struct LcdHardware<
    TIM: Instance,
    RS: OutputPin,
    RW: OutputPin,
    E: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
> {
    pub delay: DelayUs<TIM>,
    pub rs: RS,
    pub rw: RW,
    pub e: E,
    pub data4: D4,
    pub data5: D5,
    pub data6: D6,
    pub data7: D7,
}

impl<
        TIM: Instance,
        RS: OutputPin,
        RW: OutputPin,
        E: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > lcd::Hardware for LcdHardware<TIM, RS, RW, E, D4, D5, D6, D7>
{
    fn rs(&mut self, bit: bool) {
        self.rs.set_state(bit.into()).ok();
    }

    fn enable(&mut self, bit: bool) {
        self.e.set_state(bit.into()).ok();
    }

    fn data(&mut self, data: u8) {
        let b = |n| (data & (1u8 << n) != 0u8).into();
        self.data4.set_state(b(0)).ok();
        self.data5.set_state(b(1)).ok();
        self.data6.set_state(b(2)).ok();
        self.data7.set_state(b(3)).ok();
    }
}

impl<
        TIM: Instance,
        RS: OutputPin,
        RW: OutputPin,
        E: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > lcd::Delay for LcdHardware<TIM, RS, RW, E, D4, D5, D6, D7>
{
    fn delay_us(&mut self, delay_usec: u32) {
        self.delay.delay(delay_usec.micros());
    }
}
