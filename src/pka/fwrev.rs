#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FWREV {
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
pub struct FW_CAPABILITIESR {
    bits: u8,
}
impl FW_CAPABILITIESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAJOR_FW_REVISIONR {
    bits: u8,
}
impl MAJOR_FW_REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINOR_FW_REVISIONR {
    bits: u8,
}
impl MINOR_FW_REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FW_PATCH_LEVELR {
    bits: u8,
}
impl FW_PATCH_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED0R {
    bits: u16,
}
impl RESERVED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FW_CAPABILITIESW<'a> {
    w: &'a mut W,
}
impl<'a> _FW_CAPABILITIESW<'a> {
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
pub struct _MAJOR_FW_REVISIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MAJOR_FW_REVISIONW<'a> {
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
pub struct _MINOR_FW_REVISIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MINOR_FW_REVISIONW<'a> {
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
pub struct _FW_PATCH_LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _FW_PATCH_LEVELW<'a> {
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
pub struct _RESERVED0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 28:31 - 31:28\\] Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
    #[inline]
    pub fn fw_capabilities(&self) -> FW_CAPABILITIESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FW_CAPABILITIESR { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] 4-bit binary encoding of the major firmware revision number"]
    #[inline]
    pub fn major_fw_revision(&self) -> MAJOR_FW_REVISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJOR_FW_REVISIONR { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] 4-bit binary encoding of the minor firmware revision number"]
    #[inline]
    pub fn minor_fw_revision(&self) -> MINOR_FW_REVISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINOR_FW_REVISIONR { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] 4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline]
    pub fn fw_patch_level(&self) -> FW_PATCH_LEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FW_PATCH_LEVELR { bits }
    }
    #[doc = "Bits 0:15 - 15:0\\] Ignore on read"]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 558891008 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
    #[inline]
    pub fn fw_capabilities(&mut self) -> _FW_CAPABILITIESW {
        _FW_CAPABILITIESW { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] 4-bit binary encoding of the major firmware revision number"]
    #[inline]
    pub fn major_fw_revision(&mut self) -> _MAJOR_FW_REVISIONW {
        _MAJOR_FW_REVISIONW { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] 4-bit binary encoding of the minor firmware revision number"]
    #[inline]
    pub fn minor_fw_revision(&mut self) -> _MINOR_FW_REVISIONW {
        _MINOR_FW_REVISIONW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] 4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline]
    pub fn fw_patch_level(&mut self) -> _FW_PATCH_LEVELW {
        _FW_PATCH_LEVELW { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\] Ignore on read"]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
}
