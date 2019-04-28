#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTCSEL {
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
pub struct RESERVED6R {
    bits: u32,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `RTC_CH1_CAPT_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CH1_CAPT_EVR {
    #[doc = "0"]
    NONE,
    #[doc = "0"]
    AUX_COMPB_ASYNC_N,
    #[doc = "0"]
    AUX_COMPB_ASYNC,
    #[doc = "0"]
    BATMON_VOLT,
    #[doc = "0"]
    BATMON_TEMP,
    #[doc = "0"]
    AUX_TIMER1_EV,
    #[doc = "0"]
    AUX_TIMER0_EV,
    #[doc = "0"]
    AUX_TDC_DONE,
    #[doc = "0"]
    AUX_ADC_DONE,
    #[doc = "0"]
    AUX_COMPB,
    #[doc = "0"]
    AUX_COMPA,
    #[doc = "0"]
    AUX_SWEV2,
    #[doc = "0"]
    AUX_SWEV1,
    #[doc = "0"]
    AUX_SWEV0,
    #[doc = "0"]
    JTAG,
    #[doc = "0"]
    RTC_UPD,
    #[doc = "0"]
    RTC_COMB_DLY,
    #[doc = "0"]
    RTC_CH2_DLY,
    #[doc = "0"]
    RTC_CH1_DLY,
    #[doc = "0"]
    RTC_CH0_DLY,
    #[doc = "0"]
    RTC_CH2,
    #[doc = "0"]
    RTC_CH1,
    #[doc = "0"]
    RTC_CH0,
    #[doc = "0"]
    PAD,
    #[doc = "0"]
    BATMON_COMBINED,
    #[doc = "0"]
    BATMON_TEMP_LL,
    #[doc = "0"]
    BATMON_TEMP_UL,
    #[doc = "0"]
    BATMON_BATT_LL,
    #[doc = "0"]
    BATMON_BATT_UL,
    #[doc = "0"]
    AUX_TIMER2_EV3,
    #[doc = "0"]
    AUX_TIMER2_EV2,
    #[doc = "0"]
    AUX_TIMER2_EV1,
    #[doc = "0"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_RTC in \\[MCU_IOC:IOCFGx.IOEV_RTC_EN\\]"]
    IOEV_RTC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RTC_CH1_CAPT_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTC_CH1_CAPT_EVR::NONE => 63,
            RTC_CH1_CAPT_EVR::AUX_COMPB_ASYNC_N => 56,
            RTC_CH1_CAPT_EVR::AUX_COMPB_ASYNC => 55,
            RTC_CH1_CAPT_EVR::BATMON_VOLT => 54,
            RTC_CH1_CAPT_EVR::BATMON_TEMP => 53,
            RTC_CH1_CAPT_EVR::AUX_TIMER1_EV => 52,
            RTC_CH1_CAPT_EVR::AUX_TIMER0_EV => 51,
            RTC_CH1_CAPT_EVR::AUX_TDC_DONE => 50,
            RTC_CH1_CAPT_EVR::AUX_ADC_DONE => 49,
            RTC_CH1_CAPT_EVR::AUX_COMPB => 48,
            RTC_CH1_CAPT_EVR::AUX_COMPA => 47,
            RTC_CH1_CAPT_EVR::AUX_SWEV2 => 46,
            RTC_CH1_CAPT_EVR::AUX_SWEV1 => 45,
            RTC_CH1_CAPT_EVR::AUX_SWEV0 => 44,
            RTC_CH1_CAPT_EVR::JTAG => 43,
            RTC_CH1_CAPT_EVR::RTC_UPD => 42,
            RTC_CH1_CAPT_EVR::RTC_COMB_DLY => 41,
            RTC_CH1_CAPT_EVR::RTC_CH2_DLY => 40,
            RTC_CH1_CAPT_EVR::RTC_CH1_DLY => 39,
            RTC_CH1_CAPT_EVR::RTC_CH0_DLY => 38,
            RTC_CH1_CAPT_EVR::RTC_CH2 => 37,
            RTC_CH1_CAPT_EVR::RTC_CH1 => 36,
            RTC_CH1_CAPT_EVR::RTC_CH0 => 35,
            RTC_CH1_CAPT_EVR::PAD => 32,
            RTC_CH1_CAPT_EVR::BATMON_COMBINED => 9,
            RTC_CH1_CAPT_EVR::BATMON_TEMP_LL => 8,
            RTC_CH1_CAPT_EVR::BATMON_TEMP_UL => 7,
            RTC_CH1_CAPT_EVR::BATMON_BATT_LL => 6,
            RTC_CH1_CAPT_EVR::BATMON_BATT_UL => 5,
            RTC_CH1_CAPT_EVR::AUX_TIMER2_EV3 => 4,
            RTC_CH1_CAPT_EVR::AUX_TIMER2_EV2 => 3,
            RTC_CH1_CAPT_EVR::AUX_TIMER2_EV1 => 2,
            RTC_CH1_CAPT_EVR::AUX_TIMER2_EV0 => 1,
            RTC_CH1_CAPT_EVR::IOEV_RTC => 0,
            RTC_CH1_CAPT_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTC_CH1_CAPT_EVR {
        match value {
            63 => RTC_CH1_CAPT_EVR::NONE,
            56 => RTC_CH1_CAPT_EVR::AUX_COMPB_ASYNC_N,
            55 => RTC_CH1_CAPT_EVR::AUX_COMPB_ASYNC,
            54 => RTC_CH1_CAPT_EVR::BATMON_VOLT,
            53 => RTC_CH1_CAPT_EVR::BATMON_TEMP,
            52 => RTC_CH1_CAPT_EVR::AUX_TIMER1_EV,
            51 => RTC_CH1_CAPT_EVR::AUX_TIMER0_EV,
            50 => RTC_CH1_CAPT_EVR::AUX_TDC_DONE,
            49 => RTC_CH1_CAPT_EVR::AUX_ADC_DONE,
            48 => RTC_CH1_CAPT_EVR::AUX_COMPB,
            47 => RTC_CH1_CAPT_EVR::AUX_COMPA,
            46 => RTC_CH1_CAPT_EVR::AUX_SWEV2,
            45 => RTC_CH1_CAPT_EVR::AUX_SWEV1,
            44 => RTC_CH1_CAPT_EVR::AUX_SWEV0,
            43 => RTC_CH1_CAPT_EVR::JTAG,
            42 => RTC_CH1_CAPT_EVR::RTC_UPD,
            41 => RTC_CH1_CAPT_EVR::RTC_COMB_DLY,
            40 => RTC_CH1_CAPT_EVR::RTC_CH2_DLY,
            39 => RTC_CH1_CAPT_EVR::RTC_CH1_DLY,
            38 => RTC_CH1_CAPT_EVR::RTC_CH0_DLY,
            37 => RTC_CH1_CAPT_EVR::RTC_CH2,
            36 => RTC_CH1_CAPT_EVR::RTC_CH1,
            35 => RTC_CH1_CAPT_EVR::RTC_CH0,
            32 => RTC_CH1_CAPT_EVR::PAD,
            9 => RTC_CH1_CAPT_EVR::BATMON_COMBINED,
            8 => RTC_CH1_CAPT_EVR::BATMON_TEMP_LL,
            7 => RTC_CH1_CAPT_EVR::BATMON_TEMP_UL,
            6 => RTC_CH1_CAPT_EVR::BATMON_BATT_LL,
            5 => RTC_CH1_CAPT_EVR::BATMON_BATT_UL,
            4 => RTC_CH1_CAPT_EVR::AUX_TIMER2_EV3,
            3 => RTC_CH1_CAPT_EVR::AUX_TIMER2_EV2,
            2 => RTC_CH1_CAPT_EVR::AUX_TIMER2_EV1,
            1 => RTC_CH1_CAPT_EVR::AUX_TIMER2_EV0,
            0 => RTC_CH1_CAPT_EVR::IOEV_RTC,
            i => RTC_CH1_CAPT_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline]
    pub fn is_batmon_combined(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_RTC`"]
    #[inline]
    pub fn is_ioev_rtc(&self) -> bool {
        *self == RTC_CH1_CAPT_EVR::IOEV_RTC
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 67108863;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_CH1_CAPT_EV`"]
pub enum RTC_CH1_CAPT_EVW {
    #[doc = "0"]
    NONE,
    #[doc = "0"]
    AUX_COMPB_ASYNC_N,
    #[doc = "0"]
    AUX_COMPB_ASYNC,
    #[doc = "0"]
    BATMON_VOLT,
    #[doc = "0"]
    BATMON_TEMP,
    #[doc = "0"]
    AUX_TIMER1_EV,
    #[doc = "0"]
    AUX_TIMER0_EV,
    #[doc = "0"]
    AUX_TDC_DONE,
    #[doc = "0"]
    AUX_ADC_DONE,
    #[doc = "0"]
    AUX_COMPB,
    #[doc = "0"]
    AUX_COMPA,
    #[doc = "0"]
    AUX_SWEV2,
    #[doc = "0"]
    AUX_SWEV1,
    #[doc = "0"]
    AUX_SWEV0,
    #[doc = "0"]
    JTAG,
    #[doc = "0"]
    RTC_UPD,
    #[doc = "0"]
    RTC_COMB_DLY,
    #[doc = "0"]
    RTC_CH2_DLY,
    #[doc = "0"]
    RTC_CH1_DLY,
    #[doc = "0"]
    RTC_CH0_DLY,
    #[doc = "0"]
    RTC_CH2,
    #[doc = "0"]
    RTC_CH1,
    #[doc = "0"]
    RTC_CH0,
    #[doc = "0"]
    PAD,
    #[doc = "0"]
    BATMON_COMBINED,
    #[doc = "0"]
    BATMON_TEMP_LL,
    #[doc = "0"]
    BATMON_TEMP_UL,
    #[doc = "0"]
    BATMON_BATT_LL,
    #[doc = "0"]
    BATMON_BATT_UL,
    #[doc = "0"]
    AUX_TIMER2_EV3,
    #[doc = "0"]
    AUX_TIMER2_EV2,
    #[doc = "0"]
    AUX_TIMER2_EV1,
    #[doc = "0"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_RTC in \\[MCU_IOC:IOCFGx.IOEV_RTC_EN\\]"]
    IOEV_RTC,
}
impl RTC_CH1_CAPT_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTC_CH1_CAPT_EVW::NONE => 63,
            RTC_CH1_CAPT_EVW::AUX_COMPB_ASYNC_N => 56,
            RTC_CH1_CAPT_EVW::AUX_COMPB_ASYNC => 55,
            RTC_CH1_CAPT_EVW::BATMON_VOLT => 54,
            RTC_CH1_CAPT_EVW::BATMON_TEMP => 53,
            RTC_CH1_CAPT_EVW::AUX_TIMER1_EV => 52,
            RTC_CH1_CAPT_EVW::AUX_TIMER0_EV => 51,
            RTC_CH1_CAPT_EVW::AUX_TDC_DONE => 50,
            RTC_CH1_CAPT_EVW::AUX_ADC_DONE => 49,
            RTC_CH1_CAPT_EVW::AUX_COMPB => 48,
            RTC_CH1_CAPT_EVW::AUX_COMPA => 47,
            RTC_CH1_CAPT_EVW::AUX_SWEV2 => 46,
            RTC_CH1_CAPT_EVW::AUX_SWEV1 => 45,
            RTC_CH1_CAPT_EVW::AUX_SWEV0 => 44,
            RTC_CH1_CAPT_EVW::JTAG => 43,
            RTC_CH1_CAPT_EVW::RTC_UPD => 42,
            RTC_CH1_CAPT_EVW::RTC_COMB_DLY => 41,
            RTC_CH1_CAPT_EVW::RTC_CH2_DLY => 40,
            RTC_CH1_CAPT_EVW::RTC_CH1_DLY => 39,
            RTC_CH1_CAPT_EVW::RTC_CH0_DLY => 38,
            RTC_CH1_CAPT_EVW::RTC_CH2 => 37,
            RTC_CH1_CAPT_EVW::RTC_CH1 => 36,
            RTC_CH1_CAPT_EVW::RTC_CH0 => 35,
            RTC_CH1_CAPT_EVW::PAD => 32,
            RTC_CH1_CAPT_EVW::BATMON_COMBINED => 9,
            RTC_CH1_CAPT_EVW::BATMON_TEMP_LL => 8,
            RTC_CH1_CAPT_EVW::BATMON_TEMP_UL => 7,
            RTC_CH1_CAPT_EVW::BATMON_BATT_LL => 6,
            RTC_CH1_CAPT_EVW::BATMON_BATT_UL => 5,
            RTC_CH1_CAPT_EVW::AUX_TIMER2_EV3 => 4,
            RTC_CH1_CAPT_EVW::AUX_TIMER2_EV2 => 3,
            RTC_CH1_CAPT_EVW::AUX_TIMER2_EV1 => 2,
            RTC_CH1_CAPT_EVW::AUX_TIMER2_EV0 => 1,
            RTC_CH1_CAPT_EVW::IOEV_RTC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_CH1_CAPT_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_CH1_CAPT_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_CH1_CAPT_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::NONE)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "0"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::BATMON_VOLT)
    }
    #[doc = "0"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::BATMON_TEMP)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_TIMER1_EV)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_TIMER0_EV)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_TDC_DONE)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_ADC_DONE)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_COMPB)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_COMPA)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_SWEV2)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_SWEV1)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_SWEV0)
    }
    #[doc = "0"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::JTAG)
    }
    #[doc = "0"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::RTC_UPD)
    }
    #[doc = "0"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::RTC_COMB_DLY)
    }
    #[doc = "0"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::RTC_CH2_DLY)
    }
    #[doc = "0"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::RTC_CH1_DLY)
    }
    #[doc = "0"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::RTC_CH0_DLY)
    }
    #[doc = "0"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::RTC_CH2)
    }
    #[doc = "0"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::RTC_CH1)
    }
    #[doc = "0"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::RTC_CH0)
    }
    #[doc = "0"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::PAD)
    }
    #[doc = "0"]
    #[inline]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::BATMON_COMBINED)
    }
    #[doc = "0"]
    #[inline]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::BATMON_TEMP_LL)
    }
    #[doc = "0"]
    #[inline]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::BATMON_TEMP_UL)
    }
    #[doc = "0"]
    #[inline]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::BATMON_BATT_LL)
    }
    #[doc = "0"]
    #[inline]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::BATMON_BATT_UL)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_TIMER2_EV3)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_TIMER2_EV2)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_TIMER2_EV1)
    }
    #[doc = "0"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_RTC in \\[MCU_IOC:IOCFGx.IOEV_RTC_EN\\]"]
    #[inline]
    pub fn ioev_rtc(self) -> &'a mut W {
        self.variant(RTC_CH1_CAPT_EVW::IOEV_RTC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u32 = 67108863;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED6R { bits }
    }
    #[doc = "Bits 0:5 - 5:0\\] AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
    #[inline]
    pub fn rtc_ch1_capt_ev(&self) -> RTC_CH1_CAPT_EVR {
        RTC_CH1_CAPT_EVR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 63 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
    #[inline]
    pub fn rtc_ch1_capt_ev(&mut self) -> _RTC_CH1_CAPT_EVW {
        _RTC_CH1_CAPT_EVW { w: self }
    }
}
