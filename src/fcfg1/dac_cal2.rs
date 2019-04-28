#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DAC_CAL2 {
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
pub struct SOC_DAC_VOUT_CAL_ADCREF_C2R {
    bits: u16,
}
impl SOC_DAC_VOUT_CAL_ADCREF_C2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SOC_DAC_VOUT_CAL_ADCREF_C1R {
    bits: u16,
}
impl SOC_DAC_VOUT_CAL_ADCREF_C1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SOC_DAC_VOUT_CAL_ADCREF_C2W<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_DAC_VOUT_CAL_ADCREF_C2W<'a> {
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
pub struct _SOC_DAC_VOUT_CAL_ADCREF_C1W<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_DAC_VOUT_CAL_ADCREF_C1W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn soc_dac_vout_cal_adcref_c2(&self) -> SOC_DAC_VOUT_CAL_ADCREF_C2R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SOC_DAC_VOUT_CAL_ADCREF_C2R { bits }
    }
    #[doc = "Bits 0:15 - 15:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn soc_dac_vout_cal_adcref_c1(&self) -> SOC_DAC_VOUT_CAL_ADCREF_C1R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SOC_DAC_VOUT_CAL_ADCREF_C1R { bits }
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
    #[doc = "Bits 16:31 - 31:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn soc_dac_vout_cal_adcref_c2(&mut self) -> _SOC_DAC_VOUT_CAL_ADCREF_C2W {
        _SOC_DAC_VOUT_CAL_ADCREF_C2W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn soc_dac_vout_cal_adcref_c1(&mut self) -> _SOC_DAC_VOUT_CAL_ADCREF_C1W {
        _SOC_DAC_VOUT_CAL_ADCREF_C1W { w: self }
    }
}
