#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::T0CFG {
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
#[doc = "Possible values of the field `TICK_SRC_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICK_SRC_POLR {
    #[doc = "Count on falling edges of TICK_SRC."]
    FALL,
    #[doc = "Count on rising edges of TICK_SRC."]
    RISE,
}
impl TICK_SRC_POLR {
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
            TICK_SRC_POLR::FALL => true,
            TICK_SRC_POLR::RISE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TICK_SRC_POLR {
        match value {
            true => TICK_SRC_POLR::FALL,
            false => TICK_SRC_POLR::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == TICK_SRC_POLR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == TICK_SRC_POLR::RISE
    }
}
#[doc = "Possible values of the field `TICK_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICK_SRCR {
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    AUX_TIMER2_CLKSW_RDY,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    AUX_DAC_HOLD_ACTIVE,
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
    #[doc = "No event."]
    NO_EVENT,
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
impl TICK_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TICK_SRCR::AUX_TIMER2_CLKSW_RDY => 63,
            TICK_SRCR::AUX_DAC_HOLD_ACTIVE => 62,
            TICK_SRCR::AUX_SMPH_AUTOTAKE_DONE => 61,
            TICK_SRCR::AUX_ADC_FIFO_NOT_EMPTY => 60,
            TICK_SRCR::AUX_ADC_FIFO_ALMOST_FULL => 59,
            TICK_SRCR::AUX_ADC_IRQ => 58,
            TICK_SRCR::AUX_ADC_DONE => 57,
            TICK_SRCR::AUX_ISRC_RESET_N => 56,
            TICK_SRCR::AUX_TDC_DONE => 55,
            TICK_SRCR::NO_EVENT => 54,
            TICK_SRCR::AUX_TIMER1_EV => 53,
            TICK_SRCR::AUX_TIMER2_PULSE => 52,
            TICK_SRCR::AUX_TIMER2_EV3 => 51,
            TICK_SRCR::AUX_TIMER2_EV2 => 50,
            TICK_SRCR::AUX_TIMER2_EV1 => 49,
            TICK_SRCR::AUX_TIMER2_EV0 => 48,
            TICK_SRCR::AUX_COMPB => 47,
            TICK_SRCR::AUX_COMPA => 46,
            TICK_SRCR::MCU_OBSMUX1 => 45,
            TICK_SRCR::MCU_OBSMUX0 => 44,
            TICK_SRCR::MCU_EV => 43,
            TICK_SRCR::ACLK_REF => 42,
            TICK_SRCR::VDDR_RECHARGE => 41,
            TICK_SRCR::MCU_ACTIVE => 40,
            TICK_SRCR::PWR_DWN => 39,
            TICK_SRCR::SCLK_LF => 38,
            TICK_SRCR::AON_BATMON_TEMP_UPD => 37,
            TICK_SRCR::AON_BATMON_BAT_UPD => 36,
            TICK_SRCR::AON_RTC_4KHZ => 35,
            TICK_SRCR::AON_RTC_CH2_DLY => 34,
            TICK_SRCR::AON_RTC_CH2 => 33,
            TICK_SRCR::MANUAL_EV => 32,
            TICK_SRCR::AUXIO31 => 31,
            TICK_SRCR::AUXIO30 => 30,
            TICK_SRCR::AUXIO29 => 29,
            TICK_SRCR::AUXIO28 => 28,
            TICK_SRCR::AUXIO27 => 27,
            TICK_SRCR::AUXIO26 => 26,
            TICK_SRCR::AUXIO25 => 25,
            TICK_SRCR::AUXIO24 => 24,
            TICK_SRCR::AUXIO23 => 23,
            TICK_SRCR::AUXIO22 => 22,
            TICK_SRCR::AUXIO21 => 21,
            TICK_SRCR::AUXIO20 => 20,
            TICK_SRCR::AUXIO19 => 19,
            TICK_SRCR::AUXIO18 => 18,
            TICK_SRCR::AUXIO17 => 17,
            TICK_SRCR::AUXIO16 => 16,
            TICK_SRCR::AUXIO15 => 15,
            TICK_SRCR::AUXIO14 => 14,
            TICK_SRCR::AUXIO13 => 13,
            TICK_SRCR::AUXIO12 => 12,
            TICK_SRCR::AUXIO11 => 11,
            TICK_SRCR::AUXIO10 => 10,
            TICK_SRCR::AUXIO9 => 9,
            TICK_SRCR::AUXIO8 => 8,
            TICK_SRCR::AUXIO7 => 7,
            TICK_SRCR::AUXIO6 => 6,
            TICK_SRCR::AUXIO5 => 5,
            TICK_SRCR::AUXIO4 => 4,
            TICK_SRCR::AUXIO3 => 3,
            TICK_SRCR::AUXIO2 => 2,
            TICK_SRCR::AUXIO1 => 1,
            TICK_SRCR::AUXIO0 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TICK_SRCR {
        match value {
            63 => TICK_SRCR::AUX_TIMER2_CLKSW_RDY,
            62 => TICK_SRCR::AUX_DAC_HOLD_ACTIVE,
            61 => TICK_SRCR::AUX_SMPH_AUTOTAKE_DONE,
            60 => TICK_SRCR::AUX_ADC_FIFO_NOT_EMPTY,
            59 => TICK_SRCR::AUX_ADC_FIFO_ALMOST_FULL,
            58 => TICK_SRCR::AUX_ADC_IRQ,
            57 => TICK_SRCR::AUX_ADC_DONE,
            56 => TICK_SRCR::AUX_ISRC_RESET_N,
            55 => TICK_SRCR::AUX_TDC_DONE,
            54 => TICK_SRCR::NO_EVENT,
            53 => TICK_SRCR::AUX_TIMER1_EV,
            52 => TICK_SRCR::AUX_TIMER2_PULSE,
            51 => TICK_SRCR::AUX_TIMER2_EV3,
            50 => TICK_SRCR::AUX_TIMER2_EV2,
            49 => TICK_SRCR::AUX_TIMER2_EV1,
            48 => TICK_SRCR::AUX_TIMER2_EV0,
            47 => TICK_SRCR::AUX_COMPB,
            46 => TICK_SRCR::AUX_COMPA,
            45 => TICK_SRCR::MCU_OBSMUX1,
            44 => TICK_SRCR::MCU_OBSMUX0,
            43 => TICK_SRCR::MCU_EV,
            42 => TICK_SRCR::ACLK_REF,
            41 => TICK_SRCR::VDDR_RECHARGE,
            40 => TICK_SRCR::MCU_ACTIVE,
            39 => TICK_SRCR::PWR_DWN,
            38 => TICK_SRCR::SCLK_LF,
            37 => TICK_SRCR::AON_BATMON_TEMP_UPD,
            36 => TICK_SRCR::AON_BATMON_BAT_UPD,
            35 => TICK_SRCR::AON_RTC_4KHZ,
            34 => TICK_SRCR::AON_RTC_CH2_DLY,
            33 => TICK_SRCR::AON_RTC_CH2,
            32 => TICK_SRCR::MANUAL_EV,
            31 => TICK_SRCR::AUXIO31,
            30 => TICK_SRCR::AUXIO30,
            29 => TICK_SRCR::AUXIO29,
            28 => TICK_SRCR::AUXIO28,
            27 => TICK_SRCR::AUXIO27,
            26 => TICK_SRCR::AUXIO26,
            25 => TICK_SRCR::AUXIO25,
            24 => TICK_SRCR::AUXIO24,
            23 => TICK_SRCR::AUXIO23,
            22 => TICK_SRCR::AUXIO22,
            21 => TICK_SRCR::AUXIO21,
            20 => TICK_SRCR::AUXIO20,
            19 => TICK_SRCR::AUXIO19,
            18 => TICK_SRCR::AUXIO18,
            17 => TICK_SRCR::AUXIO17,
            16 => TICK_SRCR::AUXIO16,
            15 => TICK_SRCR::AUXIO15,
            14 => TICK_SRCR::AUXIO14,
            13 => TICK_SRCR::AUXIO13,
            12 => TICK_SRCR::AUXIO12,
            11 => TICK_SRCR::AUXIO11,
            10 => TICK_SRCR::AUXIO10,
            9 => TICK_SRCR::AUXIO9,
            8 => TICK_SRCR::AUXIO8,
            7 => TICK_SRCR::AUXIO7,
            6 => TICK_SRCR::AUXIO6,
            5 => TICK_SRCR::AUXIO5,
            4 => TICK_SRCR::AUXIO4,
            3 => TICK_SRCR::AUXIO3,
            2 => TICK_SRCR::AUXIO2,
            1 => TICK_SRCR::AUXIO1,
            0 => TICK_SRCR::AUXIO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_CLKSW_RDY`"]
    #[inline]
    pub fn is_aux_timer2_clksw_rdy(&self) -> bool {
        *self == TICK_SRCR::AUX_TIMER2_CLKSW_RDY
    }
    #[doc = "Checks if the value of the field is `AUX_DAC_HOLD_ACTIVE`"]
    #[inline]
    pub fn is_aux_dac_hold_active(&self) -> bool {
        *self == TICK_SRCR::AUX_DAC_HOLD_ACTIVE
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == TICK_SRCR::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == TICK_SRCR::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == TICK_SRCR::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == TICK_SRCR::AUX_ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == TICK_SRCR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ISRC_RESET_N`"]
    #[inline]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == TICK_SRCR::AUX_ISRC_RESET_N
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == TICK_SRCR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline]
    pub fn is_no_event(&self) -> bool {
        *self == TICK_SRCR::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == TICK_SRCR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == TICK_SRCR::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == TICK_SRCR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == TICK_SRCR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == TICK_SRCR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == TICK_SRCR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == TICK_SRCR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == TICK_SRCR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX1`"]
    #[inline]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == TICK_SRCR::MCU_OBSMUX1
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX0`"]
    #[inline]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == TICK_SRCR::MCU_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == TICK_SRCR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == TICK_SRCR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `VDDR_RECHARGE`"]
    #[inline]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == TICK_SRCR::VDDR_RECHARGE
    }
    #[doc = "Checks if the value of the field is `MCU_ACTIVE`"]
    #[inline]
    pub fn is_mcu_active(&self) -> bool {
        *self == TICK_SRCR::MCU_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PWR_DWN`"]
    #[inline]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == TICK_SRCR::PWR_DWN
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline]
    pub fn is_sclk_lf(&self) -> bool {
        *self == TICK_SRCR::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_TEMP_UPD`"]
    #[inline]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == TICK_SRCR::AON_BATMON_TEMP_UPD
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_BAT_UPD`"]
    #[inline]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == TICK_SRCR::AON_BATMON_BAT_UPD
    }
    #[doc = "Checks if the value of the field is `AON_RTC_4KHZ`"]
    #[inline]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == TICK_SRCR::AON_RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == TICK_SRCR::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == TICK_SRCR::AON_RTC_CH2
    }
    #[doc = "Checks if the value of the field is `MANUAL_EV`"]
    #[inline]
    pub fn is_manual_ev(&self) -> bool {
        *self == TICK_SRCR::MANUAL_EV
    }
    #[doc = "Checks if the value of the field is `AUXIO31`"]
    #[inline]
    pub fn is_auxio31(&self) -> bool {
        *self == TICK_SRCR::AUXIO31
    }
    #[doc = "Checks if the value of the field is `AUXIO30`"]
    #[inline]
    pub fn is_auxio30(&self) -> bool {
        *self == TICK_SRCR::AUXIO30
    }
    #[doc = "Checks if the value of the field is `AUXIO29`"]
    #[inline]
    pub fn is_auxio29(&self) -> bool {
        *self == TICK_SRCR::AUXIO29
    }
    #[doc = "Checks if the value of the field is `AUXIO28`"]
    #[inline]
    pub fn is_auxio28(&self) -> bool {
        *self == TICK_SRCR::AUXIO28
    }
    #[doc = "Checks if the value of the field is `AUXIO27`"]
    #[inline]
    pub fn is_auxio27(&self) -> bool {
        *self == TICK_SRCR::AUXIO27
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline]
    pub fn is_auxio26(&self) -> bool {
        *self == TICK_SRCR::AUXIO26
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline]
    pub fn is_auxio25(&self) -> bool {
        *self == TICK_SRCR::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline]
    pub fn is_auxio24(&self) -> bool {
        *self == TICK_SRCR::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline]
    pub fn is_auxio23(&self) -> bool {
        *self == TICK_SRCR::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline]
    pub fn is_auxio22(&self) -> bool {
        *self == TICK_SRCR::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline]
    pub fn is_auxio21(&self) -> bool {
        *self == TICK_SRCR::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline]
    pub fn is_auxio20(&self) -> bool {
        *self == TICK_SRCR::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline]
    pub fn is_auxio19(&self) -> bool {
        *self == TICK_SRCR::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO18`"]
    #[inline]
    pub fn is_auxio18(&self) -> bool {
        *self == TICK_SRCR::AUXIO18
    }
    #[doc = "Checks if the value of the field is `AUXIO17`"]
    #[inline]
    pub fn is_auxio17(&self) -> bool {
        *self == TICK_SRCR::AUXIO17
    }
    #[doc = "Checks if the value of the field is `AUXIO16`"]
    #[inline]
    pub fn is_auxio16(&self) -> bool {
        *self == TICK_SRCR::AUXIO16
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == TICK_SRCR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == TICK_SRCR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == TICK_SRCR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == TICK_SRCR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == TICK_SRCR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == TICK_SRCR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == TICK_SRCR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == TICK_SRCR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == TICK_SRCR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == TICK_SRCR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == TICK_SRCR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == TICK_SRCR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == TICK_SRCR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == TICK_SRCR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == TICK_SRCR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == TICK_SRCR::AUXIO0
    }
}
#[doc = r" Value of the field"]
pub struct PRER {
    bits: u8,
}
impl PRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    TICK,
    #[doc = "Use clock as source for prescaler. Note that AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the clock frequency."]
    CLK,
}
impl MODER {
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
            MODER::TICK => true,
            MODER::CLK => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            true => MODER::TICK,
            false => MODER::CLK,
        }
    }
    #[doc = "Checks if the value of the field is `TICK`"]
    #[inline]
    pub fn is_tick(&self) -> bool {
        *self == MODER::TICK
    }
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline]
    pub fn is_clk(&self) -> bool {
        *self == MODER::CLK
    }
}
#[doc = "Possible values of the field `RELOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOADR {
    #[doc = "Continuous mode.\n\nTimer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    CONT,
    #[doc = "Manual mode.\n\nTimer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    MAN,
}
impl RELOADR {
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
            RELOADR::CONT => true,
            RELOADR::MAN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RELOADR {
        match value {
            true => RELOADR::CONT,
            false => RELOADR::MAN,
        }
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline]
    pub fn is_cont(&self) -> bool {
        *self == RELOADR::CONT
    }
    #[doc = "Checks if the value of the field is `MAN`"]
    #[inline]
    pub fn is_man(&self) -> bool {
        *self == RELOADR::MAN
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
#[doc = "Values that can be written to the field `TICK_SRC_POL`"]
pub enum TICK_SRC_POLW {
    #[doc = "Count on falling edges of TICK_SRC."]
    FALL,
    #[doc = "Count on rising edges of TICK_SRC."]
    RISE,
}
impl TICK_SRC_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TICK_SRC_POLW::FALL => true,
            TICK_SRC_POLW::RISE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TICK_SRC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TICK_SRC_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TICK_SRC_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Count on falling edges of TICK_SRC."]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(TICK_SRC_POLW::FALL)
    }
    #[doc = "Count on rising edges of TICK_SRC."]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(TICK_SRC_POLW::RISE)
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
#[doc = "Values that can be written to the field `TICK_SRC`"]
pub enum TICK_SRCW {
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    AUX_TIMER2_CLKSW_RDY,
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    AUX_DAC_HOLD_ACTIVE,
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
    #[doc = "No event."]
    NO_EVENT,
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
impl TICK_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TICK_SRCW::AUX_TIMER2_CLKSW_RDY => 63,
            TICK_SRCW::AUX_DAC_HOLD_ACTIVE => 62,
            TICK_SRCW::AUX_SMPH_AUTOTAKE_DONE => 61,
            TICK_SRCW::AUX_ADC_FIFO_NOT_EMPTY => 60,
            TICK_SRCW::AUX_ADC_FIFO_ALMOST_FULL => 59,
            TICK_SRCW::AUX_ADC_IRQ => 58,
            TICK_SRCW::AUX_ADC_DONE => 57,
            TICK_SRCW::AUX_ISRC_RESET_N => 56,
            TICK_SRCW::AUX_TDC_DONE => 55,
            TICK_SRCW::NO_EVENT => 54,
            TICK_SRCW::AUX_TIMER1_EV => 53,
            TICK_SRCW::AUX_TIMER2_PULSE => 52,
            TICK_SRCW::AUX_TIMER2_EV3 => 51,
            TICK_SRCW::AUX_TIMER2_EV2 => 50,
            TICK_SRCW::AUX_TIMER2_EV1 => 49,
            TICK_SRCW::AUX_TIMER2_EV0 => 48,
            TICK_SRCW::AUX_COMPB => 47,
            TICK_SRCW::AUX_COMPA => 46,
            TICK_SRCW::MCU_OBSMUX1 => 45,
            TICK_SRCW::MCU_OBSMUX0 => 44,
            TICK_SRCW::MCU_EV => 43,
            TICK_SRCW::ACLK_REF => 42,
            TICK_SRCW::VDDR_RECHARGE => 41,
            TICK_SRCW::MCU_ACTIVE => 40,
            TICK_SRCW::PWR_DWN => 39,
            TICK_SRCW::SCLK_LF => 38,
            TICK_SRCW::AON_BATMON_TEMP_UPD => 37,
            TICK_SRCW::AON_BATMON_BAT_UPD => 36,
            TICK_SRCW::AON_RTC_4KHZ => 35,
            TICK_SRCW::AON_RTC_CH2_DLY => 34,
            TICK_SRCW::AON_RTC_CH2 => 33,
            TICK_SRCW::MANUAL_EV => 32,
            TICK_SRCW::AUXIO31 => 31,
            TICK_SRCW::AUXIO30 => 30,
            TICK_SRCW::AUXIO29 => 29,
            TICK_SRCW::AUXIO28 => 28,
            TICK_SRCW::AUXIO27 => 27,
            TICK_SRCW::AUXIO26 => 26,
            TICK_SRCW::AUXIO25 => 25,
            TICK_SRCW::AUXIO24 => 24,
            TICK_SRCW::AUXIO23 => 23,
            TICK_SRCW::AUXIO22 => 22,
            TICK_SRCW::AUXIO21 => 21,
            TICK_SRCW::AUXIO20 => 20,
            TICK_SRCW::AUXIO19 => 19,
            TICK_SRCW::AUXIO18 => 18,
            TICK_SRCW::AUXIO17 => 17,
            TICK_SRCW::AUXIO16 => 16,
            TICK_SRCW::AUXIO15 => 15,
            TICK_SRCW::AUXIO14 => 14,
            TICK_SRCW::AUXIO13 => 13,
            TICK_SRCW::AUXIO12 => 12,
            TICK_SRCW::AUXIO11 => 11,
            TICK_SRCW::AUXIO10 => 10,
            TICK_SRCW::AUXIO9 => 9,
            TICK_SRCW::AUXIO8 => 8,
            TICK_SRCW::AUXIO7 => 7,
            TICK_SRCW::AUXIO6 => 6,
            TICK_SRCW::AUXIO5 => 5,
            TICK_SRCW::AUXIO4 => 4,
            TICK_SRCW::AUXIO3 => 3,
            TICK_SRCW::AUXIO2 => 2,
            TICK_SRCW::AUXIO1 => 1,
            TICK_SRCW::AUXIO0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TICK_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TICK_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TICK_SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    #[inline]
    pub fn aux_timer2_clksw_rdy(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_TIMER2_CLKSW_RDY)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline]
    pub fn aux_dac_hold_active(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_DAC_HOLD_ACTIVE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_ADC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline]
    pub fn aux_isrc_reset_n(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_ISRC_RESET_N)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_TDC_DONE)
    }
    #[doc = "No event."]
    #[inline]
    pub fn no_event(self) -> &'a mut W {
        self.variant(TICK_SRCW::NO_EVENT)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_TIMER2_PULSE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_TIMER2_EV3)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_TIMER2_EV2)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_TIMER2_EV1)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_TIMER2_EV0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline]
    pub fn mcu_obsmux1(self) -> &'a mut W {
        self.variant(TICK_SRCW::MCU_OBSMUX1)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline]
    pub fn mcu_obsmux0(self) -> &'a mut W {
        self.variant(TICK_SRCW::MCU_OBSMUX0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(TICK_SRCW::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(TICK_SRCW::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline]
    pub fn vddr_recharge(self) -> &'a mut W {
        self.variant(TICK_SRCW::VDDR_RECHARGE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline]
    pub fn mcu_active(self) -> &'a mut W {
        self.variant(TICK_SRCW::MCU_ACTIVE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline]
    pub fn pwr_dwn(self) -> &'a mut W {
        self.variant(TICK_SRCW::PWR_DWN)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(TICK_SRCW::SCLK_LF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline]
    pub fn aon_batmon_temp_upd(self) -> &'a mut W {
        self.variant(TICK_SRCW::AON_BATMON_TEMP_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline]
    pub fn aon_batmon_bat_upd(self) -> &'a mut W {
        self.variant(TICK_SRCW::AON_BATMON_BAT_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline]
    pub fn aon_rtc_4khz(self) -> &'a mut W {
        self.variant(TICK_SRCW::AON_RTC_4KHZ)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(TICK_SRCW::AON_RTC_CH2_DLY)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(TICK_SRCW::AON_RTC_CH2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline]
    pub fn manual_ev(self) -> &'a mut W {
        self.variant(TICK_SRCW::MANUAL_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline]
    pub fn auxio31(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline]
    pub fn auxio30(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline]
    pub fn auxio29(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline]
    pub fn auxio28(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline]
    pub fn auxio27(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline]
    pub fn auxio18(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline]
    pub fn auxio17(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline]
    pub fn auxio16(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PREW<'a> {
    w: &'a mut W,
}
impl<'a> _PREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
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
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    TICK,
    #[doc = "Use clock as source for prescaler. Note that AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the clock frequency."]
    CLK,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::TICK => true,
            MODEW::CLK => false,
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
            self.bit(variant._bits())
        }
    }
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    #[inline]
    pub fn tick(self) -> &'a mut W {
        self.variant(MODEW::TICK)
    }
    #[doc = "Use clock as source for prescaler. Note that AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the clock frequency."]
    #[inline]
    pub fn clk(self) -> &'a mut W {
        self.variant(MODEW::CLK)
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
#[doc = "Values that can be written to the field `RELOAD`"]
pub enum RELOADW {
    #[doc = "Continuous mode.\n\nTimer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    CONT,
    #[doc = "Manual mode.\n\nTimer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    MAN,
}
impl RELOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RELOADW::CONT => true,
            RELOADW::MAN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RELOADW<'a> {
    w: &'a mut W,
}
impl<'a> _RELOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RELOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    #[inline]
    pub fn cont(self) -> &'a mut W {
        self.variant(RELOADW::CONT)
    }
    #[doc = "Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    #[inline]
    pub fn man(self) -> &'a mut W {
        self.variant(RELOADW::MAN)
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
    #[doc = "Bit 14 - 14:14\\] Tick source polarity for Timer 0."]
    #[inline]
    pub fn tick_src_pol(&self) -> TICK_SRC_POLR {
        TICK_SRC_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:13 - 13:8\\] Select Timer 0 tick source from the synchronous event bus."]
    #[inline]
    pub fn tick_src(&self) -> TICK_SRCR {
        TICK_SRCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - 7:4\\] Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline]
    pub fn pre(&self) -> PRER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRER { bits }
    }
    #[doc = "Bits 2:3 - 3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Timer 0 reload mode."]
    #[inline]
    pub fn reload(&self) -> RELOADR {
        RELOADR::_from({
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
    #[doc = "Bits 15:31 - 31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved15(&mut self) -> _RESERVED15W {
        _RESERVED15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Tick source polarity for Timer 0."]
    #[inline]
    pub fn tick_src_pol(&mut self) -> _TICK_SRC_POLW {
        _TICK_SRC_POLW { w: self }
    }
    #[doc = "Bits 8:13 - 13:8\\] Select Timer 0 tick source from the synchronous event bus."]
    #[inline]
    pub fn tick_src(&mut self) -> _TICK_SRCW {
        _TICK_SRCW { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline]
    pub fn pre(&mut self) -> _PREW {
        _PREW { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Timer 0 reload mode."]
    #[inline]
    pub fn reload(&mut self) -> _RELOADW {
        _RELOADW { w: self }
    }
}
