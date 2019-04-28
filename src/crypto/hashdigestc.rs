#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HASHDIGESTC {
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
pub struct HASH_DIGESTR {
    bits: u32,
}
impl HASH_DIGESTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _HASH_DIGESTW<'a> {
    w: &'a mut W,
}
impl<'a> _HASH_DIGESTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - 31:0\\] HASH_DIGEST\\[95:64\\] Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline]
    pub fn hash_digest(&self) -> HASH_DIGESTR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        HASH_DIGESTR { bits }
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
    #[doc = "Bits 0:31 - 31:0\\] HASH_DIGEST\\[95:64\\] Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the HASHMODE.NEW_HASH bit is 0 when starting a hash session). New hash: When initiating a new hash session (theHASHMODE.NEW_HASH bit is 1), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline]
    pub fn hash_digest(&mut self) -> _HASH_DIGESTW {
        _HASH_DIGESTW { w: self }
    }
}
