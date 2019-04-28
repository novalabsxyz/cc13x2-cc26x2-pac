#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCOSCMFCTL {
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
pub struct SPARE16R {
    bits: u16,
}
impl SPARE16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSC_MF_CAP_ARRAYR {
    bits: u8,
}
impl RCOSC_MF_CAP_ARRAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSC_MF_REG_SELR {
    bits: bool,
}
impl RCOSC_MF_REG_SELR {
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
pub struct RCOSC_MF_RES_COARSER {
    bits: u8,
}
impl RCOSC_MF_RES_COARSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSC_MF_RES_FINER {
    bits: u8,
}
impl RCOSC_MF_RES_FINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSC_MF_BIAS_ADJR {
    bits: u8,
}
impl RCOSC_MF_BIAS_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SPARE16W<'a> {
    w: &'a mut W,
}
impl<'a> _SPARE16W<'a> {
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
pub struct _RCOSC_MF_CAP_ARRAYW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_MF_CAP_ARRAYW<'a> {
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
pub struct _RCOSC_MF_REG_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_MF_REG_SELW<'a> {
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
pub struct _RCOSC_MF_RES_COARSEW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_MF_RES_COARSEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSC_MF_RES_FINEW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_MF_RES_FINEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSC_MF_BIAS_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_MF_BIAS_ADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    pub fn spare16(&self) -> SPARE16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SPARE16R { bits }
    }
    #[doc = "Bits 9:15 - 15:9\\] Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
    #[inline]
    pub fn rcosc_mf_cap_array(&self) -> RCOSC_MF_CAP_ARRAYR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSC_MF_CAP_ARRAYR { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Choose regulator type. 0: default 1: alternate"]
    #[inline]
    pub fn rcosc_mf_reg_sel(&self) -> RCOSC_MF_REG_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSC_MF_REG_SELR { bits }
    }
    #[doc = "Bits 6:7 - 7:6\\] Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
    #[inline]
    pub fn rcosc_mf_res_coarse(&self) -> RCOSC_MF_RES_COARSER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSC_MF_RES_COARSER { bits }
    }
    #[doc = "Bits 4:5 - 5:4\\] Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
    #[inline]
    pub fn rcosc_mf_res_fine(&self) -> RCOSC_MF_RES_FINER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSC_MF_RES_FINER { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Adjusts bias current to RCOSC_MF. 0x8 minimum current 0x0 default current 0x7 maximum current"]
    #[inline]
    pub fn rcosc_mf_bias_adj(&self) -> RCOSC_MF_BIAS_ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSC_MF_BIAS_ADJR { bits }
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
    pub fn spare16(&mut self) -> _SPARE16W {
        _SPARE16W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\] Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
    #[inline]
    pub fn rcosc_mf_cap_array(&mut self) -> _RCOSC_MF_CAP_ARRAYW {
        _RCOSC_MF_CAP_ARRAYW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Choose regulator type. 0: default 1: alternate"]
    #[inline]
    pub fn rcosc_mf_reg_sel(&mut self) -> _RCOSC_MF_REG_SELW {
        _RCOSC_MF_REG_SELW { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
    #[inline]
    pub fn rcosc_mf_res_coarse(&mut self) -> _RCOSC_MF_RES_COARSEW {
        _RCOSC_MF_RES_COARSEW { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
    #[inline]
    pub fn rcosc_mf_res_fine(&mut self) -> _RCOSC_MF_RES_FINEW {
        _RCOSC_MF_RES_FINEW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Adjusts bias current to RCOSC_MF. 0x8 minimum current 0x0 default current 0x7 maximum current"]
    #[inline]
    pub fn rcosc_mf_bias_adj(&mut self) -> _RCOSC_MF_BIAS_ADJW {
        _RCOSC_MF_BIAS_ADJW { w: self }
    }
}
