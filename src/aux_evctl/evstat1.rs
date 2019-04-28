#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVSTAT1 {
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
pub struct AUXIO31R {
    bits: bool,
}
impl AUXIO31R {
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
pub struct AUXIO30R {
    bits: bool,
}
impl AUXIO30R {
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
pub struct AUXIO29R {
    bits: bool,
}
impl AUXIO29R {
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
pub struct AUXIO28R {
    bits: bool,
}
impl AUXIO28R {
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
pub struct AUXIO27R {
    bits: bool,
}
impl AUXIO27R {
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
pub struct AUXIO26R {
    bits: bool,
}
impl AUXIO26R {
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
pub struct AUXIO25R {
    bits: bool,
}
impl AUXIO25R {
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
pub struct AUXIO24R {
    bits: bool,
}
impl AUXIO24R {
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
pub struct AUXIO23R {
    bits: bool,
}
impl AUXIO23R {
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
pub struct AUXIO22R {
    bits: bool,
}
impl AUXIO22R {
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
pub struct AUXIO21R {
    bits: bool,
}
impl AUXIO21R {
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
pub struct AUXIO20R {
    bits: bool,
}
impl AUXIO20R {
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
pub struct AUXIO19R {
    bits: bool,
}
impl AUXIO19R {
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
pub struct AUXIO18R {
    bits: bool,
}
impl AUXIO18R {
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
pub struct AUXIO17R {
    bits: bool,
}
impl AUXIO17R {
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
pub struct AUXIO16R {
    bits: bool,
}
impl AUXIO16R {
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
pub struct _AUXIO31W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO31W<'a> {
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
pub struct _AUXIO30W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO30W<'a> {
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
pub struct _AUXIO29W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO29W<'a> {
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
pub struct _AUXIO28W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO28W<'a> {
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
pub struct _AUXIO27W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO27W<'a> {
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
pub struct _AUXIO26W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO26W<'a> {
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
pub struct _AUXIO25W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO25W<'a> {
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
pub struct _AUXIO24W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO24W<'a> {
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
pub struct _AUXIO23W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO23W<'a> {
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
pub struct _AUXIO22W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO22W<'a> {
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
pub struct _AUXIO21W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO21W<'a> {
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
pub struct _AUXIO20W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO20W<'a> {
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
pub struct _AUXIO19W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO19W<'a> {
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
pub struct _AUXIO18W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO18W<'a> {
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
pub struct _AUXIO17W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO17W<'a> {
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
pub struct _AUXIO16W<'a> {
    w: &'a mut W,
}
impl<'a> _AUXIO16W<'a> {
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
    #[doc = "Bit 15 - 15:15\\] AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
    #[inline]
    pub fn auxio31(&self) -> AUXIO31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO31R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
    #[inline]
    pub fn auxio30(&self) -> AUXIO30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO30R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
    #[inline]
    pub fn auxio29(&self) -> AUXIO29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO29R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
    #[inline]
    pub fn auxio28(&self) -> AUXIO28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO28R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
    #[inline]
    pub fn auxio27(&self) -> AUXIO27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO27R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
    #[inline]
    pub fn auxio26(&self) -> AUXIO26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO26R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
    #[inline]
    pub fn auxio25(&self) -> AUXIO25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO25R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
    #[inline]
    pub fn auxio24(&self) -> AUXIO24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO24R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
    #[inline]
    pub fn auxio23(&self) -> AUXIO23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO23R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
    #[inline]
    pub fn auxio22(&self) -> AUXIO22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO22R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
    #[inline]
    pub fn auxio21(&self) -> AUXIO21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO21R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
    #[inline]
    pub fn auxio20(&self) -> AUXIO20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO20R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
    #[inline]
    pub fn auxio19(&self) -> AUXIO19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO19R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
    #[inline]
    pub fn auxio18(&self) -> AUXIO18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO18R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
    #[inline]
    pub fn auxio17(&self) -> AUXIO17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO17R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
    #[inline]
    pub fn auxio16(&self) -> AUXIO16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXIO16R { bits }
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
    #[doc = "Bit 15 - 15:15\\] AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
    #[inline]
    pub fn auxio31(&mut self) -> _AUXIO31W {
        _AUXIO31W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
    #[inline]
    pub fn auxio30(&mut self) -> _AUXIO30W {
        _AUXIO30W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
    #[inline]
    pub fn auxio29(&mut self) -> _AUXIO29W {
        _AUXIO29W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
    #[inline]
    pub fn auxio28(&mut self) -> _AUXIO28W {
        _AUXIO28W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
    #[inline]
    pub fn auxio27(&mut self) -> _AUXIO27W {
        _AUXIO27W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
    #[inline]
    pub fn auxio26(&mut self) -> _AUXIO26W {
        _AUXIO26W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
    #[inline]
    pub fn auxio25(&mut self) -> _AUXIO25W {
        _AUXIO25W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
    #[inline]
    pub fn auxio24(&mut self) -> _AUXIO24W {
        _AUXIO24W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
    #[inline]
    pub fn auxio23(&mut self) -> _AUXIO23W {
        _AUXIO23W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
    #[inline]
    pub fn auxio22(&mut self) -> _AUXIO22W {
        _AUXIO22W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
    #[inline]
    pub fn auxio21(&mut self) -> _AUXIO21W {
        _AUXIO21W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
    #[inline]
    pub fn auxio20(&mut self) -> _AUXIO20W {
        _AUXIO20W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
    #[inline]
    pub fn auxio19(&mut self) -> _AUXIO19W {
        _AUXIO19W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
    #[inline]
    pub fn auxio18(&mut self) -> _AUXIO18W {
        _AUXIO18W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
    #[inline]
    pub fn auxio17(&mut self) -> _AUXIO17W {
        _AUXIO17W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
    #[inline]
    pub fn auxio16(&mut self) -> _AUXIO16W {
        _AUXIO16W { w: self }
    }
}
