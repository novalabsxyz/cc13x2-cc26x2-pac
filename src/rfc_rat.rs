#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Radio Timer Counter Value"]
    pub ratcnt: RATCNT,
}
#[doc = "Radio Timer Counter Value"]
pub struct RATCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio Timer Counter Value"]
pub mod ratcnt;
