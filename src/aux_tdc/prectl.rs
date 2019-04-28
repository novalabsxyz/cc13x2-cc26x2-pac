#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRECTL {
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
pub struct RESERVED8R {
    bits: u32,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESET_NR {
    bits: bool,
}
impl RESET_NR {
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
#[doc = "Possible values of the field `RATIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATIOR {
    #[doc = "Prescaler divides input by 64. \n\nAUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    DIV64,
    #[doc = "Prescaler divides input by 16. \n\nAUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    DIV16,
}
impl RATIOR {
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
            RATIOR::DIV64 => true,
            RATIOR::DIV16 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RATIOR {
        match value {
            true => RATIOR::DIV64,
            false => RATIOR::DIV16,
        }
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == RATIOR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == RATIOR::DIV16
    }
}
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
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
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::NO_EVENT => 63,
            SRCR::AUX_SMPH_AUTOTAKE_DONE => 61,
            SRCR::AUX_ADC_FIFO_NOT_EMPTY => 60,
            SRCR::AUX_ADC_FIFO_ALMOST_FULL => 59,
            SRCR::AUX_ADC_IRQ => 58,
            SRCR::AUX_ADC_DONE => 57,
            SRCR::AUX_ISRC_RESET_N => 56,
            SRCR::AUX_TDC_DONE => 55,
            SRCR::AUX_TIMER0_EV => 54,
            SRCR::AUX_TIMER1_EV => 53,
            SRCR::AUX_TIMER2_PULSE => 52,
            SRCR::AUX_TIMER2_EV3 => 51,
            SRCR::AUX_TIMER2_EV2 => 50,
            SRCR::AUX_TIMER2_EV1 => 49,
            SRCR::AUX_TIMER2_EV0 => 48,
            SRCR::AUX_COMPB => 47,
            SRCR::AUX_COMPA => 46,
            SRCR::MCU_OBSMUX1 => 45,
            SRCR::MCU_OBSMUX0 => 44,
            SRCR::MCU_EV => 43,
            SRCR::ACLK_REF => 42,
            SRCR::VDDR_RECHARGE => 41,
            SRCR::MCU_ACTIVE => 40,
            SRCR::PWR_DWN => 39,
            SRCR::SCLK_LF => 38,
            SRCR::AON_BATMON_TEMP_UPD => 37,
            SRCR::AON_BATMON_BAT_UPD => 36,
            SRCR::AON_RTC_4KHZ => 35,
            SRCR::AON_RTC_CH2_DLY => 34,
            SRCR::AON_RTC_CH2 => 33,
            SRCR::MANUAL_EV => 32,
            SRCR::AUXIO31 => 31,
            SRCR::AUXIO30 => 30,
            SRCR::AUXIO29 => 29,
            SRCR::AUXIO28 => 28,
            SRCR::AUXIO27 => 27,
            SRCR::AUXIO26 => 26,
            SRCR::AUXIO25 => 25,
            SRCR::AUXIO24 => 24,
            SRCR::AUXIO23 => 23,
            SRCR::AUXIO22 => 22,
            SRCR::AUXIO21 => 21,
            SRCR::AUXIO20 => 20,
            SRCR::AUXIO19 => 19,
            SRCR::AUXIO18 => 18,
            SRCR::AUXIO17 => 17,
            SRCR::AUXIO16 => 16,
            SRCR::AUXIO15 => 15,
            SRCR::AUXIO14 => 14,
            SRCR::AUXIO13 => 13,
            SRCR::AUXIO12 => 12,
            SRCR::AUXIO11 => 11,
            SRCR::AUXIO10 => 10,
            SRCR::AUXIO9 => 9,
            SRCR::AUXIO8 => 8,
            SRCR::AUXIO7 => 7,
            SRCR::AUXIO6 => 6,
            SRCR::AUXIO5 => 5,
            SRCR::AUXIO4 => 4,
            SRCR::AUXIO3 => 3,
            SRCR::AUXIO2 => 2,
            SRCR::AUXIO1 => 1,
            SRCR::AUXIO0 => 0,
            SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            63 => SRCR::NO_EVENT,
            61 => SRCR::AUX_SMPH_AUTOTAKE_DONE,
            60 => SRCR::AUX_ADC_FIFO_NOT_EMPTY,
            59 => SRCR::AUX_ADC_FIFO_ALMOST_FULL,
            58 => SRCR::AUX_ADC_IRQ,
            57 => SRCR::AUX_ADC_DONE,
            56 => SRCR::AUX_ISRC_RESET_N,
            55 => SRCR::AUX_TDC_DONE,
            54 => SRCR::AUX_TIMER0_EV,
            53 => SRCR::AUX_TIMER1_EV,
            52 => SRCR::AUX_TIMER2_PULSE,
            51 => SRCR::AUX_TIMER2_EV3,
            50 => SRCR::AUX_TIMER2_EV2,
            49 => SRCR::AUX_TIMER2_EV1,
            48 => SRCR::AUX_TIMER2_EV0,
            47 => SRCR::AUX_COMPB,
            46 => SRCR::AUX_COMPA,
            45 => SRCR::MCU_OBSMUX1,
            44 => SRCR::MCU_OBSMUX0,
            43 => SRCR::MCU_EV,
            42 => SRCR::ACLK_REF,
            41 => SRCR::VDDR_RECHARGE,
            40 => SRCR::MCU_ACTIVE,
            39 => SRCR::PWR_DWN,
            38 => SRCR::SCLK_LF,
            37 => SRCR::AON_BATMON_TEMP_UPD,
            36 => SRCR::AON_BATMON_BAT_UPD,
            35 => SRCR::AON_RTC_4KHZ,
            34 => SRCR::AON_RTC_CH2_DLY,
            33 => SRCR::AON_RTC_CH2,
            32 => SRCR::MANUAL_EV,
            31 => SRCR::AUXIO31,
            30 => SRCR::AUXIO30,
            29 => SRCR::AUXIO29,
            28 => SRCR::AUXIO28,
            27 => SRCR::AUXIO27,
            26 => SRCR::AUXIO26,
            25 => SRCR::AUXIO25,
            24 => SRCR::AUXIO24,
            23 => SRCR::AUXIO23,
            22 => SRCR::AUXIO22,
            21 => SRCR::AUXIO21,
            20 => SRCR::AUXIO20,
            19 => SRCR::AUXIO19,
            18 => SRCR::AUXIO18,
            17 => SRCR::AUXIO17,
            16 => SRCR::AUXIO16,
            15 => SRCR::AUXIO15,
            14 => SRCR::AUXIO14,
            13 => SRCR::AUXIO13,
            12 => SRCR::AUXIO12,
            11 => SRCR::AUXIO11,
            10 => SRCR::AUXIO10,
            9 => SRCR::AUXIO9,
            8 => SRCR::AUXIO8,
            7 => SRCR::AUXIO7,
            6 => SRCR::AUXIO6,
            5 => SRCR::AUXIO5,
            4 => SRCR::AUXIO4,
            3 => SRCR::AUXIO3,
            2 => SRCR::AUXIO2,
            1 => SRCR::AUXIO1,
            0 => SRCR::AUXIO0,
            i => SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline]
    pub fn is_no_event(&self) -> bool {
        *self == SRCR::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == SRCR::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == SRCR::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == SRCR::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == SRCR::AUX_ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == SRCR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ISRC_RESET_N`"]
    #[inline]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == SRCR::AUX_ISRC_RESET_N
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == SRCR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == SRCR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == SRCR::AUX_TIMER1_EV
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
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == SRCR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == SRCR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX1`"]
    #[inline]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == SRCR::MCU_OBSMUX1
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX0`"]
    #[inline]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == SRCR::MCU_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == SRCR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == SRCR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `VDDR_RECHARGE`"]
    #[inline]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == SRCR::VDDR_RECHARGE
    }
    #[doc = "Checks if the value of the field is `MCU_ACTIVE`"]
    #[inline]
    pub fn is_mcu_active(&self) -> bool {
        *self == SRCR::MCU_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PWR_DWN`"]
    #[inline]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == SRCR::PWR_DWN
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline]
    pub fn is_sclk_lf(&self) -> bool {
        *self == SRCR::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_TEMP_UPD`"]
    #[inline]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == SRCR::AON_BATMON_TEMP_UPD
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_BAT_UPD`"]
    #[inline]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == SRCR::AON_BATMON_BAT_UPD
    }
    #[doc = "Checks if the value of the field is `AON_RTC_4KHZ`"]
    #[inline]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == SRCR::AON_RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == SRCR::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == SRCR::AON_RTC_CH2
    }
    #[doc = "Checks if the value of the field is `MANUAL_EV`"]
    #[inline]
    pub fn is_manual_ev(&self) -> bool {
        *self == SRCR::MANUAL_EV
    }
    #[doc = "Checks if the value of the field is `AUXIO31`"]
    #[inline]
    pub fn is_auxio31(&self) -> bool {
        *self == SRCR::AUXIO31
    }
    #[doc = "Checks if the value of the field is `AUXIO30`"]
    #[inline]
    pub fn is_auxio30(&self) -> bool {
        *self == SRCR::AUXIO30
    }
    #[doc = "Checks if the value of the field is `AUXIO29`"]
    #[inline]
    pub fn is_auxio29(&self) -> bool {
        *self == SRCR::AUXIO29
    }
    #[doc = "Checks if the value of the field is `AUXIO28`"]
    #[inline]
    pub fn is_auxio28(&self) -> bool {
        *self == SRCR::AUXIO28
    }
    #[doc = "Checks if the value of the field is `AUXIO27`"]
    #[inline]
    pub fn is_auxio27(&self) -> bool {
        *self == SRCR::AUXIO27
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline]
    pub fn is_auxio26(&self) -> bool {
        *self == SRCR::AUXIO26
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline]
    pub fn is_auxio25(&self) -> bool {
        *self == SRCR::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline]
    pub fn is_auxio24(&self) -> bool {
        *self == SRCR::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline]
    pub fn is_auxio23(&self) -> bool {
        *self == SRCR::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline]
    pub fn is_auxio22(&self) -> bool {
        *self == SRCR::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline]
    pub fn is_auxio21(&self) -> bool {
        *self == SRCR::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline]
    pub fn is_auxio20(&self) -> bool {
        *self == SRCR::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline]
    pub fn is_auxio19(&self) -> bool {
        *self == SRCR::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO18`"]
    #[inline]
    pub fn is_auxio18(&self) -> bool {
        *self == SRCR::AUXIO18
    }
    #[doc = "Checks if the value of the field is `AUXIO17`"]
    #[inline]
    pub fn is_auxio17(&self) -> bool {
        *self == SRCR::AUXIO17
    }
    #[doc = "Checks if the value of the field is `AUXIO16`"]
    #[inline]
    pub fn is_auxio16(&self) -> bool {
        *self == SRCR::AUXIO16
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == SRCR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == SRCR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == SRCR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == SRCR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == SRCR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == SRCR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == SRCR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == SRCR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == SRCR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == SRCR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == SRCR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == SRCR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == SRCR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == SRCR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == SRCR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == SRCR::AUXIO0
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED8W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESET_NW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_NW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RATIO`"]
pub enum RATIOW {
    #[doc = "Prescaler divides input by 64. \n\nAUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    DIV64,
    #[doc = "Prescaler divides input by 16. \n\nAUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    DIV16,
}
impl RATIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RATIOW::DIV64 => true,
            RATIOW::DIV16 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RATIOW<'a> {
    w: &'a mut W,
}
impl<'a> _RATIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RATIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prescaler divides input by 64. AUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(RATIOW::DIV64)
    }
    #[doc = "Prescaler divides input by 16. AUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(RATIOW::DIV16)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC`"]
pub enum SRCW {
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
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCW::NO_EVENT => 63,
            SRCW::AUX_SMPH_AUTOTAKE_DONE => 61,
            SRCW::AUX_ADC_FIFO_NOT_EMPTY => 60,
            SRCW::AUX_ADC_FIFO_ALMOST_FULL => 59,
            SRCW::AUX_ADC_IRQ => 58,
            SRCW::AUX_ADC_DONE => 57,
            SRCW::AUX_ISRC_RESET_N => 56,
            SRCW::AUX_TDC_DONE => 55,
            SRCW::AUX_TIMER0_EV => 54,
            SRCW::AUX_TIMER1_EV => 53,
            SRCW::AUX_TIMER2_PULSE => 52,
            SRCW::AUX_TIMER2_EV3 => 51,
            SRCW::AUX_TIMER2_EV2 => 50,
            SRCW::AUX_TIMER2_EV1 => 49,
            SRCW::AUX_TIMER2_EV0 => 48,
            SRCW::AUX_COMPB => 47,
            SRCW::AUX_COMPA => 46,
            SRCW::MCU_OBSMUX1 => 45,
            SRCW::MCU_OBSMUX0 => 44,
            SRCW::MCU_EV => 43,
            SRCW::ACLK_REF => 42,
            SRCW::VDDR_RECHARGE => 41,
            SRCW::MCU_ACTIVE => 40,
            SRCW::PWR_DWN => 39,
            SRCW::SCLK_LF => 38,
            SRCW::AON_BATMON_TEMP_UPD => 37,
            SRCW::AON_BATMON_BAT_UPD => 36,
            SRCW::AON_RTC_4KHZ => 35,
            SRCW::AON_RTC_CH2_DLY => 34,
            SRCW::AON_RTC_CH2 => 33,
            SRCW::MANUAL_EV => 32,
            SRCW::AUXIO31 => 31,
            SRCW::AUXIO30 => 30,
            SRCW::AUXIO29 => 29,
            SRCW::AUXIO28 => 28,
            SRCW::AUXIO27 => 27,
            SRCW::AUXIO26 => 26,
            SRCW::AUXIO25 => 25,
            SRCW::AUXIO24 => 24,
            SRCW::AUXIO23 => 23,
            SRCW::AUXIO22 => 22,
            SRCW::AUXIO21 => 21,
            SRCW::AUXIO20 => 20,
            SRCW::AUXIO19 => 19,
            SRCW::AUXIO18 => 18,
            SRCW::AUXIO17 => 17,
            SRCW::AUXIO16 => 16,
            SRCW::AUXIO15 => 15,
            SRCW::AUXIO14 => 14,
            SRCW::AUXIO13 => 13,
            SRCW::AUXIO12 => 12,
            SRCW::AUXIO11 => 11,
            SRCW::AUXIO10 => 10,
            SRCW::AUXIO9 => 9,
            SRCW::AUXIO8 => 8,
            SRCW::AUXIO7 => 7,
            SRCW::AUXIO6 => 6,
            SRCW::AUXIO5 => 5,
            SRCW::AUXIO4 => 4,
            SRCW::AUXIO3 => 3,
            SRCW::AUXIO2 => 2,
            SRCW::AUXIO1 => 1,
            SRCW::AUXIO0 => 0,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event."]
    #[inline]
    pub fn no_event(self) -> &'a mut W {
        self.variant(SRCW::NO_EVENT)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(SRCW::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(SRCW::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(SRCW::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(SRCW::AUX_ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(SRCW::AUX_ADC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline]
    pub fn aux_isrc_reset_n(self) -> &'a mut W {
        self.variant(SRCW::AUX_ISRC_RESET_N)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(SRCW::AUX_TDC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER0_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_PULSE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_EV3)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_EV2)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_EV1)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(SRCW::AUX_TIMER2_EV0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(SRCW::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(SRCW::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline]
    pub fn mcu_obsmux1(self) -> &'a mut W {
        self.variant(SRCW::MCU_OBSMUX1)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline]
    pub fn mcu_obsmux0(self) -> &'a mut W {
        self.variant(SRCW::MCU_OBSMUX0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(SRCW::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(SRCW::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline]
    pub fn vddr_recharge(self) -> &'a mut W {
        self.variant(SRCW::VDDR_RECHARGE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline]
    pub fn mcu_active(self) -> &'a mut W {
        self.variant(SRCW::MCU_ACTIVE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline]
    pub fn pwr_dwn(self) -> &'a mut W {
        self.variant(SRCW::PWR_DWN)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(SRCW::SCLK_LF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline]
    pub fn aon_batmon_temp_upd(self) -> &'a mut W {
        self.variant(SRCW::AON_BATMON_TEMP_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline]
    pub fn aon_batmon_bat_upd(self) -> &'a mut W {
        self.variant(SRCW::AON_BATMON_BAT_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline]
    pub fn aon_rtc_4khz(self) -> &'a mut W {
        self.variant(SRCW::AON_RTC_4KHZ)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(SRCW::AON_RTC_CH2_DLY)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(SRCW::AON_RTC_CH2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline]
    pub fn manual_ev(self) -> &'a mut W {
        self.variant(SRCW::MANUAL_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline]
    pub fn auxio31(self) -> &'a mut W {
        self.variant(SRCW::AUXIO31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline]
    pub fn auxio30(self) -> &'a mut W {
        self.variant(SRCW::AUXIO30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline]
    pub fn auxio29(self) -> &'a mut W {
        self.variant(SRCW::AUXIO29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline]
    pub fn auxio28(self) -> &'a mut W {
        self.variant(SRCW::AUXIO28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline]
    pub fn auxio27(self) -> &'a mut W {
        self.variant(SRCW::AUXIO27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(SRCW::AUXIO26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(SRCW::AUXIO25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(SRCW::AUXIO24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(SRCW::AUXIO23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(SRCW::AUXIO22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(SRCW::AUXIO21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(SRCW::AUXIO20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(SRCW::AUXIO19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline]
    pub fn auxio18(self) -> &'a mut W {
        self.variant(SRCW::AUXIO18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline]
    pub fn auxio17(self) -> &'a mut W {
        self.variant(SRCW::AUXIO17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline]
    pub fn auxio16(self) -> &'a mut W {
        self.variant(SRCW::AUXIO16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(SRCW::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(SRCW::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(SRCW::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(SRCW::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(SRCW::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(SRCW::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(SRCW::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(SRCW::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(SRCW::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(SRCW::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(SRCW::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(SRCW::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(SRCW::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(SRCW::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(SRCW::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(SRCW::AUXIO0)
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[inline]
    pub fn reset_n(&self) -> RESET_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESET_NR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[inline]
    pub fn ratio(&self) -> RATIOR {
        RATIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:5 - 5:0\\] Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&mut self) -> _RESERVED8W {
        _RESERVED8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[inline]
    pub fn reset_n(&mut self) -> _RESET_NW {
        _RESET_NW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[inline]
    pub fn ratio(&mut self) -> _RATIOW {
        _RATIOW { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
}
