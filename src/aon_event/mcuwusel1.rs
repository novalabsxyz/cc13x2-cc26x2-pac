#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCUWUSEL1 {
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
#[doc = "Possible values of the field `WU7_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU7_EVR {
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
impl WU7_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU7_EVR::NONE => 63,
            WU7_EVR::AUX_COMPB_ASYNC_N => 56,
            WU7_EVR::AUX_COMPB_ASYNC => 55,
            WU7_EVR::BATMON_VOLT => 54,
            WU7_EVR::BATMON_TEMP => 53,
            WU7_EVR::AUX_TIMER1_EV => 52,
            WU7_EVR::AUX_TIMER0_EV => 51,
            WU7_EVR::AUX_TDC_DONE => 50,
            WU7_EVR::AUX_ADC_DONE => 49,
            WU7_EVR::AUX_COMPB => 48,
            WU7_EVR::AUX_COMPA => 47,
            WU7_EVR::AUX_SWEV2 => 46,
            WU7_EVR::AUX_SWEV1 => 45,
            WU7_EVR::AUX_SWEV0 => 44,
            WU7_EVR::JTAG => 43,
            WU7_EVR::RTC_UPD => 42,
            WU7_EVR::RTC_COMB_DLY => 41,
            WU7_EVR::RTC_CH2_DLY => 40,
            WU7_EVR::RTC_CH1_DLY => 39,
            WU7_EVR::RTC_CH0_DLY => 38,
            WU7_EVR::RTC_CH2 => 37,
            WU7_EVR::RTC_CH1 => 36,
            WU7_EVR::RTC_CH0 => 35,
            WU7_EVR::PAD => 32,
            WU7_EVR::BATMON_COMBINED => 9,
            WU7_EVR::BATMON_TEMP_LL => 8,
            WU7_EVR::BATMON_TEMP_UL => 7,
            WU7_EVR::BATMON_BATT_LL => 6,
            WU7_EVR::BATMON_BATT_UL => 5,
            WU7_EVR::AUX_TIMER2_EV3 => 4,
            WU7_EVR::AUX_TIMER2_EV2 => 3,
            WU7_EVR::AUX_TIMER2_EV1 => 2,
            WU7_EVR::AUX_TIMER2_EV0 => 1,
            WU7_EVR::IOEV_MCU_WU => 0,
            WU7_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU7_EVR {
        match value {
            63 => WU7_EVR::NONE,
            56 => WU7_EVR::AUX_COMPB_ASYNC_N,
            55 => WU7_EVR::AUX_COMPB_ASYNC,
            54 => WU7_EVR::BATMON_VOLT,
            53 => WU7_EVR::BATMON_TEMP,
            52 => WU7_EVR::AUX_TIMER1_EV,
            51 => WU7_EVR::AUX_TIMER0_EV,
            50 => WU7_EVR::AUX_TDC_DONE,
            49 => WU7_EVR::AUX_ADC_DONE,
            48 => WU7_EVR::AUX_COMPB,
            47 => WU7_EVR::AUX_COMPA,
            46 => WU7_EVR::AUX_SWEV2,
            45 => WU7_EVR::AUX_SWEV1,
            44 => WU7_EVR::AUX_SWEV0,
            43 => WU7_EVR::JTAG,
            42 => WU7_EVR::RTC_UPD,
            41 => WU7_EVR::RTC_COMB_DLY,
            40 => WU7_EVR::RTC_CH2_DLY,
            39 => WU7_EVR::RTC_CH1_DLY,
            38 => WU7_EVR::RTC_CH0_DLY,
            37 => WU7_EVR::RTC_CH2,
            36 => WU7_EVR::RTC_CH1,
            35 => WU7_EVR::RTC_CH0,
            32 => WU7_EVR::PAD,
            9 => WU7_EVR::BATMON_COMBINED,
            8 => WU7_EVR::BATMON_TEMP_LL,
            7 => WU7_EVR::BATMON_TEMP_UL,
            6 => WU7_EVR::BATMON_BATT_LL,
            5 => WU7_EVR::BATMON_BATT_UL,
            4 => WU7_EVR::AUX_TIMER2_EV3,
            3 => WU7_EVR::AUX_TIMER2_EV2,
            2 => WU7_EVR::AUX_TIMER2_EV1,
            1 => WU7_EVR::AUX_TIMER2_EV0,
            0 => WU7_EVR::IOEV_MCU_WU,
            i => WU7_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU7_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU7_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU7_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU7_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU7_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU7_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU7_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU7_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU7_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU7_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU7_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU7_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU7_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU7_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU7_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU7_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU7_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU7_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU7_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU7_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU7_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU7_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU7_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU7_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU7_EVR::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU7_EVR::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU7_EVR::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU7_EVR::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU7_EVR::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU7_EVR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU7_EVR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU7_EVR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU7_EVR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU7_EVR::IOEV_MCU_WU
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
#[doc = "Possible values of the field `WU6_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU6_EVR {
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
impl WU6_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU6_EVR::NONE => 63,
            WU6_EVR::AUX_COMPB_ASYNC_N => 56,
            WU6_EVR::AUX_COMPB_ASYNC => 55,
            WU6_EVR::BATMON_VOLT => 54,
            WU6_EVR::BATMON_TEMP => 53,
            WU6_EVR::AUX_TIMER1_EV => 52,
            WU6_EVR::AUX_TIMER0_EV => 51,
            WU6_EVR::AUX_TDC_DONE => 50,
            WU6_EVR::AUX_ADC_DONE => 49,
            WU6_EVR::AUX_COMPB => 48,
            WU6_EVR::AUX_COMPA => 47,
            WU6_EVR::AUX_SWEV2 => 46,
            WU6_EVR::AUX_SWEV1 => 45,
            WU6_EVR::AUX_SWEV0 => 44,
            WU6_EVR::JTAG => 43,
            WU6_EVR::RTC_UPD => 42,
            WU6_EVR::RTC_COMB_DLY => 41,
            WU6_EVR::RTC_CH2_DLY => 40,
            WU6_EVR::RTC_CH1_DLY => 39,
            WU6_EVR::RTC_CH0_DLY => 38,
            WU6_EVR::RTC_CH2 => 37,
            WU6_EVR::RTC_CH1 => 36,
            WU6_EVR::RTC_CH0 => 35,
            WU6_EVR::PAD => 32,
            WU6_EVR::BATMON_COMBINED => 9,
            WU6_EVR::BATMON_TEMP_LL => 8,
            WU6_EVR::BATMON_TEMP_UL => 7,
            WU6_EVR::BATMON_BATT_LL => 6,
            WU6_EVR::BATMON_BATT_UL => 5,
            WU6_EVR::AUX_TIMER2_EV3 => 4,
            WU6_EVR::AUX_TIMER2_EV2 => 3,
            WU6_EVR::AUX_TIMER2_EV1 => 2,
            WU6_EVR::AUX_TIMER2_EV0 => 1,
            WU6_EVR::IOEV_MCU_WU => 0,
            WU6_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU6_EVR {
        match value {
            63 => WU6_EVR::NONE,
            56 => WU6_EVR::AUX_COMPB_ASYNC_N,
            55 => WU6_EVR::AUX_COMPB_ASYNC,
            54 => WU6_EVR::BATMON_VOLT,
            53 => WU6_EVR::BATMON_TEMP,
            52 => WU6_EVR::AUX_TIMER1_EV,
            51 => WU6_EVR::AUX_TIMER0_EV,
            50 => WU6_EVR::AUX_TDC_DONE,
            49 => WU6_EVR::AUX_ADC_DONE,
            48 => WU6_EVR::AUX_COMPB,
            47 => WU6_EVR::AUX_COMPA,
            46 => WU6_EVR::AUX_SWEV2,
            45 => WU6_EVR::AUX_SWEV1,
            44 => WU6_EVR::AUX_SWEV0,
            43 => WU6_EVR::JTAG,
            42 => WU6_EVR::RTC_UPD,
            41 => WU6_EVR::RTC_COMB_DLY,
            40 => WU6_EVR::RTC_CH2_DLY,
            39 => WU6_EVR::RTC_CH1_DLY,
            38 => WU6_EVR::RTC_CH0_DLY,
            37 => WU6_EVR::RTC_CH2,
            36 => WU6_EVR::RTC_CH1,
            35 => WU6_EVR::RTC_CH0,
            32 => WU6_EVR::PAD,
            9 => WU6_EVR::BATMON_COMBINED,
            8 => WU6_EVR::BATMON_TEMP_LL,
            7 => WU6_EVR::BATMON_TEMP_UL,
            6 => WU6_EVR::BATMON_BATT_LL,
            5 => WU6_EVR::BATMON_BATT_UL,
            4 => WU6_EVR::AUX_TIMER2_EV3,
            3 => WU6_EVR::AUX_TIMER2_EV2,
            2 => WU6_EVR::AUX_TIMER2_EV1,
            1 => WU6_EVR::AUX_TIMER2_EV0,
            0 => WU6_EVR::IOEV_MCU_WU,
            i => WU6_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU6_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU6_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU6_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU6_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU6_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU6_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU6_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU6_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU6_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU6_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU6_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU6_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU6_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU6_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU6_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU6_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU6_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU6_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU6_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU6_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU6_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU6_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU6_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU6_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU6_EVR::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU6_EVR::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU6_EVR::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU6_EVR::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU6_EVR::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU6_EVR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU6_EVR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU6_EVR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU6_EVR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU6_EVR::IOEV_MCU_WU
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
#[doc = "Possible values of the field `WU5_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU5_EVR {
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
impl WU5_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU5_EVR::NONE => 63,
            WU5_EVR::AUX_COMPB_ASYNC_N => 56,
            WU5_EVR::AUX_COMPB_ASYNC => 55,
            WU5_EVR::BATMON_VOLT => 54,
            WU5_EVR::BATMON_TEMP => 53,
            WU5_EVR::AUX_TIMER1_EV => 52,
            WU5_EVR::AUX_TIMER0_EV => 51,
            WU5_EVR::AUX_TDC_DONE => 50,
            WU5_EVR::AUX_ADC_DONE => 49,
            WU5_EVR::AUX_COMPB => 48,
            WU5_EVR::AUX_COMPA => 47,
            WU5_EVR::AUX_SWEV2 => 46,
            WU5_EVR::AUX_SWEV1 => 45,
            WU5_EVR::AUX_SWEV0 => 44,
            WU5_EVR::JTAG => 43,
            WU5_EVR::RTC_UPD => 42,
            WU5_EVR::RTC_COMB_DLY => 41,
            WU5_EVR::RTC_CH2_DLY => 40,
            WU5_EVR::RTC_CH1_DLY => 39,
            WU5_EVR::RTC_CH0_DLY => 38,
            WU5_EVR::RTC_CH2 => 37,
            WU5_EVR::RTC_CH1 => 36,
            WU5_EVR::RTC_CH0 => 35,
            WU5_EVR::PAD => 32,
            WU5_EVR::BATMON_COMBINED => 9,
            WU5_EVR::BATMON_TEMP_LL => 8,
            WU5_EVR::BATMON_TEMP_UL => 7,
            WU5_EVR::BATMON_BATT_LL => 6,
            WU5_EVR::BATMON_BATT_UL => 5,
            WU5_EVR::AUX_TIMER2_EV3 => 4,
            WU5_EVR::AUX_TIMER2_EV2 => 3,
            WU5_EVR::AUX_TIMER2_EV1 => 2,
            WU5_EVR::AUX_TIMER2_EV0 => 1,
            WU5_EVR::IOEV_MCU_WU => 0,
            WU5_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU5_EVR {
        match value {
            63 => WU5_EVR::NONE,
            56 => WU5_EVR::AUX_COMPB_ASYNC_N,
            55 => WU5_EVR::AUX_COMPB_ASYNC,
            54 => WU5_EVR::BATMON_VOLT,
            53 => WU5_EVR::BATMON_TEMP,
            52 => WU5_EVR::AUX_TIMER1_EV,
            51 => WU5_EVR::AUX_TIMER0_EV,
            50 => WU5_EVR::AUX_TDC_DONE,
            49 => WU5_EVR::AUX_ADC_DONE,
            48 => WU5_EVR::AUX_COMPB,
            47 => WU5_EVR::AUX_COMPA,
            46 => WU5_EVR::AUX_SWEV2,
            45 => WU5_EVR::AUX_SWEV1,
            44 => WU5_EVR::AUX_SWEV0,
            43 => WU5_EVR::JTAG,
            42 => WU5_EVR::RTC_UPD,
            41 => WU5_EVR::RTC_COMB_DLY,
            40 => WU5_EVR::RTC_CH2_DLY,
            39 => WU5_EVR::RTC_CH1_DLY,
            38 => WU5_EVR::RTC_CH0_DLY,
            37 => WU5_EVR::RTC_CH2,
            36 => WU5_EVR::RTC_CH1,
            35 => WU5_EVR::RTC_CH0,
            32 => WU5_EVR::PAD,
            9 => WU5_EVR::BATMON_COMBINED,
            8 => WU5_EVR::BATMON_TEMP_LL,
            7 => WU5_EVR::BATMON_TEMP_UL,
            6 => WU5_EVR::BATMON_BATT_LL,
            5 => WU5_EVR::BATMON_BATT_UL,
            4 => WU5_EVR::AUX_TIMER2_EV3,
            3 => WU5_EVR::AUX_TIMER2_EV2,
            2 => WU5_EVR::AUX_TIMER2_EV1,
            1 => WU5_EVR::AUX_TIMER2_EV0,
            0 => WU5_EVR::IOEV_MCU_WU,
            i => WU5_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU5_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU5_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU5_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU5_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU5_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU5_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU5_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU5_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU5_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU5_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU5_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU5_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU5_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU5_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU5_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU5_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU5_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU5_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU5_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU5_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU5_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU5_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU5_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU5_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU5_EVR::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU5_EVR::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU5_EVR::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU5_EVR::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU5_EVR::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU5_EVR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU5_EVR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU5_EVR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU5_EVR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU5_EVR::IOEV_MCU_WU
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
#[doc = "Possible values of the field `WU4_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU4_EVR {
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
impl WU4_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU4_EVR::NONE => 63,
            WU4_EVR::AUX_COMPB_ASYNC_N => 56,
            WU4_EVR::AUX_COMPB_ASYNC => 55,
            WU4_EVR::BATMON_VOLT => 54,
            WU4_EVR::BATMON_TEMP => 53,
            WU4_EVR::AUX_TIMER1_EV => 52,
            WU4_EVR::AUX_TIMER0_EV => 51,
            WU4_EVR::AUX_TDC_DONE => 50,
            WU4_EVR::AUX_ADC_DONE => 49,
            WU4_EVR::AUX_COMPB => 48,
            WU4_EVR::AUX_COMPA => 47,
            WU4_EVR::AUX_SWEV2 => 46,
            WU4_EVR::AUX_SWEV1 => 45,
            WU4_EVR::AUX_SWEV0 => 44,
            WU4_EVR::JTAG => 43,
            WU4_EVR::RTC_UPD => 42,
            WU4_EVR::RTC_COMB_DLY => 41,
            WU4_EVR::RTC_CH2_DLY => 40,
            WU4_EVR::RTC_CH1_DLY => 39,
            WU4_EVR::RTC_CH0_DLY => 38,
            WU4_EVR::RTC_CH2 => 37,
            WU4_EVR::RTC_CH1 => 36,
            WU4_EVR::RTC_CH0 => 35,
            WU4_EVR::PAD => 32,
            WU4_EVR::BATMON_COMBINED => 9,
            WU4_EVR::BATMON_TEMP_LL => 8,
            WU4_EVR::BATMON_TEMP_UL => 7,
            WU4_EVR::BATMON_BATT_LL => 6,
            WU4_EVR::BATMON_BATT_UL => 5,
            WU4_EVR::AUX_TIMER2_EV3 => 4,
            WU4_EVR::AUX_TIMER2_EV2 => 3,
            WU4_EVR::AUX_TIMER2_EV1 => 2,
            WU4_EVR::AUX_TIMER2_EV0 => 1,
            WU4_EVR::IOEV_MCU_WU => 0,
            WU4_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU4_EVR {
        match value {
            63 => WU4_EVR::NONE,
            56 => WU4_EVR::AUX_COMPB_ASYNC_N,
            55 => WU4_EVR::AUX_COMPB_ASYNC,
            54 => WU4_EVR::BATMON_VOLT,
            53 => WU4_EVR::BATMON_TEMP,
            52 => WU4_EVR::AUX_TIMER1_EV,
            51 => WU4_EVR::AUX_TIMER0_EV,
            50 => WU4_EVR::AUX_TDC_DONE,
            49 => WU4_EVR::AUX_ADC_DONE,
            48 => WU4_EVR::AUX_COMPB,
            47 => WU4_EVR::AUX_COMPA,
            46 => WU4_EVR::AUX_SWEV2,
            45 => WU4_EVR::AUX_SWEV1,
            44 => WU4_EVR::AUX_SWEV0,
            43 => WU4_EVR::JTAG,
            42 => WU4_EVR::RTC_UPD,
            41 => WU4_EVR::RTC_COMB_DLY,
            40 => WU4_EVR::RTC_CH2_DLY,
            39 => WU4_EVR::RTC_CH1_DLY,
            38 => WU4_EVR::RTC_CH0_DLY,
            37 => WU4_EVR::RTC_CH2,
            36 => WU4_EVR::RTC_CH1,
            35 => WU4_EVR::RTC_CH0,
            32 => WU4_EVR::PAD,
            9 => WU4_EVR::BATMON_COMBINED,
            8 => WU4_EVR::BATMON_TEMP_LL,
            7 => WU4_EVR::BATMON_TEMP_UL,
            6 => WU4_EVR::BATMON_BATT_LL,
            5 => WU4_EVR::BATMON_BATT_UL,
            4 => WU4_EVR::AUX_TIMER2_EV3,
            3 => WU4_EVR::AUX_TIMER2_EV2,
            2 => WU4_EVR::AUX_TIMER2_EV1,
            1 => WU4_EVR::AUX_TIMER2_EV0,
            0 => WU4_EVR::IOEV_MCU_WU,
            i => WU4_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU4_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU4_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU4_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU4_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU4_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU4_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU4_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU4_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU4_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU4_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU4_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU4_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU4_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU4_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU4_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU4_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU4_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU4_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU4_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU4_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU4_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU4_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU4_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU4_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU4_EVR::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU4_EVR::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU4_EVR::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU4_EVR::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU4_EVR::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU4_EVR::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU4_EVR::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU4_EVR::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU4_EVR::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU4_EVR::IOEV_MCU_WU
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
#[doc = "Values that can be written to the field `WU7_EV`"]
pub enum WU7_EVW {
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
impl WU7_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU7_EVW::NONE => 63,
            WU7_EVW::AUX_COMPB_ASYNC_N => 56,
            WU7_EVW::AUX_COMPB_ASYNC => 55,
            WU7_EVW::BATMON_VOLT => 54,
            WU7_EVW::BATMON_TEMP => 53,
            WU7_EVW::AUX_TIMER1_EV => 52,
            WU7_EVW::AUX_TIMER0_EV => 51,
            WU7_EVW::AUX_TDC_DONE => 50,
            WU7_EVW::AUX_ADC_DONE => 49,
            WU7_EVW::AUX_COMPB => 48,
            WU7_EVW::AUX_COMPA => 47,
            WU7_EVW::AUX_SWEV2 => 46,
            WU7_EVW::AUX_SWEV1 => 45,
            WU7_EVW::AUX_SWEV0 => 44,
            WU7_EVW::JTAG => 43,
            WU7_EVW::RTC_UPD => 42,
            WU7_EVW::RTC_COMB_DLY => 41,
            WU7_EVW::RTC_CH2_DLY => 40,
            WU7_EVW::RTC_CH1_DLY => 39,
            WU7_EVW::RTC_CH0_DLY => 38,
            WU7_EVW::RTC_CH2 => 37,
            WU7_EVW::RTC_CH1 => 36,
            WU7_EVW::RTC_CH0 => 35,
            WU7_EVW::PAD => 32,
            WU7_EVW::BATMON_COMBINED => 9,
            WU7_EVW::BATMON_TEMP_LL => 8,
            WU7_EVW::BATMON_TEMP_UL => 7,
            WU7_EVW::BATMON_BATT_LL => 6,
            WU7_EVW::BATMON_BATT_UL => 5,
            WU7_EVW::AUX_TIMER2_EV3 => 4,
            WU7_EVW::AUX_TIMER2_EV2 => 3,
            WU7_EVW::AUX_TIMER2_EV1 => 2,
            WU7_EVW::AUX_TIMER2_EV0 => 1,
            WU7_EVW::IOEV_MCU_WU => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU7_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU7_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU7_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU7_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU7_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU7_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU7_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU7_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU7_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU7_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU7_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU7_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU7_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU7_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU7_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU7_EVW::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU7_EVW::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU7_EVW::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU7_EVW::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU7_EVW::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU7_EVW::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU7_EVW::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU7_EVW::IOEV_MCU_WU)
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
#[doc = "Values that can be written to the field `WU6_EV`"]
pub enum WU6_EVW {
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
impl WU6_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU6_EVW::NONE => 63,
            WU6_EVW::AUX_COMPB_ASYNC_N => 56,
            WU6_EVW::AUX_COMPB_ASYNC => 55,
            WU6_EVW::BATMON_VOLT => 54,
            WU6_EVW::BATMON_TEMP => 53,
            WU6_EVW::AUX_TIMER1_EV => 52,
            WU6_EVW::AUX_TIMER0_EV => 51,
            WU6_EVW::AUX_TDC_DONE => 50,
            WU6_EVW::AUX_ADC_DONE => 49,
            WU6_EVW::AUX_COMPB => 48,
            WU6_EVW::AUX_COMPA => 47,
            WU6_EVW::AUX_SWEV2 => 46,
            WU6_EVW::AUX_SWEV1 => 45,
            WU6_EVW::AUX_SWEV0 => 44,
            WU6_EVW::JTAG => 43,
            WU6_EVW::RTC_UPD => 42,
            WU6_EVW::RTC_COMB_DLY => 41,
            WU6_EVW::RTC_CH2_DLY => 40,
            WU6_EVW::RTC_CH1_DLY => 39,
            WU6_EVW::RTC_CH0_DLY => 38,
            WU6_EVW::RTC_CH2 => 37,
            WU6_EVW::RTC_CH1 => 36,
            WU6_EVW::RTC_CH0 => 35,
            WU6_EVW::PAD => 32,
            WU6_EVW::BATMON_COMBINED => 9,
            WU6_EVW::BATMON_TEMP_LL => 8,
            WU6_EVW::BATMON_TEMP_UL => 7,
            WU6_EVW::BATMON_BATT_LL => 6,
            WU6_EVW::BATMON_BATT_UL => 5,
            WU6_EVW::AUX_TIMER2_EV3 => 4,
            WU6_EVW::AUX_TIMER2_EV2 => 3,
            WU6_EVW::AUX_TIMER2_EV1 => 2,
            WU6_EVW::AUX_TIMER2_EV0 => 1,
            WU6_EVW::IOEV_MCU_WU => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU6_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU6_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU6_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU6_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU6_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU6_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU6_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU6_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU6_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU6_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU6_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU6_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU6_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU6_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU6_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU6_EVW::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU6_EVW::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU6_EVW::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU6_EVW::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU6_EVW::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU6_EVW::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU6_EVW::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU6_EVW::IOEV_MCU_WU)
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
#[doc = "Values that can be written to the field `WU5_EV`"]
pub enum WU5_EVW {
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
impl WU5_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU5_EVW::NONE => 63,
            WU5_EVW::AUX_COMPB_ASYNC_N => 56,
            WU5_EVW::AUX_COMPB_ASYNC => 55,
            WU5_EVW::BATMON_VOLT => 54,
            WU5_EVW::BATMON_TEMP => 53,
            WU5_EVW::AUX_TIMER1_EV => 52,
            WU5_EVW::AUX_TIMER0_EV => 51,
            WU5_EVW::AUX_TDC_DONE => 50,
            WU5_EVW::AUX_ADC_DONE => 49,
            WU5_EVW::AUX_COMPB => 48,
            WU5_EVW::AUX_COMPA => 47,
            WU5_EVW::AUX_SWEV2 => 46,
            WU5_EVW::AUX_SWEV1 => 45,
            WU5_EVW::AUX_SWEV0 => 44,
            WU5_EVW::JTAG => 43,
            WU5_EVW::RTC_UPD => 42,
            WU5_EVW::RTC_COMB_DLY => 41,
            WU5_EVW::RTC_CH2_DLY => 40,
            WU5_EVW::RTC_CH1_DLY => 39,
            WU5_EVW::RTC_CH0_DLY => 38,
            WU5_EVW::RTC_CH2 => 37,
            WU5_EVW::RTC_CH1 => 36,
            WU5_EVW::RTC_CH0 => 35,
            WU5_EVW::PAD => 32,
            WU5_EVW::BATMON_COMBINED => 9,
            WU5_EVW::BATMON_TEMP_LL => 8,
            WU5_EVW::BATMON_TEMP_UL => 7,
            WU5_EVW::BATMON_BATT_LL => 6,
            WU5_EVW::BATMON_BATT_UL => 5,
            WU5_EVW::AUX_TIMER2_EV3 => 4,
            WU5_EVW::AUX_TIMER2_EV2 => 3,
            WU5_EVW::AUX_TIMER2_EV1 => 2,
            WU5_EVW::AUX_TIMER2_EV0 => 1,
            WU5_EVW::IOEV_MCU_WU => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU5_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU5_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU5_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU5_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU5_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU5_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU5_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU5_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU5_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU5_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU5_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU5_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU5_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU5_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU5_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU5_EVW::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU5_EVW::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU5_EVW::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU5_EVW::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU5_EVW::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU5_EVW::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU5_EVW::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU5_EVW::IOEV_MCU_WU)
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
#[doc = "Values that can be written to the field `WU4_EV`"]
pub enum WU4_EVW {
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
impl WU4_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU4_EVW::NONE => 63,
            WU4_EVW::AUX_COMPB_ASYNC_N => 56,
            WU4_EVW::AUX_COMPB_ASYNC => 55,
            WU4_EVW::BATMON_VOLT => 54,
            WU4_EVW::BATMON_TEMP => 53,
            WU4_EVW::AUX_TIMER1_EV => 52,
            WU4_EVW::AUX_TIMER0_EV => 51,
            WU4_EVW::AUX_TDC_DONE => 50,
            WU4_EVW::AUX_ADC_DONE => 49,
            WU4_EVW::AUX_COMPB => 48,
            WU4_EVW::AUX_COMPA => 47,
            WU4_EVW::AUX_SWEV2 => 46,
            WU4_EVW::AUX_SWEV1 => 45,
            WU4_EVW::AUX_SWEV0 => 44,
            WU4_EVW::JTAG => 43,
            WU4_EVW::RTC_UPD => 42,
            WU4_EVW::RTC_COMB_DLY => 41,
            WU4_EVW::RTC_CH2_DLY => 40,
            WU4_EVW::RTC_CH1_DLY => 39,
            WU4_EVW::RTC_CH0_DLY => 38,
            WU4_EVW::RTC_CH2 => 37,
            WU4_EVW::RTC_CH1 => 36,
            WU4_EVW::RTC_CH0 => 35,
            WU4_EVW::PAD => 32,
            WU4_EVW::BATMON_COMBINED => 9,
            WU4_EVW::BATMON_TEMP_LL => 8,
            WU4_EVW::BATMON_TEMP_UL => 7,
            WU4_EVW::BATMON_BATT_LL => 6,
            WU4_EVW::BATMON_BATT_UL => 5,
            WU4_EVW::AUX_TIMER2_EV3 => 4,
            WU4_EVW::AUX_TIMER2_EV2 => 3,
            WU4_EVW::AUX_TIMER2_EV1 => 2,
            WU4_EVW::AUX_TIMER2_EV0 => 1,
            WU4_EVW::IOEV_MCU_WU => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU4_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU4_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU4_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU4_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU4_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU4_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU4_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU4_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU4_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU4_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU4_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU4_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU4_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU4_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU4_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU4_EVW::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU4_EVW::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU4_EVW::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU4_EVW::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU4_EVW::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU4_EVW::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU4_EVW::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU4_EVW::IOEV_MCU_WU)
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
    #[doc = "Bits 24:29 - 29:24\\] MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu7_ev(&self) -> WU7_EVR {
        WU7_EVR::_from({
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
    #[doc = "Bits 16:21 - 21:16\\] MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu6_ev(&self) -> WU6_EVR {
        WU6_EVR::_from({
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
    #[doc = "Bits 8:13 - 13:8\\] MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu5_ev(&self) -> WU5_EVR {
        WU5_EVR::_from({
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
    #[doc = "Bits 0:5 - 5:0\\] MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu4_ev(&self) -> WU4_EVR {
        WU4_EVR::_from({
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
    #[doc = "Bits 24:29 - 29:24\\] MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu7_ev(&mut self) -> _WU7_EVW {
        _WU7_EVW { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved22(&mut self) -> _RESERVED22W {
        _RESERVED22W { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\] MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu6_ev(&mut self) -> _WU6_EVW {
        _WU6_EVW { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&mut self) -> _RESERVED14W {
        _RESERVED14W { w: self }
    }
    #[doc = "Bits 8:13 - 13:8\\] MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu5_ev(&mut self) -> _WU5_EVW {
        _WU5_EVW { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu4_ev(&mut self) -> _WU4_EVW {
        _WU4_EVW { w: self }
    }
}
