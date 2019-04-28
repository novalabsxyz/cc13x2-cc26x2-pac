#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AESDATAIN0 {
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
pub struct AES_DATA_IN_OUTR {
    bits: u32,
}
impl AES_DATA_IN_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _AES_DATA_IN_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_DATA_IN_OUTW<'a> {
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
    #[doc = "Bits 0:31 - 31:0\\] AES input data\\[31:0\\] / AES output data\\[31:0\\] Data registers for input/output block data to/from the EIP-120t. For normal operations, this register is not used, since data input and output is transferred from and to the AES core via DMA. For a host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range stores the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to the input_ready flag of the AES_CTRL register. For a host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range reads one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, the output_ready flag of the AES_CTRL register must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]). For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The EIP-120t automatically pads or masks misaligned ending data blocks with 0s for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline]
    pub fn aes_data_in_out(&self) -> AES_DATA_IN_OUTR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        AES_DATA_IN_OUTR { bits }
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
    #[doc = "Bits 0:31 - 31:0\\] AES input data\\[31:0\\] / AES output data\\[31:0\\] Data registers for input/output block data to/from the EIP-120t. For normal operations, this register is not used, since data input and output is transferred from and to the AES core via DMA. For a host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range stores the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to the input_ready flag of the AES_CTRL register. For a host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range reads one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, the output_ready flag of the AES_CTRL register must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]). For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The EIP-120t automatically pads or masks misaligned ending data blocks with 0s for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline]
    pub fn aes_data_in_out(&mut self) -> _AES_DATA_IN_OUTW {
        _AES_DATA_IN_OUTW { w: self }
    }
}
