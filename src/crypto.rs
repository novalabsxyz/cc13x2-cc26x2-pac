#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmach0ctl: DMACH0CTL,
    #[doc = "0x04 - Channel 0 External Address"]
    pub dmach0extaddr: DMACH0EXTADDR,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Channel 0 DMA Length"]
    pub dmach0len: DMACH0LEN,
    _reserved1: [u8; 8usize],
    #[doc = "0x18 - DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
    pub dmastat: DMASTAT,
    #[doc = "0x1c - DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT."]
    pub dmaswreset: DMASWRESET,
    #[doc = "0x20 - Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmach1ctl: DMACH1CTL,
    #[doc = "0x24 - Channel 1 External Address"]
    pub dmach1extaddr: DMACH1EXTADDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x2c - Channel 1 DMA Length"]
    pub dmach1len: DMACH1LEN,
    _reserved3: [u8; 72usize],
    #[doc = "0x78 - DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
    pub dmabuscfg: DMABUSCFG,
    #[doc = "0x7c - DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register."]
    pub dmaporterr: DMAPORTERR,
    _reserved4: [u8; 124usize],
    #[doc = "0xfc - DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
    pub dmahwver: DMAHWVER,
    _reserved5: [u8; 768usize],
    #[doc = "0x400 - Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
    pub keywritearea: KEYWRITEAREA,
    #[doc = "0x404 - Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
    pub keywrittenarea: KEYWRITTENAREA,
    #[doc = "0x408 - Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
    pub keysize: KEYSIZE,
    #[doc = "0x40c - Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
    pub keyreadarea: KEYREADAREA,
    _reserved6: [u8; 240usize],
    #[doc = "0x500 - AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aeskey2: AESKEY2,
    _reserved7: [u8; 12usize],
    #[doc = "0x510 - AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aeskey3: AESKEY3,
    _reserved8: [u8; 44usize],
    #[doc = "0x540 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aesiv: AESIV,
    _reserved9: [u8; 12usize],
    #[doc = "0x550 - AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\] of this register are all 0."]
    pub aesctl: AESCTL,
    #[doc = "0x554 - AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aesdatalen0: AESDATALEN0,
    #[doc = "0x558 - AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aesdatalen1: AESDATALEN1,
    #[doc = "0x55c - AES Authentication Length"]
    pub aesauthlen: AESAUTHLEN,
    #[doc = "0x560 - Data Input/Output"]
    pub aesdataout0: AESDATAOUT0,
    #[doc = "0x564 - Data Input/Output"]
    pub aesdataout1: AESDATAOUT1,
    #[doc = "0x568 - Data Input/Output"]
    pub aesdataout2: AESDATAOUT2,
    #[doc = "0x56c - Data Input/Output"]
    pub aesdataout3: AESDATAOUT3,
    #[doc = "0x570 - AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
    pub aestagout: AESTAGOUT,
    _reserved10: [u8; 144usize],
    #[doc = "0x604 - HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain1: HASHDATAIN1,
    #[doc = "0x608 - HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain2: HASHDATAIN2,
    #[doc = "0x60c - HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain3: HASHDATAIN3,
    #[doc = "0x610 - HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain4: HASHDATAIN4,
    #[doc = "0x614 - HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain5: HASHDATAIN5,
    #[doc = "0x618 - HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain6: HASHDATAIN6,
    #[doc = "0x61c - HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain7: HASHDATAIN7,
    #[doc = "0x620 - HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain8: HASHDATAIN8,
    #[doc = "0x624 - HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain9: HASHDATAIN9,
    #[doc = "0x628 - HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain10: HASHDATAIN10,
    #[doc = "0x62c - HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain11: HASHDATAIN11,
    #[doc = "0x630 - HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain12: HASHDATAIN12,
    #[doc = "0x634 - HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain13: HASHDATAIN13,
    #[doc = "0x638 - HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain14: HASHDATAIN14,
    #[doc = "0x63c - HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain15: HASHDATAIN15,
    #[doc = "0x640 - HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain16: HASHDATAIN16,
    #[doc = "0x644 - HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain17: HASHDATAIN17,
    #[doc = "0x648 - HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain18: HASHDATAIN18,
    #[doc = "0x64c - HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain19: HASHDATAIN19,
    #[doc = "0x650 - HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain20: HASHDATAIN20,
    #[doc = "0x654 - HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain21: HASHDATAIN21,
    #[doc = "0x658 - HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain22: HASHDATAIN22,
    #[doc = "0x65c - HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain23: HASHDATAIN23,
    #[doc = "0x660 - HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain24: HASHDATAIN24,
    #[doc = "0x664 - HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain25: HASHDATAIN25,
    #[doc = "0x668 - HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain26: HASHDATAIN26,
    #[doc = "0x66c - HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain27: HASHDATAIN27,
    #[doc = "0x670 - HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain28: HASHDATAIN28,
    #[doc = "0x674 - HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain29: HASHDATAIN29,
    #[doc = "0x678 - HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain30: HASHDATAIN30,
    #[doc = "0x67c - HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain31: HASHDATAIN31,
    #[doc = "0x680 - HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
    pub hashiobufctrl: HASHIOBUFCTRL,
    #[doc = "0x684 - HASH Mode"]
    pub hashmode: HASHMODE,
    #[doc = "0x688 - HASH Input Length LSB"]
    pub hashinlenl: HASHINLENL,
    #[doc = "0x68c - HASH Input Length MSB"]
    pub hashinlenh: HASHINLENH,
    _reserved11: [u8; 48usize],
    #[doc = "0x6c0 - HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesta: HASHDIGESTA,
    #[doc = "0x6c4 - HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestb: HASHDIGESTB,
    #[doc = "0x6c8 - HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestc: HASHDIGESTC,
    #[doc = "0x6cc - HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestd: HASHDIGESTD,
    #[doc = "0x6d0 - HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigeste: HASHDIGESTE,
    #[doc = "0x6d4 - HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestf: HASHDIGESTF,
    #[doc = "0x6d8 - HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestg: HASHDIGESTG,
    #[doc = "0x6dc - HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesth: HASHDIGESTH,
    #[doc = "0x6e0 - HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesti: HASHDIGESTI,
    #[doc = "0x6e4 - HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestj: HASHDIGESTJ,
    #[doc = "0x6e8 - HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestk: HASHDIGESTK,
    #[doc = "0x6ec - HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestl: HASHDIGESTL,
    #[doc = "0x6f0 - HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestm: HASHDIGESTM,
    #[doc = "0x6f4 - HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestn: HASHDIGESTN,
    #[doc = "0x6f8 - HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesto: HASHDIGESTO,
    #[doc = "0x6fc - HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestp: HASHDIGESTP,
    #[doc = "0x700 - Algorithm Select This algorithm selection register configures the internal destination of the DMA controller."]
    pub algsel: ALGSEL,
    #[doc = "0x704 - DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
    pub dmaprotctl: DMAPROTCTL,
    _reserved12: [u8; 56usize],
    #[doc = "0x740 - Software Reset"]
    pub swreset: SWRESET,
    _reserved13: [u8; 60usize],
    #[doc = "0x780 - Control Interrupt Configuration"]
    pub irqtype: IRQTYPE,
    #[doc = "0x784 - Control Interrupt Enable"]
    pub irqen: IRQEN,
    #[doc = "0x788 - Control Interrupt Clear"]
    pub irqclr: IRQCLR,
    #[doc = "0x78c - Control Interrupt Set"]
    pub irqset: IRQSET,
    #[doc = "0x790 - Control Interrupt Status"]
    pub irqstat: IRQSTAT,
    _reserved14: [u8; 104usize],
    #[doc = "0x7fc - Hardware Version"]
    pub hwver: HWVER,
}
#[doc = "Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub struct DMACH0CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmach0ctl;
#[doc = "Channel 0 External Address"]
pub struct DMACH0EXTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 External Address"]
pub mod dmach0extaddr;
#[doc = "Channel 0 DMA Length"]
pub struct DMACH0LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 DMA Length"]
pub mod dmach0len;
#[doc = "DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
pub struct DMASTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
pub mod dmastat;
#[doc = "DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT."]
pub struct DMASWRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT."]
pub mod dmaswreset;
#[doc = "Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub struct DMACH1CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmach1ctl;
#[doc = "Channel 1 External Address"]
pub struct DMACH1EXTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 External Address"]
pub mod dmach1extaddr;
#[doc = "Channel 1 DMA Length"]
pub struct DMACH1LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 DMA Length"]
pub mod dmach1len;
#[doc = "DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
pub struct DMABUSCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
pub mod dmabuscfg;
#[doc = "DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register."]
pub struct DMAPORTERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register."]
pub mod dmaporterr;
#[doc = "DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
pub struct DMAHWVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
pub mod dmahwver;
#[doc = "Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
pub struct KEYWRITEAREA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
pub mod keywritearea;
#[doc = "Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
pub struct KEYWRITTENAREA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
pub mod keywrittenarea;
#[doc = "Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
pub struct KEYSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
pub mod keysize;
#[doc = "Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
pub struct KEYREADAREA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
pub mod keyreadarea;
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub struct AESKEY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aeskey2;
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub struct AESKEY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aeskey3;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub struct AESIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aesiv;
#[doc = "AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\] of this register are all 0."]
pub struct AESCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\] of this register are all 0."]
pub mod aesctl;
#[doc = "AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub struct AESDATALEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aesdatalen0;
#[doc = "AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub struct AESDATALEN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aesdatalen1;
#[doc = "AES Authentication Length"]
pub struct AESAUTHLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Authentication Length"]
pub mod aesauthlen;
#[doc = "Data Input/Output"]
pub struct AESDATAOUT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input/Output"]
pub mod aesdataout0;
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub struct AESDATAIN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain0;
#[doc = "Data Input/Output"]
pub struct AESDATAOUT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input/Output"]
pub mod aesdataout1;
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub struct AESDATAIN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain1;
#[doc = "Data Input/Output"]
pub struct AESDATAOUT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input/Output"]
pub mod aesdataout2;
#[doc = "AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub struct AESDATAIN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain2;
#[doc = "Data Input/Output"]
pub struct AESDATAOUT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input/Output"]
pub mod aesdataout3;
#[doc = "AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub struct AESDATAIN3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain3;
#[doc = "AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
pub struct AESTAGOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
pub mod aestagout;
#[doc = "HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain1;
#[doc = "HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain2;
#[doc = "HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain3;
#[doc = "HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain4;
#[doc = "HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain5;
#[doc = "HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain6;
#[doc = "HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain7;
#[doc = "HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain8;
#[doc = "HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain9;
#[doc = "HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain10;
#[doc = "HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain11;
#[doc = "HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain12;
#[doc = "HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain13;
#[doc = "HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain14;
#[doc = "HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain15;
#[doc = "HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain16;
#[doc = "HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain17;
#[doc = "HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain18;
#[doc = "HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain19;
#[doc = "HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain20;
#[doc = "HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain21;
#[doc = "HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain22;
#[doc = "HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain23;
#[doc = "HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain24;
#[doc = "HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain25;
#[doc = "HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain26;
#[doc = "HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain27;
#[doc = "HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain28;
#[doc = "HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain29;
#[doc = "HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain30;
#[doc = "HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub struct HASHDATAIN31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain31;
#[doc = "HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
pub struct HASHIOBUFCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
pub mod hashiobufctrl;
#[doc = "HASH Mode"]
pub struct HASHMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Mode"]
pub mod hashmode;
#[doc = "HASH Input Length LSB"]
pub struct HASHINLENL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Input Length LSB"]
pub mod hashinlenl;
#[doc = "HASH Input Length MSB"]
pub struct HASHINLENH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Input Length MSB"]
pub mod hashinlenh;
#[doc = "HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesta;
#[doc = "HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestb;
#[doc = "HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestc;
#[doc = "HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestd;
#[doc = "HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigeste;
#[doc = "HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestf;
#[doc = "HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestg;
#[doc = "HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesth;
#[doc = "HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesti;
#[doc = "HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTJ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestj;
#[doc = "HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestk;
#[doc = "HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestl;
#[doc = "HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestm;
#[doc = "HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestn;
#[doc = "HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesto;
#[doc = "HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub struct HASHDIGESTP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestp;
#[doc = "Algorithm Select This algorithm selection register configures the internal destination of the DMA controller."]
pub struct ALGSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Algorithm Select This algorithm selection register configures the internal destination of the DMA controller."]
pub mod algsel;
#[doc = "DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
pub struct DMAPROTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
pub mod dmaprotctl;
#[doc = "Software Reset"]
pub struct SWRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "Control Interrupt Configuration"]
pub struct IRQTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Interrupt Configuration"]
pub mod irqtype;
#[doc = "Control Interrupt Enable"]
pub struct IRQEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Interrupt Enable"]
pub mod irqen;
#[doc = "Control Interrupt Clear"]
pub struct IRQCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Interrupt Clear"]
pub mod irqclr;
#[doc = "Control Interrupt Set"]
pub struct IRQSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Interrupt Set"]
pub mod irqset;
#[doc = "Control Interrupt Status"]
pub struct IRQSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Interrupt Status"]
pub mod irqstat;
#[doc = "Hardware Version"]
pub struct HWVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware Version"]
pub mod hwver;
