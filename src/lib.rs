#![cfg_attr(not(test), no_std)]

use embedded_hal::digital::v2::{OutputPin, PinState};
use core::fmt::Debug;
use embedded_hal::digital::v2::InputPin;

#[derive(Debug)]
pub enum Error<E> {
    DigitNotFound,
    GPIO(E),
}

pub struct LED3461BS1<
    'a,
    GPIOa,
    GPIOb,
    GPIOc,
    GPIOd,
    GPIOe,
    GPIOf,
    GPIOg,
    GPIODp,
    GPIOD1,
    GPIOD2,
    GPIOD3,
    GPIOD4,
> {
    a: &'a mut GPIOa,
    b: &'a mut GPIOb,
    c: &'a mut GPIOc,
    d: &'a mut GPIOd,
    e: &'a mut GPIOe,
    f: &'a mut GPIOf,
    g: &'a mut GPIOg,
    dp: &'a mut GPIODp,
    d1: &'a mut GPIOD1,
    d2: &'a mut GPIOD2,
    d3: &'a mut GPIOD3,
    d4: &'a mut GPIOD4,
}

impl<
        'a,
        GPIOa,
        GPIOb,
        GPIOc,
        GPIOd,
        GPIOe,
        GPIOf,
        GPIOg,
        GPIODp,
        GPIOD1,
        GPIOD2,
        GPIOD3,
        GPIOD4,
        E,
    >
    LED3461BS1<
        'a,
        GPIOa,
        GPIOb,
        GPIOc,
        GPIOd,
        GPIOe,
        GPIOf,
        GPIOg,
        GPIODp,
        GPIOD1,
        GPIOD2,
        GPIOD3,
        GPIOD4,
    >
where
    GPIOa: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOb: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOc: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOd: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOe: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOf: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOg: InputPin<Error = E> + OutputPin<Error = E>,
    GPIODp: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOD1: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOD2: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOD3: InputPin<Error = E> + OutputPin<Error = E>,
    GPIOD4: InputPin<Error = E> + OutputPin<Error = E>,

    E: Debug,
{
    pub fn new(
        a: &'a mut GPIOa,
        b: &'a mut GPIOb,
        c: &'a mut GPIOc,
        d: &'a mut GPIOd,
        e: &'a mut GPIOe,
        f: &'a mut GPIOf,
        g: &'a mut GPIOg,
        dp: &'a mut GPIODp,
        d1: &'a mut GPIOD1,
        d2: &'a mut GPIOD2,
        d3: &'a mut GPIOD3,
        d4: &'a mut GPIOD4,
    ) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            dp,
            d1,
            d2,
            d3,
            d4,
        }
    }

    pub fn clear(&mut self) -> Result<(), Error<E>> {
        self.d1.set_low().unwrap();
        self.d2.set_low().unwrap();
        self.d3.set_low().unwrap();
        self.d4.set_low().unwrap();
        self.a.set_low().unwrap();
        self.b.set_low().unwrap();
        self.c.set_low().unwrap();
        self.d.set_low().unwrap();
        self.e.set_low().unwrap();
        self.f.set_low().unwrap();
        self.g.set_low().unwrap();
        self.dp.set_low().unwrap();
        Ok(())
    }

    pub fn set_custom_character(
        &mut self,
        digit: char,
        seg_a: bool,
        seg_b: bool,
        seg_c: bool,
        seg_d: bool,
        seg_e: bool,
        seg_f: bool,
        seg_g: bool,
        seg_dp: bool,
    ) -> Result<(), Error<E>> {
        match digit {
            '1' => {
                self.d1.set_high().unwrap();
            }
            '2' => {
                self.d2.set_high().unwrap();
            }
            '3' => {
                self.d3.set_high().unwrap();
            }
            '4' => {
                self.d4.set_high().unwrap();
            }
            _ => {
                return Err(Error::DigitNotFound);
            }
        }

        self.a.set_state(PinState::from(!seg_a)).unwrap();
        self.b.set_state(PinState::from(!seg_b)).unwrap();
        self.c.set_state(PinState::from(!seg_c)).unwrap();
        self.d.set_state(PinState::from(!seg_d)).unwrap();
        self.e.set_state(PinState::from(!seg_e)).unwrap();
        self.f.set_state(PinState::from(!seg_f)).unwrap();
        self.g.set_state(PinState::from(!seg_g)).unwrap();
        self.dp.set_state(PinState::from(!seg_dp)).unwrap();

        Ok(())
    }

    pub fn set_number_on_digit(&mut self, digit: char, number: char) -> Result<(), Error<E>> {
        match digit {
            '1' => {
                self.d1.set_high().unwrap();
            }
            '2' => {
                self.d2.set_high().unwrap();
            }
            '3' => {
                self.d3.set_high().unwrap();
            }
            '4' => {
                self.d4.set_high().unwrap();
            }
            _ => {
                return Err(Error::DigitNotFound);
            }
        }

        match number {
            '0' => {
                self.dp.set_high().unwrap();
                self.g.set_high().unwrap();
            }
            '1' => {
                self.a.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_high().unwrap();
                self.f.set_high().unwrap();
                self.g.set_high().unwrap();
                self.dp.set_high().unwrap();
            }
            '2' => {
                self.dp.set_high().unwrap();
                self.f.set_high().unwrap();
                self.c.set_high().unwrap();
            }
            '3' => {
                self.f.set_high().unwrap();
                self.e.set_high().unwrap();
                self.dp.set_high().unwrap();
            }
            '4' => {
                self.a.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_high().unwrap();
                self.dp.set_high().unwrap();
            }
            '5' => {
                self.b.set_high().unwrap();
                self.e.set_high().unwrap();
                self.dp.set_high().unwrap();
            }
            '6' => {
                self.b.set_high().unwrap();
                self.dp.set_high().unwrap();
            }
            '7' => {
                self.dp.set_high().unwrap();
                self.g.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_high().unwrap();
                self.f.set_high().unwrap();
            }
            '8' => {
                self.dp.set_high().unwrap();
            }
            '9' => {
                self.e.set_high().unwrap();
                self.dp.set_high().unwrap();
            }
            _ => {
                // Set everything high
                self.a.set_high().unwrap();
                self.b.set_high().unwrap();
                self.c.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_high().unwrap();
                self.f.set_high().unwrap();
                self.g.set_high().unwrap();
                self.dp.set_high().unwrap();
            }
        }

        Ok(())
    }
}
