#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MUX4 {
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
#[doc = "Possible values of the field `COMPA_REF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPA_REFR {
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO19,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO20,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO21,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO22,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO23,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO24,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO25,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO26,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl COMPA_REFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMPA_REFR::AUXIO19 => 128,
            COMPA_REFR::AUXIO20 => 64,
            COMPA_REFR::AUXIO21 => 32,
            COMPA_REFR::AUXIO22 => 16,
            COMPA_REFR::AUXIO23 => 8,
            COMPA_REFR::AUXIO24 => 4,
            COMPA_REFR::AUXIO25 => 2,
            COMPA_REFR::AUXIO26 => 1,
            COMPA_REFR::NC => 0,
            COMPA_REFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMPA_REFR {
        match value {
            128 => COMPA_REFR::AUXIO19,
            64 => COMPA_REFR::AUXIO20,
            32 => COMPA_REFR::AUXIO21,
            16 => COMPA_REFR::AUXIO22,
            8 => COMPA_REFR::AUXIO23,
            4 => COMPA_REFR::AUXIO24,
            2 => COMPA_REFR::AUXIO25,
            1 => COMPA_REFR::AUXIO26,
            0 => COMPA_REFR::NC,
            i => COMPA_REFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline]
    pub fn is_auxio19(&self) -> bool {
        *self == COMPA_REFR::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline]
    pub fn is_auxio20(&self) -> bool {
        *self == COMPA_REFR::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline]
    pub fn is_auxio21(&self) -> bool {
        *self == COMPA_REFR::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline]
    pub fn is_auxio22(&self) -> bool {
        *self == COMPA_REFR::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline]
    pub fn is_auxio23(&self) -> bool {
        *self == COMPA_REFR::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline]
    pub fn is_auxio24(&self) -> bool {
        *self == COMPA_REFR::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline]
    pub fn is_auxio25(&self) -> bool {
        *self == COMPA_REFR::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline]
    pub fn is_auxio26(&self) -> bool {
        *self == COMPA_REFR::AUXIO26
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == COMPA_REFR::NC
    }
}
#[doc = "Values that can be written to the field `COMPA_REF`"]
pub enum COMPA_REFW {
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO19,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO20,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO21,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO22,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO23,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO24,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO25,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO26,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
}
impl COMPA_REFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMPA_REFW::AUXIO19 => 128,
            COMPA_REFW::AUXIO20 => 64,
            COMPA_REFW::AUXIO21 => 32,
            COMPA_REFW::AUXIO22 => 16,
            COMPA_REFW::AUXIO23 => 8,
            COMPA_REFW::AUXIO24 => 4,
            COMPA_REFW::AUXIO25 => 2,
            COMPA_REFW::AUXIO26 => 1,
            COMPA_REFW::NC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPA_REFW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPA_REFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPA_REFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO19)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO20)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO21)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO22)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO23)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO24)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO25)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO26)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPA_REFW::NC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn compa_ref(&self) -> COMPA_REFR {
        COMPA_REFR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn compa_ref(&mut self) -> _COMPA_REFW {
        _COMPA_REFW { w: self }
    }
}
