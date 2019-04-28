#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub iostrmin: IOSTRMIN,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub iostrmed: IOSTRMED,
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    pub iostrmax: IOSTRMAX,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - SCLK_LF External Output Control"]
    pub clk32kctl: CLK32KCTL,
    #[doc = "0x14 - TCK IO Pin Control"]
    pub tckctl: TCKCTL,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub struct IOSTRMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmin;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct IOSTRMED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmed;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct IOSTRMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmax;
#[doc = "SCLK_LF External Output Control"]
pub struct CLK32KCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCLK_LF External Output Control"]
pub mod clk32kctl;
#[doc = "TCK IO Pin Control"]
pub struct TCKCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCK IO Pin Control"]
pub mod tckctl;
