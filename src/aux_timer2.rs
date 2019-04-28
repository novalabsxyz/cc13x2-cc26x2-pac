#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Target User defined counter target."]
    pub target: TARGET,
    #[doc = "0x08 - Shadow Target"]
    pub shdwtarget: SHDWTARGET,
    #[doc = "0x0c - Counter"]
    pub cntr: CNTR,
    #[doc = "0x10 - Clock Prescaler Configuration"]
    pub precfg: PRECFG,
    #[doc = "0x14 - Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    pub evctl: EVCTL,
    #[doc = "0x18 - Pulse Trigger"]
    pub pulsetrig: PULSETRIG,
    _reserved0: [u8; 100usize],
    #[doc = "0x80 - Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch0evcfg: CH0EVCFG,
    #[doc = "0x84 - Channel 0 Capture Configuration"]
    pub ch0ccfg: CH0CCFG,
    #[doc = "0x88 - Channel 0 Pipeline Capture Compare"]
    pub ch0pcc: CH0PCC,
    #[doc = "0x8c - Channel 0 Capture Compare"]
    pub ch0cc: CH0CC,
    #[doc = "0x90 - Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch1evcfg: CH1EVCFG,
    #[doc = "0x94 - Channel 1 Capture Configuration"]
    pub ch1ccfg: CH1CCFG,
    #[doc = "0x98 - Channel 1 Pipeline Capture Compare"]
    pub ch1pcc: CH1PCC,
    #[doc = "0x9c - Channel 1 Capture Compare"]
    pub ch1cc: CH1CC,
    #[doc = "0xa0 - Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch2evcfg: CH2EVCFG,
    #[doc = "0xa4 - Channel 2 Capture Configuration"]
    pub ch2ccfg: CH2CCFG,
    #[doc = "0xa8 - Channel 2 Pipeline Capture Compare"]
    pub ch2pcc: CH2PCC,
    #[doc = "0xac - Channel 2 Capture Compare"]
    pub ch2cc: CH2CC,
    #[doc = "0xb0 - Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch3evcfg: CH3EVCFG,
    #[doc = "0xb4 - Channel 3 Capture Configuration"]
    pub ch3ccfg: CH3CCFG,
    #[doc = "0xb8 - Channel 3 Pipeline Capture Compare"]
    pub ch3pcc: CH3PCC,
    #[doc = "0xbc - Channel 3 Capture Compare"]
    pub ch3cc: CH3CC,
}
#[doc = "Timer Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control"]
pub mod ctl;
#[doc = "Target User defined counter target."]
pub struct TARGET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target User defined counter target."]
pub mod target;
#[doc = "Shadow Target"]
pub struct SHDWTARGET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Target"]
pub mod shdwtarget;
#[doc = "Counter"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter"]
pub mod cntr;
#[doc = "Clock Prescaler Configuration"]
pub struct PRECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Prescaler Configuration"]
pub mod precfg;
#[doc = "Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
pub struct EVCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
pub mod evctl;
#[doc = "Pulse Trigger"]
pub struct PULSETRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Trigger"]
pub mod pulsetrig;
#[doc = "Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub struct CH0EVCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch0evcfg;
#[doc = "Channel 0 Capture Configuration"]
pub struct CH0CCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Capture Configuration"]
pub mod ch0ccfg;
#[doc = "Channel 0 Pipeline Capture Compare"]
pub struct CH0PCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Pipeline Capture Compare"]
pub mod ch0pcc;
#[doc = "Channel 0 Capture Compare"]
pub struct CH0CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Capture Compare"]
pub mod ch0cc;
#[doc = "Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub struct CH1EVCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch1evcfg;
#[doc = "Channel 1 Capture Configuration"]
pub struct CH1CCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 Capture Configuration"]
pub mod ch1ccfg;
#[doc = "Channel 1 Pipeline Capture Compare"]
pub struct CH1PCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 Pipeline Capture Compare"]
pub mod ch1pcc;
#[doc = "Channel 1 Capture Compare"]
pub struct CH1CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 Capture Compare"]
pub mod ch1cc;
#[doc = "Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub struct CH2EVCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch2evcfg;
#[doc = "Channel 2 Capture Configuration"]
pub struct CH2CCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 Capture Configuration"]
pub mod ch2ccfg;
#[doc = "Channel 2 Pipeline Capture Compare"]
pub struct CH2PCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 Pipeline Capture Compare"]
pub mod ch2pcc;
#[doc = "Channel 2 Capture Compare"]
pub struct CH2CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 Capture Compare"]
pub mod ch2cc;
#[doc = "Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub struct CH3EVCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch3evcfg;
#[doc = "Channel 3 Capture Configuration"]
pub struct CH3CCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 Capture Configuration"]
pub mod ch3ccfg;
#[doc = "Channel 3 Pipeline Capture Compare"]
pub struct CH3PCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 Pipeline Capture Compare"]
pub mod ch3pcc;
#[doc = "Channel 3 Capture Compare"]
pub struct CH3CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 Capture Compare"]
pub mod ch3cc;
