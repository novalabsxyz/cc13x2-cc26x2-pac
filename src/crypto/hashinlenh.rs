#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HASHINLENH {
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
pub struct LENGTH_INR {
    bits: u32,
}
impl LENGTH_INR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LENGTH_INW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTH_INW<'a> {
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
    #[doc = "Bits 0:31 - 31:0\\] LENGTH_IN\\[63:32\\] Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 512-bits which is SHA-256 data block size), the length field does not need to be programmed since not used by the operation. If the message length in bits is below (2^32-1), then only HASHINLENL needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
    #[inline]
    pub fn length_in(&self) -> LENGTH_INR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        LENGTH_INR { bits }
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
    #[doc = "Bits 0:31 - 31:0\\] LENGTH_IN\\[63:32\\] Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 512-bits which is SHA-256 data block size), the length field does not need to be programmed since not used by the operation. If the message length in bits is below (2^32-1), then only HASHINLENL needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
    #[inline]
    pub fn length_in(&mut self) -> _LENGTH_INW {
        _LENGTH_INW { w: self }
    }
}
