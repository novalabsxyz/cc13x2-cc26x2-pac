#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVTOMCUPOL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED16R {
    bits: u16,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `AUX_TIMER2_PULSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_PULSER {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_PULSER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_TIMER2_PULSER::LOW => true,
            AUX_TIMER2_PULSER::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_TIMER2_PULSER {
        match value {
            true => AUX_TIMER2_PULSER::LOW,
            false => AUX_TIMER2_PULSER::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_PULSER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_PULSER::HIGH
    }
}
#[doc = "Possible values of the field `AUX_TIMER2_EV3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_EV3R {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_EV3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_TIMER2_EV3R::LOW => true,
            AUX_TIMER2_EV3R::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_TIMER2_EV3R {
        match value {
            true => AUX_TIMER2_EV3R::LOW,
            false => AUX_TIMER2_EV3R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV3R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV3R::HIGH
    }
}
#[doc = "Possible values of the field `AUX_TIMER2_EV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_EV2R {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_EV2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_TIMER2_EV2R::LOW => true,
            AUX_TIMER2_EV2R::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_TIMER2_EV2R {
        match value {
            true => AUX_TIMER2_EV2R::LOW,
            false => AUX_TIMER2_EV2R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV2R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV2R::HIGH
    }
}
#[doc = "Possible values of the field `AUX_TIMER2_EV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_EV1R {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_EV1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_TIMER2_EV1R::LOW => true,
            AUX_TIMER2_EV1R::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_TIMER2_EV1R {
        match value {
            true => AUX_TIMER2_EV1R::LOW,
            false => AUX_TIMER2_EV1R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV1R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV1R::HIGH
    }
}
#[doc = "Possible values of the field `AUX_TIMER2_EV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_EV0R {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_EV0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_TIMER2_EV0R::LOW => true,
            AUX_TIMER2_EV0R::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_TIMER2_EV0R {
        match value {
            true => AUX_TIMER2_EV0R::LOW,
            false => AUX_TIMER2_EV0R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV0R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV0R::HIGH
    }
}
#[doc = "Possible values of the field `AUX_ADC_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_ADC_IRQR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_ADC_IRQR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_ADC_IRQR::LOW => true,
            AUX_ADC_IRQR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_ADC_IRQR {
        match value {
            true => AUX_ADC_IRQR::LOW,
            false => AUX_ADC_IRQR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_IRQR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_IRQR::HIGH
    }
}
#[doc = "Possible values of the field `MCU_OBSMUX0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCU_OBSMUX0R {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl MCU_OBSMUX0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MCU_OBSMUX0R::LOW => true,
            MCU_OBSMUX0R::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCU_OBSMUX0R {
        match value {
            true => MCU_OBSMUX0R::LOW,
            false => MCU_OBSMUX0R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == MCU_OBSMUX0R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == MCU_OBSMUX0R::HIGH
    }
}
#[doc = "Possible values of the field `AUX_ADC_FIFO_ALMOST_FULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_ADC_FIFO_ALMOST_FULLR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_ADC_FIFO_ALMOST_FULLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_ADC_FIFO_ALMOST_FULLR::LOW => true,
            AUX_ADC_FIFO_ALMOST_FULLR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_ADC_FIFO_ALMOST_FULLR {
        match value {
            true => AUX_ADC_FIFO_ALMOST_FULLR::LOW,
            false => AUX_ADC_FIFO_ALMOST_FULLR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_FIFO_ALMOST_FULLR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_FIFO_ALMOST_FULLR::HIGH
    }
}
#[doc = "Possible values of the field `AUX_ADC_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_ADC_DONER {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_ADC_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_ADC_DONER::LOW => true,
            AUX_ADC_DONER::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_ADC_DONER {
        match value {
            true => AUX_ADC_DONER::LOW,
            false => AUX_ADC_DONER::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_DONER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_DONER::HIGH
    }
}
#[doc = "Possible values of the field `AUX_SMPH_AUTOTAKE_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_SMPH_AUTOTAKE_DONER {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_SMPH_AUTOTAKE_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_SMPH_AUTOTAKE_DONER::LOW => true,
            AUX_SMPH_AUTOTAKE_DONER::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_SMPH_AUTOTAKE_DONER {
        match value {
            true => AUX_SMPH_AUTOTAKE_DONER::LOW,
            false => AUX_SMPH_AUTOTAKE_DONER::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_SMPH_AUTOTAKE_DONER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_SMPH_AUTOTAKE_DONER::HIGH
    }
}
#[doc = "Possible values of the field `AUX_TIMER1_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER1_EVR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER1_EVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_TIMER1_EVR::LOW => true,
            AUX_TIMER1_EVR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_TIMER1_EVR {
        match value {
            true => AUX_TIMER1_EVR::LOW,
            false => AUX_TIMER1_EVR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER1_EVR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER1_EVR::HIGH
    }
}
#[doc = "Possible values of the field `AUX_TIMER0_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER0_EVR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER0_EVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_TIMER0_EVR::LOW => true,
            AUX_TIMER0_EVR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_TIMER0_EVR {
        match value {
            true => AUX_TIMER0_EVR::LOW,
            false => AUX_TIMER0_EVR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER0_EVR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER0_EVR::HIGH
    }
}
#[doc = "Possible values of the field `AUX_TDC_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TDC_DONER {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TDC_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_TDC_DONER::LOW => true,
            AUX_TDC_DONER::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_TDC_DONER {
        match value {
            true => AUX_TDC_DONER::LOW,
            false => AUX_TDC_DONER::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_TDC_DONER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_TDC_DONER::HIGH
    }
}
#[doc = "Possible values of the field `AUX_COMPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPBR {
    #[doc = "Falling edge"]
    FALL,
    #[doc = "Rising edge"]
    RISE,
}
impl AUX_COMPBR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_COMPBR::FALL => true,
            AUX_COMPBR::RISE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_COMPBR {
        match value {
            true => AUX_COMPBR::FALL,
            false => AUX_COMPBR::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == AUX_COMPBR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == AUX_COMPBR::RISE
    }
}
#[doc = "Possible values of the field `AUX_COMPA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPAR {
    #[doc = "Falling edge"]
    FALL,
    #[doc = "Rising edge"]
    RISE,
}
impl AUX_COMPAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_COMPAR::FALL => true,
            AUX_COMPAR::RISE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_COMPAR {
        match value {
            true => AUX_COMPAR::FALL,
            false => AUX_COMPAR::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == AUX_COMPAR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == AUX_COMPAR::RISE
    }
}
#[doc = "Possible values of the field `AUX_WU_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_WU_EVR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_WU_EVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_WU_EVR::LOW => true,
            AUX_WU_EVR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_WU_EVR {
        match value {
            true => AUX_WU_EVR::LOW,
            false => AUX_WU_EVR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_WU_EVR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_WU_EVR::HIGH
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED16W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_TIMER2_PULSE`"]
pub enum AUX_TIMER2_PULSEW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_PULSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_TIMER2_PULSEW::LOW => true,
            AUX_TIMER2_PULSEW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_PULSEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_PULSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_TIMER2_PULSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_PULSEW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_PULSEW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_TIMER2_EV3`"]
pub enum AUX_TIMER2_EV3W {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_EV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_TIMER2_EV3W::LOW => true,
            AUX_TIMER2_EV3W::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_EV3W<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_EV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_TIMER2_EV3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV3W::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV3W::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_TIMER2_EV2`"]
pub enum AUX_TIMER2_EV2W {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_EV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_TIMER2_EV2W::LOW => true,
            AUX_TIMER2_EV2W::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_EV2W<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_EV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_TIMER2_EV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV2W::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV2W::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_TIMER2_EV1`"]
pub enum AUX_TIMER2_EV1W {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_EV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_TIMER2_EV1W::LOW => true,
            AUX_TIMER2_EV1W::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_EV1W<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_EV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_TIMER2_EV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV1W::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV1W::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_TIMER2_EV0`"]
pub enum AUX_TIMER2_EV0W {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER2_EV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_TIMER2_EV0W::LOW => true,
            AUX_TIMER2_EV0W::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_EV0W<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_EV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_TIMER2_EV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV0W::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV0W::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_ADC_IRQ`"]
pub enum AUX_ADC_IRQW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_ADC_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_ADC_IRQW::LOW => true,
            AUX_ADC_IRQW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_ADC_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_ADC_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_ADC_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_IRQW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_IRQW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCU_OBSMUX0`"]
pub enum MCU_OBSMUX0W {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl MCU_OBSMUX0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCU_OBSMUX0W::LOW => true,
            MCU_OBSMUX0W::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCU_OBSMUX0W<'a> {
    w: &'a mut W,
}
impl<'a> _MCU_OBSMUX0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCU_OBSMUX0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(MCU_OBSMUX0W::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(MCU_OBSMUX0W::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_ADC_FIFO_ALMOST_FULL`"]
pub enum AUX_ADC_FIFO_ALMOST_FULLW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_ADC_FIFO_ALMOST_FULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_ADC_FIFO_ALMOST_FULLW::LOW => true,
            AUX_ADC_FIFO_ALMOST_FULLW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_ADC_FIFO_ALMOST_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_ADC_FIFO_ALMOST_FULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_ADC_FIFO_ALMOST_FULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_FIFO_ALMOST_FULLW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_FIFO_ALMOST_FULLW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_ADC_DONE`"]
pub enum AUX_ADC_DONEW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_ADC_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_ADC_DONEW::LOW => true,
            AUX_ADC_DONEW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_ADC_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_ADC_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_ADC_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_DONEW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_DONEW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_SMPH_AUTOTAKE_DONE`"]
pub enum AUX_SMPH_AUTOTAKE_DONEW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_SMPH_AUTOTAKE_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_SMPH_AUTOTAKE_DONEW::LOW => true,
            AUX_SMPH_AUTOTAKE_DONEW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_SMPH_AUTOTAKE_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_SMPH_AUTOTAKE_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_SMPH_AUTOTAKE_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_SMPH_AUTOTAKE_DONEW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_SMPH_AUTOTAKE_DONEW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_TIMER1_EV`"]
pub enum AUX_TIMER1_EVW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER1_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_TIMER1_EVW::LOW => true,
            AUX_TIMER1_EVW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_TIMER1_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER1_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_TIMER1_EVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER1_EVW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER1_EVW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_TIMER0_EV`"]
pub enum AUX_TIMER0_EVW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TIMER0_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_TIMER0_EVW::LOW => true,
            AUX_TIMER0_EVW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_TIMER0_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER0_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_TIMER0_EVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER0_EVW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER0_EVW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_TDC_DONE`"]
pub enum AUX_TDC_DONEW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_TDC_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_TDC_DONEW::LOW => true,
            AUX_TDC_DONEW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_TDC_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TDC_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_TDC_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TDC_DONEW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TDC_DONEW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_COMPB`"]
pub enum AUX_COMPBW {
    #[doc = "Falling edge"]
    FALL,
    #[doc = "Rising edge"]
    RISE,
}
impl AUX_COMPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_COMPBW::FALL => true,
            AUX_COMPBW::RISE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_COMPBW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_COMPBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(AUX_COMPBW::FALL)
    }
    #[doc = "Rising edge"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(AUX_COMPBW::RISE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_COMPA`"]
pub enum AUX_COMPAW {
    #[doc = "Falling edge"]
    FALL,
    #[doc = "Rising edge"]
    RISE,
}
impl AUX_COMPAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_COMPAW::FALL => true,
            AUX_COMPAW::RISE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_COMPAW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_COMPAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(AUX_COMPAW::FALL)
    }
    #[doc = "Rising edge"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(AUX_COMPAW::RISE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_WU_EV`"]
pub enum AUX_WU_EVW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_WU_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_WU_EVW::LOW => true,
            AUX_WU_EVW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_WU_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_WU_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_WU_EVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_WU_EVW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_WU_EVW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED16R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE."]
    #[inline]
    pub fn aux_timer2_pulse(&self) -> AUX_TIMER2_PULSER {
        AUX_TIMER2_PULSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - 14:14\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3."]
    #[inline]
    pub fn aux_timer2_ev3(&self) -> AUX_TIMER2_EV3R {
        AUX_TIMER2_EV3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - 13:13\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2."]
    #[inline]
    pub fn aux_timer2_ev2(&self) -> AUX_TIMER2_EV2R {
        AUX_TIMER2_EV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - 12:12\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1."]
    #[inline]
    pub fn aux_timer2_ev1(&self) -> AUX_TIMER2_EV1R {
        AUX_TIMER2_EV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - 11:11\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0."]
    #[inline]
    pub fn aux_timer2_ev0(&self) -> AUX_TIMER2_EV0R {
        AUX_TIMER2_EV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - 10:10\\] Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
    #[inline]
    pub fn aux_adc_irq(&self) -> AUX_ADC_IRQR {
        AUX_ADC_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - 9:9\\] Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
    #[inline]
    pub fn mcu_obsmux0(&self) -> MCU_OBSMUX0R {
        MCU_OBSMUX0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - 8:8\\] Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
    #[inline]
    pub fn aux_adc_fifo_almost_full(&self) -> AUX_ADC_FIFO_ALMOST_FULLR {
        AUX_ADC_FIFO_ALMOST_FULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - 7:7\\] Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
    #[inline]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONER {
        AUX_ADC_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
    #[inline]
    pub fn aux_smph_autotake_done(&self) -> AUX_SMPH_AUTOTAKE_DONER {
        AUX_SMPH_AUTOTAKE_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
    #[inline]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EVR {
        AUX_TIMER1_EVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
    #[inline]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EVR {
        AUX_TIMER0_EVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
    #[inline]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONER {
        AUX_TDC_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline]
    pub fn aux_compb(&self) -> AUX_COMPBR {
        AUX_COMPBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline]
    pub fn aux_compa(&self) -> AUX_COMPAR {
        AUX_COMPAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV."]
    #[inline]
    pub fn aux_wu_ev(&self) -> AUX_WU_EVR {
        AUX_WU_EVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&mut self) -> _RESERVED16W {
        _RESERVED16W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE."]
    #[inline]
    pub fn aux_timer2_pulse(&mut self) -> _AUX_TIMER2_PULSEW {
        _AUX_TIMER2_PULSEW { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3."]
    #[inline]
    pub fn aux_timer2_ev3(&mut self) -> _AUX_TIMER2_EV3W {
        _AUX_TIMER2_EV3W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2."]
    #[inline]
    pub fn aux_timer2_ev2(&mut self) -> _AUX_TIMER2_EV2W {
        _AUX_TIMER2_EV2W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1."]
    #[inline]
    pub fn aux_timer2_ev1(&mut self) -> _AUX_TIMER2_EV1W {
        _AUX_TIMER2_EV1W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0."]
    #[inline]
    pub fn aux_timer2_ev0(&mut self) -> _AUX_TIMER2_EV0W {
        _AUX_TIMER2_EV0W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
    #[inline]
    pub fn aux_adc_irq(&mut self) -> _AUX_ADC_IRQW {
        _AUX_ADC_IRQW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
    #[inline]
    pub fn mcu_obsmux0(&mut self) -> _MCU_OBSMUX0W {
        _MCU_OBSMUX0W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
    #[inline]
    pub fn aux_adc_fifo_almost_full(&mut self) -> _AUX_ADC_FIFO_ALMOST_FULLW {
        _AUX_ADC_FIFO_ALMOST_FULLW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
    #[inline]
    pub fn aux_adc_done(&mut self) -> _AUX_ADC_DONEW {
        _AUX_ADC_DONEW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
    #[inline]
    pub fn aux_smph_autotake_done(&mut self) -> _AUX_SMPH_AUTOTAKE_DONEW {
        _AUX_SMPH_AUTOTAKE_DONEW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
    #[inline]
    pub fn aux_timer1_ev(&mut self) -> _AUX_TIMER1_EVW {
        _AUX_TIMER1_EVW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
    #[inline]
    pub fn aux_timer0_ev(&mut self) -> _AUX_TIMER0_EVW {
        _AUX_TIMER0_EVW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
    #[inline]
    pub fn aux_tdc_done(&mut self) -> _AUX_TDC_DONEW {
        _AUX_TDC_DONEW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline]
    pub fn aux_compb(&mut self) -> _AUX_COMPBW {
        _AUX_COMPBW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline]
    pub fn aux_compa(&mut self) -> _AUX_COMPAW {
        _AUX_COMPAW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV."]
    #[inline]
    pub fn aux_wu_ev(&mut self) -> _AUX_WU_EVW {
        _AUX_WU_EVW { w: self }
    }
}
