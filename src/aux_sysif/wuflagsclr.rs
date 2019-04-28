#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUFLAGSCLR {
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
pub struct SW_WU3R {
    bits: bool,
}
impl SW_WU3R {
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
pub struct SW_WU2R {
    bits: bool,
}
impl SW_WU2R {
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
pub struct SW_WU1R {
    bits: bool,
}
impl SW_WU1R {
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
pub struct SW_WU0R {
    bits: bool,
}
impl SW_WU0R {
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
pub struct PROG_WU3R {
    bits: bool,
}
impl PROG_WU3R {
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
pub struct PROG_WU2R {
    bits: bool,
}
impl PROG_WU2R {
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
pub struct PROG_WU1R {
    bits: bool,
}
impl PROG_WU1R {
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
pub struct PROG_WU0R {
    bits: bool,
}
impl PROG_WU0R {
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
pub struct _SW_WU3W<'a> {
    w: &'a mut W,
}
impl<'a> _SW_WU3W<'a> {
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
pub struct _SW_WU2W<'a> {
    w: &'a mut W,
}
impl<'a> _SW_WU2W<'a> {
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
pub struct _SW_WU1W<'a> {
    w: &'a mut W,
}
impl<'a> _SW_WU1W<'a> {
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
pub struct _SW_WU0W<'a> {
    w: &'a mut W,
}
impl<'a> _SW_WU0W<'a> {
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
pub struct _PROG_WU3W<'a> {
    w: &'a mut W,
}
impl<'a> _PROG_WU3W<'a> {
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
pub struct _PROG_WU2W<'a> {
    w: &'a mut W,
}
impl<'a> _PROG_WU2W<'a> {
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
pub struct _PROG_WU1W<'a> {
    w: &'a mut W,
}
impl<'a> _PROG_WU1W<'a> {
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
pub struct _PROG_WU0W<'a> {
    w: &'a mut W,
}
impl<'a> _PROG_WU0W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
    #[inline]
    pub fn sw_wu3(&self) -> SW_WU3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_WU3R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
    #[inline]
    pub fn sw_wu2(&self) -> SW_WU2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_WU2R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
    #[inline]
    pub fn sw_wu1(&self) -> SW_WU1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_WU1R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
    #[inline]
    pub fn sw_wu0(&self) -> SW_WU0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_WU0R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
    #[inline]
    pub fn prog_wu3(&self) -> PROG_WU3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PROG_WU3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
    #[inline]
    pub fn prog_wu2(&self) -> PROG_WU2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PROG_WU2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
    #[inline]
    pub fn prog_wu1(&self) -> PROG_WU1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PROG_WU1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
    #[inline]
    pub fn prog_wu0(&self) -> PROG_WU0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PROG_WU0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15 }
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
    #[doc = "Bit 7 - 7:7\\] Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
    #[inline]
    pub fn sw_wu3(&mut self) -> _SW_WU3W {
        _SW_WU3W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
    #[inline]
    pub fn sw_wu2(&mut self) -> _SW_WU2W {
        _SW_WU2W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
    #[inline]
    pub fn sw_wu1(&mut self) -> _SW_WU1W {
        _SW_WU1W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
    #[inline]
    pub fn sw_wu0(&mut self) -> _SW_WU0W {
        _SW_WU0W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
    #[inline]
    pub fn prog_wu3(&mut self) -> _PROG_WU3W {
        _PROG_WU3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
    #[inline]
    pub fn prog_wu2(&mut self) -> _PROG_WU2W {
        _PROG_WU2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
    #[inline]
    pub fn prog_wu1(&mut self) -> _PROG_WU1W {
        _PROG_WU1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
    #[inline]
    pub fn prog_wu0(&mut self) -> _PROG_WU0W {
        _PROG_WU0W { w: self }
    }
}
