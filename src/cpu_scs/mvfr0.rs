#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MVFR0 {
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
pub struct FP_ROUNDING_MODESR {
    bits: u8,
}
impl FP_ROUNDING_MODESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SHORT_VECTORSR {
    bits: u8,
}
impl SHORT_VECTORSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SQUARE_ROOTR {
    bits: u8,
}
impl SQUARE_ROOTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIVIDER {
    bits: u8,
}
impl DIVIDER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FP_EXCEPTION_TRAPPINGR {
    bits: u8,
}
impl FP_EXCEPTION_TRAPPINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DOUBLE_PRECISIONR {
    bits: u8,
}
impl DOUBLE_PRECISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SINGLE_PRECISIONR {
    bits: u8,
}
impl SINGLE_PRECISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct A_SIMDR {
    bits: u8,
}
impl A_SIMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FP_ROUNDING_MODESW<'a> {
    w: &'a mut W,
}
impl<'a> _FP_ROUNDING_MODESW<'a> {
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
pub struct _SHORT_VECTORSW<'a> {
    w: &'a mut W,
}
impl<'a> _SHORT_VECTORSW<'a> {
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
pub struct _SQUARE_ROOTW<'a> {
    w: &'a mut W,
}
impl<'a> _SQUARE_ROOTW<'a> {
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
pub struct _DIVIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVIDEW<'a> {
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
pub struct _FP_EXCEPTION_TRAPPINGW<'a> {
    w: &'a mut W,
}
impl<'a> _FP_EXCEPTION_TRAPPINGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DOUBLE_PRECISIONW<'a> {
    w: &'a mut W,
}
impl<'a> _DOUBLE_PRECISIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SINGLE_PRECISIONW<'a> {
    w: &'a mut W,
}
impl<'a> _SINGLE_PRECISIONW<'a> {
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
pub struct _A_SIMDW<'a> {
    w: &'a mut W,
}
impl<'a> _A_SIMDW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
    #[inline]
    pub fn fp_rounding_modes(&self) -> FP_ROUNDING_MODESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FP_ROUNDING_MODESR { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
    #[inline]
    pub fn short_vectors(&self) -> SHORT_VECTORSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SHORT_VECTORSR { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
    #[inline]
    pub fn square_root(&self) -> SQUARE_ROOTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQUARE_ROOTR { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
    #[inline]
    pub fn divide(&self) -> DIVIDER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVIDER { bits }
    }
    #[doc = "Bits 12:15 - 15:12\\] Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
    #[inline]
    pub fn fp_exception_trapping(&self) -> FP_EXCEPTION_TRAPPINGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FP_EXCEPTION_TRAPPINGR { bits }
    }
    #[doc = "Bits 8:11 - 11:8\\] Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
    #[inline]
    pub fn double_precision(&self) -> DOUBLE_PRECISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DOUBLE_PRECISIONR { bits }
    }
    #[doc = "Bits 4:7 - 7:4\\] Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
    #[inline]
    pub fn single_precision(&self) -> SINGLE_PRECISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SINGLE_PRECISIONR { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
    #[inline]
    pub fn a_simd(&self) -> A_SIMDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        A_SIMDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 269549601 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
    #[inline]
    pub fn fp_rounding_modes(&mut self) -> _FP_ROUNDING_MODESW {
        _FP_ROUNDING_MODESW { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
    #[inline]
    pub fn short_vectors(&mut self) -> _SHORT_VECTORSW {
        _SHORT_VECTORSW { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
    #[inline]
    pub fn square_root(&mut self) -> _SQUARE_ROOTW {
        _SQUARE_ROOTW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
    #[inline]
    pub fn divide(&mut self) -> _DIVIDEW {
        _DIVIDEW { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
    #[inline]
    pub fn fp_exception_trapping(&mut self) -> _FP_EXCEPTION_TRAPPINGW {
        _FP_EXCEPTION_TRAPPINGW { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\] Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
    #[inline]
    pub fn double_precision(&mut self) -> _DOUBLE_PRECISIONW {
        _DOUBLE_PRECISIONW { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
    #[inline]
    pub fn single_precision(&mut self) -> _SINGLE_PRECISIONW {
        _SINGLE_PRECISIONW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
    #[inline]
    pub fn a_simd(&mut self) -> _A_SIMDW {
        _A_SIMDW { w: self }
    }
}
