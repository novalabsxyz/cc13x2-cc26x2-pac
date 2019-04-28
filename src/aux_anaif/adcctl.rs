#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADCCTL {
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
pub struct RESERVED15R {
    bits: u32,
}
impl RESERVED15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `START_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_POLR {
    #[doc = "Set ADC trigger on falling edge of event source."]
    FALL,
    #[doc = "Set ADC trigger on rising edge of event source."]
    RISE,
}
impl START_POLR {
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
            START_POLR::FALL => true,
            START_POLR::RISE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> START_POLR {
        match value {
            true => START_POLR::FALL,
            false => START_POLR::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == START_POLR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == START_POLR::RISE
    }
}
#[doc = "Possible values of the field `START_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_SRCR {
    #[doc = "No event."]
    NO_EVENT,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AUX_SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    AUX_ISRC_RESET_N,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    AUX_TDC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    AUX_TIMER0_EV,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    AUX_TIMER1_EV,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0,
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    MCU_EV,
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    ACLK_REF,
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    VDDR_RECHARGE,
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    MCU_ACTIVE,
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    PWR_DWN,
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    SCLK_LF,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    AON_BATMON_TEMP_UPD,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    AON_BATMON_BAT_UPD,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    AON_RTC_4KHZ,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    AON_RTC_CH2,
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    MANUAL_EV,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    AUXIO31,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    AUXIO30,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    AUXIO29,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    AUXIO28,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    AUXIO27,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    AUXIO26,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    AUXIO25,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    AUXIO24,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    AUXIO23,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    AUXIO22,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    AUXIO21,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    AUXIO20,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    AUXIO19,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    AUXIO18,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    AUXIO17,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    AUXIO16,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    AUXIO15,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    AUXIO14,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    AUXIO13,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    AUXIO12,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    AUXIO11,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    AUXIO10,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    AUXIO9,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    AUXIO8,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    AUXIO7,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    AUXIO6,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    AUXIO5,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    AUXIO4,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    AUXIO3,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    AUXIO0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl START_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            START_SRCR::NO_EVENT => 63,
            START_SRCR::AUX_SMPH_AUTOTAKE_DONE => 61,
            START_SRCR::AUX_ISRC_RESET_N => 56,
            START_SRCR::AUX_TDC_DONE => 55,
            START_SRCR::AUX_TIMER0_EV => 54,
            START_SRCR::AUX_TIMER1_EV => 53,
            START_SRCR::AUX_TIMER2_PULSE => 52,
            START_SRCR::AUX_TIMER2_EV3 => 51,
            START_SRCR::AUX_TIMER2_EV2 => 50,
            START_SRCR::AUX_TIMER2_EV1 => 49,
            START_SRCR::AUX_TIMER2_EV0 => 48,
            START_SRCR::AUX_COMPB => 47,
            START_SRCR::AUX_COMPA => 46,
            START_SRCR::MCU_EV => 43,
            START_SRCR::ACLK_REF => 42,
            START_SRCR::VDDR_RECHARGE => 41,
            START_SRCR::MCU_ACTIVE => 40,
            START_SRCR::PWR_DWN => 39,
            START_SRCR::SCLK_LF => 38,
            START_SRCR::AON_BATMON_TEMP_UPD => 37,
            START_SRCR::AON_BATMON_BAT_UPD => 36,
            START_SRCR::AON_RTC_4KHZ => 35,
            START_SRCR::AON_RTC_CH2_DLY => 34,
            START_SRCR::AON_RTC_CH2 => 33,
            START_SRCR::MANUAL_EV => 32,
            START_SRCR::AUXIO31 => 31,
            START_SRCR::AUXIO30 => 30,
            START_SRCR::AUXIO29 => 29,
            START_SRCR::AUXIO28 => 28,
            START_SRCR::AUXIO27 => 27,
            START_SRCR::AUXIO26 => 26,
            START_SRCR::AUXIO25 => 25,
            START_SRCR::AUXIO24 => 24,
            START_SRCR::AUXIO23 => 23,
            START_SRCR::AUXIO22 => 22,
            START_SRCR::AUXIO21 => 21,
            START_SRCR::AUXIO20 => 20,
            START_SRCR::AUXIO19 => 19,
            START_SRCR::AUXIO18 => 18,
            START_SRCR::AUXIO17 => 17,
            START_SRCR::AUXIO16 => 16,
            START_SRCR::AUXIO15 => 15,
            START_SRCR::AUXIO14 => 14,
            START_SRCR::AUXIO13 => 13,
            START_SRCR::AUXIO12 => 12,
            START_SRCR::AUXIO11 => 11,
            START_SRCR::AUXIO10 => 10,
            START_SRCR::AUXIO9 => 9,
            START_SRCR::AUXIO8 => 8,
            START_SRCR::AUXIO7 => 7,
            START_SRCR::AUXIO6 => 6,
            START_SRCR::AUXIO5 => 5,
            START_SRCR::AUXIO4 => 4,
            START_SRCR::AUXIO3 => 3,
            START_SRCR::AUXIO2 => 2,
            START_SRCR::AUXIO1 => 1,
            START_SRCR::AUXIO0 => 0,
            START_SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> START_SRCR {
        match value {
            63 => START_SRCR::NO_EVENT,
            61 => START_SRCR::AUX_SMPH_AUTOTAKE_DONE,
            56 => START_SRCR::AUX_ISRC_RESET_N,
            55 => START_SRCR::AUX_TDC_DONE,
            54 => START_SRCR::AUX_TIMER0_EV,
            53 => START_SRCR::AUX_TIMER1_EV,
            52 => START_SRCR::AUX_TIMER2_PULSE,
            51 => START_SRCR::AUX_TIMER2_EV3,
            50 => START_SRCR::AUX_TIMER2_EV2,
            49 => START_SRCR::AUX_TIMER2_EV1,
            48 => START_SRCR::AUX_TIMER2_EV0,
            47 => START_SRCR::AUX_COMPB,
            46 => START_SRCR::AUX_COMPA,
            43 => START_SRCR::MCU_EV,
            42 => START_SRCR::ACLK_REF,
            41 => START_SRCR::VDDR_RECHARGE,
            40 => START_SRCR::MCU_ACTIVE,
            39 => START_SRCR::PWR_DWN,
            38 => START_SRCR::SCLK_LF,
            37 => START_SRCR::AON_BATMON_TEMP_UPD,
            36 => START_SRCR::AON_BATMON_BAT_UPD,
            35 => START_SRCR::AON_RTC_4KHZ,
            34 => START_SRCR::AON_RTC_CH2_DLY,
            33 => START_SRCR::AON_RTC_CH2,
            32 => START_SRCR::MANUAL_EV,
            31 => START_SRCR::AUXIO31,
            30 => START_SRCR::AUXIO30,
            29 => START_SRCR::AUXIO29,
            28 => START_SRCR::AUXIO28,
            27 => START_SRCR::AUXIO27,
            26 => START_SRCR::AUXIO26,
            25 => START_SRCR::AUXIO25,
            24 => START_SRCR::AUXIO24,
            23 => START_SRCR::AUXIO23,
            22 => START_SRCR::AUXIO22,
            21 => START_SRCR::AUXIO21,
            20 => START_SRCR::AUXIO20,
            19 => START_SRCR::AUXIO19,
            18 => START_SRCR::AUXIO18,
            17 => START_SRCR::AUXIO17,
            16 => START_SRCR::AUXIO16,
            15 => START_SRCR::AUXIO15,
            14 => START_SRCR::AUXIO14,
            13 => START_SRCR::AUXIO13,
            12 => START_SRCR::AUXIO12,
            11 => START_SRCR::AUXIO11,
            10 => START_SRCR::AUXIO10,
            9 => START_SRCR::AUXIO9,
            8 => START_SRCR::AUXIO8,
            7 => START_SRCR::AUXIO7,
            6 => START_SRCR::AUXIO6,
            5 => START_SRCR::AUXIO5,
            4 => START_SRCR::AUXIO4,
            3 => START_SRCR::AUXIO3,
            2 => START_SRCR::AUXIO2,
            1 => START_SRCR::AUXIO1,
            0 => START_SRCR::AUXIO0,
            i => START_SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline]
    pub fn is_no_event(&self) -> bool {
        *self == START_SRCR::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == START_SRCR::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ISRC_RESET_N`"]
    #[inline]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == START_SRCR::AUX_ISRC_RESET_N
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == START_SRCR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == START_SRCR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == START_SRCR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == START_SRCR::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == START_SRCR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == START_SRCR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == START_SRCR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == START_SRCR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == START_SRCR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == START_SRCR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == START_SRCR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == START_SRCR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `VDDR_RECHARGE`"]
    #[inline]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == START_SRCR::VDDR_RECHARGE
    }
    #[doc = "Checks if the value of the field is `MCU_ACTIVE`"]
    #[inline]
    pub fn is_mcu_active(&self) -> bool {
        *self == START_SRCR::MCU_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PWR_DWN`"]
    #[inline]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == START_SRCR::PWR_DWN
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline]
    pub fn is_sclk_lf(&self) -> bool {
        *self == START_SRCR::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_TEMP_UPD`"]
    #[inline]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == START_SRCR::AON_BATMON_TEMP_UPD
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_BAT_UPD`"]
    #[inline]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == START_SRCR::AON_BATMON_BAT_UPD
    }
    #[doc = "Checks if the value of the field is `AON_RTC_4KHZ`"]
    #[inline]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == START_SRCR::AON_RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == START_SRCR::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == START_SRCR::AON_RTC_CH2
    }
    #[doc = "Checks if the value of the field is `MANUAL_EV`"]
    #[inline]
    pub fn is_manual_ev(&self) -> bool {
        *self == START_SRCR::MANUAL_EV
    }
    #[doc = "Checks if the value of the field is `AUXIO31`"]
    #[inline]
    pub fn is_auxio31(&self) -> bool {
        *self == START_SRCR::AUXIO31
    }
    #[doc = "Checks if the value of the field is `AUXIO30`"]
    #[inline]
    pub fn is_auxio30(&self) -> bool {
        *self == START_SRCR::AUXIO30
    }
    #[doc = "Checks if the value of the field is `AUXIO29`"]
    #[inline]
    pub fn is_auxio29(&self) -> bool {
        *self == START_SRCR::AUXIO29
    }
    #[doc = "Checks if the value of the field is `AUXIO28`"]
    #[inline]
    pub fn is_auxio28(&self) -> bool {
        *self == START_SRCR::AUXIO28
    }
    #[doc = "Checks if the value of the field is `AUXIO27`"]
    #[inline]
    pub fn is_auxio27(&self) -> bool {
        *self == START_SRCR::AUXIO27
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline]
    pub fn is_auxio26(&self) -> bool {
        *self == START_SRCR::AUXIO26
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline]
    pub fn is_auxio25(&self) -> bool {
        *self == START_SRCR::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline]
    pub fn is_auxio24(&self) -> bool {
        *self == START_SRCR::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline]
    pub fn is_auxio23(&self) -> bool {
        *self == START_SRCR::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline]
    pub fn is_auxio22(&self) -> bool {
        *self == START_SRCR::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline]
    pub fn is_auxio21(&self) -> bool {
        *self == START_SRCR::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline]
    pub fn is_auxio20(&self) -> bool {
        *self == START_SRCR::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline]
    pub fn is_auxio19(&self) -> bool {
        *self == START_SRCR::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO18`"]
    #[inline]
    pub fn is_auxio18(&self) -> bool {
        *self == START_SRCR::AUXIO18
    }
    #[doc = "Checks if the value of the field is `AUXIO17`"]
    #[inline]
    pub fn is_auxio17(&self) -> bool {
        *self == START_SRCR::AUXIO17
    }
    #[doc = "Checks if the value of the field is `AUXIO16`"]
    #[inline]
    pub fn is_auxio16(&self) -> bool {
        *self == START_SRCR::AUXIO16
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == START_SRCR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == START_SRCR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == START_SRCR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == START_SRCR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == START_SRCR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == START_SRCR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == START_SRCR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == START_SRCR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == START_SRCR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == START_SRCR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == START_SRCR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == START_SRCR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == START_SRCR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == START_SRCR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == START_SRCR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == START_SRCR::AUXIO0
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: u8,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "Flush ADC FIFO.\n\nYou must set CMD to EN or DIS after flush.\n\nSystem CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    FLUSH,
    #[doc = "Enable ADC interface."]
    EN,
    #[doc = "Disable ADC interface."]
    DIS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::FLUSH => 3,
            CMDR::EN => 1,
            CMDR::DIS => 0,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            3 => CMDR::FLUSH,
            1 => CMDR::EN,
            0 => CMDR::DIS,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline]
    pub fn is_flush(&self) -> bool {
        *self == CMDR::FLUSH
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CMDR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CMDR::DIS
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED15W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 131071;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `START_POL`"]
pub enum START_POLW {
    #[doc = "Set ADC trigger on falling edge of event source."]
    FALL,
    #[doc = "Set ADC trigger on rising edge of event source."]
    RISE,
}
impl START_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            START_POLW::FALL => true,
            START_POLW::RISE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _START_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _START_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: START_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set ADC trigger on falling edge of event source."]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(START_POLW::FALL)
    }
    #[doc = "Set ADC trigger on rising edge of event source."]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(START_POLW::RISE)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `START_SRC`"]
pub enum START_SRCW {
    #[doc = "No event."]
    NO_EVENT,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AUX_SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    AUX_ISRC_RESET_N,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    AUX_TDC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    AUX_TIMER0_EV,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    AUX_TIMER1_EV,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0,
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    MCU_EV,
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    ACLK_REF,
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    VDDR_RECHARGE,
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    MCU_ACTIVE,
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    PWR_DWN,
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    SCLK_LF,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    AON_BATMON_TEMP_UPD,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    AON_BATMON_BAT_UPD,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    AON_RTC_4KHZ,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY,
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    AON_RTC_CH2,
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    MANUAL_EV,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    AUXIO31,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    AUXIO30,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    AUXIO29,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    AUXIO28,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    AUXIO27,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    AUXIO26,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    AUXIO25,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    AUXIO24,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    AUXIO23,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    AUXIO22,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    AUXIO21,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    AUXIO20,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    AUXIO19,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    AUXIO18,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    AUXIO17,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    AUXIO16,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    AUXIO15,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    AUXIO14,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    AUXIO13,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    AUXIO12,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    AUXIO11,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    AUXIO10,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    AUXIO9,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    AUXIO8,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    AUXIO7,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    AUXIO6,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    AUXIO5,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    AUXIO4,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    AUXIO3,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    AUXIO0,
}
impl START_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            START_SRCW::NO_EVENT => 63,
            START_SRCW::AUX_SMPH_AUTOTAKE_DONE => 61,
            START_SRCW::AUX_ISRC_RESET_N => 56,
            START_SRCW::AUX_TDC_DONE => 55,
            START_SRCW::AUX_TIMER0_EV => 54,
            START_SRCW::AUX_TIMER1_EV => 53,
            START_SRCW::AUX_TIMER2_PULSE => 52,
            START_SRCW::AUX_TIMER2_EV3 => 51,
            START_SRCW::AUX_TIMER2_EV2 => 50,
            START_SRCW::AUX_TIMER2_EV1 => 49,
            START_SRCW::AUX_TIMER2_EV0 => 48,
            START_SRCW::AUX_COMPB => 47,
            START_SRCW::AUX_COMPA => 46,
            START_SRCW::MCU_EV => 43,
            START_SRCW::ACLK_REF => 42,
            START_SRCW::VDDR_RECHARGE => 41,
            START_SRCW::MCU_ACTIVE => 40,
            START_SRCW::PWR_DWN => 39,
            START_SRCW::SCLK_LF => 38,
            START_SRCW::AON_BATMON_TEMP_UPD => 37,
            START_SRCW::AON_BATMON_BAT_UPD => 36,
            START_SRCW::AON_RTC_4KHZ => 35,
            START_SRCW::AON_RTC_CH2_DLY => 34,
            START_SRCW::AON_RTC_CH2 => 33,
            START_SRCW::MANUAL_EV => 32,
            START_SRCW::AUXIO31 => 31,
            START_SRCW::AUXIO30 => 30,
            START_SRCW::AUXIO29 => 29,
            START_SRCW::AUXIO28 => 28,
            START_SRCW::AUXIO27 => 27,
            START_SRCW::AUXIO26 => 26,
            START_SRCW::AUXIO25 => 25,
            START_SRCW::AUXIO24 => 24,
            START_SRCW::AUXIO23 => 23,
            START_SRCW::AUXIO22 => 22,
            START_SRCW::AUXIO21 => 21,
            START_SRCW::AUXIO20 => 20,
            START_SRCW::AUXIO19 => 19,
            START_SRCW::AUXIO18 => 18,
            START_SRCW::AUXIO17 => 17,
            START_SRCW::AUXIO16 => 16,
            START_SRCW::AUXIO15 => 15,
            START_SRCW::AUXIO14 => 14,
            START_SRCW::AUXIO13 => 13,
            START_SRCW::AUXIO12 => 12,
            START_SRCW::AUXIO11 => 11,
            START_SRCW::AUXIO10 => 10,
            START_SRCW::AUXIO9 => 9,
            START_SRCW::AUXIO8 => 8,
            START_SRCW::AUXIO7 => 7,
            START_SRCW::AUXIO6 => 6,
            START_SRCW::AUXIO5 => 5,
            START_SRCW::AUXIO4 => 4,
            START_SRCW::AUXIO3 => 3,
            START_SRCW::AUXIO2 => 2,
            START_SRCW::AUXIO1 => 1,
            START_SRCW::AUXIO0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _START_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _START_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: START_SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event."]
    #[inline]
    pub fn no_event(self) -> &'a mut W {
        self.variant(START_SRCW::NO_EVENT)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline]
    pub fn aux_isrc_reset_n(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_ISRC_RESET_N)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_TDC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_TIMER0_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_TIMER2_PULSE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_TIMER2_EV3)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_TIMER2_EV2)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_TIMER2_EV1)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_TIMER2_EV0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(START_SRCW::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(START_SRCW::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline]
    pub fn vddr_recharge(self) -> &'a mut W {
        self.variant(START_SRCW::VDDR_RECHARGE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline]
    pub fn mcu_active(self) -> &'a mut W {
        self.variant(START_SRCW::MCU_ACTIVE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline]
    pub fn pwr_dwn(self) -> &'a mut W {
        self.variant(START_SRCW::PWR_DWN)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(START_SRCW::SCLK_LF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline]
    pub fn aon_batmon_temp_upd(self) -> &'a mut W {
        self.variant(START_SRCW::AON_BATMON_TEMP_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline]
    pub fn aon_batmon_bat_upd(self) -> &'a mut W {
        self.variant(START_SRCW::AON_BATMON_BAT_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline]
    pub fn aon_rtc_4khz(self) -> &'a mut W {
        self.variant(START_SRCW::AON_RTC_4KHZ)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(START_SRCW::AON_RTC_CH2_DLY)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(START_SRCW::AON_RTC_CH2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline]
    pub fn manual_ev(self) -> &'a mut W {
        self.variant(START_SRCW::MANUAL_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline]
    pub fn auxio31(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline]
    pub fn auxio30(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline]
    pub fn auxio29(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline]
    pub fn auxio28(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline]
    pub fn auxio27(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline]
    pub fn auxio18(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline]
    pub fn auxio17(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline]
    pub fn auxio16(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "Flush ADC FIFO.\n\nYou must set CMD to EN or DIS after flush.\n\nSystem CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    FLUSH,
    #[doc = "Enable ADC interface."]
    EN,
    #[doc = "Disable ADC interface."]
    DIS,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::FLUSH => 3,
            CMDW::EN => 1,
            CMDW::DIS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    #[inline]
    pub fn flush(self) -> &'a mut W {
        self.variant(CMDW::FLUSH)
    }
    #[doc = "Enable ADC interface."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CMDW::EN)
    }
    #[doc = "Disable ADC interface."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CMDW::DIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 15:31 - 31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved15(&self) -> RESERVED15R {
        let bits = {
            const MASK: u32 = 131071;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED15R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] Select active polarity for START_SRC event."]
    #[inline]
    pub fn start_pol(&self) -> START_POLR {
        START_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:13 - 13:8\\] Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT if you want to trigger the ADC manually through ADCTRIG.START. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline]
    pub fn start_src(&self) -> START_SRCR {
        START_SRCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:7 - 7:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
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
        W { bits: 16128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 15:31 - 31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved15(&mut self) -> _RESERVED15W {
        _RESERVED15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Select active polarity for START_SRC event."]
    #[inline]
    pub fn start_pol(&mut self) -> _START_POLW {
        _START_POLW { w: self }
    }
    #[doc = "Bits 8:13 - 13:8\\] Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT if you want to trigger the ADC manually through ADCTRIG.START. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline]
    pub fn start_src(&mut self) -> _START_SRCW {
        _START_SRCW { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
