#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMAHWVER {
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
pub struct HW_MAJOR_VERSIONR {
    bits: u8,
}
impl HW_MAJOR_VERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HW_MINOR_VERSIONR {
    bits: u8,
}
impl HW_MINOR_VERSIONR {
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
pub struct EIP_NUMBER_COMPLR {
    bits: u8,
}
impl EIP_NUMBER_COMPLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EIP_NUMBERR {
    bits: u8,
}
impl EIP_NUMBERR {
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
pub struct _HW_MAJOR_VERSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _HW_MAJOR_VERSIONW<'a> {
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
pub struct _HW_MINOR_VERSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _HW_MINOR_VERSIONW<'a> {
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
pub struct _EIP_NUMBER_COMPLW<'a> {
    w: &'a mut W,
}
impl<'a> _EIP_NUMBER_COMPLW<'a> {
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
pub struct _EIP_NUMBERW<'a> {
    w: &'a mut W,
}
impl<'a> _EIP_NUMBERW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved28(&self) -> RESERVED28R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED28R { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] Major version number"]
    #[inline]
    pub fn hw_major_version(&self) -> HW_MAJOR_VERSIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_MAJOR_VERSIONR { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] Minor version number"]
    #[inline]
    pub fn hw_minor_version(&self) -> HW_MINOR_VERSIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_MINOR_VERSIONR { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Patch level Starts at 0 at first delivery of this version"]
    #[inline]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_PATCH_LEVELR { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline]
    pub fn eip_number_compl(&self) -> EIP_NUMBER_COMPLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EIP_NUMBER_COMPLR { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline]
    pub fn eip_number(&self) -> EIP_NUMBERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EIP_NUMBERR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16854737 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved28(&mut self) -> _RESERVED28W {
        _RESERVED28W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] Major version number"]
    #[inline]
    pub fn hw_major_version(&mut self) -> _HW_MAJOR_VERSIONW {
        _HW_MAJOR_VERSIONW { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Minor version number"]
    #[inline]
    pub fn hw_minor_version(&mut self) -> _HW_MINOR_VERSIONW {
        _HW_MINOR_VERSIONW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Patch level Starts at 0 at first delivery of this version"]
    #[inline]
    pub fn hw_patch_level(&mut self) -> _HW_PATCH_LEVELW {
        _HW_PATCH_LEVELW { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline]
    pub fn eip_number_compl(&mut self) -> _EIP_NUMBER_COMPLW {
        _EIP_NUMBER_COMPLW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline]
    pub fn eip_number(&mut self) -> _EIP_NUMBERW {
        _EIP_NUMBERW { w: self }
    }
}
