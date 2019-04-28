#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH0CCFG {
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
#[doc = "Possible values of the field `CAPT_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_SRCR {
    #[doc = "No event."]
    NO_EVENT,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AUX_SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AUX_ADC_FIFO_NOT_EMPTY,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    AUX_ADC_IRQ,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    AUX_ADC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    AUX_ISRC_RESET_N,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    AUX_TDC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    AUX_TIMER0_EV,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    AUX_TIMER1_EV,
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
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    MCU_OBSMUX1,
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    MCU_OBSMUX0,
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
impl CAPT_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAPT_SRCR::NO_EVENT => 63,
            CAPT_SRCR::AUX_SMPH_AUTOTAKE_DONE => 61,
            CAPT_SRCR::AUX_ADC_FIFO_NOT_EMPTY => 60,
            CAPT_SRCR::AUX_ADC_FIFO_ALMOST_FULL => 59,
            CAPT_SRCR::AUX_ADC_IRQ => 58,
            CAPT_SRCR::AUX_ADC_DONE => 57,
            CAPT_SRCR::AUX_ISRC_RESET_N => 56,
            CAPT_SRCR::AUX_TDC_DONE => 55,
            CAPT_SRCR::AUX_TIMER0_EV => 54,
            CAPT_SRCR::AUX_TIMER1_EV => 53,
            CAPT_SRCR::AUX_TIMER2_EV3 => 51,
            CAPT_SRCR::AUX_TIMER2_EV2 => 50,
            CAPT_SRCR::AUX_TIMER2_EV1 => 49,
            CAPT_SRCR::AUX_TIMER2_EV0 => 48,
            CAPT_SRCR::AUX_COMPB => 47,
            CAPT_SRCR::AUX_COMPA => 46,
            CAPT_SRCR::MCU_OBSMUX1 => 45,
            CAPT_SRCR::MCU_OBSMUX0 => 44,
            CAPT_SRCR::MCU_EV => 43,
            CAPT_SRCR::ACLK_REF => 42,
            CAPT_SRCR::VDDR_RECHARGE => 41,
            CAPT_SRCR::MCU_ACTIVE => 40,
            CAPT_SRCR::PWR_DWN => 39,
            CAPT_SRCR::SCLK_LF => 38,
            CAPT_SRCR::AON_BATMON_TEMP_UPD => 37,
            CAPT_SRCR::AON_BATMON_BAT_UPD => 36,
            CAPT_SRCR::AON_RTC_4KHZ => 35,
            CAPT_SRCR::AON_RTC_CH2_DLY => 34,
            CAPT_SRCR::AON_RTC_CH2 => 33,
            CAPT_SRCR::MANUAL_EV => 32,
            CAPT_SRCR::AUXIO31 => 31,
            CAPT_SRCR::AUXIO30 => 30,
            CAPT_SRCR::AUXIO29 => 29,
            CAPT_SRCR::AUXIO28 => 28,
            CAPT_SRCR::AUXIO27 => 27,
            CAPT_SRCR::AUXIO26 => 26,
            CAPT_SRCR::AUXIO25 => 25,
            CAPT_SRCR::AUXIO24 => 24,
            CAPT_SRCR::AUXIO23 => 23,
            CAPT_SRCR::AUXIO22 => 22,
            CAPT_SRCR::AUXIO21 => 21,
            CAPT_SRCR::AUXIO20 => 20,
            CAPT_SRCR::AUXIO19 => 19,
            CAPT_SRCR::AUXIO18 => 18,
            CAPT_SRCR::AUXIO17 => 17,
            CAPT_SRCR::AUXIO16 => 16,
            CAPT_SRCR::AUXIO15 => 15,
            CAPT_SRCR::AUXIO14 => 14,
            CAPT_SRCR::AUXIO13 => 13,
            CAPT_SRCR::AUXIO12 => 12,
            CAPT_SRCR::AUXIO11 => 11,
            CAPT_SRCR::AUXIO10 => 10,
            CAPT_SRCR::AUXIO9 => 9,
            CAPT_SRCR::AUXIO8 => 8,
            CAPT_SRCR::AUXIO7 => 7,
            CAPT_SRCR::AUXIO6 => 6,
            CAPT_SRCR::AUXIO5 => 5,
            CAPT_SRCR::AUXIO4 => 4,
            CAPT_SRCR::AUXIO3 => 3,
            CAPT_SRCR::AUXIO2 => 2,
            CAPT_SRCR::AUXIO1 => 1,
            CAPT_SRCR::AUXIO0 => 0,
            CAPT_SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAPT_SRCR {
        match value {
            63 => CAPT_SRCR::NO_EVENT,
            61 => CAPT_SRCR::AUX_SMPH_AUTOTAKE_DONE,
            60 => CAPT_SRCR::AUX_ADC_FIFO_NOT_EMPTY,
            59 => CAPT_SRCR::AUX_ADC_FIFO_ALMOST_FULL,
            58 => CAPT_SRCR::AUX_ADC_IRQ,
            57 => CAPT_SRCR::AUX_ADC_DONE,
            56 => CAPT_SRCR::AUX_ISRC_RESET_N,
            55 => CAPT_SRCR::AUX_TDC_DONE,
            54 => CAPT_SRCR::AUX_TIMER0_EV,
            53 => CAPT_SRCR::AUX_TIMER1_EV,
            51 => CAPT_SRCR::AUX_TIMER2_EV3,
            50 => CAPT_SRCR::AUX_TIMER2_EV2,
            49 => CAPT_SRCR::AUX_TIMER2_EV1,
            48 => CAPT_SRCR::AUX_TIMER2_EV0,
            47 => CAPT_SRCR::AUX_COMPB,
            46 => CAPT_SRCR::AUX_COMPA,
            45 => CAPT_SRCR::MCU_OBSMUX1,
            44 => CAPT_SRCR::MCU_OBSMUX0,
            43 => CAPT_SRCR::MCU_EV,
            42 => CAPT_SRCR::ACLK_REF,
            41 => CAPT_SRCR::VDDR_RECHARGE,
            40 => CAPT_SRCR::MCU_ACTIVE,
            39 => CAPT_SRCR::PWR_DWN,
            38 => CAPT_SRCR::SCLK_LF,
            37 => CAPT_SRCR::AON_BATMON_TEMP_UPD,
            36 => CAPT_SRCR::AON_BATMON_BAT_UPD,
            35 => CAPT_SRCR::AON_RTC_4KHZ,
            34 => CAPT_SRCR::AON_RTC_CH2_DLY,
            33 => CAPT_SRCR::AON_RTC_CH2,
            32 => CAPT_SRCR::MANUAL_EV,
            31 => CAPT_SRCR::AUXIO31,
            30 => CAPT_SRCR::AUXIO30,
            29 => CAPT_SRCR::AUXIO29,
            28 => CAPT_SRCR::AUXIO28,
            27 => CAPT_SRCR::AUXIO27,
            26 => CAPT_SRCR::AUXIO26,
            25 => CAPT_SRCR::AUXIO25,
            24 => CAPT_SRCR::AUXIO24,
            23 => CAPT_SRCR::AUXIO23,
            22 => CAPT_SRCR::AUXIO22,
            21 => CAPT_SRCR::AUXIO21,
            20 => CAPT_SRCR::AUXIO20,
            19 => CAPT_SRCR::AUXIO19,
            18 => CAPT_SRCR::AUXIO18,
            17 => CAPT_SRCR::AUXIO17,
            16 => CAPT_SRCR::AUXIO16,
            15 => CAPT_SRCR::AUXIO15,
            14 => CAPT_SRCR::AUXIO14,
            13 => CAPT_SRCR::AUXIO13,
            12 => CAPT_SRCR::AUXIO12,
            11 => CAPT_SRCR::AUXIO11,
            10 => CAPT_SRCR::AUXIO10,
            9 => CAPT_SRCR::AUXIO9,
            8 => CAPT_SRCR::AUXIO8,
            7 => CAPT_SRCR::AUXIO7,
            6 => CAPT_SRCR::AUXIO6,
            5 => CAPT_SRCR::AUXIO5,
            4 => CAPT_SRCR::AUXIO4,
            3 => CAPT_SRCR::AUXIO3,
            2 => CAPT_SRCR::AUXIO2,
            1 => CAPT_SRCR::AUXIO1,
            0 => CAPT_SRCR::AUXIO0,
            i => CAPT_SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline]
    pub fn is_no_event(&self) -> bool {
        *self == CAPT_SRCR::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == CAPT_SRCR::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == CAPT_SRCR::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == CAPT_SRCR::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == CAPT_SRCR::AUX_ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == CAPT_SRCR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ISRC_RESET_N`"]
    #[inline]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == CAPT_SRCR::AUX_ISRC_RESET_N
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == CAPT_SRCR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == CAPT_SRCR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == CAPT_SRCR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == CAPT_SRCR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == CAPT_SRCR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == CAPT_SRCR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == CAPT_SRCR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == CAPT_SRCR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == CAPT_SRCR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX1`"]
    #[inline]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == CAPT_SRCR::MCU_OBSMUX1
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX0`"]
    #[inline]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == CAPT_SRCR::MCU_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == CAPT_SRCR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == CAPT_SRCR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `VDDR_RECHARGE`"]
    #[inline]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == CAPT_SRCR::VDDR_RECHARGE
    }
    #[doc = "Checks if the value of the field is `MCU_ACTIVE`"]
    #[inline]
    pub fn is_mcu_active(&self) -> bool {
        *self == CAPT_SRCR::MCU_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PWR_DWN`"]
    #[inline]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == CAPT_SRCR::PWR_DWN
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline]
    pub fn is_sclk_lf(&self) -> bool {
        *self == CAPT_SRCR::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_TEMP_UPD`"]
    #[inline]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == CAPT_SRCR::AON_BATMON_TEMP_UPD
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_BAT_UPD`"]
    #[inline]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == CAPT_SRCR::AON_BATMON_BAT_UPD
    }
    #[doc = "Checks if the value of the field is `AON_RTC_4KHZ`"]
    #[inline]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == CAPT_SRCR::AON_RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == CAPT_SRCR::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == CAPT_SRCR::AON_RTC_CH2
    }
    #[doc = "Checks if the value of the field is `MANUAL_EV`"]
    #[inline]
    pub fn is_manual_ev(&self) -> bool {
        *self == CAPT_SRCR::MANUAL_EV
    }
    #[doc = "Checks if the value of the field is `AUXIO31`"]
    #[inline]
    pub fn is_auxio31(&self) -> bool {
        *self == CAPT_SRCR::AUXIO31
    }
    #[doc = "Checks if the value of the field is `AUXIO30`"]
    #[inline]
    pub fn is_auxio30(&self) -> bool {
        *self == CAPT_SRCR::AUXIO30
    }
    #[doc = "Checks if the value of the field is `AUXIO29`"]
    #[inline]
    pub fn is_auxio29(&self) -> bool {
        *self == CAPT_SRCR::AUXIO29
    }
    #[doc = "Checks if the value of the field is `AUXIO28`"]
    #[inline]
    pub fn is_auxio28(&self) -> bool {
        *self == CAPT_SRCR::AUXIO28
    }
    #[doc = "Checks if the value of the field is `AUXIO27`"]
    #[inline]
    pub fn is_auxio27(&self) -> bool {
        *self == CAPT_SRCR::AUXIO27
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline]
    pub fn is_auxio26(&self) -> bool {
        *self == CAPT_SRCR::AUXIO26
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline]
    pub fn is_auxio25(&self) -> bool {
        *self == CAPT_SRCR::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline]
    pub fn is_auxio24(&self) -> bool {
        *self == CAPT_SRCR::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline]
    pub fn is_auxio23(&self) -> bool {
        *self == CAPT_SRCR::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline]
    pub fn is_auxio22(&self) -> bool {
        *self == CAPT_SRCR::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline]
    pub fn is_auxio21(&self) -> bool {
        *self == CAPT_SRCR::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline]
    pub fn is_auxio20(&self) -> bool {
        *self == CAPT_SRCR::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline]
    pub fn is_auxio19(&self) -> bool {
        *self == CAPT_SRCR::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO18`"]
    #[inline]
    pub fn is_auxio18(&self) -> bool {
        *self == CAPT_SRCR::AUXIO18
    }
    #[doc = "Checks if the value of the field is `AUXIO17`"]
    #[inline]
    pub fn is_auxio17(&self) -> bool {
        *self == CAPT_SRCR::AUXIO17
    }
    #[doc = "Checks if the value of the field is `AUXIO16`"]
    #[inline]
    pub fn is_auxio16(&self) -> bool {
        *self == CAPT_SRCR::AUXIO16
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == CAPT_SRCR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == CAPT_SRCR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == CAPT_SRCR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == CAPT_SRCR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == CAPT_SRCR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == CAPT_SRCR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == CAPT_SRCR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == CAPT_SRCR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == CAPT_SRCR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == CAPT_SRCR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == CAPT_SRCR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == CAPT_SRCR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == CAPT_SRCR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == CAPT_SRCR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == CAPT_SRCR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == CAPT_SRCR::AUXIO0
    }
}
#[doc = "Possible values of the field `EDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGER {
    #[doc = "Capture CNTR.VALUE at rising edge of CAPT_SRC."]
    RISING,
    #[doc = "Capture CNTR.VALUE at falling edge of CAPT_SRC."]
    FALLING,
}
impl EDGER {
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
            EDGER::RISING => true,
            EDGER::FALLING => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDGER {
        match value {
            true => EDGER::RISING,
            false => EDGER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == EDGER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == EDGER::FALLING
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
#[doc = "Values that can be written to the field `CAPT_SRC`"]
pub enum CAPT_SRCW {
    #[doc = "No event."]
    NO_EVENT,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AUX_SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AUX_ADC_FIFO_NOT_EMPTY,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    AUX_ADC_IRQ,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    AUX_ADC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    AUX_ISRC_RESET_N,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    AUX_TDC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    AUX_TIMER0_EV,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    AUX_TIMER1_EV,
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
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    MCU_OBSMUX1,
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    MCU_OBSMUX0,
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
impl CAPT_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAPT_SRCW::NO_EVENT => 63,
            CAPT_SRCW::AUX_SMPH_AUTOTAKE_DONE => 61,
            CAPT_SRCW::AUX_ADC_FIFO_NOT_EMPTY => 60,
            CAPT_SRCW::AUX_ADC_FIFO_ALMOST_FULL => 59,
            CAPT_SRCW::AUX_ADC_IRQ => 58,
            CAPT_SRCW::AUX_ADC_DONE => 57,
            CAPT_SRCW::AUX_ISRC_RESET_N => 56,
            CAPT_SRCW::AUX_TDC_DONE => 55,
            CAPT_SRCW::AUX_TIMER0_EV => 54,
            CAPT_SRCW::AUX_TIMER1_EV => 53,
            CAPT_SRCW::AUX_TIMER2_EV3 => 51,
            CAPT_SRCW::AUX_TIMER2_EV2 => 50,
            CAPT_SRCW::AUX_TIMER2_EV1 => 49,
            CAPT_SRCW::AUX_TIMER2_EV0 => 48,
            CAPT_SRCW::AUX_COMPB => 47,
            CAPT_SRCW::AUX_COMPA => 46,
            CAPT_SRCW::MCU_OBSMUX1 => 45,
            CAPT_SRCW::MCU_OBSMUX0 => 44,
            CAPT_SRCW::MCU_EV => 43,
            CAPT_SRCW::ACLK_REF => 42,
            CAPT_SRCW::VDDR_RECHARGE => 41,
            CAPT_SRCW::MCU_ACTIVE => 40,
            CAPT_SRCW::PWR_DWN => 39,
            CAPT_SRCW::SCLK_LF => 38,
            CAPT_SRCW::AON_BATMON_TEMP_UPD => 37,
            CAPT_SRCW::AON_BATMON_BAT_UPD => 36,
            CAPT_SRCW::AON_RTC_4KHZ => 35,
            CAPT_SRCW::AON_RTC_CH2_DLY => 34,
            CAPT_SRCW::AON_RTC_CH2 => 33,
            CAPT_SRCW::MANUAL_EV => 32,
            CAPT_SRCW::AUXIO31 => 31,
            CAPT_SRCW::AUXIO30 => 30,
            CAPT_SRCW::AUXIO29 => 29,
            CAPT_SRCW::AUXIO28 => 28,
            CAPT_SRCW::AUXIO27 => 27,
            CAPT_SRCW::AUXIO26 => 26,
            CAPT_SRCW::AUXIO25 => 25,
            CAPT_SRCW::AUXIO24 => 24,
            CAPT_SRCW::AUXIO23 => 23,
            CAPT_SRCW::AUXIO22 => 22,
            CAPT_SRCW::AUXIO21 => 21,
            CAPT_SRCW::AUXIO20 => 20,
            CAPT_SRCW::AUXIO19 => 19,
            CAPT_SRCW::AUXIO18 => 18,
            CAPT_SRCW::AUXIO17 => 17,
            CAPT_SRCW::AUXIO16 => 16,
            CAPT_SRCW::AUXIO15 => 15,
            CAPT_SRCW::AUXIO14 => 14,
            CAPT_SRCW::AUXIO13 => 13,
            CAPT_SRCW::AUXIO12 => 12,
            CAPT_SRCW::AUXIO11 => 11,
            CAPT_SRCW::AUXIO10 => 10,
            CAPT_SRCW::AUXIO9 => 9,
            CAPT_SRCW::AUXIO8 => 8,
            CAPT_SRCW::AUXIO7 => 7,
            CAPT_SRCW::AUXIO6 => 6,
            CAPT_SRCW::AUXIO5 => 5,
            CAPT_SRCW::AUXIO4 => 4,
            CAPT_SRCW::AUXIO3 => 3,
            CAPT_SRCW::AUXIO2 => 2,
            CAPT_SRCW::AUXIO1 => 1,
            CAPT_SRCW::AUXIO0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event."]
    #[inline]
    pub fn no_event(self) -> &'a mut W {
        self.variant(CAPT_SRCW::NO_EVENT)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_ADC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline]
    pub fn aux_isrc_reset_n(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_ISRC_RESET_N)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_TDC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_TIMER0_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_TIMER2_EV3)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_TIMER2_EV2)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_TIMER2_EV1)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_TIMER2_EV0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline]
    pub fn mcu_obsmux1(self) -> &'a mut W {
        self.variant(CAPT_SRCW::MCU_OBSMUX1)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline]
    pub fn mcu_obsmux0(self) -> &'a mut W {
        self.variant(CAPT_SRCW::MCU_OBSMUX0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(CAPT_SRCW::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(CAPT_SRCW::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline]
    pub fn vddr_recharge(self) -> &'a mut W {
        self.variant(CAPT_SRCW::VDDR_RECHARGE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline]
    pub fn mcu_active(self) -> &'a mut W {
        self.variant(CAPT_SRCW::MCU_ACTIVE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline]
    pub fn pwr_dwn(self) -> &'a mut W {
        self.variant(CAPT_SRCW::PWR_DWN)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(CAPT_SRCW::SCLK_LF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline]
    pub fn aon_batmon_temp_upd(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AON_BATMON_TEMP_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline]
    pub fn aon_batmon_bat_upd(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AON_BATMON_BAT_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline]
    pub fn aon_rtc_4khz(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AON_RTC_4KHZ)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AON_RTC_CH2_DLY)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AON_RTC_CH2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline]
    pub fn manual_ev(self) -> &'a mut W {
        self.variant(CAPT_SRCW::MANUAL_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline]
    pub fn auxio31(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline]
    pub fn auxio30(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline]
    pub fn auxio29(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline]
    pub fn auxio28(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline]
    pub fn auxio27(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline]
    pub fn auxio18(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline]
    pub fn auxio17(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline]
    pub fn auxio16(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(CAPT_SRCW::AUXIO0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGE`"]
pub enum EDGEW {
    #[doc = "Capture CNTR.VALUE at rising edge of CAPT_SRC."]
    RISING,
    #[doc = "Capture CNTR.VALUE at falling edge of CAPT_SRC."]
    FALLING,
}
impl EDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGEW::RISING => true,
            EDGEW::FALLING => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture CNTR.VALUE at rising edge of CAPT_SRC."]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGEW::RISING)
    }
    #[doc = "Capture CNTR.VALUE at falling edge of CAPT_SRC."]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGEW::FALLING)
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
    #[doc = "Bits 1:6 - 6:1\\] Select capture signal source from the asynchronous AUX event bus. The selected signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described in CH0EVCFG - this register is reconfigured while CTL.MODE is different from DIS. You can avoid false capture events. When wanted channel function is: - SET_ON_CAPT_DIS, see description for SET_ON_CAPT_DIS in CH0EVCFG.CCACT. - SET_ON_CAPT, see description for SET_ON_CAPT in CH0EVCFG.CCACT. - PER_PULSE_WIDTH_MEAS, see description for PER_PULSE_WIDTH_MEAS in CH0EVCFG.CCACT. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline]
    pub fn capt_src(&self) -> CAPT_SRCR {
        CAPT_SRCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - 0:0\\] Edge configuration. Channel captures counter value at selected edge on signal source selected by CAPT_SRC. See CH0EVCFG.CCACT."]
    #[inline]
    pub fn edge(&self) -> EDGER {
        EDGER::_from({
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
    #[doc = "Bits 7:31 - 31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved7(&mut self) -> _RESERVED7W {
        _RESERVED7W { w: self }
    }
    #[doc = "Bits 1:6 - 6:1\\] Select capture signal source from the asynchronous AUX event bus. The selected signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described in CH0EVCFG - this register is reconfigured while CTL.MODE is different from DIS. You can avoid false capture events. When wanted channel function is: - SET_ON_CAPT_DIS, see description for SET_ON_CAPT_DIS in CH0EVCFG.CCACT. - SET_ON_CAPT, see description for SET_ON_CAPT in CH0EVCFG.CCACT. - PER_PULSE_WIDTH_MEAS, see description for PER_PULSE_WIDTH_MEAS in CH0EVCFG.CCACT. If you write a non-enumerated value the behavior is identical to NO_EVENT. The written value is returned when read."]
    #[inline]
    pub fn capt_src(&mut self) -> _CAPT_SRCW {
        _CAPT_SRCW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Edge configuration. Channel captures counter value at selected edge on signal source selected by CAPT_SRC. See CH0EVCFG.CCACT."]
    #[inline]
    pub fn edge(&mut self) -> _EDGEW {
        _EDGEW { w: self }
    }
}
