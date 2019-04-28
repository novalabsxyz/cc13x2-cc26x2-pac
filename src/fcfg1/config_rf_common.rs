#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG_RF_COMMON {
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
pub struct DISABLE_CORNER_CAPR {
    bits: bool,
}
impl DISABLE_CORNER_CAPR {
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
pub struct SLDO_TRIM_OUTPUTR {
    bits: u8,
}
impl SLDO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA20DBMTRIMCOMPLETE_NR {
    bits: bool,
}
impl PA20DBMTRIMCOMPLETE_NR {
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
pub struct CTL_PA_20DBM_TRIMR {
    bits: u8,
}
impl CTL_PA_20DBM_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFLDO_TRIM_OUTPUTR {
    bits: u8,
}
impl RFLDO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct QUANTCTLTHRESR {
    bits: u8,
}
impl QUANTCTLTHRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DACTRIMR {
    bits: u8,
}
impl DACTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_CORNER_CAPW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_CORNER_CAPW<'a> {
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
pub struct _SLDO_TRIM_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SLDO_TRIM_OUTPUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA20DBMTRIMCOMPLETE_NW<'a> {
    w: &'a mut W,
}
impl<'a> _PA20DBMTRIMCOMPLETE_NW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTL_PA_20DBM_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CTL_PA_20DBM_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFLDO_TRIM_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _RFLDO_TRIM_OUTPUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _QUANTCTLTHRESW<'a> {
    w: &'a mut W,
}
impl<'a> _QUANTCTLTHRESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DACTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _DACTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bit 31 - 31:31\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn disable_corner_cap(&self) -> DISABLE_CORNER_CAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_CORNER_CAPR { bits }
    }
    #[doc = "Bits 25:30 - 30:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sldo_trim_output(&self) -> SLDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLDO_TRIM_OUTPUTR { bits }
    }
    #[doc = "Bit 21 - 21:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pa20dbmtrimcomplete_n(&self) -> PA20DBMTRIMCOMPLETE_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PA20DBMTRIMCOMPLETE_NR { bits }
    }
    #[doc = "Bits 16:20 - 20:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ctl_pa_20dbm_trim(&self) -> CTL_PA_20DBM_TRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL_PA_20DBM_TRIMR { bits }
    }
    #[doc = "Bits 9:15 - 15:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rfldo_trim_output(&self) -> RFLDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFLDO_TRIM_OUTPUTR { bits }
    }
    #[doc = "Bits 6:8 - 8:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn quantctlthres(&self) -> QUANTCTLTHRESR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        QUANTCTLTHRESR { bits }
    }
    #[doc = "Bits 0:5 - 5:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dactrim(&self) -> DACTRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DACTRIMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 29360461 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - 31:31\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn disable_corner_cap(&mut self) -> _DISABLE_CORNER_CAPW {
        _DISABLE_CORNER_CAPW { w: self }
    }
    #[doc = "Bits 25:30 - 30:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sldo_trim_output(&mut self) -> _SLDO_TRIM_OUTPUTW {
        _SLDO_TRIM_OUTPUTW { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pa20dbmtrimcomplete_n(&mut self) -> _PA20DBMTRIMCOMPLETE_NW {
        _PA20DBMTRIMCOMPLETE_NW { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ctl_pa_20dbm_trim(&mut self) -> _CTL_PA_20DBM_TRIMW {
        _CTL_PA_20DBM_TRIMW { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rfldo_trim_output(&mut self) -> _RFLDO_TRIM_OUTPUTW {
        _RFLDO_TRIM_OUTPUTW { w: self }
    }
    #[doc = "Bits 6:8 - 8:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn quantctlthres(&mut self) -> _QUANTCTLTHRESW {
        _QUANTCTLTHRESW { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dactrim(&mut self) -> _DACTRIMW {
        _DACTRIMW { w: self }
    }
}
