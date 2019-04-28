#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_IPR8 {
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
pub struct PRI_35R {
    bits: u8,
}
impl PRI_35R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_34R {
    bits: u8,
}
impl PRI_34R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_33R {
    bits: u8,
}
impl PRI_33R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_32R {
    bits: u8,
}
impl PRI_32R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PRI_35W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_35W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_34W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_34W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_33W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_33W<'a> {
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
pub struct _PRI_32W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_32W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline]
    pub fn pri_35(&self) -> PRI_35R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_35R { bits }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline]
    pub fn pri_34(&self) -> PRI_34R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_34R { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline]
    pub fn pri_33(&self) -> PRI_33R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_33R { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline]
    pub fn pri_32(&self) -> PRI_32R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_32R { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline]
    pub fn pri_35(&mut self) -> _PRI_35W {
        _PRI_35W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline]
    pub fn pri_34(&mut self) -> _PRI_34W {
        _PRI_34W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline]
    pub fn pri_33(&mut self) -> _PRI_33W {
        _PRI_33W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline]
    pub fn pri_32(&mut self) -> _PRI_32W {
        _PRI_32W { w: self }
    }
}
