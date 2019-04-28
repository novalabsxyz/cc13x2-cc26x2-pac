#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Signed Operand 0"]
    pub op0s: OP0S,
    #[doc = "0x04 - Unsigned Operand 0"]
    pub op0u: OP0U,
    #[doc = "0x08 - Signed Operand 1 and Multiply"]
    pub op1smul: OP1SMUL,
    #[doc = "0x0c - Unsigned Operand 1 and Multiply"]
    pub op1umul: OP1UMUL,
    #[doc = "0x10 - Signed Operand 1 and Multiply-Accumulate"]
    pub op1smac: OP1SMAC,
    #[doc = "0x14 - Unsigned Operand 1 and Multiply-Accumulate"]
    pub op1umac: OP1UMAC,
    #[doc = "0x18 - Signed Operand 1 and 16-bit Addition"]
    pub op1sadd16: OP1SADD16,
    #[doc = "0x1c - Unsigned Operand 1 and 16-bit Addition"]
    pub op1uadd16: OP1UADD16,
    #[doc = "0x20 - Signed Operand 1 and 32-bit Addition"]
    pub op1sadd32: OP1SADD32,
    #[doc = "0x24 - Unsigned Operand 1 and 32-bit Addition"]
    pub op1uadd32: OP1UADD32,
    #[doc = "0x28 - Count Leading Zero"]
    pub clz: CLZ,
    #[doc = "0x2c - Count Leading Sign"]
    pub cls: CLS,
    #[doc = "0x30 - Accumulator Shift Only one shift operation can be triggered per register write."]
    pub accshift: ACCSHIFT,
    #[doc = "0x34 - Accumulator Reset"]
    pub accreset: ACCRESET,
    #[doc = "0x38 - Accumulator Bits 15:0"]
    pub acc15_0: ACC15_0,
    #[doc = "0x3c - Accumulator Bits 16:1"]
    pub acc16_1: ACC16_1,
    #[doc = "0x40 - Accumulator Bits 17:2"]
    pub acc17_2: ACC17_2,
    #[doc = "0x44 - Accumulator Bits 18:3"]
    pub acc18_3: ACC18_3,
    #[doc = "0x48 - Accumulator Bits 19:4"]
    pub acc19_4: ACC19_4,
    #[doc = "0x4c - Accumulator Bits 20:5"]
    pub acc20_5: ACC20_5,
    #[doc = "0x50 - Accumulator Bits 21:6"]
    pub acc21_6: ACC21_6,
    #[doc = "0x54 - Accumulator Bits 22:7"]
    pub acc22_7: ACC22_7,
    #[doc = "0x58 - Accumulator Bits 23:8"]
    pub acc23_8: ACC23_8,
    #[doc = "0x5c - Accumulator Bits 24:9"]
    pub acc24_9: ACC24_9,
    #[doc = "0x60 - Accumulator Bits 25:10"]
    pub acc25_10: ACC25_10,
    #[doc = "0x64 - Accumulator Bits 26:11"]
    pub acc26_11: ACC26_11,
    #[doc = "0x68 - Accumulator Bits 27:12"]
    pub acc27_12: ACC27_12,
    #[doc = "0x6c - Accumulator Bits 28:13"]
    pub acc28_13: ACC28_13,
    #[doc = "0x70 - Accumulator Bits 29:14"]
    pub acc29_14: ACC29_14,
    #[doc = "0x74 - Accumulator Bits 30:15"]
    pub acc30_15: ACC30_15,
    #[doc = "0x78 - Accumulator Bits 31:16"]
    pub acc31_16: ACC31_16,
    #[doc = "0x7c - Accumulator Bits 32:17"]
    pub acc32_17: ACC32_17,
    #[doc = "0x80 - Accumulator Bits 33:18"]
    pub acc33_18: ACC33_18,
    #[doc = "0x84 - Accumulator Bits 34:19"]
    pub acc34_19: ACC34_19,
    #[doc = "0x88 - Accumulator Bits 35:20"]
    pub acc35_20: ACC35_20,
    #[doc = "0x8c - Accumulator Bits 36:21"]
    pub acc36_21: ACC36_21,
    #[doc = "0x90 - Accumulator Bits 37:22"]
    pub acc37_22: ACC37_22,
    #[doc = "0x94 - Accumulator Bits 38:23"]
    pub acc38_23: ACC38_23,
    #[doc = "0x98 - Accumulator Bits 39:24"]
    pub acc39_24: ACC39_24,
    #[doc = "0x9c - Accumulator Bits 39:32"]
    pub acc39_32: ACC39_32,
}
#[doc = "Signed Operand 0"]
pub struct OP0S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signed Operand 0"]
pub mod op0s;
#[doc = "Unsigned Operand 0"]
pub struct OP0U {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unsigned Operand 0"]
pub mod op0u;
#[doc = "Signed Operand 1 and Multiply"]
pub struct OP1SMUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signed Operand 1 and Multiply"]
pub mod op1smul;
#[doc = "Unsigned Operand 1 and Multiply"]
pub struct OP1UMUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unsigned Operand 1 and Multiply"]
pub mod op1umul;
#[doc = "Signed Operand 1 and Multiply-Accumulate"]
pub struct OP1SMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signed Operand 1 and Multiply-Accumulate"]
pub mod op1smac;
#[doc = "Unsigned Operand 1 and Multiply-Accumulate"]
pub struct OP1UMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unsigned Operand 1 and Multiply-Accumulate"]
pub mod op1umac;
#[doc = "Signed Operand 1 and 16-bit Addition"]
pub struct OP1SADD16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signed Operand 1 and 16-bit Addition"]
pub mod op1sadd16;
#[doc = "Unsigned Operand 1 and 16-bit Addition"]
pub struct OP1UADD16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unsigned Operand 1 and 16-bit Addition"]
pub mod op1uadd16;
#[doc = "Signed Operand 1 and 32-bit Addition"]
pub struct OP1SADD32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signed Operand 1 and 32-bit Addition"]
pub mod op1sadd32;
#[doc = "Unsigned Operand 1 and 32-bit Addition"]
pub struct OP1UADD32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unsigned Operand 1 and 32-bit Addition"]
pub mod op1uadd32;
#[doc = "Count Leading Zero"]
pub struct CLZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count Leading Zero"]
pub mod clz;
#[doc = "Count Leading Sign"]
pub struct CLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count Leading Sign"]
pub mod cls;
#[doc = "Accumulator Shift Only one shift operation can be triggered per register write."]
pub struct ACCSHIFT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Shift Only one shift operation can be triggered per register write."]
pub mod accshift;
#[doc = "Accumulator Reset"]
pub struct ACCRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Reset"]
pub mod accreset;
#[doc = "Accumulator Bits 15:0"]
pub struct ACC15_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 15:0"]
pub mod acc15_0;
#[doc = "Accumulator Bits 16:1"]
pub struct ACC16_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 16:1"]
pub mod acc16_1;
#[doc = "Accumulator Bits 17:2"]
pub struct ACC17_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 17:2"]
pub mod acc17_2;
#[doc = "Accumulator Bits 18:3"]
pub struct ACC18_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 18:3"]
pub mod acc18_3;
#[doc = "Accumulator Bits 19:4"]
pub struct ACC19_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 19:4"]
pub mod acc19_4;
#[doc = "Accumulator Bits 20:5"]
pub struct ACC20_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 20:5"]
pub mod acc20_5;
#[doc = "Accumulator Bits 21:6"]
pub struct ACC21_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 21:6"]
pub mod acc21_6;
#[doc = "Accumulator Bits 22:7"]
pub struct ACC22_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 22:7"]
pub mod acc22_7;
#[doc = "Accumulator Bits 23:8"]
pub struct ACC23_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 23:8"]
pub mod acc23_8;
#[doc = "Accumulator Bits 24:9"]
pub struct ACC24_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 24:9"]
pub mod acc24_9;
#[doc = "Accumulator Bits 25:10"]
pub struct ACC25_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 25:10"]
pub mod acc25_10;
#[doc = "Accumulator Bits 26:11"]
pub struct ACC26_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 26:11"]
pub mod acc26_11;
#[doc = "Accumulator Bits 27:12"]
pub struct ACC27_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 27:12"]
pub mod acc27_12;
#[doc = "Accumulator Bits 28:13"]
pub struct ACC28_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 28:13"]
pub mod acc28_13;
#[doc = "Accumulator Bits 29:14"]
pub struct ACC29_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 29:14"]
pub mod acc29_14;
#[doc = "Accumulator Bits 30:15"]
pub struct ACC30_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 30:15"]
pub mod acc30_15;
#[doc = "Accumulator Bits 31:16"]
pub struct ACC31_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 31:16"]
pub mod acc31_16;
#[doc = "Accumulator Bits 32:17"]
pub struct ACC32_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 32:17"]
pub mod acc32_17;
#[doc = "Accumulator Bits 33:18"]
pub struct ACC33_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 33:18"]
pub mod acc33_18;
#[doc = "Accumulator Bits 34:19"]
pub struct ACC34_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 34:19"]
pub mod acc34_19;
#[doc = "Accumulator Bits 35:20"]
pub struct ACC35_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 35:20"]
pub mod acc35_20;
#[doc = "Accumulator Bits 36:21"]
pub struct ACC36_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 36:21"]
pub mod acc36_21;
#[doc = "Accumulator Bits 37:22"]
pub struct ACC37_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 37:22"]
pub mod acc37_22;
#[doc = "Accumulator Bits 38:23"]
pub struct ACC38_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 38:23"]
pub mod acc38_23;
#[doc = "Accumulator Bits 39:24"]
pub struct ACC39_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 39:24"]
pub mod acc39_24;
#[doc = "Accumulator Bits 39:32"]
pub struct ACC39_32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator Bits 39:32"]
pub mod acc39_32;
