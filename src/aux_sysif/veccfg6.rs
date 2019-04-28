#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VECCFG6 {
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
#[doc = "Possible values of the field `VEC_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VEC_EVR {
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY,
    #[doc = "WUFLAGS.SW_WU3"]
    SW_WU3,
    #[doc = "WUFLAGS.SW_WU2"]
    SW_WU2,
    #[doc = "WUFLAGS.SW_WU1"]
    SW_WU1,
    #[doc = "WUFLAGS.SW_WU0"]
    SW_WU0,
    #[doc = "WUFLAGS.PROG_WU3"]
    PROG_WU3,
    #[doc = "WUFLAGS.PROG_WU2"]
    PROG_WU2,
    #[doc = "WUFLAGS.PROG_WU1"]
    PROG_WU1,
    #[doc = "WUFLAGS.PROG_WU0"]
    PROG_WU0,
    #[doc = "Vector is disabled."]
    NONE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VEC_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VEC_EVR::AON_RTC_CH2_DLY => 9,
            VEC_EVR::SW_WU3 => 8,
            VEC_EVR::SW_WU2 => 7,
            VEC_EVR::SW_WU1 => 6,
            VEC_EVR::SW_WU0 => 5,
            VEC_EVR::PROG_WU3 => 4,
            VEC_EVR::PROG_WU2 => 3,
            VEC_EVR::PROG_WU1 => 2,
            VEC_EVR::PROG_WU0 => 1,
            VEC_EVR::NONE => 0,
            VEC_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VEC_EVR {
        match value {
            9 => VEC_EVR::AON_RTC_CH2_DLY,
            8 => VEC_EVR::SW_WU3,
            7 => VEC_EVR::SW_WU2,
            6 => VEC_EVR::SW_WU1,
            5 => VEC_EVR::SW_WU0,
            4 => VEC_EVR::PROG_WU3,
            3 => VEC_EVR::PROG_WU2,
            2 => VEC_EVR::PROG_WU1,
            1 => VEC_EVR::PROG_WU0,
            0 => VEC_EVR::NONE,
            i => VEC_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == VEC_EVR::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `SW_WU3`"]
    #[inline]
    pub fn is_sw_wu3(&self) -> bool {
        *self == VEC_EVR::SW_WU3
    }
    #[doc = "Checks if the value of the field is `SW_WU2`"]
    #[inline]
    pub fn is_sw_wu2(&self) -> bool {
        *self == VEC_EVR::SW_WU2
    }
    #[doc = "Checks if the value of the field is `SW_WU1`"]
    #[inline]
    pub fn is_sw_wu1(&self) -> bool {
        *self == VEC_EVR::SW_WU1
    }
    #[doc = "Checks if the value of the field is `SW_WU0`"]
    #[inline]
    pub fn is_sw_wu0(&self) -> bool {
        *self == VEC_EVR::SW_WU0
    }
    #[doc = "Checks if the value of the field is `PROG_WU3`"]
    #[inline]
    pub fn is_prog_wu3(&self) -> bool {
        *self == VEC_EVR::PROG_WU3
    }
    #[doc = "Checks if the value of the field is `PROG_WU2`"]
    #[inline]
    pub fn is_prog_wu2(&self) -> bool {
        *self == VEC_EVR::PROG_WU2
    }
    #[doc = "Checks if the value of the field is `PROG_WU1`"]
    #[inline]
    pub fn is_prog_wu1(&self) -> bool {
        *self == VEC_EVR::PROG_WU1
    }
    #[doc = "Checks if the value of the field is `PROG_WU0`"]
    #[inline]
    pub fn is_prog_wu0(&self) -> bool {
        *self == VEC_EVR::PROG_WU0
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == VEC_EVR::NONE
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
#[doc = "Values that can be written to the field `VEC_EV`"]
pub enum VEC_EVW {
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY,
    #[doc = "WUFLAGS.SW_WU3"]
    SW_WU3,
    #[doc = "WUFLAGS.SW_WU2"]
    SW_WU2,
    #[doc = "WUFLAGS.SW_WU1"]
    SW_WU1,
    #[doc = "WUFLAGS.SW_WU0"]
    SW_WU0,
    #[doc = "WUFLAGS.PROG_WU3"]
    PROG_WU3,
    #[doc = "WUFLAGS.PROG_WU2"]
    PROG_WU2,
    #[doc = "WUFLAGS.PROG_WU1"]
    PROG_WU1,
    #[doc = "WUFLAGS.PROG_WU0"]
    PROG_WU0,
    #[doc = "Vector is disabled."]
    NONE,
}
impl VEC_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VEC_EVW::AON_RTC_CH2_DLY => 9,
            VEC_EVW::SW_WU3 => 8,
            VEC_EVW::SW_WU2 => 7,
            VEC_EVW::SW_WU1 => 6,
            VEC_EVW::SW_WU0 => 5,
            VEC_EVW::PROG_WU3 => 4,
            VEC_EVW::PROG_WU2 => 3,
            VEC_EVW::PROG_WU1 => 2,
            VEC_EVW::PROG_WU0 => 1,
            VEC_EVW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VEC_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _VEC_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VEC_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(VEC_EVW::AON_RTC_CH2_DLY)
    }
    #[doc = "WUFLAGS.SW_WU3"]
    #[inline]
    pub fn sw_wu3(self) -> &'a mut W {
        self.variant(VEC_EVW::SW_WU3)
    }
    #[doc = "WUFLAGS.SW_WU2"]
    #[inline]
    pub fn sw_wu2(self) -> &'a mut W {
        self.variant(VEC_EVW::SW_WU2)
    }
    #[doc = "WUFLAGS.SW_WU1"]
    #[inline]
    pub fn sw_wu1(self) -> &'a mut W {
        self.variant(VEC_EVW::SW_WU1)
    }
    #[doc = "WUFLAGS.SW_WU0"]
    #[inline]
    pub fn sw_wu0(self) -> &'a mut W {
        self.variant(VEC_EVW::SW_WU0)
    }
    #[doc = "WUFLAGS.PROG_WU3"]
    #[inline]
    pub fn prog_wu3(self) -> &'a mut W {
        self.variant(VEC_EVW::PROG_WU3)
    }
    #[doc = "WUFLAGS.PROG_WU2"]
    #[inline]
    pub fn prog_wu2(self) -> &'a mut W {
        self.variant(VEC_EVW::PROG_WU2)
    }
    #[doc = "WUFLAGS.PROG_WU1"]
    #[inline]
    pub fn prog_wu1(self) -> &'a mut W {
        self.variant(VEC_EVW::PROG_WU1)
    }
    #[doc = "WUFLAGS.PROG_WU0"]
    #[inline]
    pub fn prog_wu0(self) -> &'a mut W {
        self.variant(VEC_EVW::PROG_WU0)
    }
    #[doc = "Vector is disabled."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(VEC_EVW::NONE)
    }
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
    #[doc = "Bits 0:3 - 3:0\\] Select trigger event for vector 6. Non-enumerated values are treated as NONE."]
    #[inline]
    pub fn vec_ev(&self) -> VEC_EVR {
        VEC_EVR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Select trigger event for vector 6. Non-enumerated values are treated as NONE."]
    #[inline]
    pub fn vec_ev(&mut self) -> _VEC_EVW {
        _VEC_EVW { w: self }
    }
}
