#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
    pub opmodereq: OPMODEREQ,
    #[doc = "0x04 - Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged."]
    pub opmodeack: OPMODEACK,
    #[doc = "0x08 - Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu0cfg: PROGWU0CFG,
    #[doc = "0x0c - Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu1cfg: PROGWU1CFG,
    #[doc = "0x10 - Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu2cfg: PROGWU2CFG,
    #[doc = "0x14 - Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu3cfg: PROGWU3CFG,
    #[doc = "0x18 - Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn."]
    pub swwutrig: SWWUTRIG,
    #[doc = "0x1c - Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again."]
    pub wuflags: WUFLAGS,
    #[doc = "0x20 - Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup."]
    pub wuflagsclr: WUFLAGSCLR,
    #[doc = "0x24 - Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\] or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration."]
    pub wugate: WUGATE,
    #[doc = "0x28 - Vector Configuration 0 AUX_SCE wakeup vector 0 configuration"]
    pub veccfg0: VECCFG0,
    #[doc = "0x2c - Vector Configuration 1 AUX_SCE wakeup vector 1 configuration"]
    pub veccfg1: VECCFG1,
    #[doc = "0x30 - Vector Configuration 2 AUX_SCE wakeup vector 2 configuration"]
    pub veccfg2: VECCFG2,
    #[doc = "0x34 - Vector Configuration 3 AUX_SCE wakeup vector 3 configuration"]
    pub veccfg3: VECCFG3,
    #[doc = "0x38 - Vector Configuration 4 AUX_SCE wakeup vector 4 configuration"]
    pub veccfg4: VECCFG4,
    #[doc = "0x3c - Vector Configuration 5 AUX_SCE wakeup vector 5 configuration"]
    pub veccfg5: VECCFG5,
    #[doc = "0x40 - Vector Configuration 6 AUX_SCE wakeup vector 6 configuration"]
    pub veccfg6: VECCFG6,
    #[doc = "0x44 - Vector Configuration 7 AUX_SCE wakeup vector 7 configuration"]
    pub veccfg7: VECCFG7,
    #[doc = "0x48 - Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
    pub evsyncrate: EVSYNCRATE,
    #[doc = "0x4c - Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
    pub peroprate: PEROPRATE,
    #[doc = "0x50 - ADC Clock Control"]
    pub adcclkctl: ADCCLKCTL,
    #[doc = "0x54 - TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. TDC counter clock source is configured in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL."]
    pub tdcclkctl: TDCCLKCTL,
    #[doc = "0x58 - TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. This clock is compared against the AUX_TDC counter clock. TDC reference clock source is configured in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL."]
    pub tdcrefclkctl: TDCREFCLKCTL,
    #[doc = "0x5c - AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE."]
    pub timer2clkctl: TIMER2CLKCTL,
    #[doc = "0x60 - AUX_TIMER2 Clock Status"]
    pub timer2clkstat: TIMER2CLKSTAT,
    #[doc = "0x64 - AUX_TIMER2 Clock Switch"]
    pub timer2clkswitch: TIMER2CLKSWITCH,
    #[doc = "0x68 - AUX_TIMER2 Debug Control"]
    pub timer2dbgctl: TIMER2DBGCTL,
    _reserved0: [u8; 4usize],
    #[doc = "0x70 - Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT."]
    pub clkshiftdet: CLKSHIFTDET,
    #[doc = "0x74 - VDDR Recharge Trigger"]
    pub rechargetrig: RECHARGETRIG,
    #[doc = "0x78 - VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs."]
    pub rechargedet: RECHARGEDET,
    #[doc = "0x7c - Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
    pub rtcsubsecinc0: RTCSUBSECINC0,
    #[doc = "0x80 - Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
    pub rtcsubsecinc1: RTCSUBSECINC1,
    #[doc = "0x84 - Real Time Counter Sub Second Increment Control"]
    pub rtcsubsecincctl: RTCSUBSECINCCTL,
    #[doc = "0x88 - Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly."]
    pub rtcsec: RTCSEC,
    #[doc = "0x8c - Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly."]
    pub rtcsubsec: RTCSUBSEC,
    #[doc = "0x90 - AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
    pub rtcevclr: RTCEVCLR,
    #[doc = "0x94 - AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode."]
    pub batmonbat: BATMONBAT,
    _reserved1: [u8; 4usize],
    #[doc = "0x9c - AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode."]
    pub batmontemp: BATMONTEMP,
    #[doc = "0xa0 - Timer Halt Debug register"]
    pub timerhalt: TIMERHALT,
    _reserved2: [u8; 12usize],
    #[doc = "0xb0 - AUX_TIMER2 Bridge"]
    pub timer2bridge: TIMER2BRIDGE,
    #[doc = "0xb4 - Software Power Profiler"]
    pub swpwrprof: SWPWRPROF,
}
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
pub struct OPMODEREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
pub mod opmodereq;
#[doc = "Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged."]
pub struct OPMODEACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged."]
pub mod opmodeack;
#[doc = "Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub struct PROGWU0CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu0cfg;
#[doc = "Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub struct PROGWU1CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu1cfg;
#[doc = "Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub struct PROGWU2CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu2cfg;
#[doc = "Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub struct PROGWU3CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu3cfg;
#[doc = "Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn."]
pub struct SWWUTRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn."]
pub mod swwutrig;
#[doc = "Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again."]
pub struct WUFLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again."]
pub mod wuflags;
#[doc = "Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup."]
pub struct WUFLAGSCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup."]
pub mod wuflagsclr;
#[doc = "Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\] or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration."]
pub struct WUGATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\] or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration."]
pub mod wugate;
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 configuration"]
pub struct VECCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 configuration"]
pub mod veccfg0;
#[doc = "Vector Configuration 1 AUX_SCE wakeup vector 1 configuration"]
pub struct VECCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Configuration 1 AUX_SCE wakeup vector 1 configuration"]
pub mod veccfg1;
#[doc = "Vector Configuration 2 AUX_SCE wakeup vector 2 configuration"]
pub struct VECCFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Configuration 2 AUX_SCE wakeup vector 2 configuration"]
pub mod veccfg2;
#[doc = "Vector Configuration 3 AUX_SCE wakeup vector 3 configuration"]
pub struct VECCFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Configuration 3 AUX_SCE wakeup vector 3 configuration"]
pub mod veccfg3;
#[doc = "Vector Configuration 4 AUX_SCE wakeup vector 4 configuration"]
pub struct VECCFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Configuration 4 AUX_SCE wakeup vector 4 configuration"]
pub mod veccfg4;
#[doc = "Vector Configuration 5 AUX_SCE wakeup vector 5 configuration"]
pub struct VECCFG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Configuration 5 AUX_SCE wakeup vector 5 configuration"]
pub mod veccfg5;
#[doc = "Vector Configuration 6 AUX_SCE wakeup vector 6 configuration"]
pub struct VECCFG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Configuration 6 AUX_SCE wakeup vector 6 configuration"]
pub mod veccfg6;
#[doc = "Vector Configuration 7 AUX_SCE wakeup vector 7 configuration"]
pub struct VECCFG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Configuration 7 AUX_SCE wakeup vector 7 configuration"]
pub mod veccfg7;
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
pub struct EVSYNCRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
pub mod evsyncrate;
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
pub struct PEROPRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
pub mod peroprate;
#[doc = "ADC Clock Control"]
pub struct ADCCLKCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Clock Control"]
pub mod adcclkctl;
#[doc = "TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. TDC counter clock source is configured in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL."]
pub struct TDCCLKCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. TDC counter clock source is configured in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL."]
pub mod tdcclkctl;
#[doc = "TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. This clock is compared against the AUX_TDC counter clock. TDC reference clock source is configured in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL."]
pub struct TDCREFCLKCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. This clock is compared against the AUX_TDC counter clock. TDC reference clock source is configured in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL."]
pub mod tdcrefclkctl;
#[doc = "AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE."]
pub struct TIMER2CLKCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE."]
pub mod timer2clkctl;
#[doc = "AUX_TIMER2 Clock Status"]
pub struct TIMER2CLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX_TIMER2 Clock Status"]
pub mod timer2clkstat;
#[doc = "AUX_TIMER2 Clock Switch"]
pub struct TIMER2CLKSWITCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX_TIMER2 Clock Switch"]
pub mod timer2clkswitch;
#[doc = "AUX_TIMER2 Debug Control"]
pub struct TIMER2DBGCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX_TIMER2 Debug Control"]
pub mod timer2dbgctl;
#[doc = "Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT."]
pub struct CLKSHIFTDET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT."]
pub mod clkshiftdet;
#[doc = "VDDR Recharge Trigger"]
pub struct RECHARGETRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VDDR Recharge Trigger"]
pub mod rechargetrig;
#[doc = "VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs."]
pub struct RECHARGEDET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs."]
pub mod rechargedet;
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub struct RTCSUBSECINC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub mod rtcsubsecinc0;
#[doc = "Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub struct RTCSUBSECINC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub mod rtcsubsecinc1;
#[doc = "Real Time Counter Sub Second Increment Control"]
pub struct RTCSUBSECINCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Real Time Counter Sub Second Increment Control"]
pub mod rtcsubsecincctl;
#[doc = "Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly."]
pub struct RTCSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly."]
pub mod rtcsec;
#[doc = "Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly."]
pub struct RTCSUBSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly."]
pub mod rtcsubsec;
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
pub struct RTCEVCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
pub mod rtcevclr;
#[doc = "AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode."]
pub struct BATMONBAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode."]
pub mod batmonbat;
#[doc = "AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode."]
pub struct BATMONTEMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode."]
pub mod batmontemp;
#[doc = "Timer Halt Debug register"]
pub struct TIMERHALT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Halt Debug register"]
pub mod timerhalt;
#[doc = "AUX_TIMER2 Bridge"]
pub struct TIMER2BRIDGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX_TIMER2 Bridge"]
pub mod timer2bridge;
#[doc = "Software Power Profiler"]
pub struct SWPWRPROF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Power Profiler"]
pub mod swpwrprof;
