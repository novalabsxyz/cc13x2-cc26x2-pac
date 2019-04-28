#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTMASK {
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
pub struct RESERVED6R {
    bits: u32,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TEMP_UPDATE_MASKR {
    bits: bool,
}
impl TEMP_UPDATE_MASKR {
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
pub struct BATT_UPDATE_MASKR {
    bits: bool,
}
impl BATT_UPDATE_MASKR {
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
pub struct TEMP_BELOW_LL_MASKR {
    bits: bool,
}
impl TEMP_BELOW_LL_MASKR {
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
pub struct TEMP_OVER_UL_MASKR {
    bits: bool,
}
impl TEMP_OVER_UL_MASKR {
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
pub struct BATT_BELOW_LL_MASKR {
    bits: bool,
}
impl BATT_BELOW_LL_MASKR {
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
pub struct BATT_OVER_UL_MASKR {
    bits: bool,
}
impl BATT_OVER_UL_MASKR {
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
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 67108863;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEMP_UPDATE_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMP_UPDATE_MASKW<'a> {
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
pub struct _BATT_UPDATE_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _BATT_UPDATE_MASKW<'a> {
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
pub struct _TEMP_BELOW_LL_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMP_BELOW_LL_MASKW<'a> {
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
pub struct _TEMP_OVER_UL_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMP_OVER_UL_MASKW<'a> {
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
pub struct _BATT_BELOW_LL_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _BATT_BELOW_LL_MASKW<'a> {
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
pub struct _BATT_OVER_UL_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _BATT_OVER_UL_MASKW<'a> {
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
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u32 = 67108863;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] 1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
    #[inline]
    pub fn temp_update_mask(&self) -> TEMP_UPDATE_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEMP_UPDATE_MASKR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] 1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
    #[inline]
    pub fn batt_update_mask(&self) -> BATT_UPDATE_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BATT_UPDATE_MASKR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] 1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline]
    pub fn temp_below_ll_mask(&self) -> TEMP_BELOW_LL_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEMP_BELOW_LL_MASKR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] 1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
    #[inline]
    pub fn temp_over_ul_mask(&self) -> TEMP_OVER_UL_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEMP_OVER_UL_MASKR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] 1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline]
    pub fn batt_below_ll_mask(&self) -> BATT_BELOW_LL_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BATT_BELOW_LL_MASKR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] 1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
    #[inline]
    pub fn batt_over_ul_mask(&self) -> BATT_OVER_UL_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BATT_OVER_UL_MASKR { bits }
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
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] 1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
    #[inline]
    pub fn temp_update_mask(&mut self) -> _TEMP_UPDATE_MASKW {
        _TEMP_UPDATE_MASKW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] 1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
    #[inline]
    pub fn batt_update_mask(&mut self) -> _BATT_UPDATE_MASKW {
        _BATT_UPDATE_MASKW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline]
    pub fn temp_below_ll_mask(&mut self) -> _TEMP_BELOW_LL_MASKW {
        _TEMP_BELOW_LL_MASKW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
    #[inline]
    pub fn temp_over_ul_mask(&mut self) -> _TEMP_OVER_UL_MASKW {
        _TEMP_OVER_UL_MASKW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline]
    pub fn batt_below_ll_mask(&mut self) -> _BATT_BELOW_LL_MASKW {
        _BATT_BELOW_LL_MASKW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
    #[inline]
    pub fn batt_over_ul_mask(&mut self) -> _BATT_OVER_UL_MASKW {
        _BATT_OVER_UL_MASKW { w: self }
    }
}
