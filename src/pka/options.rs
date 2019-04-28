#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPTIONS {
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
pub struct RESERVED12R {
    bits: u32,
}
impl RESERVED12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INT_MASKINGR {
    bits: bool,
}
impl INT_MASKINGR {
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
pub struct PROTECTION_OPTIONR {
    bits: u8,
}
impl PROTECTION_OPTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PROGRAM_RAMR {
    bits: bool,
}
impl PROGRAM_RAMR {
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
pub struct SEQUENCER_CONFIGURATIONR {
    bits: u8,
}
impl SEQUENCER_CONFIGURATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: u8,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKCP_CONFIGURATIONR {
    bits: u8,
}
impl PKCP_CONFIGURATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED12W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1048575;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INT_MASKINGW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_MASKINGW<'a> {
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
pub struct _PROTECTION_OPTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTECTION_OPTIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PROGRAM_RAMW<'a> {
    w: &'a mut W,
}
impl<'a> _PROGRAM_RAMW<'a> {
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
pub struct _SEQUENCER_CONFIGURATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQUENCER_CONFIGURATIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PKCP_CONFIGURATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _PKCP_CONFIGURATIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 12:31 - 31:12\\] Ignore on read"]
    #[inline]
    pub fn reserved12(&self) -> RESERVED12R {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED12R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\] of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline]
    pub fn int_masking(&self) -> INT_MASKINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INT_MASKINGR { bits }
    }
    #[doc = "Bits 8:10 - 10:8\\] Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
    #[inline]
    pub fn protection_option(&self) -> PROTECTION_OPTIONR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROTECTION_OPTIONR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
    #[inline]
    pub fn program_ram(&self) -> PROGRAM_RAMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PROGRAM_RAMR { bits }
    }
    #[doc = "Bits 5:6 - 6:5\\] Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
    #[inline]
    pub fn sequencer_configuration(&self) -> SEQUENCER_CONFIGURATIONR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEQUENCER_CONFIGURATIONR { bits }
    }
    #[doc = "Bits 2:4 - 4:2\\] Ignore on read"]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
    #[inline]
    pub fn pkcp_configuration(&self) -> PKCP_CONFIGURATIONR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PKCP_CONFIGURATIONR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:31 - 31:12\\] Ignore on read"]
    #[inline]
    pub fn reserved12(&mut self) -> _RESERVED12W {
        _RESERVED12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\] of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline]
    pub fn int_masking(&mut self) -> _INT_MASKINGW {
        _INT_MASKINGW { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
    #[inline]
    pub fn protection_option(&mut self) -> _PROTECTION_OPTIONW {
        _PROTECTION_OPTIONW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
    #[inline]
    pub fn program_ram(&mut self) -> _PROGRAM_RAMW {
        _PROGRAM_RAMW { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\] Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
    #[inline]
    pub fn sequencer_configuration(&mut self) -> _SEQUENCER_CONFIGURATIONW {
        _SEQUENCER_CONFIGURATIONW { w: self }
    }
    #[doc = "Bits 2:4 - 4:2\\] Ignore on read"]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
    #[inline]
    pub fn pkcp_configuration(&mut self) -> _PKCP_CONFIGURATIONW {
        _PKCP_CONFIGURATIONW { w: self }
    }
}
