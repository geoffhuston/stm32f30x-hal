use stm32f30x::{TIM1};

use rcc::{APB2, CFGR3};

pub struct AdvancedTimer<TIM> {
    tim: TIM,
    count: u16,
}

pub enum Event {
    Update
}

impl AdvancedTimer<TIM1> {
    pub fn new(tim: TIM1, count: u16, cfgr3: &mut CFGR3, apb2: &mut APB2) -> AdvancedTimer<TIM1> {
        cfgr3.get().write(|w| w.tim1sw().set_bit());

        apb2.enr().write(|w| w.tim1en().set_bit());
        apb2.rstr().write(|w| w.tim1rst().set_bit());
        apb2.rstr().write(|w| w.tim1rst().clear_bit());

        let timer = AdvancedTimer {
            tim,
            count
        };

        // timer.tim.ccmr1_output.write(|w|
        //     unsafe { w.oc1m().bits(0b011) }
        // );

        timer.tim.dier.write(|w| {
            w.uie().set_bit()
        });

        timer.tim.rcr.write(|w| unsafe {
            w.bits(1)
        });

        timer.tim.arr.write(|w| unsafe {
            w.arr().bits(count)
        });

        timer.tim.cr1.write(|w| {
            w.dir().set_bit() // Direction: Downcown
             // .urs().set_bit() // Update Request Source: Only counter overflow/underflow
             .cen().set_bit() // Counter Enable
             .arpe().set_bit() // Counter auto-reload buffer
        });

        timer.tim.egr.write(|w| {
            w.ug().set_bit()
        });

        timer
    }
}