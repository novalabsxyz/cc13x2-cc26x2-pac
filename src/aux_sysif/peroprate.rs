#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PEROPRATE {
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
pub struct RESERVED4R {
    bits: u32,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `ANAIF_DAC_OP_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANAIF_DAC_OP_RATER {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl ANAIF_DAC_OP_RATER {
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
            ANAIF_DAC_OP_RATER::BUS_RATE => true,
            ANAIF_DAC_OP_RATER::SCE_RATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANAIF_DAC_OP_RATER {
        match value {
            true => ANAIF_DAC_OP_RATER::BUS_RATE,
            false => ANAIF_DAC_OP_RATER::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline]
    pub fn is_bus_rate(&self) -> bool {
        *self == ANAIF_DAC_OP_RATER::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline]
    pub fn is_sce_rate(&self) -> bool {
        *self == ANAIF_DAC_OP_RATER::SCE_RATE
    }
}
#[doc = "Possible values of the field `TIMER01_OP_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER01_OP_RATER {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl TIMER01_OP_RATER {
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
            TIMER01_OP_RATER::BUS_RATE => true,
            TIMER01_OP_RATER::SCE_RATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER01_OP_RATER {
        match value {
            true => TIMER01_OP_RATER::BUS_RATE,
            false => TIMER01_OP_RATER::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline]
    pub fn is_bus_rate(&self) -> bool {
        *self == TIMER01_OP_RATER::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline]
    pub fn is_sce_rate(&self) -> bool {
        *self == TIMER01_OP_RATER::SCE_RATE
    }
}
#[doc = "Possible values of the field `SPIM_OP_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIM_OP_RATER {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl SPIM_OP_RATER {
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
            SPIM_OP_RATER::BUS_RATE => true,
            SPIM_OP_RATER::SCE_RATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPIM_OP_RATER {
        match value {
            true => SPIM_OP_RATER::BUS_RATE,
            false => SPIM_OP_RATER::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline]
    pub fn is_bus_rate(&self) -> bool {
        *self == SPIM_OP_RATER::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline]
    pub fn is_sce_rate(&self) -> bool {
        *self == SPIM_OP_RATER::SCE_RATE
    }
}
#[doc = "Possible values of the field `MAC_OP_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_OP_RATER {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl MAC_OP_RATER {
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
            MAC_OP_RATER::BUS_RATE => true,
            MAC_OP_RATER::SCE_RATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAC_OP_RATER {
        match value {
            true => MAC_OP_RATER::BUS_RATE,
            false => MAC_OP_RATER::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline]
    pub fn is_bus_rate(&self) -> bool {
        *self == MAC_OP_RATER::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline]
    pub fn is_sce_rate(&self) -> bool {
        *self == MAC_OP_RATER::SCE_RATE
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ANAIF_DAC_OP_RATE`"]
pub enum ANAIF_DAC_OP_RATEW {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl ANAIF_DAC_OP_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANAIF_DAC_OP_RATEW::BUS_RATE => true,
            ANAIF_DAC_OP_RATEW::SCE_RATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANAIF_DAC_OP_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _ANAIF_DAC_OP_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANAIF_DAC_OP_RATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(ANAIF_DAC_OP_RATEW::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(ANAIF_DAC_OP_RATEW::SCE_RATE)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMER01_OP_RATE`"]
pub enum TIMER01_OP_RATEW {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl TIMER01_OP_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER01_OP_RATEW::BUS_RATE => true,
            TIMER01_OP_RATEW::SCE_RATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER01_OP_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER01_OP_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER01_OP_RATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(TIMER01_OP_RATEW::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(TIMER01_OP_RATEW::SCE_RATE)
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
#[doc = "Values that can be written to the field `SPIM_OP_RATE`"]
pub enum SPIM_OP_RATEW {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl SPIM_OP_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPIM_OP_RATEW::BUS_RATE => true,
            SPIM_OP_RATEW::SCE_RATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPIM_OP_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIM_OP_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIM_OP_RATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(SPIM_OP_RATEW::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(SPIM_OP_RATEW::SCE_RATE)
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
#[doc = "Values that can be written to the field `MAC_OP_RATE`"]
pub enum MAC_OP_RATEW {
    #[doc = "AUX bus rate"]
    BUS_RATE,
    #[doc = "SCE rate"]
    SCE_RATE,
}
impl MAC_OP_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAC_OP_RATEW::BUS_RATE => true,
            MAC_OP_RATEW::SCE_RATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAC_OP_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _MAC_OP_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAC_OP_RATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(MAC_OP_RATEW::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(MAC_OP_RATEW::SCE_RATE)
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
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED4R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Select operational rate for AUX_ANAIF DAC sample clock state machine."]
    #[inline]
    pub fn anaif_dac_op_rate(&self) -> ANAIF_DAC_OP_RATER {
        ANAIF_DAC_OP_RATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] Select operational rate for AUX_TIMER01."]
    #[inline]
    pub fn timer01_op_rate(&self) -> TIMER01_OP_RATER {
        TIMER01_OP_RATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Select operational rate for AUX_SPIM."]
    #[inline]
    pub fn spim_op_rate(&self) -> SPIM_OP_RATER {
        SPIM_OP_RATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Select operational rate for AUX_MAC."]
    #[inline]
    pub fn mac_op_rate(&self) -> MAC_OP_RATER {
        MAC_OP_RATER::_from({
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
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Select operational rate for AUX_ANAIF DAC sample clock state machine."]
    #[inline]
    pub fn anaif_dac_op_rate(&mut self) -> _ANAIF_DAC_OP_RATEW {
        _ANAIF_DAC_OP_RATEW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Select operational rate for AUX_TIMER01."]
    #[inline]
    pub fn timer01_op_rate(&mut self) -> _TIMER01_OP_RATEW {
        _TIMER01_OP_RATEW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Select operational rate for AUX_SPIM."]
    #[inline]
    pub fn spim_op_rate(&mut self) -> _SPIM_OP_RATEW {
        _SPIM_OP_RATEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Select operational rate for AUX_MAC."]
    #[inline]
    pub fn mac_op_rate(&mut self) -> _MAC_OP_RATEW {
        _MAC_OP_RATEW { w: self }
    }
}
