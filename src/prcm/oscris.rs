#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSCRIS {
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
pub struct RESERVED8R {
    bits: u32,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HFSRCPENDRISR {
    bits: bool,
}
impl HFSRCPENDRISR {
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
pub struct LFSRCDONERISR {
    bits: bool,
}
impl LFSRCDONERISR {
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
pub struct XOSCDLFRISR {
    bits: bool,
}
impl XOSCDLFRISR {
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
pub struct XOSCLFRISR {
    bits: bool,
}
impl XOSCLFRISR {
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
pub struct RCOSCDLFRISR {
    bits: bool,
}
impl RCOSCDLFRISR {
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
pub struct RCOSCLFRISR {
    bits: bool,
}
impl RCOSCLFRISR {
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
pub struct XOSCHFRISR {
    bits: bool,
}
impl XOSCHFRISR {
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
pub struct RCOSCHFRISR {
    bits: bool,
}
impl RCOSCHFRISR {
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
pub struct _RESERVED8W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HFSRCPENDRISW<'a> {
    w: &'a mut W,
}
impl<'a> _HFSRCPENDRISW<'a> {
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
pub struct _LFSRCDONERISW<'a> {
    w: &'a mut W,
}
impl<'a> _LFSRCDONERISW<'a> {
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
pub struct _XOSCDLFRISW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCDLFRISW<'a> {
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
pub struct _XOSCLFRISW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCLFRISW<'a> {
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
pub struct _RCOSCDLFRISW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCDLFRISW<'a> {
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
pub struct _RCOSCLFRISW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCLFRISW<'a> {
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
pub struct _XOSCHFRISW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCHFRISW<'a> {
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
pub struct _RCOSCHFRISW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCHFRISW<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] 0: HFSRCPEND has not been qualified 1: HFSRCPEND has been qualified since last clear Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
    #[inline]
    pub fn hfsrcpendris(&self) -> HFSRCPENDRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HFSRCPENDRISR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] 0: LFSRCDONE has not been qualified 1: LFSRCDONE has been qualified since last clear Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
    #[inline]
    pub fn lfsrcdoneris(&self) -> LFSRCDONERISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LFSRCDONERISR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
    #[inline]
    pub fn xoscdlfris(&self) -> XOSCDLFRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSCDLFRISR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] 0: XOSCLF has not been qualified 1: XOSCLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
    #[inline]
    pub fn xosclfris(&self) -> XOSCLFRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSCLFRISR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
    #[inline]
    pub fn rcoscdlfris(&self) -> RCOSCDLFRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSCDLFRISR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
    #[inline]
    pub fn rcosclfris(&self) -> RCOSCLFRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSCLFRISR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] 0: XOSCHF has not been qualified 1: XOSCHF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
    #[inline]
    pub fn xoschfris(&self) -> XOSCHFRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSCHFRISR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] 0: RCOSCHF has not been qualified 1: RCOSCHF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
    #[inline]
    pub fn rcoschfris(&self) -> RCOSCHFRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSCHFRISR { bits }
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&mut self) -> _RESERVED8W {
        _RESERVED8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] 0: HFSRCPEND has not been qualified 1: HFSRCPEND has been qualified since last clear Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
    #[inline]
    pub fn hfsrcpendris(&mut self) -> _HFSRCPENDRISW {
        _HFSRCPENDRISW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] 0: LFSRCDONE has not been qualified 1: LFSRCDONE has been qualified since last clear Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
    #[inline]
    pub fn lfsrcdoneris(&mut self) -> _LFSRCDONERISW {
        _LFSRCDONERISW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] 0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
    #[inline]
    pub fn xoscdlfris(&mut self) -> _XOSCDLFRISW {
        _XOSCDLFRISW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] 0: XOSCLF has not been qualified 1: XOSCLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
    #[inline]
    pub fn xosclfris(&mut self) -> _XOSCLFRISW {
        _XOSCLFRISW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
    #[inline]
    pub fn rcoscdlfris(&mut self) -> _RCOSCDLFRISW {
        _RCOSCDLFRISW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
    #[inline]
    pub fn rcosclfris(&mut self) -> _RCOSCLFRISW {
        _RCOSCLFRISW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: XOSCHF has not been qualified 1: XOSCHF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
    #[inline]
    pub fn xoschfris(&mut self) -> _XOSCHFRISW {
        _XOSCHFRISW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: RCOSCHF has not been qualified 1: RCOSCHF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
    #[inline]
    pub fn rcoschfris(&mut self) -> _RCOSCHFRISW {
        _RCOSCHFRISW { w: self }
    }
}
