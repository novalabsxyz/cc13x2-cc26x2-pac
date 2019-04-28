#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MVFR1 {
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
pub struct FP_FUSED_MACR {
    bits: u8,
}
impl FP_FUSED_MACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FP_HPFPR {
    bits: u8,
}
impl FP_HPFPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED8R {
    bits: u16,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct D_NAN_MODER {
    bits: u8,
}
impl D_NAN_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FTZ_MODER {
    bits: u8,
}
impl FTZ_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FP_FUSED_MACW<'a> {
    w: &'a mut W,
}
impl<'a> _FP_FUSED_MACW<'a> {
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
pub struct _FP_HPFPW<'a> {
    w: &'a mut W,
}
impl<'a> _FP_HPFPW<'a> {
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
pub struct _RESERVED8W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _D_NAN_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _D_NAN_MODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FTZ_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FTZ_MODEW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
    #[inline]
    pub fn fp_fused_mac(&self) -> FP_FUSED_MACR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FP_FUSED_MACR { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
    #[inline]
    pub fn fp_hpfp(&self) -> FP_HPFPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FP_HPFPR { bits }
    }
    #[doc = "Bits 8:23 - 23:8\\] Software should not rely on the value of a reserved."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED8R { bits }
    }
    #[doc = "Bits 4:7 - 7:4\\] Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
    #[inline]
    pub fn d_nan_mode(&self) -> D_NAN_MODER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        D_NAN_MODER { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
    #[inline]
    pub fn ftz_mode(&self) -> FTZ_MODER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FTZ_MODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 285212689 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
    #[inline]
    pub fn fp_fused_mac(&mut self) -> _FP_FUSED_MACW {
        _FP_FUSED_MACW { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
    #[inline]
    pub fn fp_hpfp(&mut self) -> _FP_HPFPW {
        _FP_HPFPW { w: self }
    }
    #[doc = "Bits 8:23 - 23:8\\] Software should not rely on the value of a reserved."]
    #[inline]
    pub fn reserved8(&mut self) -> _RESERVED8W {
        _RESERVED8W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
    #[inline]
    pub fn d_nan_mode(&mut self) -> _D_NAN_MODEW {
        _D_NAN_MODEW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
    #[inline]
    pub fn ftz_mode(&mut self) -> _FTZ_MODEW {
        _FTZ_MODEW { w: self }
    }
}
