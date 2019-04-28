#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct RESERVED7R {
    bits: u32,
}
impl RESERVED7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH3_RESETR {
    bits: bool,
}
impl CH3_RESETR {
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
pub struct CH2_RESETR {
    bits: bool,
}
impl CH2_RESETR {
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
pub struct CH1_RESETR {
    bits: bool,
}
impl CH1_RESETR {
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
pub struct CH0_RESETR {
    bits: bool,
}
impl CH0_RESETR {
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
#[doc = "Possible values of the field `TARGET_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARGET_ENR {
    #[doc = "TARGET.VALUE"]
    TARGET,
    #[doc = "65535"]
    CNTR_MAX,
}
impl TARGET_ENR {
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
            TARGET_ENR::TARGET => true,
            TARGET_ENR::CNTR_MAX => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TARGET_ENR {
        match value {
            true => TARGET_ENR::TARGET,
            false => TARGET_ENR::CNTR_MAX,
        }
    }
    #[doc = "Checks if the value of the field is `TARGET`"]
    #[inline]
    pub fn is_target(&self) -> bool {
        *self == TARGET_ENR::TARGET
    }
    #[doc = "Checks if the value of the field is `CNTR_MAX`"]
    #[inline]
    pub fn is_cntr_max(&self) -> bool {
        *self == TARGET_ENR::CNTR_MAX
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly.\n\nPeriod =  (target value * 2) * timer clock period"]
    UPDWN_PER,
    #[doc = "Count up periodically. The timer increments from 0 to target value, repeatedly.\n\nPeriod = (target value + 1) * timer clock period"]
    UP_PER,
    #[doc = "Count up once. The timer increments from 0 to target value,  then stops and sets MODE to DIS."]
    UP_ONCE,
    #[doc = "Disable timer. Updates to counter, channels, and events stop."]
    DIS,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::UPDWN_PER => 3,
            MODER::UP_PER => 2,
            MODER::UP_ONCE => 1,
            MODER::DIS => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            3 => MODER::UPDWN_PER,
            2 => MODER::UP_PER,
            1 => MODER::UP_ONCE,
            0 => MODER::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UPDWN_PER`"]
    #[inline]
    pub fn is_updwn_per(&self) -> bool {
        *self == MODER::UPDWN_PER
    }
    #[doc = "Checks if the value of the field is `UP_PER`"]
    #[inline]
    pub fn is_up_per(&self) -> bool {
        *self == MODER::UP_PER
    }
    #[doc = "Checks if the value of the field is `UP_ONCE`"]
    #[inline]
    pub fn is_up_once(&self) -> bool {
        *self == MODER::UP_ONCE
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == MODER::DIS
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED7W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 33554431;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH3_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_RESETW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_RESETW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH1_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_RESETW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH0_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0_RESETW<'a> {
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
#[doc = "Values that can be written to the field `TARGET_EN`"]
pub enum TARGET_ENW {
    #[doc = "TARGET.VALUE"]
    TARGET,
    #[doc = "65535"]
    CNTR_MAX,
}
impl TARGET_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TARGET_ENW::TARGET => true,
            TARGET_ENW::CNTR_MAX => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TARGET_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TARGET_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TARGET_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TARGET.VALUE"]
    #[inline]
    pub fn target(self) -> &'a mut W {
        self.variant(TARGET_ENW::TARGET)
    }
    #[doc = "65535"]
    #[inline]
    pub fn cntr_max(self) -> &'a mut W {
        self.variant(TARGET_ENW::CNTR_MAX)
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly.\n\nPeriod =  (target value * 2) * timer clock period"]
    UPDWN_PER,
    #[doc = "Count up periodically. The timer increments from 0 to target value, repeatedly.\n\nPeriod = (target value + 1) * timer clock period"]
    UP_PER,
    #[doc = "Count up once. The timer increments from 0 to target value,  then stops and sets MODE to DIS."]
    UP_ONCE,
    #[doc = "Disable timer. Updates to counter, channels, and events stop."]
    DIS,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::UPDWN_PER => 3,
            MODEW::UP_PER => 2,
            MODEW::UP_ONCE => 1,
            MODEW::DIS => 0,
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
    #[doc = "Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline]
    pub fn updwn_per(self) -> &'a mut W {
        self.variant(MODEW::UPDWN_PER)
    }
    #[doc = "Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    #[inline]
    pub fn up_per(self) -> &'a mut W {
        self.variant(MODEW::UP_PER)
    }
    #[doc = "Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    #[inline]
    pub fn up_once(self) -> &'a mut W {
        self.variant(MODEW::UP_ONCE)
    }
    #[doc = "Disable timer. Updates to counter, channels, and events stop."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(MODEW::DIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 7:31 - 31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved7(&self) -> RESERVED7R {
        let bits = {
            const MASK: u32 = 33554431;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED7R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
    #[inline]
    pub fn ch3_reset(&self) -> CH3_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH3_RESETR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
    #[inline]
    pub fn ch2_reset(&self) -> CH2_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH2_RESETR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
    #[inline]
    pub fn ch1_reset(&self) -> CH1_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH1_RESETR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
    #[inline]
    pub fn ch0_reset(&self) -> CH0_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH0_RESETR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Select counter target value. You must select TARGET to use shadow target functionality."]
    #[inline]
    pub fn target_en(&self) -> TARGET_ENR {
        TARGET_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:1 - 1:0\\] Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\] and TARGET clear."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 7:31 - 31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved7(&mut self) -> _RESERVED7W {
        _RESERVED7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
    #[inline]
    pub fn ch3_reset(&mut self) -> _CH3_RESETW {
        _CH3_RESETW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
    #[inline]
    pub fn ch2_reset(&mut self) -> _CH2_RESETW {
        _CH2_RESETW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
    #[inline]
    pub fn ch1_reset(&mut self) -> _CH1_RESETW {
        _CH1_RESETW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
    #[inline]
    pub fn ch0_reset(&mut self) -> _CH0_RESETW {
        _CH0_RESETW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Select counter target value. You must select TARGET to use shadow target functionality."]
    #[inline]
    pub fn target_en(&mut self) -> _TARGET_ENW {
        _TARGET_ENW { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\] and TARGET clear."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
