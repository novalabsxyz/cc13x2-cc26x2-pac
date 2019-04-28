#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Master Configuration Write operation stalls until current transfer completes."]
    pub spimcfg: SPIMCFG,
    #[doc = "0x04 - MISO Configuration Write operation stalls until current transfer completes."]
    pub misocfg: MISOCFG,
    #[doc = "0x08 - MOSI Control Write operation stalls until current transfer completes."]
    pub mosictl: MOSICTL,
    #[doc = "0x0c - Transmit 8 Bit Write operation stalls until current transfer completes."]
    pub tx8: TX8,
    #[doc = "0x10 - Transmit 16 Bit Write operation stalls until current transfer completes."]
    pub tx16: TX16,
    #[doc = "0x14 - Receive 8 Bit Read operation stalls until current transfer completes."]
    pub rx8: RX8,
    #[doc = "0x18 - Receive 16 Bit Read operation stalls until current transfer completes."]
    pub rx16: RX16,
    #[doc = "0x1c - SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges."]
    pub sclkidle: SCLKIDLE,
    #[doc = "0x20 - Data Idle Read operation stalls until current transfer completes."]
    pub dataidle: DATAIDLE,
}
#[doc = "SPI Master Configuration Write operation stalls until current transfer completes."]
pub struct SPIMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master Configuration Write operation stalls until current transfer completes."]
pub mod spimcfg;
#[doc = "MISO Configuration Write operation stalls until current transfer completes."]
pub struct MISOCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MISO Configuration Write operation stalls until current transfer completes."]
pub mod misocfg;
#[doc = "MOSI Control Write operation stalls until current transfer completes."]
pub struct MOSICTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MOSI Control Write operation stalls until current transfer completes."]
pub mod mosictl;
#[doc = "Transmit 8 Bit Write operation stalls until current transfer completes."]
pub struct TX8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit 8 Bit Write operation stalls until current transfer completes."]
pub mod tx8;
#[doc = "Transmit 16 Bit Write operation stalls until current transfer completes."]
pub struct TX16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit 16 Bit Write operation stalls until current transfer completes."]
pub mod tx16;
#[doc = "Receive 8 Bit Read operation stalls until current transfer completes."]
pub struct RX8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive 8 Bit Read operation stalls until current transfer completes."]
pub mod rx8;
#[doc = "Receive 16 Bit Read operation stalls until current transfer completes."]
pub struct RX16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive 16 Bit Read operation stalls until current transfer completes."]
pub mod rx16;
#[doc = "SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges."]
pub struct SCLKIDLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges."]
pub mod sclkidle;
#[doc = "Data Idle Read operation stalls until current transfer completes."]
pub struct DATAIDLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Idle Read operation stalls until current transfer completes."]
pub mod dataidle;
