#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUSTAT {
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
pub struct RESERVED20R {
    bits: u16,
}
impl RESERVED20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXC_VECTORR {
    bits: u8,
}
impl EXC_VECTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED9R {
    bits: u8,
}
impl RESERVED9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WU_SIGNALR {
    bits: bool,
}
impl WU_SIGNALR {
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
#[doc = "Possible values of the field `EV_SIGNALS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV_SIGNALSR {
    #[doc = "Internal. Only to be used through TI provided API."]
    SCEWEV_PROG,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_ADC_FIFO_NOT_EMPTY,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_TIMER1_EV_OR_IDLE,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_TIMER0_EV_OR_IDLE,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_TDC_DONE,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_COMPB,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_COMPA,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_PROG_DLY_IDLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EV_SIGNALSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EV_SIGNALSR::SCEWEV_PROG => 128,
            EV_SIGNALSR::AUX_ADC_FIFO_NOT_EMPTY => 64,
            EV_SIGNALSR::AUX_TIMER1_EV_OR_IDLE => 32,
            EV_SIGNALSR::AUX_TIMER0_EV_OR_IDLE => 16,
            EV_SIGNALSR::AUX_TDC_DONE => 8,
            EV_SIGNALSR::AUX_COMPB => 4,
            EV_SIGNALSR::AUX_COMPA => 2,
            EV_SIGNALSR::AUX_PROG_DLY_IDLE => 1,
            EV_SIGNALSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EV_SIGNALSR {
        match value {
            128 => EV_SIGNALSR::SCEWEV_PROG,
            64 => EV_SIGNALSR::AUX_ADC_FIFO_NOT_EMPTY,
            32 => EV_SIGNALSR::AUX_TIMER1_EV_OR_IDLE,
            16 => EV_SIGNALSR::AUX_TIMER0_EV_OR_IDLE,
            8 => EV_SIGNALSR::AUX_TDC_DONE,
            4 => EV_SIGNALSR::AUX_COMPB,
            2 => EV_SIGNALSR::AUX_COMPA,
            1 => EV_SIGNALSR::AUX_PROG_DLY_IDLE,
            i => EV_SIGNALSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCEWEV_PROG`"]
    #[inline]
    pub fn is_scewev_prog(&self) -> bool {
        *self == EV_SIGNALSR::SCEWEV_PROG
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == EV_SIGNALSR::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV_OR_IDLE`"]
    #[inline]
    pub fn is_aux_timer1_ev_or_idle(&self) -> bool {
        *self == EV_SIGNALSR::AUX_TIMER1_EV_OR_IDLE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV_OR_IDLE`"]
    #[inline]
    pub fn is_aux_timer0_ev_or_idle(&self) -> bool {
        *self == EV_SIGNALSR::AUX_TIMER0_EV_OR_IDLE
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EV_SIGNALSR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == EV_SIGNALSR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == EV_SIGNALSR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_PROG_DLY_IDLE`"]
    #[inline]
    pub fn is_aux_prog_dly_idle(&self) -> bool {
        *self == EV_SIGNALSR::AUX_PROG_DLY_IDLE
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED20W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED20W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXC_VECTORW<'a> {
    w: &'a mut W,
}
impl<'a> _EXC_VECTORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED9W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WU_SIGNALW<'a> {
    w: &'a mut W,
}
impl<'a> _WU_SIGNALW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV_SIGNALS`"]
pub enum EV_SIGNALSW {
    #[doc = "Internal. Only to be used through TI provided API."]
    SCEWEV_PROG,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_ADC_FIFO_NOT_EMPTY,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_TIMER1_EV_OR_IDLE,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_TIMER0_EV_OR_IDLE,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_TDC_DONE,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_COMPB,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_COMPA,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUX_PROG_DLY_IDLE,
}
impl EV_SIGNALSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EV_SIGNALSW::SCEWEV_PROG => 128,
            EV_SIGNALSW::AUX_ADC_FIFO_NOT_EMPTY => 64,
            EV_SIGNALSW::AUX_TIMER1_EV_OR_IDLE => 32,
            EV_SIGNALSW::AUX_TIMER0_EV_OR_IDLE => 16,
            EV_SIGNALSW::AUX_TDC_DONE => 8,
            EV_SIGNALSW::AUX_COMPB => 4,
            EV_SIGNALSW::AUX_COMPA => 2,
            EV_SIGNALSW::AUX_PROG_DLY_IDLE => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV_SIGNALSW<'a> {
    w: &'a mut W,
}
impl<'a> _EV_SIGNALSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV_SIGNALSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn scewev_prog(self) -> &'a mut W {
        self.variant(EV_SIGNALSW::SCEWEV_PROG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(EV_SIGNALSW::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aux_timer1_ev_or_idle(self) -> &'a mut W {
        self.variant(EV_SIGNALSW::AUX_TIMER1_EV_OR_IDLE)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aux_timer0_ev_or_idle(self) -> &'a mut W {
        self.variant(EV_SIGNALSW::AUX_TIMER0_EV_OR_IDLE)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(EV_SIGNALSW::AUX_TDC_DONE)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(EV_SIGNALSW::AUX_COMPB)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(EV_SIGNALSW::AUX_COMPA)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aux_prog_dly_idle(self) -> &'a mut W {
        self.variant(EV_SIGNALSW::AUX_PROG_DLY_IDLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 19:31 - 31:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved20(&self) -> RESERVED20R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED20R { bits }
    }
    #[doc = "Bits 16:18 - 18:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn exc_vector(&self) -> EXC_VECTORR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXC_VECTORR { bits }
    }
    #[doc = "Bits 9:15 - 15:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved9(&self) -> RESERVED9R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED9R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn wu_signal(&self) -> WU_SIGNALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WU_SIGNALR { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ev_signals(&self) -> EV_SIGNALSR {
        EV_SIGNALSR::_from({
            const MASK: u8 = 255;
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
    #[doc = "Bits 19:31 - 31:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved20(&mut self) -> _RESERVED20W {
        _RESERVED20W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn exc_vector(&mut self) -> _EXC_VECTORW {
        _EXC_VECTORW { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved9(&mut self) -> _RESERVED9W {
        _RESERVED9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn wu_signal(&mut self) -> _WU_SIGNALW {
        _WU_SIGNALW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ev_signals(&mut self) -> _EV_SIGNALSW {
        _EV_SIGNALSW { w: self }
    }
}
