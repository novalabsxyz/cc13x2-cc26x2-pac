#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MUX0 {
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
pub struct RESERVED7R {
    bits: bool,
}
impl RESERVED7R {
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
#[doc = "Possible values of the field `ADCCOMPB_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCCOMPB_INR {
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDR_1P8V,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
}
impl ADCCOMPB_INR {
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
            ADCCOMPB_INR::VDDR_1P8V => true,
            ADCCOMPB_INR::NC => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCCOMPB_INR {
        match value {
            true => ADCCOMPB_INR::VDDR_1P8V,
            false => ADCCOMPB_INR::NC,
        }
    }
    #[doc = "Checks if the value of the field is `VDDR_1P8V`"]
    #[inline]
    pub fn is_vddr_1p8v(&self) -> bool {
        *self == ADCCOMPB_INR::VDDR_1P8V
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == ADCCOMPB_INR::NC
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED4R {
    bits: u8,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `COMPA_REF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPA_REFR {
    #[doc = "Internal. Only to be used through TI provided API."]
    ADCVREFP,
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDS,
    #[doc = "Internal. Only to be used through TI provided API."]
    VSS,
    #[doc = "Internal. Only to be used through TI provided API."]
    DCOUPL,
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
            COMPA_REFR::ADCVREFP => 8,
            COMPA_REFR::VDDS => 4,
            COMPA_REFR::VSS => 2,
            COMPA_REFR::DCOUPL => 1,
            COMPA_REFR::NC => 0,
            COMPA_REFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMPA_REFR {
        match value {
            8 => COMPA_REFR::ADCVREFP,
            4 => COMPA_REFR::VDDS,
            2 => COMPA_REFR::VSS,
            1 => COMPA_REFR::DCOUPL,
            0 => COMPA_REFR::NC,
            i => COMPA_REFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCVREFP`"]
    #[inline]
    pub fn is_adcvrefp(&self) -> bool {
        *self == COMPA_REFR::ADCVREFP
    }
    #[doc = "Checks if the value of the field is `VDDS`"]
    #[inline]
    pub fn is_vdds(&self) -> bool {
        *self == COMPA_REFR::VDDS
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline]
    pub fn is_vss(&self) -> bool {
        *self == COMPA_REFR::VSS
    }
    #[doc = "Checks if the value of the field is `DCOUPL`"]
    #[inline]
    pub fn is_dcoupl(&self) -> bool {
        *self == COMPA_REFR::DCOUPL
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == COMPA_REFR::NC
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED7W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED7W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCCOMPB_IN`"]
pub enum ADCCOMPB_INW {
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDR_1P8V,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
}
impl ADCCOMPB_INW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCCOMPB_INW::VDDR_1P8V => true,
            ADCCOMPB_INW::NC => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCCOMPB_INW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCCOMPB_INW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCCOMPB_INW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_1p8v(self) -> &'a mut W {
        self.variant(ADCCOMPB_INW::VDDR_1P8V)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(ADCCOMPB_INW::NC)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPA_REF`"]
pub enum COMPA_REFW {
    #[doc = "Internal. Only to be used through TI provided API."]
    ADCVREFP,
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDS,
    #[doc = "Internal. Only to be used through TI provided API."]
    VSS,
    #[doc = "Internal. Only to be used through TI provided API."]
    DCOUPL,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
}
impl COMPA_REFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMPA_REFW::ADCVREFP => 8,
            COMPA_REFW::VDDS => 4,
            COMPA_REFW::VSS => 2,
            COMPA_REFW::DCOUPL => 1,
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
    pub fn adcvrefp(self) -> &'a mut W {
        self.variant(COMPA_REFW::ADCVREFP)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vdds(self) -> &'a mut W {
        self.variant(COMPA_REFW::VDDS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vss(self) -> &'a mut W {
        self.variant(COMPA_REFW::VSS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcoupl(self) -> &'a mut W {
        self.variant(COMPA_REFW::DCOUPL)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPA_REFW::NC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 7 - 7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved7(&self) -> RESERVED7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RESERVED7R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn adccompb_in(&self) -> ADCCOMPB_INR {
        ADCCOMPB_INR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 4:5 - 5:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        RESERVED4R { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn compa_ref(&self) -> COMPA_REFR {
        COMPA_REFR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bit 7 - 7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved7(&mut self) -> _RESERVED7W {
        _RESERVED7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn adccompb_in(&mut self) -> _ADCCOMPB_INW {
        _ADCCOMPB_INW { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn compa_ref(&mut self) -> _COMPA_REFW {
        _COMPA_REFW { w: self }
    }
}
