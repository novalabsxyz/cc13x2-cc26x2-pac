#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUXSCECLK {
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
#[doc = "Possible values of the field `PD_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_SRCR {
    #[doc = "LF clock (SCLK_LF )"]
    SCLK_LF,
    #[doc = "No clock"]
    NO_CLOCK,
}
impl PD_SRCR {
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
            PD_SRCR::SCLK_LF => true,
            PD_SRCR::NO_CLOCK => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PD_SRCR {
        match value {
            true => PD_SRCR::SCLK_LF,
            false => PD_SRCR::NO_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline]
    pub fn is_sclk_lf(&self) -> bool {
        *self == PD_SRCR::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline]
    pub fn is_no_clock(&self) -> bool {
        *self == PD_SRCR::NO_CLOCK
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED3R {
    bits: u8,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "MF Clock (SCLK_MF)"]
    SCLK_MF,
    #[doc = "HF Clock divided by 2 (SCLK_HFDIV2)"]
    SCLK_HFDIV2,
}
impl SRCR {
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
            SRCR::SCLK_MF => true,
            SRCR::SCLK_HFDIV2 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRCR {
        match value {
            true => SRCR::SCLK_MF,
            false => SRCR::SCLK_HFDIV2,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_MF`"]
    #[inline]
    pub fn is_sclk_mf(&self) -> bool {
        *self == SRCR::SCLK_MF
    }
    #[doc = "Checks if the value of the field is `SCLK_HFDIV2`"]
    #[inline]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == SRCR::SCLK_HFDIV2
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
#[doc = "Values that can be written to the field `PD_SRC`"]
pub enum PD_SRCW {
    #[doc = "LF clock (SCLK_LF )"]
    SCLK_LF,
    #[doc = "No clock"]
    NO_CLOCK,
}
impl PD_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PD_SRCW::SCLK_LF => true,
            PD_SRCW::NO_CLOCK => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _PD_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD_SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LF clock (SCLK_LF )"]
    #[inline]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(PD_SRCW::SCLK_LF)
    }
    #[doc = "No clock"]
    #[inline]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(PD_SRCW::NO_CLOCK)
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
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC`"]
pub enum SRCW {
    #[doc = "MF Clock (SCLK_MF)"]
    SCLK_MF,
    #[doc = "HF Clock divided by 2 (SCLK_HFDIV2)"]
    SCLK_HFDIV2,
}
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRCW::SCLK_MF => true,
            SRCW::SCLK_HFDIV2 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MF Clock (SCLK_MF)"]
    #[inline]
    pub fn sclk_mf(self) -> &'a mut W {
        self.variant(SRCW::SCLK_MF)
    }
    #[doc = "HF Clock divided by 2 (SCLK_HFDIV2)"]
    #[inline]
    pub fn sclk_hfdiv2(self) -> &'a mut W {
        self.variant(SRCW::SCLK_HFDIV2)
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
    #[doc = "Bit 8 - 8:8\\] Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline]
    pub fn pd_src(&self) -> PD_SRCR {
        PD_SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:7 - 7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
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
    #[doc = "Bits 9:31 - 31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved9(&mut self) -> _RESERVED9W {
        _RESERVED9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline]
    pub fn pd_src(&mut self) -> _PD_SRCW {
        _PD_SRCW { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
}
