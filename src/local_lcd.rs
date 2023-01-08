use hal::gpio::ErasedPin;
use hal::gpio::Output;
use hal::prelude::*;
use hal::timer::DelayUs;
use hal::timer::Instance;
use stm32f4xx_hal as hal;

pub struct LcdHardware<TIM: Instance> {
    pub delay: DelayUs<TIM>,
    pub rs: ErasedPin<Output>,
    pub rw: ErasedPin<Output>,
    pub e: ErasedPin<Output>,
    pub data4: ErasedPin<Output>,
    pub data5: ErasedPin<Output>,
    pub data6: ErasedPin<Output>,
    pub data7: ErasedPin<Output>,
}

impl<TIM: Instance> lcd::Hardware for LcdHardware<TIM> {
    fn rs(&mut self, bit: bool) {
        self.rs.set_state(bit.into());
    }

    fn enable(&mut self, bit: bool) {
        self.e.set_state(bit.into());
    }

    fn data(&mut self, data: u8) {
        let b = |n| (data & (1u8 << n) != 0u8).into();
        self.data4.set_state(b(0));
        self.data5.set_state(b(1));
        self.data6.set_state(b(2));
        self.data7.set_state(b(3));
    }
}

impl<TIM: Instance> lcd::Delay for LcdHardware<TIM> {
    fn delay_us(&mut self, delay_usec: u32) {
        self.delay.delay(delay_usec.micros());
    }
}
