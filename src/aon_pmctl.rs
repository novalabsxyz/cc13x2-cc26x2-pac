#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
    pub auxsceclk: AUXSCECLK,
    #[doc = "0x08 - RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain."]
    pub ramcfg: RAMCFG,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
    pub pwrctl: PWRCTL,
    #[doc = "0x14 - AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only."]
    pub pwrstat: PWRSTAT,
    #[doc = "0x18 - Shutdown Control This register contains bitfields required for entering shutdown mode"]
    pub shutdown: SHUTDOWN,
    #[doc = "0x1c - Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm."]
    pub rechargecfg: RECHARGECFG,
    #[doc = "0x20 - Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
    pub rechargestat: RECHARGESTAT,
    #[doc = "0x24 - Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
    pub osccfg: OSCCFG,
    #[doc = "0x28 - Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets."]
    pub resetctl: RESETCTL,
    #[doc = "0x2c - Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
    pub sleepctl: SLEEPCTL,
    _reserved2: [u8; 4usize],
    #[doc = "0x34 - JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP."]
    pub jtagcfg: JTAGCFG,
    _reserved3: [u8; 4usize],
    #[doc = "0x3c - JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
    pub jtagusercode: JTAGUSERCODE,
}
#[doc = "AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub struct AUXSCECLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub mod auxsceclk;
#[doc = "RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain."]
pub struct RAMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain."]
pub mod ramcfg;
#[doc = "Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub struct PWRCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub mod pwrctl;
#[doc = "AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only."]
pub struct PWRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only."]
pub mod pwrstat;
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub struct SHUTDOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub mod shutdown;
#[doc = "Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm."]
pub struct RECHARGECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm."]
pub mod rechargecfg;
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
pub struct RECHARGESTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
pub mod rechargestat;
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
pub struct OSCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
pub mod osccfg;
#[doc = "Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets."]
pub struct RESETCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets."]
pub mod resetctl;
#[doc = "Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub struct SLEEPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub mod sleepctl;
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP."]
pub struct JTAGCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP."]
pub mod jtagcfg;
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub struct JTAGUSERCODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub mod jtagusercode;
