#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVSYNCRATE {
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
pub struct RESERVED3R {
    bits: u32,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `AUX_COMPA_SYNC_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPA_SYNC_RATER {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl AUX_COMPA_SYNC_RATER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_COMPA_SYNC_RATER::BUS_RATE => true,
            AUX_COMPA_SYNC_RATER::SCE_RATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_COMPA_SYNC_RATER {
        match value {
            true => AUX_COMPA_SYNC_RATER::BUS_RATE,
            false => AUX_COMPA_SYNC_RATER::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline]
    pub fn is_bus_rate(&self) -> bool {
        *self == AUX_COMPA_SYNC_RATER::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline]
    pub fn is_sce_rate(&self) -> bool {
        *self == AUX_COMPA_SYNC_RATER::SCE_RATE
    }
}
#[doc = "Possible values of the field `AUX_COMPB_SYNC_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPB_SYNC_RATER {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl AUX_COMPB_SYNC_RATER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_COMPB_SYNC_RATER::BUS_RATE => true,
            AUX_COMPB_SYNC_RATER::SCE_RATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_COMPB_SYNC_RATER {
        match value {
            true => AUX_COMPB_SYNC_RATER::BUS_RATE,
            false => AUX_COMPB_SYNC_RATER::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline]
    pub fn is_bus_rate(&self) -> bool {
        *self == AUX_COMPB_SYNC_RATER::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline]
    pub fn is_sce_rate(&self) -> bool {
        *self == AUX_COMPB_SYNC_RATER::SCE_RATE
    }
}
#[doc = "Possible values of the field `AUX_TIMER2_SYNC_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_SYNC_RATER {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl AUX_TIMER2_SYNC_RATER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUX_TIMER2_SYNC_RATER::BUS_RATE => true,
            AUX_TIMER2_SYNC_RATER::SCE_RATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_TIMER2_SYNC_RATER {
        match value {
            true => AUX_TIMER2_SYNC_RATER::BUS_RATE,
            false => AUX_TIMER2_SYNC_RATER::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline]
    pub fn is_bus_rate(&self) -> bool {
        *self == AUX_TIMER2_SYNC_RATER::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline]
    pub fn is_sce_rate(&self) -> bool {
        *self == AUX_TIMER2_SYNC_RATER::SCE_RATE
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_COMPA_SYNC_RATE`"]
pub enum AUX_COMPA_SYNC_RATEW {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl AUX_COMPA_SYNC_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_COMPA_SYNC_RATEW::BUS_RATE => true,
            AUX_COMPA_SYNC_RATEW::SCE_RATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_COMPA_SYNC_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPA_SYNC_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_COMPA_SYNC_RATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(AUX_COMPA_SYNC_RATEW::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(AUX_COMPA_SYNC_RATEW::SCE_RATE)
    }
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_COMPB_SYNC_RATE`"]
pub enum AUX_COMPB_SYNC_RATEW {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl AUX_COMPB_SYNC_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_COMPB_SYNC_RATEW::BUS_RATE => true,
            AUX_COMPB_SYNC_RATEW::SCE_RATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_COMPB_SYNC_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPB_SYNC_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_COMPB_SYNC_RATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(AUX_COMPB_SYNC_RATEW::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(AUX_COMPB_SYNC_RATEW::SCE_RATE)
    }
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUX_TIMER2_SYNC_RATE`"]
pub enum AUX_TIMER2_SYNC_RATEW {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl AUX_TIMER2_SYNC_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_TIMER2_SYNC_RATEW::BUS_RATE => true,
            AUX_TIMER2_SYNC_RATEW::SCE_RATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_TIMER2_SYNC_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_TIMER2_SYNC_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_TIMER2_SYNC_RATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(AUX_TIMER2_SYNC_RATEW::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(AUX_TIMER2_SYNC_RATEW::SCE_RATE)
    }
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
    #[inline]
    pub fn aux_compa_sync_rate(&self) -> AUX_COMPA_SYNC_RATER {
        AUX_COMPA_SYNC_RATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
    #[inline]
    pub fn aux_compb_sync_rate(&self) -> AUX_COMPB_SYNC_RATER {
        AUX_COMPB_SYNC_RATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline]
    pub fn aux_timer2_sync_rate(&self) -> AUX_TIMER2_SYNC_RATER {
        AUX_TIMER2_SYNC_RATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
    #[inline]
    pub fn aux_compa_sync_rate(&mut self) -> _AUX_COMPA_SYNC_RATEW {
        _AUX_COMPA_SYNC_RATEW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
    #[inline]
    pub fn aux_compb_sync_rate(&mut self) -> _AUX_COMPB_SYNC_RATEW {
        _AUX_COMPB_SYNC_RATEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline]
    pub fn aux_timer2_sync_rate(&mut self) -> _AUX_TIMER2_SYNC_RATEW {
        _AUX_TIMER2_SYNC_RATEW { w: self }
    }
}
