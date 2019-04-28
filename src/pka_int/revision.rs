#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REVISION {
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
pub struct MAJOR_REVISIONR {
    bits: u8,
}
impl MAJOR_REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINOR_REVISIONR {
    bits: u8,
}
impl MINOR_REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PATCH_LEVELR {
    bits: u8,
}
impl PATCH_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COMP_EIP_NUMR {
    bits: u8,
}
impl COMP_EIP_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EIP_NUMR {
    bits: u8,
}
impl EIP_NUMR {
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
pub struct _MAJOR_REVISIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MAJOR_REVISIONW<'a> {
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
pub struct _MINOR_REVISIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MINOR_REVISIONW<'a> {
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
pub struct _PATCH_LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _PATCH_LEVELW<'a> {
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
pub struct _COMP_EIP_NUMW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_EIP_NUMW<'a> {
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
pub struct _EIP_NUMW<'a> {
    w: &'a mut W,
}
impl<'a> _EIP_NUMW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] These bits should be ignored on read"]
    #[inline]
    pub fn reserved28(&self) -> RESERVED28R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED28R { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] These bits encode the major version number for this module"]
    #[inline]
    pub fn major_revision(&self) -> MAJOR_REVISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJOR_REVISIONR { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] These bits encode the minor version number for this module"]
    #[inline]
    pub fn minor_revision(&self) -> MINOR_REVISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINOR_REVISIONR { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] These bits encode the hardware patch level for this module they start at value 0 on the first release"]
    #[inline]
    pub fn patch_level(&self) -> PATCH_LEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PATCH_LEVELR { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
    #[inline]
    pub fn comp_eip_num(&self) -> COMP_EIP_NUMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COMP_EIP_NUMR { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] These bits encode the AuthenTec EIP number for the EIP150"]
    #[inline]
    pub fn eip_num(&self) -> EIP_NUMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EIP_NUMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 33581462 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] These bits should be ignored on read"]
    #[inline]
    pub fn reserved28(&mut self) -> _RESERVED28W {
        _RESERVED28W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] These bits encode the major version number for this module"]
    #[inline]
    pub fn major_revision(&mut self) -> _MAJOR_REVISIONW {
        _MAJOR_REVISIONW { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] These bits encode the minor version number for this module"]
    #[inline]
    pub fn minor_revision(&mut self) -> _MINOR_REVISIONW {
        _MINOR_REVISIONW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] These bits encode the hardware patch level for this module they start at value 0 on the first release"]
    #[inline]
    pub fn patch_level(&mut self) -> _PATCH_LEVELW {
        _PATCH_LEVELW { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
    #[inline]
    pub fn comp_eip_num(&mut self) -> _COMP_EIP_NUMW {
        _COMP_EIP_NUMW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] These bits encode the AuthenTec EIP number for the EIP150"]
    #[inline]
    pub fn eip_num(&mut self) -> _EIP_NUMW {
        _EIP_NUMW { w: self }
    }
}
