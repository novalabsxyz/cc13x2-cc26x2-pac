#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG_SYNTH_DIV6_CC26 {
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
pub struct MIN_ALLOWED_RTRIMR {
    bits: u8,
}
impl MIN_ALLOWED_RTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFC_MDM_DEMIQMC0R {
    bits: u16,
}
impl RFC_MDM_DEMIQMC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LDOVCO_TRIM_OUTPUTR {
    bits: u8,
}
impl LDOVCO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFC_MDM_DEMIQMC0_TRIMCOMPLETE_NR {
    bits: bool,
}
impl RFC_MDM_DEMIQMC0_TRIMCOMPLETE_NR {
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
pub struct _MIN_ALLOWED_RTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _MIN_ALLOWED_RTRIMW<'a> {
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
pub struct _RFC_MDM_DEMIQMC0W<'a> {
    w: &'a mut W,
}
impl<'a> _RFC_MDM_DEMIQMC0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LDOVCO_TRIM_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _LDOVCO_TRIM_OUTPUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFC_MDM_DEMIQMC0_TRIMCOMPLETE_NW<'a> {
    w: &'a mut W,
}
impl<'a> _RFC_MDM_DEMIQMC0_TRIMCOMPLETE_NW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn min_allowed_rtrim(&self) -> MIN_ALLOWED_RTRIMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MIN_ALLOWED_RTRIMR { bits }
    }
    #[doc = "Bits 12:27 - 27:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rfc_mdm_demiqmc0(&self) -> RFC_MDM_DEMIQMC0R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RFC_MDM_DEMIQMC0R { bits }
    }
    #[doc = "Bits 6:11 - 11:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ldovco_trim_output(&self) -> LDOVCO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LDOVCO_TRIM_OUTPUTR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rfc_mdm_demiqmc0_trimcomplete_n(&self) -> RFC_MDM_DEMIQMC0_TRIMCOMPLETE_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFC_MDM_DEMIQMC0_TRIMCOMPLETE_NR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 31 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn min_allowed_rtrim(&mut self) -> _MIN_ALLOWED_RTRIMW {
        _MIN_ALLOWED_RTRIMW { w: self }
    }
    #[doc = "Bits 12:27 - 27:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rfc_mdm_demiqmc0(&mut self) -> _RFC_MDM_DEMIQMC0W {
        _RFC_MDM_DEMIQMC0W { w: self }
    }
    #[doc = "Bits 6:11 - 11:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ldovco_trim_output(&mut self) -> _LDOVCO_TRIM_OUTPUTW {
        _LDOVCO_TRIM_OUTPUTW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rfc_mdm_demiqmc0_trimcomplete_n(&mut self) -> _RFC_MDM_DEMIQMC0_TRIMCOMPLETE_NW {
        _RFC_MDM_DEMIQMC0_TRIMCOMPLETE_NW { w: self }
    }
}
