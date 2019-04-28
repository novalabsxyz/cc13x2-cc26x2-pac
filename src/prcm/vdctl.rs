#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VDCTL {
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
pub struct SPARE1R {
    bits: u32,
}
impl SPARE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ULDOR {
    bits: bool,
}
impl ULDOR {
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
pub struct _SPARE1W<'a> {
    w: &'a mut W,
}
impl<'a> _SPARE1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 2147483647;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ULDOW<'a> {
    w: &'a mut W,
}
impl<'a> _ULDOW<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare1(&self) -> SPARE1R {
        let bits = {
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        SPARE1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and S.CRYPTO_CLK_EN\\] = 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
    #[inline]
    pub fn uldo(&self) -> ULDOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ULDOR { bits }
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
    #[doc = "Bits 1:31 - 31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare1(&mut self) -> _SPARE1W {
        _SPARE1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and S.CRYPTO_CLK_EN\\] = 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
    #[inline]
    pub fn uldo(&mut self) -> _ULDOW {
        _ULDOW { w: self }
    }
}
