#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPMODEREQ {
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
pub struct RESERVED2R {
    bits: u32,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQR {
    #[doc = "Powerdown operational mode with wakeup to lowpower mode, characterized by:\n- Powerdown system power supply state (uLDO) request.\n- AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE).\n- An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    PDLP,
    #[doc = "Powerdown operational mode with wakeup to active mode, characterized by:\n- Powerdown system power supply state (uLDO) request.\n- AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE).\n- An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    PDA,
    #[doc = "Lowpower operational mode, characterized by:\n- Powerdown system power supply state (uLDO) request.\n- SCE clock frequency (SCE_RATE) equals SCLK_MF.\n- An active wakeup flag does not change operational mode."]
    LP,
    #[doc = "Active operational mode, characterized by:\n- Active system power supply state (GLDO or DCDC) request. \n- AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE).\n- An active wakeup flag does not change operational mode."]
    A,
}
impl REQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REQR::PDLP => 3,
            REQR::PDA => 2,
            REQR::LP => 1,
            REQR::A => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REQR {
        match value {
            3 => REQR::PDLP,
            2 => REQR::PDA,
            1 => REQR::LP,
            0 => REQR::A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PDLP`"]
    #[inline]
    pub fn is_pdlp(&self) -> bool {
        *self == REQR::PDLP
    }
    #[doc = "Checks if the value of the field is `PDA`"]
    #[inline]
    pub fn is_pda(&self) -> bool {
        *self == REQR::PDA
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline]
    pub fn is_lp(&self) -> bool {
        *self == REQR::LP
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline]
    pub fn is_a(&self) -> bool {
        *self == REQR::A
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REQ`"]
pub enum REQW {
    #[doc = "Powerdown operational mode with wakeup to lowpower mode, characterized by:\n- Powerdown system power supply state (uLDO) request.\n- AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE).\n- An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    PDLP,
    #[doc = "Powerdown operational mode with wakeup to active mode, characterized by:\n- Powerdown system power supply state (uLDO) request.\n- AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE).\n- An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    PDA,
    #[doc = "Lowpower operational mode, characterized by:\n- Powerdown system power supply state (uLDO) request.\n- SCE clock frequency (SCE_RATE) equals SCLK_MF.\n- An active wakeup flag does not change operational mode."]
    LP,
    #[doc = "Active operational mode, characterized by:\n- Active system power supply state (GLDO or DCDC) request. \n- AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE).\n- An active wakeup flag does not change operational mode."]
    A,
}
impl REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REQW::PDLP => 3,
            REQW::PDA => 2,
            REQW::LP => 1,
            REQW::A => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REQW<'a> {
    w: &'a mut W,
}
impl<'a> _REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REQW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Powerdown operational mode with wakeup to lowpower mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    #[inline]
    pub fn pdlp(self) -> &'a mut W {
        self.variant(REQW::PDLP)
    }
    #[doc = "Powerdown operational mode with wakeup to active mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    #[inline]
    pub fn pda(self) -> &'a mut W {
        self.variant(REQW::PDA)
    }
    #[doc = "Lowpower operational mode, characterized by: - Powerdown system power supply state (uLDO) request. - SCE clock frequency (SCE_RATE) equals SCLK_MF. - An active wakeup flag does not change operational mode."]
    #[inline]
    pub fn lp(self) -> &'a mut W {
        self.variant(REQW::LP)
    }
    #[doc = "Active operational mode, characterized by: - Active system power supply state (GLDO or DCDC) request. - AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag does not change operational mode."]
    #[inline]
    pub fn a(self) -> &'a mut W {
        self.variant(REQW::A)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] AUX operational mode request."]
    #[inline]
    pub fn req(&self) -> REQR {
        REQR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] AUX operational mode request."]
    #[inline]
    pub fn req(&mut self) -> _REQW {
        _REQW { w: self }
    }
}
