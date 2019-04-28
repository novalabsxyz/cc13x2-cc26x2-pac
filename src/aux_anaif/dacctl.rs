#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DACCTL {
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
#[doc = r" Value of the field"]
pub struct DAC_ENR {
    bits: bool,
}
impl DAC_ENR {
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
pub struct DAC_BUFFER_ENR {
    bits: bool,
}
impl DAC_BUFFER_ENR {
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
pub struct DAC_PRECHARGE_ENR {
    bits: bool,
}
impl DAC_PRECHARGE_ENR {
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
#[doc = "Possible values of the field `DAC_VOUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC_VOUT_SELR {
    #[doc = "Connect to COMPA_IN analog node.\n\nRequired setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    COMPA_IN,
    #[doc = "Connect to COMPA_REF analog node.\n\nIt is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    COMPA_REF,
    #[doc = "Connect to COMPB_REF analog node.\n\nRequired setting to use Comparator B."]
    COMPB_REF,
    #[doc = "Connect to nothing\n\nIt is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    NC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DAC_VOUT_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DAC_VOUT_SELR::COMPA_IN => 4,
            DAC_VOUT_SELR::COMPA_REF => 2,
            DAC_VOUT_SELR::COMPB_REF => 1,
            DAC_VOUT_SELR::NC => 0,
            DAC_VOUT_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DAC_VOUT_SELR {
        match value {
            4 => DAC_VOUT_SELR::COMPA_IN,
            2 => DAC_VOUT_SELR::COMPA_REF,
            1 => DAC_VOUT_SELR::COMPB_REF,
            0 => DAC_VOUT_SELR::NC,
            i => DAC_VOUT_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPA_IN`"]
    #[inline]
    pub fn is_compa_in(&self) -> bool {
        *self == DAC_VOUT_SELR::COMPA_IN
    }
    #[doc = "Checks if the value of the field is `COMPA_REF`"]
    #[inline]
    pub fn is_compa_ref(&self) -> bool {
        *self == DAC_VOUT_SELR::COMPA_REF
    }
    #[doc = "Checks if the value of the field is `COMPB_REF`"]
    #[inline]
    pub fn is_compb_ref(&self) -> bool {
        *self == DAC_VOUT_SELR::COMPB_REF
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == DAC_VOUT_SELR::NC
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
#[doc = r" Proxy"]
pub struct _DAC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC_ENW<'a> {
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
pub struct _DAC_BUFFER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC_BUFFER_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _DAC_PRECHARGE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC_PRECHARGE_ENW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DAC_VOUT_SEL`"]
pub enum DAC_VOUT_SELW {
    #[doc = "Connect to COMPA_IN analog node.\n\nRequired setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    COMPA_IN,
    #[doc = "Connect to COMPA_REF analog node.\n\nIt is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    COMPA_REF,
    #[doc = "Connect to COMPB_REF analog node.\n\nRequired setting to use Comparator B."]
    COMPB_REF,
    #[doc = "Connect to nothing\n\nIt is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    NC,
}
impl DAC_VOUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAC_VOUT_SELW::COMPA_IN => 4,
            DAC_VOUT_SELW::COMPA_REF => 2,
            DAC_VOUT_SELW::COMPB_REF => 1,
            DAC_VOUT_SELW::NC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAC_VOUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC_VOUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAC_VOUT_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Connect to COMPA_IN analog node. Required setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    #[inline]
    pub fn compa_in(self) -> &'a mut W {
        self.variant(DAC_VOUT_SELW::COMPA_IN)
    }
    #[doc = "Connect to COMPA_REF analog node. It is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    #[inline]
    pub fn compa_ref(self) -> &'a mut W {
        self.variant(DAC_VOUT_SELW::COMPA_REF)
    }
    #[doc = "Connect to COMPB_REF analog node. Required setting to use Comparator B."]
    #[inline]
    pub fn compb_ref(self) -> &'a mut W {
        self.variant(DAC_VOUT_SELW::COMPB_REF)
    }
    #[doc = "Connect to nothing It is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(DAC_VOUT_SELW::NC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 5 - 5:5\\] DAC module enable. 0: Disable DAC. 1: Enable DAC. The Sensor Controller must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby TI-RTOS power mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle TI-RTOS power modes."]
    #[inline]
    pub fn dac_en(&self) -> DAC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DAC_ENR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
    #[inline]
    pub fn dac_buffer_en(&self) -> DAC_BUFFER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DAC_BUFFER_ENR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
    #[inline]
    pub fn dac_precharge_en(&self) -> DAC_PRECHARGE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DAC_PRECHARGE_ENR { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
    #[inline]
    pub fn dac_vout_sel(&self) -> DAC_VOUT_SELR {
        DAC_VOUT_SELR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] DAC module enable. 0: Disable DAC. 1: Enable DAC. The Sensor Controller must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby TI-RTOS power mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle TI-RTOS power modes."]
    #[inline]
    pub fn dac_en(&mut self) -> _DAC_ENW {
        _DAC_ENW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
    #[inline]
    pub fn dac_buffer_en(&mut self) -> _DAC_BUFFER_ENW {
        _DAC_BUFFER_ENW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
    #[inline]
    pub fn dac_precharge_en(&mut self) -> _DAC_PRECHARGE_ENW {
        _DAC_PRECHARGE_ENW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
    #[inline]
    pub fn dac_vout_sel(&mut self) -> _DAC_VOUT_SELW {
        _DAC_VOUT_SELW { w: self }
    }
}
