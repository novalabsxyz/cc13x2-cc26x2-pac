#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCEWEVCFG1 {
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
pub struct EV0_POLR {
    bits: bool,
}
impl EV0_POLR {
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
pub struct EV1_POLR {
    bits: bool,
}
impl EV1_POLR {
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
#[doc = "Possible values of the field `EV1_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV1_SELR {
    #[doc = "EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    AUX_TIMER2_CLKSWITCH_RDY,
    #[doc = "EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    AUX_DAC_HOLD_ACTIVE,
    #[doc = "EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AUX_SMPH_AUTOTAKE_DONE,
    #[doc = "EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AUX_ADC_FIFO_NOT_EMPTY,
    #[doc = "EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL,
    #[doc = "EVSTAT3.AUX_ADC_IRQ"]
    AUX_ADC_IRQ,
    #[doc = "EVSTAT3.AUX_ADC_DONE"]
    AUX_ADC_DONE,
    #[doc = "EVSTAT3.AUX_ISRC_RESET_N"]
    AUX_ISRC_RESET_N,
    #[doc = "EVSTAT3.AUX_TDC_DONE"]
    AUX_TDC_DONE,
    #[doc = "EVSTAT3.AUX_TIMER0_EV"]
    AUX_TIMER0_EV,
    #[doc = "EVSTAT3.AUX_TIMER1_EV"]
    AUX_TIMER1_EV,
    #[doc = "EVSTAT3.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE,
    #[doc = "EVSTAT3.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3,
    #[doc = "EVSTAT3.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2,
    #[doc = "EVSTAT3.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1,
    #[doc = "EVSTAT3.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0,
    #[doc = "EVSTAT2.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "EVSTAT2.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "EVSTAT2.MCU_OBSMUX1"]
    MCU_OBSMUX1,
    #[doc = "EVSTAT2.MCU_OBSMUX0"]
    MCU_OBSMUX0,
    #[doc = "EVSTAT2.MCU_EV"]
    MCU_EV,
    #[doc = "EVSTAT2.ACLK_REF"]
    ACLK_REF,
    #[doc = "EVSTAT2.VDDR_RECHARGE"]
    VDDR_RECHARGE,
    #[doc = "EVSTAT2.MCU_ACTIVE"]
    MCU_ACTIVE,
    #[doc = "EVSTAT2.PWR_DWN"]
    PWR_DWN,
    #[doc = "EVSTAT2.SCLK_LF"]
    SCLK_LF,
    #[doc = "EVSTAT2.AON_BATMON_TEMP_UPD"]
    AON_BATMON_TEMP_UPD,
    #[doc = "EVSTAT2.AON_BATMON_BAT_UPD"]
    AON_BATMON_BAT_UPD,
    #[doc = "EVSTAT2.AON_RTC_4KHZ"]
    AON_RTC_4KHZ,
    #[doc = "EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY,
    #[doc = "EVSTAT2.AON_RTC_CH2"]
    AON_RTC_CH2,
    #[doc = "Programmable delay event as described in PROGDLY"]
    AUX_PROG_DLY_IDLE,
    #[doc = "EVSTAT1.AUXIO31"]
    AUXIO31,
    #[doc = "EVSTAT1.AUXIO30"]
    AUXIO30,
    #[doc = "EVSTAT1.AUXIO29"]
    AUXIO29,
    #[doc = "EVSTAT1.AUXIO28"]
    AUXIO28,
    #[doc = "EVSTAT1.AUXIO27"]
    AUXIO27,
    #[doc = "EVSTAT1.AUXIO26"]
    AUXIO26,
    #[doc = "EVSTAT1.AUXIO25"]
    AUXIO25,
    #[doc = "EVSTAT1.AUXIO24"]
    AUXIO24,
    #[doc = "EVSTAT1.AUXIO23"]
    AUXIO23,
    #[doc = "EVSTAT1.AUXIO22"]
    AUXIO22,
    #[doc = "EVSTAT1.AUXIO21"]
    AUXIO21,
    #[doc = "EVSTAT1.AUXIO20"]
    AUXIO20,
    #[doc = "EVSTAT1.AUXIO19"]
    AUXIO19,
    #[doc = "EVSTAT1.AUXIO18"]
    AUXIO18,
    #[doc = "EVSTAT1.AUXIO17"]
    AUXIO17,
    #[doc = "EVSTAT1.AUXIO16"]
    AUXIO16,
    #[doc = "EVSTAT0.AUXIO15"]
    AUXIO15,
    #[doc = "EVSTAT0.AUXIO14"]
    AUXIO14,
    #[doc = "EVSTAT0.AUXIO13"]
    AUXIO13,
    #[doc = "EVSTAT0.AUXIO12"]
    AUXIO12,
    #[doc = "EVSTAT0.AUXIO11"]
    AUXIO11,
    #[doc = "EVSTAT0.AUXIO10"]
    AUXIO10,
    #[doc = "EVSTAT0.AUXIO9"]
    AUXIO9,
    #[doc = "EVSTAT0.AUXIO8"]
    AUXIO8,
    #[doc = "EVSTAT0.AUXIO7"]
    AUXIO7,
    #[doc = "EVSTAT0.AUXIO6"]
    AUXIO6,
    #[doc = "EVSTAT0.AUXIO5"]
    AUXIO5,
    #[doc = "EVSTAT0.AUXIO4"]
    AUXIO4,
    #[doc = "EVSTAT0.AUXIO3"]
    AUXIO3,
    #[doc = "EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "EVSTAT0.AUXIO0"]
    AUXIO0,
}
impl EV1_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EV1_SELR::AUX_TIMER2_CLKSWITCH_RDY => 63,
            EV1_SELR::AUX_DAC_HOLD_ACTIVE => 62,
            EV1_SELR::AUX_SMPH_AUTOTAKE_DONE => 61,
            EV1_SELR::AUX_ADC_FIFO_NOT_EMPTY => 60,
            EV1_SELR::AUX_ADC_FIFO_ALMOST_FULL => 59,
            EV1_SELR::AUX_ADC_IRQ => 58,
            EV1_SELR::AUX_ADC_DONE => 57,
            EV1_SELR::AUX_ISRC_RESET_N => 56,
            EV1_SELR::AUX_TDC_DONE => 55,
            EV1_SELR::AUX_TIMER0_EV => 54,
            EV1_SELR::AUX_TIMER1_EV => 53,
            EV1_SELR::AUX_TIMER2_PULSE => 52,
            EV1_SELR::AUX_TIMER2_EV3 => 51,
            EV1_SELR::AUX_TIMER2_EV2 => 50,
            EV1_SELR::AUX_TIMER2_EV1 => 49,
            EV1_SELR::AUX_TIMER2_EV0 => 48,
            EV1_SELR::AUX_COMPB => 47,
            EV1_SELR::AUX_COMPA => 46,
            EV1_SELR::MCU_OBSMUX1 => 45,
            EV1_SELR::MCU_OBSMUX0 => 44,
            EV1_SELR::MCU_EV => 43,
            EV1_SELR::ACLK_REF => 42,
            EV1_SELR::VDDR_RECHARGE => 41,
            EV1_SELR::MCU_ACTIVE => 40,
            EV1_SELR::PWR_DWN => 39,
            EV1_SELR::SCLK_LF => 38,
            EV1_SELR::AON_BATMON_TEMP_UPD => 37,
            EV1_SELR::AON_BATMON_BAT_UPD => 36,
            EV1_SELR::AON_RTC_4KHZ => 35,
            EV1_SELR::AON_RTC_CH2_DLY => 34,
            EV1_SELR::AON_RTC_CH2 => 33,
            EV1_SELR::AUX_PROG_DLY_IDLE => 32,
            EV1_SELR::AUXIO31 => 31,
            EV1_SELR::AUXIO30 => 30,
            EV1_SELR::AUXIO29 => 29,
            EV1_SELR::AUXIO28 => 28,
            EV1_SELR::AUXIO27 => 27,
            EV1_SELR::AUXIO26 => 26,
            EV1_SELR::AUXIO25 => 25,
            EV1_SELR::AUXIO24 => 24,
            EV1_SELR::AUXIO23 => 23,
            EV1_SELR::AUXIO22 => 22,
            EV1_SELR::AUXIO21 => 21,
            EV1_SELR::AUXIO20 => 20,
            EV1_SELR::AUXIO19 => 19,
            EV1_SELR::AUXIO18 => 18,
            EV1_SELR::AUXIO17 => 17,
            EV1_SELR::AUXIO16 => 16,
            EV1_SELR::AUXIO15 => 15,
            EV1_SELR::AUXIO14 => 14,
            EV1_SELR::AUXIO13 => 13,
            EV1_SELR::AUXIO12 => 12,
            EV1_SELR::AUXIO11 => 11,
            EV1_SELR::AUXIO10 => 10,
            EV1_SELR::AUXIO9 => 9,
            EV1_SELR::AUXIO8 => 8,
            EV1_SELR::AUXIO7 => 7,
            EV1_SELR::AUXIO6 => 6,
            EV1_SELR::AUXIO5 => 5,
            EV1_SELR::AUXIO4 => 4,
            EV1_SELR::AUXIO3 => 3,
            EV1_SELR::AUXIO2 => 2,
            EV1_SELR::AUXIO1 => 1,
            EV1_SELR::AUXIO0 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EV1_SELR {
        match value {
            63 => EV1_SELR::AUX_TIMER2_CLKSWITCH_RDY,
            62 => EV1_SELR::AUX_DAC_HOLD_ACTIVE,
            61 => EV1_SELR::AUX_SMPH_AUTOTAKE_DONE,
            60 => EV1_SELR::AUX_ADC_FIFO_NOT_EMPTY,
            59 => EV1_SELR::AUX_ADC_FIFO_ALMOST_FULL,
            58 => EV1_SELR::AUX_ADC_IRQ,
            57 => EV1_SELR::AUX_ADC_DONE,
            56 => EV1_SELR::AUX_ISRC_RESET_N,
            55 => EV1_SELR::AUX_TDC_DONE,
            54 => EV1_SELR::AUX_TIMER0_EV,
            53 => EV1_SELR::AUX_TIMER1_EV,
            52 => EV1_SELR::AUX_TIMER2_PULSE,
            51 => EV1_SELR::AUX_TIMER2_EV3,
            50 => EV1_SELR::AUX_TIMER2_EV2,
            49 => EV1_SELR::AUX_TIMER2_EV1,
            48 => EV1_SELR::AUX_TIMER2_EV0,
            47 => EV1_SELR::AUX_COMPB,
            46 => EV1_SELR::AUX_COMPA,
            45 => EV1_SELR::MCU_OBSMUX1,
            44 => EV1_SELR::MCU_OBSMUX0,
            43 => EV1_SELR::MCU_EV,
            42 => EV1_SELR::ACLK_REF,
            41 => EV1_SELR::VDDR_RECHARGE,
            40 => EV1_SELR::MCU_ACTIVE,
            39 => EV1_SELR::PWR_DWN,
            38 => EV1_SELR::SCLK_LF,
            37 => EV1_SELR::AON_BATMON_TEMP_UPD,
            36 => EV1_SELR::AON_BATMON_BAT_UPD,
            35 => EV1_SELR::AON_RTC_4KHZ,
            34 => EV1_SELR::AON_RTC_CH2_DLY,
            33 => EV1_SELR::AON_RTC_CH2,
            32 => EV1_SELR::AUX_PROG_DLY_IDLE,
            31 => EV1_SELR::AUXIO31,
            30 => EV1_SELR::AUXIO30,
            29 => EV1_SELR::AUXIO29,
            28 => EV1_SELR::AUXIO28,
            27 => EV1_SELR::AUXIO27,
            26 => EV1_SELR::AUXIO26,
            25 => EV1_SELR::AUXIO25,
            24 => EV1_SELR::AUXIO24,
            23 => EV1_SELR::AUXIO23,
            22 => EV1_SELR::AUXIO22,
            21 => EV1_SELR::AUXIO21,
            20 => EV1_SELR::AUXIO20,
            19 => EV1_SELR::AUXIO19,
            18 => EV1_SELR::AUXIO18,
            17 => EV1_SELR::AUXIO17,
            16 => EV1_SELR::AUXIO16,
            15 => EV1_SELR::AUXIO15,
            14 => EV1_SELR::AUXIO14,
            13 => EV1_SELR::AUXIO13,
            12 => EV1_SELR::AUXIO12,
            11 => EV1_SELR::AUXIO11,
            10 => EV1_SELR::AUXIO10,
            9 => EV1_SELR::AUXIO9,
            8 => EV1_SELR::AUXIO8,
            7 => EV1_SELR::AUXIO7,
            6 => EV1_SELR::AUXIO6,
            5 => EV1_SELR::AUXIO5,
            4 => EV1_SELR::AUXIO4,
            3 => EV1_SELR::AUXIO3,
            2 => EV1_SELR::AUXIO2,
            1 => EV1_SELR::AUXIO1,
            0 => EV1_SELR::AUXIO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_CLKSWITCH_RDY`"]
    #[inline]
    pub fn is_aux_timer2_clkswitch_rdy(&self) -> bool {
        *self == EV1_SELR::AUX_TIMER2_CLKSWITCH_RDY
    }
    #[doc = "Checks if the value of the field is `AUX_DAC_HOLD_ACTIVE`"]
    #[inline]
    pub fn is_aux_dac_hold_active(&self) -> bool {
        *self == EV1_SELR::AUX_DAC_HOLD_ACTIVE
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == EV1_SELR::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == EV1_SELR::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == EV1_SELR::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == EV1_SELR::AUX_ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == EV1_SELR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ISRC_RESET_N`"]
    #[inline]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == EV1_SELR::AUX_ISRC_RESET_N
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EV1_SELR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == EV1_SELR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == EV1_SELR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == EV1_SELR::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == EV1_SELR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == EV1_SELR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == EV1_SELR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == EV1_SELR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == EV1_SELR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == EV1_SELR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX1`"]
    #[inline]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == EV1_SELR::MCU_OBSMUX1
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX0`"]
    #[inline]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == EV1_SELR::MCU_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == EV1_SELR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == EV1_SELR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `VDDR_RECHARGE`"]
    #[inline]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == EV1_SELR::VDDR_RECHARGE
    }
    #[doc = "Checks if the value of the field is `MCU_ACTIVE`"]
    #[inline]
    pub fn is_mcu_active(&self) -> bool {
        *self == EV1_SELR::MCU_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PWR_DWN`"]
    #[inline]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == EV1_SELR::PWR_DWN
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline]
    pub fn is_sclk_lf(&self) -> bool {
        *self == EV1_SELR::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_TEMP_UPD`"]
    #[inline]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == EV1_SELR::AON_BATMON_TEMP_UPD
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_BAT_UPD`"]
    #[inline]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == EV1_SELR::AON_BATMON_BAT_UPD
    }
    #[doc = "Checks if the value of the field is `AON_RTC_4KHZ`"]
    #[inline]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == EV1_SELR::AON_RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == EV1_SELR::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == EV1_SELR::AON_RTC_CH2
    }
    #[doc = "Checks if the value of the field is `AUX_PROG_DLY_IDLE`"]
    #[inline]
    pub fn is_aux_prog_dly_idle(&self) -> bool {
        *self == EV1_SELR::AUX_PROG_DLY_IDLE
    }
    #[doc = "Checks if the value of the field is `AUXIO31`"]
    #[inline]
    pub fn is_auxio31(&self) -> bool {
        *self == EV1_SELR::AUXIO31
    }
    #[doc = "Checks if the value of the field is `AUXIO30`"]
    #[inline]
    pub fn is_auxio30(&self) -> bool {
        *self == EV1_SELR::AUXIO30
    }
    #[doc = "Checks if the value of the field is `AUXIO29`"]
    #[inline]
    pub fn is_auxio29(&self) -> bool {
        *self == EV1_SELR::AUXIO29
    }
    #[doc = "Checks if the value of the field is `AUXIO28`"]
    #[inline]
    pub fn is_auxio28(&self) -> bool {
        *self == EV1_SELR::AUXIO28
    }
    #[doc = "Checks if the value of the field is `AUXIO27`"]
    #[inline]
    pub fn is_auxio27(&self) -> bool {
        *self == EV1_SELR::AUXIO27
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline]
    pub fn is_auxio26(&self) -> bool {
        *self == EV1_SELR::AUXIO26
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline]
    pub fn is_auxio25(&self) -> bool {
        *self == EV1_SELR::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline]
    pub fn is_auxio24(&self) -> bool {
        *self == EV1_SELR::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline]
    pub fn is_auxio23(&self) -> bool {
        *self == EV1_SELR::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline]
    pub fn is_auxio22(&self) -> bool {
        *self == EV1_SELR::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline]
    pub fn is_auxio21(&self) -> bool {
        *self == EV1_SELR::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline]
    pub fn is_auxio20(&self) -> bool {
        *self == EV1_SELR::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline]
    pub fn is_auxio19(&self) -> bool {
        *self == EV1_SELR::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO18`"]
    #[inline]
    pub fn is_auxio18(&self) -> bool {
        *self == EV1_SELR::AUXIO18
    }
    #[doc = "Checks if the value of the field is `AUXIO17`"]
    #[inline]
    pub fn is_auxio17(&self) -> bool {
        *self == EV1_SELR::AUXIO17
    }
    #[doc = "Checks if the value of the field is `AUXIO16`"]
    #[inline]
    pub fn is_auxio16(&self) -> bool {
        *self == EV1_SELR::AUXIO16
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == EV1_SELR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == EV1_SELR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == EV1_SELR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == EV1_SELR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == EV1_SELR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == EV1_SELR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == EV1_SELR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == EV1_SELR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == EV1_SELR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == EV1_SELR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == EV1_SELR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == EV1_SELR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == EV1_SELR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == EV1_SELR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == EV1_SELR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == EV1_SELR::AUXIO0
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
pub struct _EV0_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EV0_POLW<'a> {
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
#[doc = r" Proxy"]
pub struct _EV1_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _EV1_POLW<'a> {
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
#[doc = "Values that can be written to the field `EV1_SEL`"]
pub enum EV1_SELW {
    #[doc = "EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    AUX_TIMER2_CLKSWITCH_RDY,
    #[doc = "EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    AUX_DAC_HOLD_ACTIVE,
    #[doc = "EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AUX_SMPH_AUTOTAKE_DONE,
    #[doc = "EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AUX_ADC_FIFO_NOT_EMPTY,
    #[doc = "EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL,
    #[doc = "EVSTAT3.AUX_ADC_IRQ"]
    AUX_ADC_IRQ,
    #[doc = "EVSTAT3.AUX_ADC_DONE"]
    AUX_ADC_DONE,
    #[doc = "EVSTAT3.AUX_ISRC_RESET_N"]
    AUX_ISRC_RESET_N,
    #[doc = "EVSTAT3.AUX_TDC_DONE"]
    AUX_TDC_DONE,
    #[doc = "EVSTAT3.AUX_TIMER0_EV"]
    AUX_TIMER0_EV,
    #[doc = "EVSTAT3.AUX_TIMER1_EV"]
    AUX_TIMER1_EV,
    #[doc = "EVSTAT3.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE,
    #[doc = "EVSTAT3.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3,
    #[doc = "EVSTAT3.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2,
    #[doc = "EVSTAT3.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1,
    #[doc = "EVSTAT3.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0,
    #[doc = "EVSTAT2.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "EVSTAT2.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "EVSTAT2.MCU_OBSMUX1"]
    MCU_OBSMUX1,
    #[doc = "EVSTAT2.MCU_OBSMUX0"]
    MCU_OBSMUX0,
    #[doc = "EVSTAT2.MCU_EV"]
    MCU_EV,
    #[doc = "EVSTAT2.ACLK_REF"]
    ACLK_REF,
    #[doc = "EVSTAT2.VDDR_RECHARGE"]
    VDDR_RECHARGE,
    #[doc = "EVSTAT2.MCU_ACTIVE"]
    MCU_ACTIVE,
    #[doc = "EVSTAT2.PWR_DWN"]
    PWR_DWN,
    #[doc = "EVSTAT2.SCLK_LF"]
    SCLK_LF,
    #[doc = "EVSTAT2.AON_BATMON_TEMP_UPD"]
    AON_BATMON_TEMP_UPD,
    #[doc = "EVSTAT2.AON_BATMON_BAT_UPD"]
    AON_BATMON_BAT_UPD,
    #[doc = "EVSTAT2.AON_RTC_4KHZ"]
    AON_RTC_4KHZ,
    #[doc = "EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY,
    #[doc = "EVSTAT2.AON_RTC_CH2"]
    AON_RTC_CH2,
    #[doc = "Programmable delay event as described in PROGDLY"]
    AUX_PROG_DLY_IDLE,
    #[doc = "EVSTAT1.AUXIO31"]
    AUXIO31,
    #[doc = "EVSTAT1.AUXIO30"]
    AUXIO30,
    #[doc = "EVSTAT1.AUXIO29"]
    AUXIO29,
    #[doc = "EVSTAT1.AUXIO28"]
    AUXIO28,
    #[doc = "EVSTAT1.AUXIO27"]
    AUXIO27,
    #[doc = "EVSTAT1.AUXIO26"]
    AUXIO26,
    #[doc = "EVSTAT1.AUXIO25"]
    AUXIO25,
    #[doc = "EVSTAT1.AUXIO24"]
    AUXIO24,
    #[doc = "EVSTAT1.AUXIO23"]
    AUXIO23,
    #[doc = "EVSTAT1.AUXIO22"]
    AUXIO22,
    #[doc = "EVSTAT1.AUXIO21"]
    AUXIO21,
    #[doc = "EVSTAT1.AUXIO20"]
    AUXIO20,
    #[doc = "EVSTAT1.AUXIO19"]
    AUXIO19,
    #[doc = "EVSTAT1.AUXIO18"]
    AUXIO18,
    #[doc = "EVSTAT1.AUXIO17"]
    AUXIO17,
    #[doc = "EVSTAT1.AUXIO16"]
    AUXIO16,
    #[doc = "EVSTAT0.AUXIO15"]
    AUXIO15,
    #[doc = "EVSTAT0.AUXIO14"]
    AUXIO14,
    #[doc = "EVSTAT0.AUXIO13"]
    AUXIO13,
    #[doc = "EVSTAT0.AUXIO12"]
    AUXIO12,
    #[doc = "EVSTAT0.AUXIO11"]
    AUXIO11,
    #[doc = "EVSTAT0.AUXIO10"]
    AUXIO10,
    #[doc = "EVSTAT0.AUXIO9"]
    AUXIO9,
    #[doc = "EVSTAT0.AUXIO8"]
    AUXIO8,
    #[doc = "EVSTAT0.AUXIO7"]
    AUXIO7,
    #[doc = "EVSTAT0.AUXIO6"]
    AUXIO6,
    #[doc = "EVSTAT0.AUXIO5"]
    AUXIO5,
    #[doc = "EVSTAT0.AUXIO4"]
    AUXIO4,
    #[doc = "EVSTAT0.AUXIO3"]
    AUXIO3,
    #[doc = "EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "EVSTAT0.AUXIO0"]
    AUXIO0,
}
impl EV1_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EV1_SELW::AUX_TIMER2_CLKSWITCH_RDY => 63,
            EV1_SELW::AUX_DAC_HOLD_ACTIVE => 62,
            EV1_SELW::AUX_SMPH_AUTOTAKE_DONE => 61,
            EV1_SELW::AUX_ADC_FIFO_NOT_EMPTY => 60,
            EV1_SELW::AUX_ADC_FIFO_ALMOST_FULL => 59,
            EV1_SELW::AUX_ADC_IRQ => 58,
            EV1_SELW::AUX_ADC_DONE => 57,
            EV1_SELW::AUX_ISRC_RESET_N => 56,
            EV1_SELW::AUX_TDC_DONE => 55,
            EV1_SELW::AUX_TIMER0_EV => 54,
            EV1_SELW::AUX_TIMER1_EV => 53,
            EV1_SELW::AUX_TIMER2_PULSE => 52,
            EV1_SELW::AUX_TIMER2_EV3 => 51,
            EV1_SELW::AUX_TIMER2_EV2 => 50,
            EV1_SELW::AUX_TIMER2_EV1 => 49,
            EV1_SELW::AUX_TIMER2_EV0 => 48,
            EV1_SELW::AUX_COMPB => 47,
            EV1_SELW::AUX_COMPA => 46,
            EV1_SELW::MCU_OBSMUX1 => 45,
            EV1_SELW::MCU_OBSMUX0 => 44,
            EV1_SELW::MCU_EV => 43,
            EV1_SELW::ACLK_REF => 42,
            EV1_SELW::VDDR_RECHARGE => 41,
            EV1_SELW::MCU_ACTIVE => 40,
            EV1_SELW::PWR_DWN => 39,
            EV1_SELW::SCLK_LF => 38,
            EV1_SELW::AON_BATMON_TEMP_UPD => 37,
            EV1_SELW::AON_BATMON_BAT_UPD => 36,
            EV1_SELW::AON_RTC_4KHZ => 35,
            EV1_SELW::AON_RTC_CH2_DLY => 34,
            EV1_SELW::AON_RTC_CH2 => 33,
            EV1_SELW::AUX_PROG_DLY_IDLE => 32,
            EV1_SELW::AUXIO31 => 31,
            EV1_SELW::AUXIO30 => 30,
            EV1_SELW::AUXIO29 => 29,
            EV1_SELW::AUXIO28 => 28,
            EV1_SELW::AUXIO27 => 27,
            EV1_SELW::AUXIO26 => 26,
            EV1_SELW::AUXIO25 => 25,
            EV1_SELW::AUXIO24 => 24,
            EV1_SELW::AUXIO23 => 23,
            EV1_SELW::AUXIO22 => 22,
            EV1_SELW::AUXIO21 => 21,
            EV1_SELW::AUXIO20 => 20,
            EV1_SELW::AUXIO19 => 19,
            EV1_SELW::AUXIO18 => 18,
            EV1_SELW::AUXIO17 => 17,
            EV1_SELW::AUXIO16 => 16,
            EV1_SELW::AUXIO15 => 15,
            EV1_SELW::AUXIO14 => 14,
            EV1_SELW::AUXIO13 => 13,
            EV1_SELW::AUXIO12 => 12,
            EV1_SELW::AUXIO11 => 11,
            EV1_SELW::AUXIO10 => 10,
            EV1_SELW::AUXIO9 => 9,
            EV1_SELW::AUXIO8 => 8,
            EV1_SELW::AUXIO7 => 7,
            EV1_SELW::AUXIO6 => 6,
            EV1_SELW::AUXIO5 => 5,
            EV1_SELW::AUXIO4 => 4,
            EV1_SELW::AUXIO3 => 3,
            EV1_SELW::AUXIO2 => 2,
            EV1_SELW::AUXIO1 => 1,
            EV1_SELW::AUXIO0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV1_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _EV1_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV1_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    #[inline]
    pub fn aux_timer2_clkswitch_rdy(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_TIMER2_CLKSWITCH_RDY)
    }
    #[doc = "EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline]
    pub fn aux_dac_hold_active(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_DAC_HOLD_ACTIVE)
    }
    #[doc = "EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "EVSTAT3.AUX_ADC_IRQ"]
    #[inline]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_ADC_IRQ)
    }
    #[doc = "EVSTAT3.AUX_ADC_DONE"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_ADC_DONE)
    }
    #[doc = "EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline]
    pub fn aux_isrc_reset_n(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_ISRC_RESET_N)
    }
    #[doc = "EVSTAT3.AUX_TDC_DONE"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_TDC_DONE)
    }
    #[doc = "EVSTAT3.AUX_TIMER0_EV"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_TIMER0_EV)
    }
    #[doc = "EVSTAT3.AUX_TIMER1_EV"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_TIMER1_EV)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_TIMER2_PULSE)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV3"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_TIMER2_EV3)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_TIMER2_EV2)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV1"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_TIMER2_EV1)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV0"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_TIMER2_EV0)
    }
    #[doc = "EVSTAT2.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_COMPB)
    }
    #[doc = "EVSTAT2.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_COMPA)
    }
    #[doc = "EVSTAT2.MCU_OBSMUX1"]
    #[inline]
    pub fn mcu_obsmux1(self) -> &'a mut W {
        self.variant(EV1_SELW::MCU_OBSMUX1)
    }
    #[doc = "EVSTAT2.MCU_OBSMUX0"]
    #[inline]
    pub fn mcu_obsmux0(self) -> &'a mut W {
        self.variant(EV1_SELW::MCU_OBSMUX0)
    }
    #[doc = "EVSTAT2.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(EV1_SELW::MCU_EV)
    }
    #[doc = "EVSTAT2.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(EV1_SELW::ACLK_REF)
    }
    #[doc = "EVSTAT2.VDDR_RECHARGE"]
    #[inline]
    pub fn vddr_recharge(self) -> &'a mut W {
        self.variant(EV1_SELW::VDDR_RECHARGE)
    }
    #[doc = "EVSTAT2.MCU_ACTIVE"]
    #[inline]
    pub fn mcu_active(self) -> &'a mut W {
        self.variant(EV1_SELW::MCU_ACTIVE)
    }
    #[doc = "EVSTAT2.PWR_DWN"]
    #[inline]
    pub fn pwr_dwn(self) -> &'a mut W {
        self.variant(EV1_SELW::PWR_DWN)
    }
    #[doc = "EVSTAT2.SCLK_LF"]
    #[inline]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(EV1_SELW::SCLK_LF)
    }
    #[doc = "EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline]
    pub fn aon_batmon_temp_upd(self) -> &'a mut W {
        self.variant(EV1_SELW::AON_BATMON_TEMP_UPD)
    }
    #[doc = "EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline]
    pub fn aon_batmon_bat_upd(self) -> &'a mut W {
        self.variant(EV1_SELW::AON_BATMON_BAT_UPD)
    }
    #[doc = "EVSTAT2.AON_RTC_4KHZ"]
    #[inline]
    pub fn aon_rtc_4khz(self) -> &'a mut W {
        self.variant(EV1_SELW::AON_RTC_4KHZ)
    }
    #[doc = "EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(EV1_SELW::AON_RTC_CH2_DLY)
    }
    #[doc = "EVSTAT2.AON_RTC_CH2"]
    #[inline]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(EV1_SELW::AON_RTC_CH2)
    }
    #[doc = "Programmable delay event as described in PROGDLY"]
    #[inline]
    pub fn aux_prog_dly_idle(self) -> &'a mut W {
        self.variant(EV1_SELW::AUX_PROG_DLY_IDLE)
    }
    #[doc = "EVSTAT1.AUXIO31"]
    #[inline]
    pub fn auxio31(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO31)
    }
    #[doc = "EVSTAT1.AUXIO30"]
    #[inline]
    pub fn auxio30(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO30)
    }
    #[doc = "EVSTAT1.AUXIO29"]
    #[inline]
    pub fn auxio29(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO29)
    }
    #[doc = "EVSTAT1.AUXIO28"]
    #[inline]
    pub fn auxio28(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO28)
    }
    #[doc = "EVSTAT1.AUXIO27"]
    #[inline]
    pub fn auxio27(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO27)
    }
    #[doc = "EVSTAT1.AUXIO26"]
    #[inline]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO26)
    }
    #[doc = "EVSTAT1.AUXIO25"]
    #[inline]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO25)
    }
    #[doc = "EVSTAT1.AUXIO24"]
    #[inline]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO24)
    }
    #[doc = "EVSTAT1.AUXIO23"]
    #[inline]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO23)
    }
    #[doc = "EVSTAT1.AUXIO22"]
    #[inline]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO22)
    }
    #[doc = "EVSTAT1.AUXIO21"]
    #[inline]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO21)
    }
    #[doc = "EVSTAT1.AUXIO20"]
    #[inline]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO20)
    }
    #[doc = "EVSTAT1.AUXIO19"]
    #[inline]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO19)
    }
    #[doc = "EVSTAT1.AUXIO18"]
    #[inline]
    pub fn auxio18(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO18)
    }
    #[doc = "EVSTAT1.AUXIO17"]
    #[inline]
    pub fn auxio17(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO17)
    }
    #[doc = "EVSTAT1.AUXIO16"]
    #[inline]
    pub fn auxio16(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO16)
    }
    #[doc = "EVSTAT0.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO15)
    }
    #[doc = "EVSTAT0.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO14)
    }
    #[doc = "EVSTAT0.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO13)
    }
    #[doc = "EVSTAT0.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO12)
    }
    #[doc = "EVSTAT0.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO11)
    }
    #[doc = "EVSTAT0.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO10)
    }
    #[doc = "EVSTAT0.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO9)
    }
    #[doc = "EVSTAT0.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO8)
    }
    #[doc = "EVSTAT0.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO7)
    }
    #[doc = "EVSTAT0.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO6)
    }
    #[doc = "EVSTAT0.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO5)
    }
    #[doc = "EVSTAT0.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO4)
    }
    #[doc = "EVSTAT0.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(EV1_SELW::AUXIO0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bit 7 - 7:7\\] Polarity of SCEWEVCFG0.EV0_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
    #[inline]
    pub fn ev0_pol(&self) -> EV0_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EV0_POLR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Polarity of EV1_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
    #[inline]
    pub fn ev1_pol(&self) -> EV1_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EV1_POLR { bits }
    }
    #[doc = "Bits 0:5 - 5:0\\] Select the event source from the synchronous event bus to be used in event equation."]
    #[inline]
    pub fn ev1_sel(&self) -> EV1_SELR {
        EV1_SELR::_from({
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
        W { bits: 0 }
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
    #[doc = "Bit 7 - 7:7\\] Polarity of SCEWEVCFG0.EV0_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
    #[inline]
    pub fn ev0_pol(&mut self) -> _EV0_POLW {
        _EV0_POLW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Polarity of EV1_SEL event. When SCEWEVCFG0.COMB_EV_EN is 0: 0: Non-inverted. 1: Non-inverted. When SCEWEVCFG0.COMB_EV_EN is 1. 0: Non-inverted. 1: Inverted."]
    #[inline]
    pub fn ev1_pol(&mut self) -> _EV1_POLW {
        _EV1_POLW { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] Select the event source from the synchronous event bus to be used in event equation."]
    #[inline]
    pub fn ev1_sel(&mut self) -> _EV1_SELW {
        _EV1_SELW { w: self }
    }
}
