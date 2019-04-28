#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_0: RESERVED_0,
    _reserved0: [u8; 8100usize],
    #[doc = "0x1fa8 - Extern LF clock configuration"]
    pub ext_lf_clk: EXT_LF_CLK,
    #[doc = "0x1fac - Mode Configuration 1"]
    pub mode_conf_1: MODE_CONF_1,
    #[doc = "0x1fb0 - CCFG Size and Disable Flags"]
    pub size_and_dis_flags: SIZE_AND_DIS_FLAGS,
    #[doc = "0x1fb4 - Mode Configuration 0"]
    pub mode_conf: MODE_CONF,
    #[doc = "0x1fb8 - Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_0: VOLT_LOAD_0,
    #[doc = "0x1fbc - Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_1: VOLT_LOAD_1,
    #[doc = "0x1fc0 - Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
    pub rtc_offset: RTC_OFFSET,
    #[doc = "0x1fc4 - Frequency Offset"]
    pub freq_offset: FREQ_OFFSET,
    #[doc = "0x1fc8 - IEEE MAC Address 0"]
    pub ieee_mac_0: IEEE_MAC_0,
    #[doc = "0x1fcc - IEEE MAC Address 1"]
    pub ieee_mac_1: IEEE_MAC_1,
    #[doc = "0x1fd0 - IEEE BLE Address 0"]
    pub ieee_ble_0: IEEE_BLE_0,
    #[doc = "0x1fd4 - IEEE BLE Address 1"]
    pub ieee_ble_1: IEEE_BLE_1,
    #[doc = "0x1fd8 - Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
    pub bl_config: BL_CONFIG,
    #[doc = "0x1fdc - Erase Configuration"]
    pub erase_conf: ERASE_CONF,
    #[doc = "0x1fe0 - TI Options"]
    pub ccfg_ti_options: CCFG_TI_OPTIONS,
    #[doc = "0x1fe4 - Test Access Points Enable 0"]
    pub ccfg_tap_dap_0: CCFG_TAP_DAP_0,
    #[doc = "0x1fe8 - Test Access Points Enable 1"]
    pub ccfg_tap_dap_1: CCFG_TAP_DAP_1,
    #[doc = "0x1fec - Image Valid"]
    pub image_valid_conf: IMAGE_VALID_CONF,
    #[doc = "0x1ff0 - Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    pub ccfg_prot_31_0: CCFG_PROT_31_0,
    #[doc = "0x1ff4 - Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    pub ccfg_prot_63_32: CCFG_PROT_63_32,
    #[doc = "0x1ff8 - Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
    pub ccfg_prot_95_64: CCFG_PROT_95_64,
    #[doc = "0x1ffc - Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
    pub ccfg_prot_127_96: CCFG_PROT_127_96,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_0;
#[doc = "Extern LF clock configuration"]
pub struct EXT_LF_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extern LF clock configuration"]
pub mod ext_lf_clk;
#[doc = "Mode Configuration 1"]
pub struct MODE_CONF_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Configuration 1"]
pub mod mode_conf_1;
#[doc = "CCFG Size and Disable Flags"]
pub struct SIZE_AND_DIS_FLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCFG Size and Disable Flags"]
pub mod size_and_dis_flags;
#[doc = "Mode Configuration 0"]
pub struct MODE_CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Configuration 0"]
pub mod mode_conf;
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub struct VOLT_LOAD_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_0;
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub struct VOLT_LOAD_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_1;
#[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
pub struct RTC_OFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
pub mod rtc_offset;
#[doc = "Frequency Offset"]
pub struct FREQ_OFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frequency Offset"]
pub mod freq_offset;
#[doc = "IEEE MAC Address 0"]
pub struct IEEE_MAC_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEEE MAC Address 0"]
pub mod ieee_mac_0;
#[doc = "IEEE MAC Address 1"]
pub struct IEEE_MAC_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEEE MAC Address 1"]
pub mod ieee_mac_1;
#[doc = "IEEE BLE Address 0"]
pub struct IEEE_BLE_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEEE BLE Address 0"]
pub mod ieee_ble_0;
#[doc = "IEEE BLE Address 1"]
pub struct IEEE_BLE_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEEE BLE Address 1"]
pub mod ieee_ble_1;
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
pub struct BL_CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
pub mod bl_config;
#[doc = "Erase Configuration"]
pub struct ERASE_CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Erase Configuration"]
pub mod erase_conf;
#[doc = "TI Options"]
pub struct CCFG_TI_OPTIONS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TI Options"]
pub mod ccfg_ti_options;
#[doc = "Test Access Points Enable 0"]
pub struct CCFG_TAP_DAP_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test Access Points Enable 0"]
pub mod ccfg_tap_dap_0;
#[doc = "Test Access Points Enable 1"]
pub struct CCFG_TAP_DAP_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test Access Points Enable 1"]
pub mod ccfg_tap_dap_1;
#[doc = "Image Valid"]
pub struct IMAGE_VALID_CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Image Valid"]
pub mod image_valid_conf;
#[doc = "Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub struct CCFG_PROT_31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_31_0;
#[doc = "Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub struct CCFG_PROT_63_32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_63_32;
#[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
pub struct CCFG_PROT_95_64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
pub mod ccfg_prot_95_64;
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
pub struct CCFG_PROT_127_96 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
pub mod ccfg_prot_127_96;
