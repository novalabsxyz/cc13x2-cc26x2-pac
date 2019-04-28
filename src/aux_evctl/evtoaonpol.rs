#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVTOAONPOL {
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
pub struct RESERVED9R {
    bits: u32,
}
impl RESERVED9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
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
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: u8,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED9W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 8388607;
        const OFFSET: u8 = 9;
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
        const OFFSET: u8 = 8;
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
        const OFFSET: u8 = 7;
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
        const OFFSET: u8 = 6;
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
        const OFFSET: u8 = 5;
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
        const OFFSET: u8 = 4;
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 9:31 - 31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved9(&self) -> RESERVED9R {
        let bits = {
            const MASK: u32 = 8388607;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED9R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV."]
    #[inline]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EVR {
        AUX_TIMER1_EVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - 7:7\\] Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV."]
    #[inline]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EVR {
        AUX_TIMER0_EVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE."]
    #[inline]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONER {
        AUX_TDC_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE."]
    #[inline]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONER {
        AUX_ADC_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline]
    pub fn aux_compb(&self) -> AUX_COMPBR {
        AUX_COMPBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline]
    pub fn aux_compa(&self) -> AUX_COMPAR {
        AUX_COMPAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:2 - 2:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
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
    #[doc = "Bits 9:31 - 31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved9(&mut self) -> _RESERVED9W {
        _RESERVED9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV."]
    #[inline]
    pub fn aux_timer1_ev(&mut self) -> _AUX_TIMER1_EVW {
        _AUX_TIMER1_EVW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV."]
    #[inline]
    pub fn aux_timer0_ev(&mut self) -> _AUX_TIMER0_EVW {
        _AUX_TIMER0_EVW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE."]
    #[inline]
    pub fn aux_tdc_done(&mut self) -> _AUX_TDC_DONEW {
        _AUX_TDC_DONEW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE."]
    #[inline]
    pub fn aux_adc_done(&mut self) -> _AUX_ADC_DONEW {
        _AUX_ADC_DONEW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline]
    pub fn aux_compb(&mut self) -> _AUX_COMPBW {
        _AUX_COMPBW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline]
    pub fn aux_compa(&mut self) -> _AUX_COMPAW {
        _AUX_COMPAW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
}
