#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH3EVCFG {
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
pub struct EV3_GENR {
    bits: bool,
}
impl EV3_GENR {
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
pub struct EV2_GENR {
    bits: bool,
}
impl EV2_GENR {
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
pub struct EV1_GENR {
    bits: bool,
}
impl EV1_GENR {
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
pub struct EV0_GENR {
    bits: bool,
}
impl EV0_GENR {
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
#[doc = "Possible values of the field `CCACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCACTR {
    #[doc = "Pulse on compare repeatedly. \n\nChannel function sequence: \n- Pulse enabled events when CH3CC.VALUE = CNTR.VALUE.\n\n The event is high for two timer clock periods."]
    PULSE_ON_CMP,
    #[doc = "Toggle on compare repeatedly.\n\nChannel function sequence: \n- Toggle enabled events  when CH3CC.VALUE = CNTR.VALUE."]
    TGL_ON_CMP,
    #[doc = "Set on compare repeatedly.\n\nChannel function sequence: \n- Set enabled events  when CH3CC.VALUE = CNTR.VALUE."]
    SET_ON_CMP,
    #[doc = "Clear on compare repeatedly.\n\nChannel function sequence: \n- Clear enabled events  when CH3CC.VALUE = CNTR.VALUE."]
    CLR_ON_CMP,
    #[doc = "Set on zero, toggle on compare repeatedly.\n\nChannel function sequence: \n- Set enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n\nSet CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: \n\nWhen CH3CC.VALUE <= TARGET.VALUE: \n   Duty cycle = CH3CC.VALUE / ( TARGET.VALUE + 1 ).\n\nWhen CH3CC.VALUE > TARGET.VALUE: \n   Duty cycle = 1.\n\nEnabled events are cleared when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    SET_ON_0_TGL_ON_CMP,
    #[doc = "Clear on zero, toggle on compare repeatedly.\n \nChannel function sequence: \n- Clear enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n\nSet CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: \n\nWhen CH3CC.VALUE <= TARGET.VALUE: \n   Duty cycle = 1 - ( CH3CC.VALUE / TARGET.VALUE ).\n\nWhen CH3CC.VALUE > TARGET.VALUE: \n   Duty cycle = 0.\n\nEnabled events are set when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    CLR_ON_0_TGL_ON_CMP,
    #[doc = "Set on capture repeatedly.\n\nChannel function sequence: \n- Set enabled events on capture event and copy CNTR.VALUE to CH3CC.VALUE.\n\nPrimary use scenario is to select this function before you start the timer.\nFollow these steps if you need to select this function while CTL.MODE is different from DIS: \n - Select this function with no event enable.\n - Configure CH3CCFG (optional).\n - Wait for three timer clock periods as defined in PRECFG before you enable events.\n\nThese steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SET_ON_CAPT,
    #[doc = "Period and pulse width measurement.\n\nContinuously capture period and pulse width of the signal selected by CH3CCFG.CAPT_SRC relative to the signal edge given by CH3CCFG.EDGE. \n\nSet enabled events when CH3CC.VALUE contains signal period and CH3PCC.VALUE contains signal pulse width. \n\nNotes: \n- Make sure that you configure CH3CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. \n- The counter restarts in the selected timer mode when CH3CC.VALUE contains the signal period.\n- If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. \n- If you want to observe a timeout event configure another channel to SET_ON_CAPT.\n\nSignal property requirements:\n- Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period.\n- Signal Period <= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period.\n- Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    PER_PULSE_WIDTH_MEAS,
    #[doc = "Pulse on compare, and then disable channel.\n\nChannel function sequence: \n- Pulse enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel.\n\n The event is high for two timer clock periods."]
    PULSE_ON_CMP_DIS,
    #[doc = "Toggle on compare, and then disable channel.\n\nChannel function sequence: \n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel."]
    TGL_ON_CMP_DIS,
    #[doc = "Set on compare, and then disable channel.\n\nChannel function sequence: \n- Set enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel."]
    SET_ON_CMP_DIS,
    #[doc = "Clear on compare, and then disable channel.\n\nChannel function sequence: \n- Clear enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel."]
    CLR_ON_CMP_DIS,
    #[doc = "Set on zero, toggle on compare, and then disable channel.\n\nChannel function sequence: \n- Set enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel.\n\nEnabled events are cleared when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    SET_ON_0_TGL_ON_CMP_DIS,
    #[doc = "Clear on zero, toggle on compare, and then disable channel.\n\nChannel function sequence: \n- Clear enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel.\n\nEnabled events are set when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    CLR_ON_0_TGL_ON_CMP_DIS,
    #[doc = "Set on capture, and then disable channel.\n\nChannel function sequence: \n- Set enabled events on capture event and copy CNTR.VALUE to CH3CC.VALUE.\n- Disable channel. \n\nPrimary use scenario is to select this function before you start the timer.\nFollow these steps if you need to select this function while CTL.MODE is different from DIS: \n - Set CCACT to SET_ON_CAPT with no event enable.\n - Configure CH3CCFG (optional).\n - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS.  Event enable is optional.\n\nThese steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SET_ON_CAPT_DIS,
    #[doc = "Disable channel."]
    DIS,
}
impl CCACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CCACTR::PULSE_ON_CMP => 15,
            CCACTR::TGL_ON_CMP => 14,
            CCACTR::SET_ON_CMP => 13,
            CCACTR::CLR_ON_CMP => 12,
            CCACTR::SET_ON_0_TGL_ON_CMP => 11,
            CCACTR::CLR_ON_0_TGL_ON_CMP => 10,
            CCACTR::SET_ON_CAPT => 9,
            CCACTR::PER_PULSE_WIDTH_MEAS => 8,
            CCACTR::PULSE_ON_CMP_DIS => 7,
            CCACTR::TGL_ON_CMP_DIS => 6,
            CCACTR::SET_ON_CMP_DIS => 5,
            CCACTR::CLR_ON_CMP_DIS => 4,
            CCACTR::SET_ON_0_TGL_ON_CMP_DIS => 3,
            CCACTR::CLR_ON_0_TGL_ON_CMP_DIS => 2,
            CCACTR::SET_ON_CAPT_DIS => 1,
            CCACTR::DIS => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CCACTR {
        match value {
            15 => CCACTR::PULSE_ON_CMP,
            14 => CCACTR::TGL_ON_CMP,
            13 => CCACTR::SET_ON_CMP,
            12 => CCACTR::CLR_ON_CMP,
            11 => CCACTR::SET_ON_0_TGL_ON_CMP,
            10 => CCACTR::CLR_ON_0_TGL_ON_CMP,
            9 => CCACTR::SET_ON_CAPT,
            8 => CCACTR::PER_PULSE_WIDTH_MEAS,
            7 => CCACTR::PULSE_ON_CMP_DIS,
            6 => CCACTR::TGL_ON_CMP_DIS,
            5 => CCACTR::SET_ON_CMP_DIS,
            4 => CCACTR::CLR_ON_CMP_DIS,
            3 => CCACTR::SET_ON_0_TGL_ON_CMP_DIS,
            2 => CCACTR::CLR_ON_0_TGL_ON_CMP_DIS,
            1 => CCACTR::SET_ON_CAPT_DIS,
            0 => CCACTR::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULSE_ON_CMP`"]
    #[inline]
    pub fn is_pulse_on_cmp(&self) -> bool {
        *self == CCACTR::PULSE_ON_CMP
    }
    #[doc = "Checks if the value of the field is `TGL_ON_CMP`"]
    #[inline]
    pub fn is_tgl_on_cmp(&self) -> bool {
        *self == CCACTR::TGL_ON_CMP
    }
    #[doc = "Checks if the value of the field is `SET_ON_CMP`"]
    #[inline]
    pub fn is_set_on_cmp(&self) -> bool {
        *self == CCACTR::SET_ON_CMP
    }
    #[doc = "Checks if the value of the field is `CLR_ON_CMP`"]
    #[inline]
    pub fn is_clr_on_cmp(&self) -> bool {
        *self == CCACTR::CLR_ON_CMP
    }
    #[doc = "Checks if the value of the field is `SET_ON_0_TGL_ON_CMP`"]
    #[inline]
    pub fn is_set_on_0_tgl_on_cmp(&self) -> bool {
        *self == CCACTR::SET_ON_0_TGL_ON_CMP
    }
    #[doc = "Checks if the value of the field is `CLR_ON_0_TGL_ON_CMP`"]
    #[inline]
    pub fn is_clr_on_0_tgl_on_cmp(&self) -> bool {
        *self == CCACTR::CLR_ON_0_TGL_ON_CMP
    }
    #[doc = "Checks if the value of the field is `SET_ON_CAPT`"]
    #[inline]
    pub fn is_set_on_capt(&self) -> bool {
        *self == CCACTR::SET_ON_CAPT
    }
    #[doc = "Checks if the value of the field is `PER_PULSE_WIDTH_MEAS`"]
    #[inline]
    pub fn is_per_pulse_width_meas(&self) -> bool {
        *self == CCACTR::PER_PULSE_WIDTH_MEAS
    }
    #[doc = "Checks if the value of the field is `PULSE_ON_CMP_DIS`"]
    #[inline]
    pub fn is_pulse_on_cmp_dis(&self) -> bool {
        *self == CCACTR::PULSE_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `TGL_ON_CMP_DIS`"]
    #[inline]
    pub fn is_tgl_on_cmp_dis(&self) -> bool {
        *self == CCACTR::TGL_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `SET_ON_CMP_DIS`"]
    #[inline]
    pub fn is_set_on_cmp_dis(&self) -> bool {
        *self == CCACTR::SET_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `CLR_ON_CMP_DIS`"]
    #[inline]
    pub fn is_clr_on_cmp_dis(&self) -> bool {
        *self == CCACTR::CLR_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `SET_ON_0_TGL_ON_CMP_DIS`"]
    #[inline]
    pub fn is_set_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == CCACTR::SET_ON_0_TGL_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `CLR_ON_0_TGL_ON_CMP_DIS`"]
    #[inline]
    pub fn is_clr_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == CCACTR::CLR_ON_0_TGL_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `SET_ON_CAPT_DIS`"]
    #[inline]
    pub fn is_set_on_capt_dis(&self) -> bool {
        *self == CCACTR::SET_ON_CAPT_DIS
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CCACTR::DIS
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
pub struct _EV3_GENW<'a> {
    w: &'a mut W,
}
impl<'a> _EV3_GENW<'a> {
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
pub struct _EV2_GENW<'a> {
    w: &'a mut W,
}
impl<'a> _EV2_GENW<'a> {
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
#[doc = r" Proxy"]
pub struct _EV1_GENW<'a> {
    w: &'a mut W,
}
impl<'a> _EV1_GENW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EV0_GENW<'a> {
    w: &'a mut W,
}
impl<'a> _EV0_GENW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCACT`"]
pub enum CCACTW {
    #[doc = "Pulse on compare repeatedly. \n\nChannel function sequence: \n- Pulse enabled events when CH3CC.VALUE = CNTR.VALUE.\n\n The event is high for two timer clock periods."]
    PULSE_ON_CMP,
    #[doc = "Toggle on compare repeatedly.\n\nChannel function sequence: \n- Toggle enabled events  when CH3CC.VALUE = CNTR.VALUE."]
    TGL_ON_CMP,
    #[doc = "Set on compare repeatedly.\n\nChannel function sequence: \n- Set enabled events  when CH3CC.VALUE = CNTR.VALUE."]
    SET_ON_CMP,
    #[doc = "Clear on compare repeatedly.\n\nChannel function sequence: \n- Clear enabled events  when CH3CC.VALUE = CNTR.VALUE."]
    CLR_ON_CMP,
    #[doc = "Set on zero, toggle on compare repeatedly.\n\nChannel function sequence: \n- Set enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n\nSet CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: \n\nWhen CH3CC.VALUE <= TARGET.VALUE: \n   Duty cycle = CH3CC.VALUE / ( TARGET.VALUE + 1 ).\n\nWhen CH3CC.VALUE > TARGET.VALUE: \n   Duty cycle = 1.\n\nEnabled events are cleared when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    SET_ON_0_TGL_ON_CMP,
    #[doc = "Clear on zero, toggle on compare repeatedly.\n \nChannel function sequence: \n- Clear enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n\nSet CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: \n\nWhen CH3CC.VALUE <= TARGET.VALUE: \n   Duty cycle = 1 - ( CH3CC.VALUE / TARGET.VALUE ).\n\nWhen CH3CC.VALUE > TARGET.VALUE: \n   Duty cycle = 0.\n\nEnabled events are set when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    CLR_ON_0_TGL_ON_CMP,
    #[doc = "Set on capture repeatedly.\n\nChannel function sequence: \n- Set enabled events on capture event and copy CNTR.VALUE to CH3CC.VALUE.\n\nPrimary use scenario is to select this function before you start the timer.\nFollow these steps if you need to select this function while CTL.MODE is different from DIS: \n - Select this function with no event enable.\n - Configure CH3CCFG (optional).\n - Wait for three timer clock periods as defined in PRECFG before you enable events.\n\nThese steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SET_ON_CAPT,
    #[doc = "Period and pulse width measurement.\n\nContinuously capture period and pulse width of the signal selected by CH3CCFG.CAPT_SRC relative to the signal edge given by CH3CCFG.EDGE. \n\nSet enabled events when CH3CC.VALUE contains signal period and CH3PCC.VALUE contains signal pulse width. \n\nNotes: \n- Make sure that you configure CH3CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. \n- The counter restarts in the selected timer mode when CH3CC.VALUE contains the signal period.\n- If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. \n- If you want to observe a timeout event configure another channel to SET_ON_CAPT.\n\nSignal property requirements:\n- Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period.\n- Signal Period <= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period.\n- Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    PER_PULSE_WIDTH_MEAS,
    #[doc = "Pulse on compare, and then disable channel.\n\nChannel function sequence: \n- Pulse enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel.\n\n The event is high for two timer clock periods."]
    PULSE_ON_CMP_DIS,
    #[doc = "Toggle on compare, and then disable channel.\n\nChannel function sequence: \n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel."]
    TGL_ON_CMP_DIS,
    #[doc = "Set on compare, and then disable channel.\n\nChannel function sequence: \n- Set enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel."]
    SET_ON_CMP_DIS,
    #[doc = "Clear on compare, and then disable channel.\n\nChannel function sequence: \n- Clear enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel."]
    CLR_ON_CMP_DIS,
    #[doc = "Set on zero, toggle on compare, and then disable channel.\n\nChannel function sequence: \n- Set enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel.\n\nEnabled events are cleared when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    SET_ON_0_TGL_ON_CMP_DIS,
    #[doc = "Clear on zero, toggle on compare, and then disable channel.\n\nChannel function sequence: \n- Clear enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH3CC.VALUE = CNTR.VALUE.\n- Disable channel.\n\nEnabled events are set when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    CLR_ON_0_TGL_ON_CMP_DIS,
    #[doc = "Set on capture, and then disable channel.\n\nChannel function sequence: \n- Set enabled events on capture event and copy CNTR.VALUE to CH3CC.VALUE.\n- Disable channel. \n\nPrimary use scenario is to select this function before you start the timer.\nFollow these steps if you need to select this function while CTL.MODE is different from DIS: \n - Set CCACT to SET_ON_CAPT with no event enable.\n - Configure CH3CCFG (optional).\n - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS.  Event enable is optional.\n\nThese steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SET_ON_CAPT_DIS,
    #[doc = "Disable channel."]
    DIS,
}
impl CCACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CCACTW::PULSE_ON_CMP => 15,
            CCACTW::TGL_ON_CMP => 14,
            CCACTW::SET_ON_CMP => 13,
            CCACTW::CLR_ON_CMP => 12,
            CCACTW::SET_ON_0_TGL_ON_CMP => 11,
            CCACTW::CLR_ON_0_TGL_ON_CMP => 10,
            CCACTW::SET_ON_CAPT => 9,
            CCACTW::PER_PULSE_WIDTH_MEAS => 8,
            CCACTW::PULSE_ON_CMP_DIS => 7,
            CCACTW::TGL_ON_CMP_DIS => 6,
            CCACTW::SET_ON_CMP_DIS => 5,
            CCACTW::CLR_ON_CMP_DIS => 4,
            CCACTW::SET_ON_0_TGL_ON_CMP_DIS => 3,
            CCACTW::CLR_ON_0_TGL_ON_CMP_DIS => 2,
            CCACTW::SET_ON_CAPT_DIS => 1,
            CCACTW::DIS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCACTW<'a> {
    w: &'a mut W,
}
impl<'a> _CCACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pulse on compare repeatedly. Channel function sequence: - Pulse enabled events when CH3CC.VALUE = CNTR.VALUE. The event is high for two timer clock periods."]
    #[inline]
    pub fn pulse_on_cmp(self) -> &'a mut W {
        self.variant(CCACTW::PULSE_ON_CMP)
    }
    #[doc = "Toggle on compare repeatedly. Channel function sequence: - Toggle enabled events when CH3CC.VALUE = CNTR.VALUE."]
    #[inline]
    pub fn tgl_on_cmp(self) -> &'a mut W {
        self.variant(CCACTW::TGL_ON_CMP)
    }
    #[doc = "Set on compare repeatedly. Channel function sequence: - Set enabled events when CH3CC.VALUE = CNTR.VALUE."]
    #[inline]
    pub fn set_on_cmp(self) -> &'a mut W {
        self.variant(CCACTW::SET_ON_CMP)
    }
    #[doc = "Clear on compare repeatedly. Channel function sequence: - Clear enabled events when CH3CC.VALUE = CNTR.VALUE."]
    #[inline]
    pub fn clr_on_cmp(self) -> &'a mut W {
        self.variant(CCACTW::CLR_ON_CMP)
    }
    #[doc = "Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH3CC.VALUE = CNTR.VALUE. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When CH3CC.VALUE <= TARGET.VALUE: Duty cycle = CH3CC.VALUE / ( TARGET.VALUE + 1 ). When CH3CC.VALUE > TARGET.VALUE: Duty cycle = 1. Enabled events are cleared when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline]
    pub fn set_on_0_tgl_on_cmp(self) -> &'a mut W {
        self.variant(CCACTW::SET_ON_0_TGL_ON_CMP)
    }
    #[doc = "Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH3CC.VALUE = CNTR.VALUE. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When CH3CC.VALUE <= TARGET.VALUE: Duty cycle = 1 - ( CH3CC.VALUE / TARGET.VALUE ). When CH3CC.VALUE > TARGET.VALUE: Duty cycle = 0. Enabled events are set when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline]
    pub fn clr_on_0_tgl_on_cmp(self) -> &'a mut W {
        self.variant(CCACTW::CLR_ON_0_TGL_ON_CMP)
    }
    #[doc = "Set on capture repeatedly. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH3CC.VALUE. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Select this function with no event enable. - Configure CH3CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you enable events. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline]
    pub fn set_on_capt(self) -> &'a mut W {
        self.variant(CCACTW::SET_ON_CAPT)
    }
    #[doc = "Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by CH3CCFG.CAPT_SRC relative to the signal edge given by CH3CCFG.EDGE. Set enabled events when CH3CC.VALUE contains signal period and CH3PCC.VALUE contains signal pulse width. Notes: - Make sure that you configure CH3CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when CH3CC.VALUE contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - If you want to observe a timeout event configure another channel to SET_ON_CAPT. Signal property requirements: - Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period. - Signal Period <= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period. - Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    #[inline]
    pub fn per_pulse_width_meas(self) -> &'a mut W {
        self.variant(CCACTW::PER_PULSE_WIDTH_MEAS)
    }
    #[doc = "Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled events when CH3CC.VALUE = CNTR.VALUE. - Disable channel. The event is high for two timer clock periods."]
    #[inline]
    pub fn pulse_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACTW::PULSE_ON_CMP_DIS)
    }
    #[doc = "Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled events when CH3CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline]
    pub fn tgl_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACTW::TGL_ON_CMP_DIS)
    }
    #[doc = "Set on compare, and then disable channel. Channel function sequence: - Set enabled events when CH3CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline]
    pub fn set_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACTW::SET_ON_CMP_DIS)
    }
    #[doc = "Clear on compare, and then disable channel. Channel function sequence: - Clear enabled events when CH3CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline]
    pub fn clr_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACTW::CLR_ON_CMP_DIS)
    }
    #[doc = "Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH3CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are cleared when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline]
    pub fn set_on_0_tgl_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACTW::SET_ON_0_TGL_ON_CMP_DIS)
    }
    #[doc = "Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH3CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are set when CH3CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline]
    pub fn clr_on_0_tgl_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACTW::CLR_ON_0_TGL_ON_CMP_DIS)
    }
    #[doc = "Set on capture, and then disable channel. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH3CC.VALUE. - Disable channel. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no event enable. - Configure CH3CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS. Event enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline]
    pub fn set_on_capt_dis(self) -> &'a mut W {
        self.variant(CCACTW::SET_ON_CAPT_DIS)
    }
    #[doc = "Disable channel."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CCACTW::DIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bit 7 - 7:7\\] Event 3 enable. 0: Channel 3 does not control event 3. 1: Channel 3 controls event 3. When 0 < CCACT < 8, EV3_GEN becomes zero after a capture or compare event."]
    #[inline]
    pub fn ev3_gen(&self) -> EV3_GENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EV3_GENR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Event 2 enable. 0: Channel 3 does not control event 2. 1: Channel 3 controls event 2. When 0 < CCACT < 8, EV2_GEN becomes zero after a capture or compare event."]
    #[inline]
    pub fn ev2_gen(&self) -> EV2_GENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EV2_GENR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Event 1 enable. 0: Channel 3 does not control event 1. 1: Channel 3 controls event 1. When 0 < CCACT < 8, EV1_GEN becomes zero after a capture or compare event."]
    #[inline]
    pub fn ev1_gen(&self) -> EV1_GENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EV1_GENR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Event 0 enable. 0: Channel 3 does not control event 0. 1: Channel 3 controls event 0. When 0 < CCACT < 8, EV0_GEN becomes zero after a capture or compare event."]
    #[inline]
    pub fn ev0_gen(&self) -> EV0_GENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EV0_GENR { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
    #[inline]
    pub fn ccact(&self) -> CCACTR {
        CCACTR::_from({
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&mut self) -> _RESERVED8W {
        _RESERVED8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Event 3 enable. 0: Channel 3 does not control event 3. 1: Channel 3 controls event 3. When 0 < CCACT < 8, EV3_GEN becomes zero after a capture or compare event."]
    #[inline]
    pub fn ev3_gen(&mut self) -> _EV3_GENW {
        _EV3_GENW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Event 2 enable. 0: Channel 3 does not control event 2. 1: Channel 3 controls event 2. When 0 < CCACT < 8, EV2_GEN becomes zero after a capture or compare event."]
    #[inline]
    pub fn ev2_gen(&mut self) -> _EV2_GENW {
        _EV2_GENW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Event 1 enable. 0: Channel 3 does not control event 1. 1: Channel 3 controls event 1. When 0 < CCACT < 8, EV1_GEN becomes zero after a capture or compare event."]
    #[inline]
    pub fn ev1_gen(&mut self) -> _EV1_GENW {
        _EV1_GENW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Event 0 enable. 0: Channel 3 does not control event 0. 1: Channel 3 controls event 0. When 0 < CCACT < 8, EV0_GEN becomes zero after a capture or compare event."]
    #[inline]
    pub fn ev0_gen(&mut self) -> _EV0_GENW {
        _EV0_GENW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
    #[inline]
    pub fn ccact(&mut self) -> _CCACTW {
        _CCACTW { w: self }
    }
}
