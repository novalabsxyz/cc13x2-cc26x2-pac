#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AESCTL {
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
pub struct CONTEXT_READYR {
    bits: bool,
}
impl CONTEXT_READYR {
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
pub struct SAVED_CONTEXT_RDYR {
    bits: bool,
}
impl SAVED_CONTEXT_RDYR {
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
pub struct SAVE_CONTEXTR {
    bits: bool,
}
impl SAVE_CONTEXTR {
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
pub struct RESERVED25R {
    bits: u8,
}
impl RESERVED25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCM_MR {
    bits: u8,
}
impl CCM_MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCM_LR {
    bits: u8,
}
impl CCM_LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCMR {
    bits: bool,
}
impl CCMR {
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
pub struct GCMR {
    bits: u8,
}
impl GCMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CBC_MACR {
    bits: bool,
}
impl CBC_MACR {
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
pub struct RESERVED9R {
    bits: u8,
}
impl RESERVED9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CTR_WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTR_WIDTHR {
    #[doc = "128 bits"]
    _128_BIT,
    #[doc = "96 bits"]
    _96_BIT,
    #[doc = "64 bits"]
    _64_BIT,
    #[doc = "32 bits"]
    _32_BIT,
}
impl CTR_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTR_WIDTHR::_128_BIT => 3,
            CTR_WIDTHR::_96_BIT => 2,
            CTR_WIDTHR::_64_BIT => 1,
            CTR_WIDTHR::_32_BIT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTR_WIDTHR {
        match value {
            3 => CTR_WIDTHR::_128_BIT,
            2 => CTR_WIDTHR::_96_BIT,
            1 => CTR_WIDTHR::_64_BIT,
            0 => CTR_WIDTHR::_32_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_128_BIT`"]
    #[inline]
    pub fn is_128_bit(&self) -> bool {
        *self == CTR_WIDTHR::_128_BIT
    }
    #[doc = "Checks if the value of the field is `_96_BIT`"]
    #[inline]
    pub fn is_96_bit(&self) -> bool {
        *self == CTR_WIDTHR::_96_BIT
    }
    #[doc = "Checks if the value of the field is `_64_BIT`"]
    #[inline]
    pub fn is_64_bit(&self) -> bool {
        *self == CTR_WIDTHR::_64_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline]
    pub fn is_32_bit(&self) -> bool {
        *self == CTR_WIDTHR::_32_BIT
    }
}
#[doc = r" Value of the field"]
pub struct CTRR {
    bits: bool,
}
impl CTRR {
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
pub struct CBCR {
    bits: bool,
}
impl CBCR {
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
pub struct KEY_SIZER {
    bits: u8,
}
impl KEY_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIRR {
    bits: bool,
}
impl DIRR {
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
pub struct INPUT_READYR {
    bits: bool,
}
impl INPUT_READYR {
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
pub struct OUTPUT_READYR {
    bits: bool,
}
impl OUTPUT_READYR {
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
pub struct _CONTEXT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTEXT_READYW<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAVED_CONTEXT_RDYW<'a> {
    w: &'a mut W,
}
impl<'a> _SAVED_CONTEXT_RDYW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAVE_CONTEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _SAVE_CONTEXTW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED25W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED25W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCM_MW<'a> {
    w: &'a mut W,
}
impl<'a> _CCM_MW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCM_LW<'a> {
    w: &'a mut W,
}
impl<'a> _CCM_LW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCMW<'a> {
    w: &'a mut W,
}
impl<'a> _CCMW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GCMW<'a> {
    w: &'a mut W,
}
impl<'a> _GCMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CBC_MACW<'a> {
    w: &'a mut W,
}
impl<'a> _CBC_MACW<'a> {
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
pub struct _RESERVED9W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTR_WIDTH`"]
pub enum CTR_WIDTHW {
    #[doc = "128 bits"]
    _128_BIT,
    #[doc = "96 bits"]
    _96_BIT,
    #[doc = "64 bits"]
    _64_BIT,
    #[doc = "32 bits"]
    _32_BIT,
}
impl CTR_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTR_WIDTHW::_128_BIT => 3,
            CTR_WIDTHW::_96_BIT => 2,
            CTR_WIDTHW::_64_BIT => 1,
            CTR_WIDTHW::_32_BIT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTR_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _CTR_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTR_WIDTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "128 bits"]
    #[inline]
    pub fn _128_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTHW::_128_BIT)
    }
    #[doc = "96 bits"]
    #[inline]
    pub fn _96_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTHW::_96_BIT)
    }
    #[doc = "64 bits"]
    #[inline]
    pub fn _64_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTHW::_64_BIT)
    }
    #[doc = "32 bits"]
    #[inline]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTHW::_32_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRW<'a> {
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
pub struct _CBCW<'a> {
    w: &'a mut W,
}
impl<'a> _CBCW<'a> {
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
pub struct _KEY_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_SIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
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
pub struct _INPUT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT_READYW<'a> {
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
pub struct _OUTPUT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTPUT_READYW<'a> {
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
    #[doc = "Bit 31 - 31:31\\] If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
    #[inline]
    pub fn context_ready(&self) -> CONTEXT_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONTEXT_READYR { bits }
    }
    #[doc = "Bit 30 - 30:30\\] If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
    #[inline]
    pub fn saved_context_rdy(&self) -> SAVED_CONTEXT_RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAVED_CONTEXT_RDYR { bits }
    }
    #[doc = "Bit 29 - 29:29\\] This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
    #[inline]
    pub fn save_context(&self) -> SAVE_CONTEXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAVE_CONTEXTR { bits }
    }
    #[doc = "Bits 25:28 - 28:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved25(&self) -> RESERVED25R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED25R { bits }
    }
    #[doc = "Bits 22:24 - 24:22\\] Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline]
    pub fn ccm_m(&self) -> CCM_MR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCM_MR { bits }
    }
    #[doc = "Bits 19:21 - 21:19\\] Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline]
    pub fn ccm_l(&self) -> CCM_LR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCM_LR { bits }
    }
    #[doc = "Bit 18 - 18:18\\] If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline]
    pub fn ccm(&self) -> CCMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CCMR { bits }
    }
    #[doc = "Bits 16:17 - 17:16\\] Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
    #[inline]
    pub fn gcm(&self) -> GCMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GCMR { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
    #[inline]
    pub fn cbc_mac(&self) -> CBC_MACR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CBC_MACR { bits }
    }
    #[doc = "Bits 9:14 - 14:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved9(&self) -> RESERVED9R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED9R { bits }
    }
    #[doc = "Bits 7:8 - 8:7\\] Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
    #[inline]
    pub fn ctr_width(&self) -> CTR_WIDTHR {
        CTR_WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - 6:6\\] If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline]
    pub fn ctr(&self) -> CTRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTRR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] If set to 1, cipher-block-chaining (CBC) mode is selected."]
    #[inline]
    pub fn cbc(&self) -> CBCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CBCR { bits }
    }
    #[doc = "Bits 3:4 - 4:3\\] This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
    #[inline]
    pub fn key_size(&self) -> KEY_SIZER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KEY_SIZER { bits }
    }
    #[doc = "Bit 2 - 2:2\\] If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline]
    pub fn dir(&self) -> DIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIRR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
    #[inline]
    pub fn input_ready(&self) -> INPUT_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INPUT_READYR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
    #[inline]
    pub fn output_ready(&self) -> OUTPUT_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTPUT_READYR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - 31:31\\] If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
    #[inline]
    pub fn context_ready(&mut self) -> _CONTEXT_READYW {
        _CONTEXT_READYW { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
    #[inline]
    pub fn saved_context_rdy(&mut self) -> _SAVED_CONTEXT_RDYW {
        _SAVED_CONTEXT_RDYW { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
    #[inline]
    pub fn save_context(&mut self) -> _SAVE_CONTEXTW {
        _SAVE_CONTEXTW { w: self }
    }
    #[doc = "Bits 25:28 - 28:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved25(&mut self) -> _RESERVED25W {
        _RESERVED25W { w: self }
    }
    #[doc = "Bits 22:24 - 24:22\\] Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline]
    pub fn ccm_m(&mut self) -> _CCM_MW {
        _CCM_MW { w: self }
    }
    #[doc = "Bits 19:21 - 21:19\\] Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline]
    pub fn ccm_l(&mut self) -> _CCM_LW {
        _CCM_LW { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline]
    pub fn ccm(&mut self) -> _CCMW {
        _CCMW { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\] Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
    #[inline]
    pub fn gcm(&mut self) -> _GCMW {
        _GCMW { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
    #[inline]
    pub fn cbc_mac(&mut self) -> _CBC_MACW {
        _CBC_MACW { w: self }
    }
    #[doc = "Bits 9:14 - 14:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved9(&mut self) -> _RESERVED9W {
        _RESERVED9W { w: self }
    }
    #[doc = "Bits 7:8 - 8:7\\] Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
    #[inline]
    pub fn ctr_width(&mut self) -> _CTR_WIDTHW {
        _CTR_WIDTHW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline]
    pub fn ctr(&mut self) -> _CTRW {
        _CTRW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] If set to 1, cipher-block-chaining (CBC) mode is selected."]
    #[inline]
    pub fn cbc(&mut self) -> _CBCW {
        _CBCW { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\] This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
    #[inline]
    pub fn key_size(&mut self) -> _KEY_SIZEW {
        _KEY_SIZEW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
    #[inline]
    pub fn input_ready(&mut self) -> _INPUT_READYW {
        _INPUT_READYW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
    #[inline]
    pub fn output_ready(&mut self) -> _OUTPUT_READYW {
        _OUTPUT_READYW { w: self }
    }
}
