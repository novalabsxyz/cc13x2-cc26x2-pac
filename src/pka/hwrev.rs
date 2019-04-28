#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HWREV {
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
pub struct RESERVED28R {
    bits: u8,
}
impl RESERVED28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAJOR_HW_REVISIONR {
    bits: u8,
}
impl MAJOR_HW_REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINOR_HW_REVISIONR {
    bits: u8,
}
impl MINOR_HW_REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HW_PATCH_LEVELR {
    bits: u8,
}
impl HW_PATCH_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COMPLEMENT_OF_BASIC_EIP_NUMBERR {
    bits: u8,
}
impl COMPLEMENT_OF_BASIC_EIP_NUMBERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BASIC_EIP_NUMBERR {
    bits: u8,
}
impl BASIC_EIP_NUMBERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED28W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED28W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAJOR_HW_REVISIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MAJOR_HW_REVISIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MINOR_HW_REVISIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MINOR_HW_REVISIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HW_PATCH_LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _HW_PATCH_LEVELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPLEMENT_OF_BASIC_EIP_NUMBERW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPLEMENT_OF_BASIC_EIP_NUMBERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BASIC_EIP_NUMBERW<'a> {
    w: &'a mut W,
}
impl<'a> _BASIC_EIP_NUMBERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 28:31 - 31:28\\] Ignore on read"]
    #[inline]
    pub fn reserved28(&self) -> RESERVED28R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED28R { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] 4-bit binary encoding of the major hardware revision number"]
    #[inline]
    pub fn major_hw_revision(&self) -> MAJOR_HW_REVISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJOR_HW_REVISIONR { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] 4-bit binary encoding of the minor hardware revision number"]
    #[inline]
    pub fn minor_hw_revision(&self) -> MINOR_HW_REVISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINOR_HW_REVISIONR { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] 4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_PATCH_LEVELR { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
    #[inline]
    pub fn complement_of_basic_eip_number(&self) -> COMPLEMENT_OF_BASIC_EIP_NUMBERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COMPLEMENT_OF_BASIC_EIP_NUMBERR { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] 8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
    #[inline]
    pub fn basic_eip_number(&self) -> BASIC_EIP_NUMBERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BASIC_EIP_NUMBERR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 22143772 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Ignore on read"]
    #[inline]
    pub fn reserved28(&mut self) -> _RESERVED28W {
        _RESERVED28W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] 4-bit binary encoding of the major hardware revision number"]
    #[inline]
    pub fn major_hw_revision(&mut self) -> _MAJOR_HW_REVISIONW {
        _MAJOR_HW_REVISIONW { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] 4-bit binary encoding of the minor hardware revision number"]
    #[inline]
    pub fn minor_hw_revision(&mut self) -> _MINOR_HW_REVISIONW {
        _MINOR_HW_REVISIONW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] 4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline]
    pub fn hw_patch_level(&mut self) -> _HW_PATCH_LEVELW {
        _HW_PATCH_LEVELW { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
    #[inline]
    pub fn complement_of_basic_eip_number(&mut self) -> _COMPLEMENT_OF_BASIC_EIP_NUMBERW {
        _COMPLEMENT_OF_BASIC_EIP_NUMBERW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] 8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
    #[inline]
    pub fn basic_eip_number(&mut self) -> _BASIC_EIP_NUMBERW {
        _BASIC_EIP_NUMBERW { w: self }
    }
}
