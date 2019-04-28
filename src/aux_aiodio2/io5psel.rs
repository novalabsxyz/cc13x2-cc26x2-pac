#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IO5PSEL {
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
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    AUX_TIMER2_PULSE,
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    AUX_TIMER2_EV3,
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    AUX_TIMER2_EV2,
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    AUX_TIMER2_EV1,
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    AUX_TIMER2_EV0,
    #[doc = "Peripheral output mux selects AUX_SPIM MOSI."]
    AUX_SPIM_MOSI,
    #[doc = "Peripheral output mux selects AUX_SPIM SCLK."]
    AUX_SPIM_SCLK,
    #[doc = "Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    AUX_EV_OBS,
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::AUX_TIMER2_PULSE => 7,
            SRCR::AUX_TIMER2_EV3 => 6,
            SRCR::AUX_TIMER2_EV2 => 5,
            SRCR::AUX_TIMER2_EV1 => 4,
            SRCR::AUX_TIMER2_EV0 => 3,
            SRCR::AUX_SPIM_MOSI => 2,
            SRCR::AUX_SPIM_SCLK => 1,
            SRCR::AUX_EV_OBS => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            7 => SRCR::AUX_TIMER2_PULSE,
            6 => SRCR::AUX_TIMER2_EV3,
            5 => SRCR::AUX_TIMER2_EV2,
            4 => SRCR::AUX_TIMER2_EV1,
            3 => SRCR::AUX_TIMER2_EV0,
            2 => SRCR::AUX_SPIM_MOSI,
            1 => SRCR::AUX_SPIM_SCLK,
            0 => SRCR::AUX_EV_OBS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == SRCR::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == SRCR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == SRCR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == SRCR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == SRCR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_SPIM_MOSI`"]
    #[inline]
    pub fn is_aux_spim_mosi(&self) -> bool {
        *self == SRCR::AUX_SPIM_MOSI
    }
    #[doc = "Checks if the value of the field is `AUX_SPIM_SCLK`"]
    #[inline]
    pub fn is_aux_spim_sclk(&self) -> bool {
        *self == SRCR::AUX_SPIM_SCLK
    }
    #[doc = "Checks if the value of the field is `AUX_EV_OBS`"]
    #[inline]
    pub fn is_aux_ev_obs(&self) -> bool {
        *self == SRCR::AUX_EV_OBS
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
#[doc = "Values that can be written to the field `SRC`"]
pub enum SRCW {
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    AUX_TIMER2_PULSE,
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    AUX_TIMER2_EV3,
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    AUX_TIMER2_EV2,
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    AUX_TIMER2_EV1,
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    AUX_TIMER2_EV0,
    #[doc = "Peripheral output mux selects AUX_SPIM MOSI."]
    AUX_SPIM_MOSI,
    #[doc = "Peripheral output mux selects AUX_SPIM SCLK."]
    AUX_SPIM_SCLK,
    #[doc = "Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    AUX_EV_OBS,
}
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCW::AUX_TIMER2_PULSE => 7,
            SRCW::AUX_TIMER2_EV3 => 6,
            SRCW::AUX_TIMER2_EV2 => 5,
            SRCW::AUX_TIMER2_EV1 => 4,
            SRCW::AUX_TIMER2_EV0 => 3,
            SRCW::AUX_SPIM_MOSI => 2,
            SRCW::AUX_SPIM_SCLK => 1,
            SRCW::AUX_EV_OBS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    #[inline]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_PULSE)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_EV3)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_EV2)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_EV1)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_EV0)
    }
    #[doc = "Peripheral output mux selects AUX_SPIM MOSI."]
    #[inline]
    pub fn aux_spim_mosi(self) -> &'a mut W {
        self.variant(SRCW::AUX_SPIM_MOSI)
    }
    #[doc = "Peripheral output mux selects AUX_SPIM SCLK."]
    #[inline]
    pub fn aux_spim_sclk(self) -> &'a mut W {
        self.variant(SRCW::AUX_SPIM_SCLK)
    }
    #[doc = "Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    #[inline]
    pub fn aux_ev_obs(self) -> &'a mut W {
        self.variant(SRCW::AUX_EV_OBS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:2 - 2:0\\] Select a peripheral signal that connects to AUXIO\\[8i+5\\] when IOPOE bit 5 is set."]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Select a peripheral signal that connects to AUXIO\\[8i+5\\] when IOPOE bit 5 is set."]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
}
