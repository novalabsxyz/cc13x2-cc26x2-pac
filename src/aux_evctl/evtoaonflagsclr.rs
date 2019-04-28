#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVTOAONFLAGSCLR {
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
pub struct AUX_COMPBR {
    bits: bool,
}
impl AUX_COMPBR {
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
pub struct AUX_COMPAR {
    bits: bool,
}
impl AUX_COMPAR {
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
pub struct SWEV2R {
    bits: bool,
}
impl SWEV2R {
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
pub struct SWEV1R {
    bits: bool,
}
impl SWEV1R {
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
pub struct SWEV0R {
    bits: bool,
}
impl SWEV0R {
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
        const OFFSET: u8 = 8;
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
        const OFFSET: u8 = 7;
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
        const OFFSET: u8 = 6;
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUX_COMPBW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPBW<'a> {
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
pub struct _AUX_COMPAW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPAW<'a> {
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
pub struct _SWEV2W<'a> {
    w: &'a mut W,
}
impl<'a> _SWEV2W<'a> {
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
pub struct _SWEV1W<'a> {
    w: &'a mut W,
}
impl<'a> _SWEV1W<'a> {
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
pub struct _SWEV0W<'a> {
    w: &'a mut W,
}
impl<'a> _SWEV0W<'a> {
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
    #[doc = "Bit 8 - 8:8\\] Write 1 to clear EVTOAONFLAGS.AUX_TIMER1_EV. Read value is 0."]
    #[inline]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER1_EVR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Write 1 to clear EVTOAONFLAGS.AUX_TIMER0_EV. Read value is 0."]
    #[inline]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TIMER0_EVR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Write 1 to clear EVTOAONFLAGS.AUX_TDC_DONE. Read value is 0."]
    #[inline]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_TDC_DONER { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Write 1 to clear EVTOAONFLAGS.AUX_ADC_DONE. Read value is 0."]
    #[inline]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_ADC_DONER { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
    #[inline]
    pub fn aux_compb(&self) -> AUX_COMPBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_COMPBR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
    #[inline]
    pub fn aux_compa(&self) -> AUX_COMPAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_COMPAR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
    #[inline]
    pub fn swev2(&self) -> SWEV2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWEV2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
    #[inline]
    pub fn swev1(&self) -> SWEV1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWEV1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
    #[inline]
    pub fn swev0(&self) -> SWEV0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWEV0R { bits }
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
    #[doc = "Bit 8 - 8:8\\] Write 1 to clear EVTOAONFLAGS.AUX_TIMER1_EV. Read value is 0."]
    #[inline]
    pub fn aux_timer1_ev(&mut self) -> _AUX_TIMER1_EVW {
        _AUX_TIMER1_EVW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Write 1 to clear EVTOAONFLAGS.AUX_TIMER0_EV. Read value is 0."]
    #[inline]
    pub fn aux_timer0_ev(&mut self) -> _AUX_TIMER0_EVW {
        _AUX_TIMER0_EVW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Write 1 to clear EVTOAONFLAGS.AUX_TDC_DONE. Read value is 0."]
    #[inline]
    pub fn aux_tdc_done(&mut self) -> _AUX_TDC_DONEW {
        _AUX_TDC_DONEW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Write 1 to clear EVTOAONFLAGS.AUX_ADC_DONE. Read value is 0."]
    #[inline]
    pub fn aux_adc_done(&mut self) -> _AUX_ADC_DONEW {
        _AUX_ADC_DONEW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
    #[inline]
    pub fn aux_compb(&mut self) -> _AUX_COMPBW {
        _AUX_COMPBW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
    #[inline]
    pub fn aux_compa(&mut self) -> _AUX_COMPAW {
        _AUX_COMPAW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
    #[inline]
    pub fn swev2(&mut self) -> _SWEV2W {
        _SWEV2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
    #[inline]
    pub fn swev1(&mut self) -> _SWEV1W {
        _SWEV1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
    #[inline]
    pub fn swev0(&mut self) -> _SWEV0W {
        _SWEV0W { w: self }
    }
}
