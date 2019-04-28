#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RECHARGECFG {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "External recharge comparator. \nNote that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting  this recharge algorithm."]
    COMPARATOR,
    #[doc = "Adaptive timer"]
    ADAPTIVE,
    #[doc = "Static timer"]
    STATIC,
    #[doc = "Recharge disabled"]
    OFF,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::COMPARATOR => 3,
            MODER::ADAPTIVE => 2,
            MODER::STATIC => 1,
            MODER::OFF => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            3 => MODER::COMPARATOR,
            2 => MODER::ADAPTIVE,
            1 => MODER::STATIC,
            0 => MODER::OFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARATOR`"]
    #[inline]
    pub fn is_comparator(&self) -> bool {
        *self == MODER::COMPARATOR
    }
    #[doc = "Checks if the value of the field is `ADAPTIVE`"]
    #[inline]
    pub fn is_adaptive(&self) -> bool {
        *self == MODER::ADAPTIVE
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline]
    pub fn is_static_(&self) -> bool {
        *self == MODER::STATIC
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == MODER::OFF
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED24R {
    bits: u8,
}
impl RESERVED24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C2R {
    bits: u8,
}
impl C2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C1R {
    bits: u8,
}
impl C1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAX_PER_MR {
    bits: u8,
}
impl MAX_PER_MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAX_PER_ER {
    bits: u8,
}
impl MAX_PER_ER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_MR {
    bits: u8,
}
impl PER_MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_ER {
    bits: u8,
}
impl PER_ER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "External recharge comparator. \nNote that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting  this recharge algorithm."]
    COMPARATOR,
    #[doc = "Adaptive timer"]
    ADAPTIVE,
    #[doc = "Static timer"]
    STATIC,
    #[doc = "Recharge disabled"]
    OFF,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::COMPARATOR => 3,
            MODEW::ADAPTIVE => 2,
            MODEW::STATIC => 1,
            MODEW::OFF => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External recharge comparator. Note that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting this recharge algorithm."]
    #[inline]
    pub fn comparator(self) -> &'a mut W {
        self.variant(MODEW::COMPARATOR)
    }
    #[doc = "Adaptive timer"]
    #[inline]
    pub fn adaptive(self) -> &'a mut W {
        self.variant(MODEW::ADAPTIVE)
    }
    #[doc = "Static timer"]
    #[inline]
    pub fn static_(self) -> &'a mut W {
        self.variant(MODEW::STATIC)
    }
    #[doc = "Recharge disabled"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(MODEW::OFF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED24W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED24W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C2W<'a> {
    w: &'a mut W,
}
impl<'a> _C2W<'a> {
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
pub struct _C1W<'a> {
    w: &'a mut W,
}
impl<'a> _C1W<'a> {
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
pub struct _MAX_PER_MW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_PER_MW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAX_PER_EW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_PER_EW<'a> {
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
pub struct _PER_MW<'a> {
    w: &'a mut W,
}
impl<'a> _PER_MW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PER_EW<'a> {
    w: &'a mut W,
}
impl<'a> _PER_EW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 30:31 - 31:30\\] Selects recharge algorithm for VDDR when the system is running on the uLDO"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - 29:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&self) -> RESERVED24R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED24R { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn c2(&self) -> C2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C2R { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn c1(&self) -> C1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C1R { bits }
    }
    #[doc = "Bits 11:15 - 15:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn max_per_m(&self) -> MAX_PER_MR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_PER_MR { bits }
    }
    #[doc = "Bits 8:10 - 10:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn max_per_e(&self) -> MAX_PER_ER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_PER_ER { bits }
    }
    #[doc = "Bits 3:7 - 7:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_m(&self) -> PER_MR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_MR { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_e(&self) -> PER_ER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_ER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3221225472 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - 31:30\\] Selects recharge algorithm for VDDR when the system is running on the uLDO"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 24:29 - 29:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&mut self) -> _RESERVED24W {
        _RESERVED24W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn c2(&mut self) -> _C2W {
        _C2W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn c1(&mut self) -> _C1W {
        _C1W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn max_per_m(&mut self) -> _MAX_PER_MW {
        _MAX_PER_MW { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn max_per_e(&mut self) -> _MAX_PER_EW {
        _MAX_PER_EW { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_m(&mut self) -> _PER_MW {
        _PER_MW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_e(&mut self) -> _PER_EW {
        _PER_EW { w: self }
    }
}
