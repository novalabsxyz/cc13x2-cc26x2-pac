#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCUWUSEL {
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
pub struct RESERVED30R {
    bits: u8,
}
impl RESERVED30R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WU3_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU3_EVR {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Combined event from BATMON"]
    BATMON_COMBINED,
    #[doc = "BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL,
    #[doc = "BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL,
    #[doc = "BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL,
    #[doc = "BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL,
    #[doc = "Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3,
    #[doc = "Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2,
    #[doc = "Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1,
    #[doc = "Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WU3_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU3_EVR::NONE => 63,
            WU3_EVR::AUX_COMPB_ASYNC_N => 56,
            WU3_EVR::AUX_COMPB_ASYNC => 55,
            WU3_EVR::BATMON_VOLT => 54,
            WU3_EVR::BATMON_TEMP => 53,
            WU3_EVR::AUX_TIMER1_EV => 52,
            WU3_EVR::AUX_TIMER0_EV => 51,
            WU3_EVR::AUX_TDC_DONE => 50,
            WU3_EVR::AUX_ADC_DONE => 49,
            WU3_EVR::AUX_COMPB => 48,
            WU3_EVR::AUX_COMPA => 47,
            WU3_EVR::AUX_SWEV2 => 46,
            WU3_EVR::AUX_SWEV1 => 45,
            WU3_EVR::AUX_SWEV0 => 44,
            WU3_EVR::JTAG => 43,
            WU3_EVR::RTC_UPD => 42,
            WU3_EVR::RTC_COMB_DLY => 41,
            WU3_EVR::RTC_CH2_DLY => 40,
            WU3_EVR::RTC_CH1_DLY => 39,
            WU3_EVR::RTC_CH0_DLY => 38,
            WU3_EVR::RTC_CH2 => 37,
            WU3_EVR::RTC_CH1 => 36,
            WU3_EVR::RTC_CH0 => 35,
            WU3_EVR::PAD => 32,
            WU3_EVR::BATMON_COMBINED => 9,
            WU3_EVR::BATMON_TEMP_LL => 8,
            WU3_EVR::BATMON_TEMP_UL => 7,
            WU3_EVR::BATMON_BATT_LL => 6,
            WU3_EVR::BATMON_BATT_UL => 5,
            WU3_EVR::AUX_TIMER2_EV3 => 4,
            WU3_EVR::AUX_TIMER2_EV2 => 3,
            WU3_EVR::AUX_TIMER2_EV1 => 2,
            WU3_EVR::AUX_TIMER2_EV0 => 1,
            WU3_EVR::IOEV_MCU_WU => 0,
            WU3_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU3_EVR {
        match value {
            63 => WU3_EVR::NONE,
            56 => WU3_EVR::AUX_COMPB_ASYNC_N,
            55 => WU3_EVR::AUX_COMPB_ASYNC,
            54 => WU3_EVR::BATMON_VOLT,
            53 => WU3_EVR::BATMON_TEMP,
            52 => WU3_EVR::AUX_TIMER1_EV,
            51 => WU3_EVR::AUX_TIMER0_EV,
            50 => WU3_EVR::AUX_TDC_DONE,
            49 => WU3_EVR::AUX_ADC_DONE,
            48 => WU3_EVR::AUX_COMPB,
            47 => WU3_EVR::AUX_COMPA,
            46 => WU3_EVR::AUX_SWEV2,
            45 => WU3_EVR::AUX_SWEV1,
            44 => WU3_EVR::AUX_SWEV0,
            43 => WU3_EVR::JTAG,
            42 => WU3_EVR::RTC_UPD,
            41 => WU3_EVR::RTC_COMB_DLY,
            40 => WU3_EVR::RTC_CH2_DLY,
            39 => WU3_EVR::RTC_CH1_DLY,
            38 => WU3_EVR::RTC_CH0_DLY,
            37 => WU3_EVR::RTC_CH2,
            36 => WU3_EVR::RTC_CH1,
            35 => WU3_EVR::RTC_CH0,
            32 => WU3_EVR::PAD,
            9 => WU3_EVR::BATMON_COMBINED,
            8 => WU3_EVR::BATMON_TEMP_LL,
            7 => WU3_EVR::BATMON_TEMP_UL,
            6 => WU3_EVR::BATMON_BATT_LL,
            5 => WU3_EVR::BATMON_BATT_UL,
            4 => WU3_EVR::AUX_TIMER2_EV3,
            3 => WU3_EVR::AUX_TIMER2_EV2,
            2 => WU3_EVR::AUX_TIMER2_EV1,
            1 => WU3_EVR::AUX_TIMER2_EV0,
            0 => WU3_EVR::IOEV_MCU_WU,
            i => WU3_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU3_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU3_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU3_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU3_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU3_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU3_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU3_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU3_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU3_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU3_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU3_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU3_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU3_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU3_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU3_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU3_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU3_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU3_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU3_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU3_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU3_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU3_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU3_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU3_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU3_EVR::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU3_EVR::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU3_EVR::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU3_EVR::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU3_EVR::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU3_EVR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU3_EVR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU3_EVR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU3_EVR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU3_EVR::IOEV_MCU_WU
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED22R {
    bits: u8,
}
impl RESERVED22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WU2_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU2_EVR {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Combined event from BATMON"]
    BATMON_COMBINED,
    #[doc = "BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL,
    #[doc = "BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL,
    #[doc = "BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL,
    #[doc = "BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL,
    #[doc = "Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3,
    #[doc = "Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2,
    #[doc = "Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1,
    #[doc = "Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WU2_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU2_EVR::NONE => 63,
            WU2_EVR::AUX_COMPB_ASYNC_N => 56,
            WU2_EVR::AUX_COMPB_ASYNC => 55,
            WU2_EVR::BATMON_VOLT => 54,
            WU2_EVR::BATMON_TEMP => 53,
            WU2_EVR::AUX_TIMER1_EV => 52,
            WU2_EVR::AUX_TIMER0_EV => 51,
            WU2_EVR::AUX_TDC_DONE => 50,
            WU2_EVR::AUX_ADC_DONE => 49,
            WU2_EVR::AUX_COMPB => 48,
            WU2_EVR::AUX_COMPA => 47,
            WU2_EVR::AUX_SWEV2 => 46,
            WU2_EVR::AUX_SWEV1 => 45,
            WU2_EVR::AUX_SWEV0 => 44,
            WU2_EVR::JTAG => 43,
            WU2_EVR::RTC_UPD => 42,
            WU2_EVR::RTC_COMB_DLY => 41,
            WU2_EVR::RTC_CH2_DLY => 40,
            WU2_EVR::RTC_CH1_DLY => 39,
            WU2_EVR::RTC_CH0_DLY => 38,
            WU2_EVR::RTC_CH2 => 37,
            WU2_EVR::RTC_CH1 => 36,
            WU2_EVR::RTC_CH0 => 35,
            WU2_EVR::PAD => 32,
            WU2_EVR::BATMON_COMBINED => 9,
            WU2_EVR::BATMON_TEMP_LL => 8,
            WU2_EVR::BATMON_TEMP_UL => 7,
            WU2_EVR::BATMON_BATT_LL => 6,
            WU2_EVR::BATMON_BATT_UL => 5,
            WU2_EVR::AUX_TIMER2_EV3 => 4,
            WU2_EVR::AUX_TIMER2_EV2 => 3,
            WU2_EVR::AUX_TIMER2_EV1 => 2,
            WU2_EVR::AUX_TIMER2_EV0 => 1,
            WU2_EVR::IOEV_MCU_WU => 0,
            WU2_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU2_EVR {
        match value {
            63 => WU2_EVR::NONE,
            56 => WU2_EVR::AUX_COMPB_ASYNC_N,
            55 => WU2_EVR::AUX_COMPB_ASYNC,
            54 => WU2_EVR::BATMON_VOLT,
            53 => WU2_EVR::BATMON_TEMP,
            52 => WU2_EVR::AUX_TIMER1_EV,
            51 => WU2_EVR::AUX_TIMER0_EV,
            50 => WU2_EVR::AUX_TDC_DONE,
            49 => WU2_EVR::AUX_ADC_DONE,
            48 => WU2_EVR::AUX_COMPB,
            47 => WU2_EVR::AUX_COMPA,
            46 => WU2_EVR::AUX_SWEV2,
            45 => WU2_EVR::AUX_SWEV1,
            44 => WU2_EVR::AUX_SWEV0,
            43 => WU2_EVR::JTAG,
            42 => WU2_EVR::RTC_UPD,
            41 => WU2_EVR::RTC_COMB_DLY,
            40 => WU2_EVR::RTC_CH2_DLY,
            39 => WU2_EVR::RTC_CH1_DLY,
            38 => WU2_EVR::RTC_CH0_DLY,
            37 => WU2_EVR::RTC_CH2,
            36 => WU2_EVR::RTC_CH1,
            35 => WU2_EVR::RTC_CH0,
            32 => WU2_EVR::PAD,
            9 => WU2_EVR::BATMON_COMBINED,
            8 => WU2_EVR::BATMON_TEMP_LL,
            7 => WU2_EVR::BATMON_TEMP_UL,
            6 => WU2_EVR::BATMON_BATT_LL,
            5 => WU2_EVR::BATMON_BATT_UL,
            4 => WU2_EVR::AUX_TIMER2_EV3,
            3 => WU2_EVR::AUX_TIMER2_EV2,
            2 => WU2_EVR::AUX_TIMER2_EV1,
            1 => WU2_EVR::AUX_TIMER2_EV0,
            0 => WU2_EVR::IOEV_MCU_WU,
            i => WU2_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU2_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU2_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU2_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU2_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU2_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU2_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU2_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU2_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU2_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU2_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU2_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU2_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU2_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU2_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU2_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU2_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU2_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU2_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU2_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU2_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU2_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU2_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU2_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU2_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU2_EVR::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU2_EVR::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU2_EVR::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU2_EVR::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU2_EVR::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU2_EVR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU2_EVR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU2_EVR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU2_EVR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU2_EVR::IOEV_MCU_WU
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED14R {
    bits: u8,
}
impl RESERVED14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WU1_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU1_EVR {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Combined event from BATMON"]
    BATMON_COMBINED,
    #[doc = "BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL,
    #[doc = "BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL,
    #[doc = "BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL,
    #[doc = "BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL,
    #[doc = "Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3,
    #[doc = "Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2,
    #[doc = "Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1,
    #[doc = "Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WU1_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU1_EVR::NONE => 63,
            WU1_EVR::AUX_COMPB_ASYNC_N => 56,
            WU1_EVR::AUX_COMPB_ASYNC => 55,
            WU1_EVR::BATMON_VOLT => 54,
            WU1_EVR::BATMON_TEMP => 53,
            WU1_EVR::AUX_TIMER1_EV => 52,
            WU1_EVR::AUX_TIMER0_EV => 51,
            WU1_EVR::AUX_TDC_DONE => 50,
            WU1_EVR::AUX_ADC_DONE => 49,
            WU1_EVR::AUX_COMPB => 48,
            WU1_EVR::AUX_COMPA => 47,
            WU1_EVR::AUX_SWEV2 => 46,
            WU1_EVR::AUX_SWEV1 => 45,
            WU1_EVR::AUX_SWEV0 => 44,
            WU1_EVR::JTAG => 43,
            WU1_EVR::RTC_UPD => 42,
            WU1_EVR::RTC_COMB_DLY => 41,
            WU1_EVR::RTC_CH2_DLY => 40,
            WU1_EVR::RTC_CH1_DLY => 39,
            WU1_EVR::RTC_CH0_DLY => 38,
            WU1_EVR::RTC_CH2 => 37,
            WU1_EVR::RTC_CH1 => 36,
            WU1_EVR::RTC_CH0 => 35,
            WU1_EVR::PAD => 32,
            WU1_EVR::BATMON_COMBINED => 9,
            WU1_EVR::BATMON_TEMP_LL => 8,
            WU1_EVR::BATMON_TEMP_UL => 7,
            WU1_EVR::BATMON_BATT_LL => 6,
            WU1_EVR::BATMON_BATT_UL => 5,
            WU1_EVR::AUX_TIMER2_EV3 => 4,
            WU1_EVR::AUX_TIMER2_EV2 => 3,
            WU1_EVR::AUX_TIMER2_EV1 => 2,
            WU1_EVR::AUX_TIMER2_EV0 => 1,
            WU1_EVR::IOEV_MCU_WU => 0,
            WU1_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU1_EVR {
        match value {
            63 => WU1_EVR::NONE,
            56 => WU1_EVR::AUX_COMPB_ASYNC_N,
            55 => WU1_EVR::AUX_COMPB_ASYNC,
            54 => WU1_EVR::BATMON_VOLT,
            53 => WU1_EVR::BATMON_TEMP,
            52 => WU1_EVR::AUX_TIMER1_EV,
            51 => WU1_EVR::AUX_TIMER0_EV,
            50 => WU1_EVR::AUX_TDC_DONE,
            49 => WU1_EVR::AUX_ADC_DONE,
            48 => WU1_EVR::AUX_COMPB,
            47 => WU1_EVR::AUX_COMPA,
            46 => WU1_EVR::AUX_SWEV2,
            45 => WU1_EVR::AUX_SWEV1,
            44 => WU1_EVR::AUX_SWEV0,
            43 => WU1_EVR::JTAG,
            42 => WU1_EVR::RTC_UPD,
            41 => WU1_EVR::RTC_COMB_DLY,
            40 => WU1_EVR::RTC_CH2_DLY,
            39 => WU1_EVR::RTC_CH1_DLY,
            38 => WU1_EVR::RTC_CH0_DLY,
            37 => WU1_EVR::RTC_CH2,
            36 => WU1_EVR::RTC_CH1,
            35 => WU1_EVR::RTC_CH0,
            32 => WU1_EVR::PAD,
            9 => WU1_EVR::BATMON_COMBINED,
            8 => WU1_EVR::BATMON_TEMP_LL,
            7 => WU1_EVR::BATMON_TEMP_UL,
            6 => WU1_EVR::BATMON_BATT_LL,
            5 => WU1_EVR::BATMON_BATT_UL,
            4 => WU1_EVR::AUX_TIMER2_EV3,
            3 => WU1_EVR::AUX_TIMER2_EV2,
            2 => WU1_EVR::AUX_TIMER2_EV1,
            1 => WU1_EVR::AUX_TIMER2_EV0,
            0 => WU1_EVR::IOEV_MCU_WU,
            i => WU1_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU1_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU1_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU1_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU1_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU1_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU1_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU1_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU1_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU1_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU1_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU1_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU1_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU1_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU1_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU1_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU1_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU1_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU1_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU1_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU1_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU1_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU1_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU1_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU1_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU1_EVR::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU1_EVR::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU1_EVR::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU1_EVR::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU1_EVR::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU1_EVR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU1_EVR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU1_EVR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU1_EVR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU1_EVR::IOEV_MCU_WU
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED6R {
    bits: u8,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WU0_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU0_EVR {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Combined event from BATMON"]
    BATMON_COMBINED,
    #[doc = "BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL,
    #[doc = "BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL,
    #[doc = "BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL,
    #[doc = "BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL,
    #[doc = "Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3,
    #[doc = "Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2,
    #[doc = "Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1,
    #[doc = "Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WU0_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU0_EVR::NONE => 63,
            WU0_EVR::AUX_COMPB_ASYNC_N => 56,
            WU0_EVR::AUX_COMPB_ASYNC => 55,
            WU0_EVR::BATMON_VOLT => 54,
            WU0_EVR::BATMON_TEMP => 53,
            WU0_EVR::AUX_TIMER1_EV => 52,
            WU0_EVR::AUX_TIMER0_EV => 51,
            WU0_EVR::AUX_TDC_DONE => 50,
            WU0_EVR::AUX_ADC_DONE => 49,
            WU0_EVR::AUX_COMPB => 48,
            WU0_EVR::AUX_COMPA => 47,
            WU0_EVR::AUX_SWEV2 => 46,
            WU0_EVR::AUX_SWEV1 => 45,
            WU0_EVR::AUX_SWEV0 => 44,
            WU0_EVR::JTAG => 43,
            WU0_EVR::RTC_UPD => 42,
            WU0_EVR::RTC_COMB_DLY => 41,
            WU0_EVR::RTC_CH2_DLY => 40,
            WU0_EVR::RTC_CH1_DLY => 39,
            WU0_EVR::RTC_CH0_DLY => 38,
            WU0_EVR::RTC_CH2 => 37,
            WU0_EVR::RTC_CH1 => 36,
            WU0_EVR::RTC_CH0 => 35,
            WU0_EVR::PAD => 32,
            WU0_EVR::BATMON_COMBINED => 9,
            WU0_EVR::BATMON_TEMP_LL => 8,
            WU0_EVR::BATMON_TEMP_UL => 7,
            WU0_EVR::BATMON_BATT_LL => 6,
            WU0_EVR::BATMON_BATT_UL => 5,
            WU0_EVR::AUX_TIMER2_EV3 => 4,
            WU0_EVR::AUX_TIMER2_EV2 => 3,
            WU0_EVR::AUX_TIMER2_EV1 => 2,
            WU0_EVR::AUX_TIMER2_EV0 => 1,
            WU0_EVR::IOEV_MCU_WU => 0,
            WU0_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU0_EVR {
        match value {
            63 => WU0_EVR::NONE,
            56 => WU0_EVR::AUX_COMPB_ASYNC_N,
            55 => WU0_EVR::AUX_COMPB_ASYNC,
            54 => WU0_EVR::BATMON_VOLT,
            53 => WU0_EVR::BATMON_TEMP,
            52 => WU0_EVR::AUX_TIMER1_EV,
            51 => WU0_EVR::AUX_TIMER0_EV,
            50 => WU0_EVR::AUX_TDC_DONE,
            49 => WU0_EVR::AUX_ADC_DONE,
            48 => WU0_EVR::AUX_COMPB,
            47 => WU0_EVR::AUX_COMPA,
            46 => WU0_EVR::AUX_SWEV2,
            45 => WU0_EVR::AUX_SWEV1,
            44 => WU0_EVR::AUX_SWEV0,
            43 => WU0_EVR::JTAG,
            42 => WU0_EVR::RTC_UPD,
            41 => WU0_EVR::RTC_COMB_DLY,
            40 => WU0_EVR::RTC_CH2_DLY,
            39 => WU0_EVR::RTC_CH1_DLY,
            38 => WU0_EVR::RTC_CH0_DLY,
            37 => WU0_EVR::RTC_CH2,
            36 => WU0_EVR::RTC_CH1,
            35 => WU0_EVR::RTC_CH0,
            32 => WU0_EVR::PAD,
            9 => WU0_EVR::BATMON_COMBINED,
            8 => WU0_EVR::BATMON_TEMP_LL,
            7 => WU0_EVR::BATMON_TEMP_UL,
            6 => WU0_EVR::BATMON_BATT_LL,
            5 => WU0_EVR::BATMON_BATT_UL,
            4 => WU0_EVR::AUX_TIMER2_EV3,
            3 => WU0_EVR::AUX_TIMER2_EV2,
            2 => WU0_EVR::AUX_TIMER2_EV1,
            1 => WU0_EVR::AUX_TIMER2_EV0,
            0 => WU0_EVR::IOEV_MCU_WU,
            i => WU0_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU0_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU0_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU0_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU0_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU0_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU0_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU0_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU0_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU0_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU0_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU0_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU0_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU0_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU0_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU0_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU0_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU0_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU0_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU0_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU0_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU0_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU0_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU0_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU0_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU0_EVR::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU0_EVR::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU0_EVR::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU0_EVR::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU0_EVR::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU0_EVR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU0_EVR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU0_EVR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU0_EVR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU0_EVR::IOEV_MCU_WU
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED30W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED30W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WU3_EV`"]
pub enum WU3_EVW {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Combined event from BATMON"]
    BATMON_COMBINED,
    #[doc = "BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL,
    #[doc = "BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL,
    #[doc = "BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL,
    #[doc = "BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL,
    #[doc = "Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3,
    #[doc = "Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2,
    #[doc = "Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1,
    #[doc = "Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU,
}
impl WU3_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU3_EVW::NONE => 63,
            WU3_EVW::AUX_COMPB_ASYNC_N => 56,
            WU3_EVW::AUX_COMPB_ASYNC => 55,
            WU3_EVW::BATMON_VOLT => 54,
            WU3_EVW::BATMON_TEMP => 53,
            WU3_EVW::AUX_TIMER1_EV => 52,
            WU3_EVW::AUX_TIMER0_EV => 51,
            WU3_EVW::AUX_TDC_DONE => 50,
            WU3_EVW::AUX_ADC_DONE => 49,
            WU3_EVW::AUX_COMPB => 48,
            WU3_EVW::AUX_COMPA => 47,
            WU3_EVW::AUX_SWEV2 => 46,
            WU3_EVW::AUX_SWEV1 => 45,
            WU3_EVW::AUX_SWEV0 => 44,
            WU3_EVW::JTAG => 43,
            WU3_EVW::RTC_UPD => 42,
            WU3_EVW::RTC_COMB_DLY => 41,
            WU3_EVW::RTC_CH2_DLY => 40,
            WU3_EVW::RTC_CH1_DLY => 39,
            WU3_EVW::RTC_CH0_DLY => 38,
            WU3_EVW::RTC_CH2 => 37,
            WU3_EVW::RTC_CH1 => 36,
            WU3_EVW::RTC_CH0 => 35,
            WU3_EVW::PAD => 32,
            WU3_EVW::BATMON_COMBINED => 9,
            WU3_EVW::BATMON_TEMP_LL => 8,
            WU3_EVW::BATMON_TEMP_UL => 7,
            WU3_EVW::BATMON_BATT_LL => 6,
            WU3_EVW::BATMON_BATT_UL => 5,
            WU3_EVW::AUX_TIMER2_EV3 => 4,
            WU3_EVW::AUX_TIMER2_EV2 => 3,
            WU3_EVW::AUX_TIMER2_EV1 => 2,
            WU3_EVW::AUX_TIMER2_EV0 => 1,
            WU3_EVW::IOEV_MCU_WU => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU3_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU3_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU3_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU3_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU3_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU3_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU3_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU3_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU3_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU3_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU3_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU3_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU3_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU3_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU3_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU3_EVW::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU3_EVW::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU3_EVW::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU3_EVW::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU3_EVW::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU3_EVW::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU3_EVW::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU3_EVW::IOEV_MCU_WU)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED22W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED22W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WU2_EV`"]
pub enum WU2_EVW {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Combined event from BATMON"]
    BATMON_COMBINED,
    #[doc = "BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL,
    #[doc = "BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL,
    #[doc = "BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL,
    #[doc = "BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL,
    #[doc = "Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3,
    #[doc = "Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2,
    #[doc = "Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1,
    #[doc = "Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU,
}
impl WU2_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU2_EVW::NONE => 63,
            WU2_EVW::AUX_COMPB_ASYNC_N => 56,
            WU2_EVW::AUX_COMPB_ASYNC => 55,
            WU2_EVW::BATMON_VOLT => 54,
            WU2_EVW::BATMON_TEMP => 53,
            WU2_EVW::AUX_TIMER1_EV => 52,
            WU2_EVW::AUX_TIMER0_EV => 51,
            WU2_EVW::AUX_TDC_DONE => 50,
            WU2_EVW::AUX_ADC_DONE => 49,
            WU2_EVW::AUX_COMPB => 48,
            WU2_EVW::AUX_COMPA => 47,
            WU2_EVW::AUX_SWEV2 => 46,
            WU2_EVW::AUX_SWEV1 => 45,
            WU2_EVW::AUX_SWEV0 => 44,
            WU2_EVW::JTAG => 43,
            WU2_EVW::RTC_UPD => 42,
            WU2_EVW::RTC_COMB_DLY => 41,
            WU2_EVW::RTC_CH2_DLY => 40,
            WU2_EVW::RTC_CH1_DLY => 39,
            WU2_EVW::RTC_CH0_DLY => 38,
            WU2_EVW::RTC_CH2 => 37,
            WU2_EVW::RTC_CH1 => 36,
            WU2_EVW::RTC_CH0 => 35,
            WU2_EVW::PAD => 32,
            WU2_EVW::BATMON_COMBINED => 9,
            WU2_EVW::BATMON_TEMP_LL => 8,
            WU2_EVW::BATMON_TEMP_UL => 7,
            WU2_EVW::BATMON_BATT_LL => 6,
            WU2_EVW::BATMON_BATT_UL => 5,
            WU2_EVW::AUX_TIMER2_EV3 => 4,
            WU2_EVW::AUX_TIMER2_EV2 => 3,
            WU2_EVW::AUX_TIMER2_EV1 => 2,
            WU2_EVW::AUX_TIMER2_EV0 => 1,
            WU2_EVW::IOEV_MCU_WU => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU2_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU2_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU2_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU2_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU2_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU2_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU2_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU2_EVW::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU2_EVW::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU2_EVW::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU2_EVW::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU2_EVW::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU2_EVW::IOEV_MCU_WU)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED14W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WU1_EV`"]
pub enum WU1_EVW {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Combined event from BATMON"]
    BATMON_COMBINED,
    #[doc = "BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL,
    #[doc = "BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL,
    #[doc = "BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL,
    #[doc = "BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL,
    #[doc = "Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3,
    #[doc = "Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2,
    #[doc = "Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1,
    #[doc = "Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU,
}
impl WU1_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU1_EVW::NONE => 63,
            WU1_EVW::AUX_COMPB_ASYNC_N => 56,
            WU1_EVW::AUX_COMPB_ASYNC => 55,
            WU1_EVW::BATMON_VOLT => 54,
            WU1_EVW::BATMON_TEMP => 53,
            WU1_EVW::AUX_TIMER1_EV => 52,
            WU1_EVW::AUX_TIMER0_EV => 51,
            WU1_EVW::AUX_TDC_DONE => 50,
            WU1_EVW::AUX_ADC_DONE => 49,
            WU1_EVW::AUX_COMPB => 48,
            WU1_EVW::AUX_COMPA => 47,
            WU1_EVW::AUX_SWEV2 => 46,
            WU1_EVW::AUX_SWEV1 => 45,
            WU1_EVW::AUX_SWEV0 => 44,
            WU1_EVW::JTAG => 43,
            WU1_EVW::RTC_UPD => 42,
            WU1_EVW::RTC_COMB_DLY => 41,
            WU1_EVW::RTC_CH2_DLY => 40,
            WU1_EVW::RTC_CH1_DLY => 39,
            WU1_EVW::RTC_CH0_DLY => 38,
            WU1_EVW::RTC_CH2 => 37,
            WU1_EVW::RTC_CH1 => 36,
            WU1_EVW::RTC_CH0 => 35,
            WU1_EVW::PAD => 32,
            WU1_EVW::BATMON_COMBINED => 9,
            WU1_EVW::BATMON_TEMP_LL => 8,
            WU1_EVW::BATMON_TEMP_UL => 7,
            WU1_EVW::BATMON_BATT_LL => 6,
            WU1_EVW::BATMON_BATT_UL => 5,
            WU1_EVW::AUX_TIMER2_EV3 => 4,
            WU1_EVW::AUX_TIMER2_EV2 => 3,
            WU1_EVW::AUX_TIMER2_EV1 => 2,
            WU1_EVW::AUX_TIMER2_EV0 => 1,
            WU1_EVW::IOEV_MCU_WU => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU1_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU1_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU1_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU1_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU1_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU1_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU1_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU1_EVW::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU1_EVW::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU1_EVW::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU1_EVW::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU1_EVW::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU1_EVW::IOEV_MCU_WU)
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
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WU0_EV`"]
pub enum WU0_EVW {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Combined event from BATMON"]
    BATMON_COMBINED,
    #[doc = "BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL,
    #[doc = "BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL,
    #[doc = "BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL,
    #[doc = "BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL,
    #[doc = "Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3,
    #[doc = "Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2,
    #[doc = "Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1,
    #[doc = "Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0,
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU,
}
impl WU0_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU0_EVW::NONE => 63,
            WU0_EVW::AUX_COMPB_ASYNC_N => 56,
            WU0_EVW::AUX_COMPB_ASYNC => 55,
            WU0_EVW::BATMON_VOLT => 54,
            WU0_EVW::BATMON_TEMP => 53,
            WU0_EVW::AUX_TIMER1_EV => 52,
            WU0_EVW::AUX_TIMER0_EV => 51,
            WU0_EVW::AUX_TDC_DONE => 50,
            WU0_EVW::AUX_ADC_DONE => 49,
            WU0_EVW::AUX_COMPB => 48,
            WU0_EVW::AUX_COMPA => 47,
            WU0_EVW::AUX_SWEV2 => 46,
            WU0_EVW::AUX_SWEV1 => 45,
            WU0_EVW::AUX_SWEV0 => 44,
            WU0_EVW::JTAG => 43,
            WU0_EVW::RTC_UPD => 42,
            WU0_EVW::RTC_COMB_DLY => 41,
            WU0_EVW::RTC_CH2_DLY => 40,
            WU0_EVW::RTC_CH1_DLY => 39,
            WU0_EVW::RTC_CH0_DLY => 38,
            WU0_EVW::RTC_CH2 => 37,
            WU0_EVW::RTC_CH1 => 36,
            WU0_EVW::RTC_CH0 => 35,
            WU0_EVW::PAD => 32,
            WU0_EVW::BATMON_COMBINED => 9,
            WU0_EVW::BATMON_TEMP_LL => 8,
            WU0_EVW::BATMON_TEMP_UL => 7,
            WU0_EVW::BATMON_BATT_LL => 6,
            WU0_EVW::BATMON_BATT_UL => 5,
            WU0_EVW::AUX_TIMER2_EV3 => 4,
            WU0_EVW::AUX_TIMER2_EV2 => 3,
            WU0_EVW::AUX_TIMER2_EV1 => 2,
            WU0_EVW::AUX_TIMER2_EV0 => 1,
            WU0_EVW::IOEV_MCU_WU => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU0_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU0_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU0_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU0_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU0_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU0_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU0_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU0_EVW::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU0_EVW::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU0_EVW::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU0_EVW::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU0_EVW::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU0_EVW::IOEV_MCU_WU)
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
    #[doc = "Bits 30:31 - 31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved30(&self) -> RESERVED30R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED30R { bits }
    }
    #[doc = "Bits 24:29 - 29:24\\] MCU Wakeup Source #3 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu3_ev(&self) -> WU3_EVR {
        WU3_EVR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - 23:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved22(&self) -> RESERVED22R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED22R { bits }
    }
    #[doc = "Bits 16:21 - 21:16\\] MCU Wakeup Source #2 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu2_ev(&self) -> WU2_EVR {
        WU2_EVR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - 15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&self) -> RESERVED14R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED14R { bits }
    }
    #[doc = "Bits 8:13 - 13:8\\] MCU Wakeup Source #1 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu1_ev(&self) -> WU1_EVR {
        WU1_EVR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED6R { bits }
    }
    #[doc = "Bits 0:5 - 5:0\\] MCU Wakeup Source #0 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu0_ev(&self) -> WU0_EVR {
        WU0_EVR::_from({
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
        W { bits: 1061109567 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - 31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved30(&mut self) -> _RESERVED30W {
        _RESERVED30W { w: self }
    }
    #[doc = "Bits 24:29 - 29:24\\] MCU Wakeup Source #3 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu3_ev(&mut self) -> _WU3_EVW {
        _WU3_EVW { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved22(&mut self) -> _RESERVED22W {
        _RESERVED22W { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\] MCU Wakeup Source #2 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu2_ev(&mut self) -> _WU2_EVW {
        _WU2_EVW { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&mut self) -> _RESERVED14W {
        _RESERVED14W { w: self }
    }
    #[doc = "Bits 8:13 - 13:8\\] MCU Wakeup Source #1 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu1_ev(&mut self) -> _WU1_EVW {
        _WU1_EVW { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] MCU Wakeup Source #0 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu0_ev(&mut self) -> _WU0_EVW {
        _WU0_EVW { w: self }
    }
}
