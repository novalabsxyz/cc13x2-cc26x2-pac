#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Parity Error Control Parity error check controls"]
    pub per_ctl: PER_CTL,
    #[doc = "0x04 - Parity Error Check Parity error check results"]
    pub per_chk: PER_CHK,
    #[doc = "0x08 - Parity Error Debug Parity error check debug address setting"]
    pub per_dbg: PER_DBG,
    #[doc = "0x0c - Memory Control Controls memory initialization"]
    pub mem_ctl: MEM_CTL,
}
#[doc = "Parity Error Control Parity error check controls"]
pub struct PER_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Control Parity error check controls"]
pub mod per_ctl;
#[doc = "Parity Error Check Parity error check results"]
pub struct PER_CHK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Check Parity error check results"]
pub mod per_chk;
#[doc = "Parity Error Debug Parity error check debug address setting"]
pub struct PER_DBG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Debug Parity error check debug address setting"]
pub mod per_dbg;
#[doc = "Memory Control Controls memory initialization"]
pub struct MEM_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Control Controls memory initialization"]
pub mod mem_ctl;
