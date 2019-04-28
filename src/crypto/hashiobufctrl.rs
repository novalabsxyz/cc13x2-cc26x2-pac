#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HASHIOBUFCTRL {
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
pub struct PAD_DMA_MESSAGER {
    bits: bool,
}
impl PAD_DMA_MESSAGER {
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
pub struct GET_DIGESTR {
    bits: bool,
}
impl GET_DIGESTR {
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
pub struct PAD_MESSAGER {
    bits: bool,
}
impl PAD_MESSAGER {
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
pub struct RESERVED3R {
    bits: u8,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFD_INR {
    bits: bool,
}
impl RFD_INR {
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
pub struct DATA_IN_AVR {
    bits: bool,
}
impl DATA_IN_AVR {
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
pub struct OUTPUT_FULLR {
    bits: bool,
}
impl OUTPUT_FULLR {
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
pub struct _PAD_DMA_MESSAGEW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD_DMA_MESSAGEW<'a> {
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
pub struct _GET_DIGESTW<'a> {
    w: &'a mut W,
}
impl<'a> _GET_DIGESTW<'a> {
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
pub struct _PAD_MESSAGEW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD_MESSAGEW<'a> {
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
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFD_INW<'a> {
    w: &'a mut W,
}
impl<'a> _RFD_INW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_IN_AVW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_IN_AVW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTPUT_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTPUT_FULLW<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
    #[inline]
    pub fn pad_dma_message(&self) -> PAD_DMA_MESSAGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD_DMA_MESSAGER { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASHDATAINn register. When provided without data_in_av, the current internal digest buffer value is copied to the HASHDIGESTn registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASHDATAINn register. In the period between this bit is set by the host and the actual HASHDATAINn processing, this bit reads 1."]
    #[inline]
    pub fn get_digest(&self) -> GET_DIGESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GET_DIGESTR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASHDATAINn register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASHDATAINn register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
    #[inline]
    pub fn pad_message(&self) -> PAD_MESSAGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD_MESSAGER { bits }
    }
    #[doc = "Bits 3:4 - 4:3\\] Write 0s and ignore on reading"]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASHDATAINn registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASHDATAINn; writing new data to these registers is not allowed."]
    #[inline]
    pub fn rfd_in(&self) -> RFD_INR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFD_INR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASHDATAINn; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASHDATAINn contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
    #[inline]
    pub fn data_in_av(&self) -> DATA_IN_AVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATA_IN_AVR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Indicates that the output buffer registers (HASHDIGESTn) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
    #[inline]
    pub fn output_full(&self) -> OUTPUT_FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTPUT_FULLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
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
    #[doc = "Bit 7 - 7:7\\] Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
    #[inline]
    pub fn pad_dma_message(&mut self) -> _PAD_DMA_MESSAGEW {
        _PAD_DMA_MESSAGEW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASHDATAINn register. When provided without data_in_av, the current internal digest buffer value is copied to the HASHDIGESTn registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASHDATAINn register. In the period between this bit is set by the host and the actual HASHDATAINn processing, this bit reads 1."]
    #[inline]
    pub fn get_digest(&mut self) -> _GET_DIGESTW {
        _GET_DIGESTW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASHDATAINn register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASHDATAINn register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
    #[inline]
    pub fn pad_message(&mut self) -> _PAD_MESSAGEW {
        _PAD_MESSAGEW { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\] Write 0s and ignore on reading"]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASHDATAINn registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASHDATAINn; writing new data to these registers is not allowed."]
    #[inline]
    pub fn rfd_in(&mut self) -> _RFD_INW {
        _RFD_INW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASHDATAINn; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASHDATAINn contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
    #[inline]
    pub fn data_in_av(&mut self) -> _DATA_IN_AVW {
        _DATA_IN_AVW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Indicates that the output buffer registers (HASHDIGESTn) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
    #[inline]
    pub fn output_full(&mut self) -> _OUTPUT_FULLW {
        _OUTPUT_FULLW { w: self }
    }
}
