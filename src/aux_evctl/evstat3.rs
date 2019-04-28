#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVSTAT3 {
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
#[doc = r" Value of the field"]
pub struct AUX_TIMER2_CLKSWITCH_RDYR {
    bits: bool,
}
impl AUX_TIMER2_CLKSWITCH_RDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_DAC_HOLD_ACTIVER {
    bits: bool,
}
impl AUX_DAC_HOLD_ACTIVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_SMPH_AUTOTAKE_DONER {
    bits: bool,
}
impl AUX_SMPH_AUTOTAKE_DONER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_ADC_FIFO_NOT_EMPTYR {
    bits: bool,
}
impl AUX_ADC_FIFO_NOT_EMPTYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_ADC_FIFO_ALMOST_FULLR {
    bits: bool,
}
impl AUX_ADC_FIFO_ALMOST_FULLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_ADC_IRQR {
    bits: bool,
}
impl AUX_ADC_IRQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_ADC_DONER {
    bits: bool,
}
impl AUX_ADC_DONER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_ISRC_RESET_NR {
    bits: bool,
}
impl AUX_ISRC_RESET_NR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_TDC_DONER {
    bits: bool,
}
impl AUX_TDC_DONER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_TIMER0_EVR {
    bits: bool,
}
impl AUX_TIMER0_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_TIMER1_EVR {
    bits: bool,
}
impl AUX_TIMER1_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_TIMER2_PULSER {
    bits: bool,
}
impl AUX_TIMER2_PULSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_TIMER2_EV3R {
    bits: bool,
}
impl AUX_TIMER2_EV3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_TIMER2_EV2R {
    bits: bool,
}
impl AUX_TIMER2_EV2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_TIMER2_EV1R {
    bits: bool,
}
impl AUX_TIMER2_EV1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUX_TIMER2_EV0R {
    bits: bool,
}
impl AUX_TIMER2_EV0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_CLKSWITCH_RDYW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_CLKSWITCH_RDYW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_DAC_HOLD_ACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_DAC_HOLD_ACTIVEW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_SMPH_AUTOTAKE_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_SMPH_AUTOTAKE_DONEW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_ADC_FIFO_NOT_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_ADC_FIFO_NOT_EMPTYW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_ADC_FIFO_ALMOST_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_ADC_FIFO_ALMOST_FULLW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_ADC_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_ADC_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_ADC_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_ADC_DONEW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_ISRC_RESET_NW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_ISRC_RESET_NW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_TDC_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TDC_DONEW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_TIMER0_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER0_EVW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_TIMER1_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER1_EVW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_PULSEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_PULSEW<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_EV3W<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_EV3W<'a> {
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
pub struct _AUX_TIMER2_EV2W<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_EV2W<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_EV1W<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_EV1W<'a> {
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
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_EV0W<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_EV0W<'a> {
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
    #[doc = "Bit 15 - 15:15\\] AUX_SYSIF:TIMER2CLKSWITCH.RDY"]
    #[inline]
    pub fn aux_timer2_clkswitch_rdy(&self) -> AUX_TIMER2_CLKSWITCH_RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER2_CLKSWITCH_RDYR { bits }
    }
    #[doc = "Bit 14 - 14:14\\] AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
    #[inline]
    pub fn aux_dac_hold_active(&self) -> AUX_DAC_HOLD_ACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_DAC_HOLD_ACTIVER { bits }
    }
    #[doc = "Bit 13 - 13:13\\] See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline]
    pub fn aux_smph_autotake_done(&self) -> AUX_SMPH_AUTOTAKE_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_SMPH_AUTOTAKE_DONER { bits }
    }
    #[doc = "Bit 12 - 12:12\\] AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
    #[inline]
    pub fn aux_adc_fifo_not_empty(&self) -> AUX_ADC_FIFO_NOT_EMPTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_ADC_FIFO_NOT_EMPTYR { bits }
    }
    #[doc = "Bit 11 - 11:11\\] AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline]
    pub fn aux_adc_fifo_almost_full(&self) -> AUX_ADC_FIFO_ALMOST_FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_ADC_FIFO_ALMOST_FULLR { bits }
    }
    #[doc = "Bit 10 - 10:10\\] The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline]
    pub fn aux_adc_irq(&self) -> AUX_ADC_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_ADC_IRQR { bits }
    }
    #[doc = "Bit 9 - 9:9\\] AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
    #[inline]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_ADC_DONER { bits }
    }
    #[doc = "Bit 8 - 8:8\\] AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline]
    pub fn aux_isrc_reset_n(&self) -> AUX_ISRC_RESET_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_ISRC_RESET_NR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] AUX_TDC:STAT.DONE"]
    #[inline]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TDC_DONER { bits }
    }
    #[doc = "Bit 6 - 6:6\\] AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
    #[inline]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER0_EVR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
    #[inline]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER1_EVR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] AUX_TIMER2 pulse event. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_pulse(&self) -> AUX_TIMER2_PULSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER2_PULSER { bits }
    }
    #[doc = "Bit 3 - 3:3\\] AUX_TIMER2 event output 3. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_ev3(&self) -> AUX_TIMER2_EV3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER2_EV3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] AUX_TIMER2 event output 2. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_ev2(&self) -> AUX_TIMER2_EV2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER2_EV2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] AUX_TIMER2 event output 1. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_ev1(&self) -> AUX_TIMER2_EV1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER2_EV1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] AUX_TIMER2 event output 0. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_ev0(&self) -> AUX_TIMER2_EV0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER2_EV0R { bits }
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
    #[doc = "Bit 15 - 15:15\\] AUX_SYSIF:TIMER2CLKSWITCH.RDY"]
    #[inline]
    pub fn aux_timer2_clkswitch_rdy(&mut self) -> _AUX_TIMER2_CLKSWITCH_RDYW {
        _AUX_TIMER2_CLKSWITCH_RDYW { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
    #[inline]
    pub fn aux_dac_hold_active(&mut self) -> _AUX_DAC_HOLD_ACTIVEW {
        _AUX_DAC_HOLD_ACTIVEW { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline]
    pub fn aux_smph_autotake_done(&mut self) -> _AUX_SMPH_AUTOTAKE_DONEW {
        _AUX_SMPH_AUTOTAKE_DONEW { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
    #[inline]
    pub fn aux_adc_fifo_not_empty(&mut self) -> _AUX_ADC_FIFO_NOT_EMPTYW {
        _AUX_ADC_FIFO_NOT_EMPTYW { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline]
    pub fn aux_adc_fifo_almost_full(&mut self) -> _AUX_ADC_FIFO_ALMOST_FULLW {
        _AUX_ADC_FIFO_ALMOST_FULLW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline]
    pub fn aux_adc_irq(&mut self) -> _AUX_ADC_IRQW {
        _AUX_ADC_IRQW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
    #[inline]
    pub fn aux_adc_done(&mut self) -> _AUX_ADC_DONEW {
        _AUX_ADC_DONEW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline]
    pub fn aux_isrc_reset_n(&mut self) -> _AUX_ISRC_RESET_NW {
        _AUX_ISRC_RESET_NW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] AUX_TDC:STAT.DONE"]
    #[inline]
    pub fn aux_tdc_done(&mut self) -> _AUX_TDC_DONEW {
        _AUX_TDC_DONEW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
    #[inline]
    pub fn aux_timer0_ev(&mut self) -> _AUX_TIMER0_EVW {
        _AUX_TIMER0_EVW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
    #[inline]
    pub fn aux_timer1_ev(&mut self) -> _AUX_TIMER1_EVW {
        _AUX_TIMER1_EVW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] AUX_TIMER2 pulse event. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_pulse(&mut self) -> _AUX_TIMER2_PULSEW {
        _AUX_TIMER2_PULSEW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] AUX_TIMER2 event output 3. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_ev3(&mut self) -> _AUX_TIMER2_EV3W {
        _AUX_TIMER2_EV3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] AUX_TIMER2 event output 2. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_ev2(&mut self) -> _AUX_TIMER2_EV2W {
        _AUX_TIMER2_EV2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] AUX_TIMER2 event output 1. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_ev1(&mut self) -> _AUX_TIMER2_EV1W {
        _AUX_TIMER2_EV1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] AUX_TIMER2 event output 0. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline]
    pub fn aux_timer2_ev0(&mut self) -> _AUX_TIMER2_EV0W {
        _AUX_TIMER2_EV0W { w: self }
    }
}
