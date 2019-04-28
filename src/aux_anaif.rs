#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
    pub adcctl: ADCCTL,
    #[doc = "0x14 - ADC FIFO Status FIFO can hold up to four ADC samples."]
    pub adcfifostat: ADCFIFOSTAT,
    #[doc = "0x18 - ADC FIFO"]
    pub adcfifo: ADCFIFO,
    #[doc = "0x1c - ADC Trigger"]
    pub adctrig: ADCTRIG,
    #[doc = "0x20 - Current Source Control"]
    pub isrcctl: ISRCCTL,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - DAC Control This register controls the analog part of the DAC."]
    pub dacctl: DACCTL,
    #[doc = "0x34 - Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A."]
    pub lpmbiasctl: LPMBIASCTL,
    #[doc = "0x38 - DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform."]
    pub dacsmplctl: DACSMPLCTL,
    #[doc = "0x3c - DAC Sample Configuration 0"]
    pub dacsmplcfg0: DACSMPLCFG0,
    #[doc = "0x40 - DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) < 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0."]
    pub dacsmplcfg1: DACSMPLCFG1,
    #[doc = "0x44 - DAC Value"]
    pub dacvalue: DACVALUE,
    #[doc = "0x48 - DAC Status"]
    pub dacstat: DACSTAT,
}
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
pub struct ADCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
pub mod adcctl;
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples."]
pub struct ADCFIFOSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples."]
pub mod adcfifostat;
#[doc = "ADC FIFO"]
pub struct ADCFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC FIFO"]
pub mod adcfifo;
#[doc = "ADC Trigger"]
pub struct ADCTRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Trigger"]
pub mod adctrig;
#[doc = "Current Source Control"]
pub struct ISRCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Source Control"]
pub mod isrcctl;
#[doc = "DAC Control This register controls the analog part of the DAC."]
pub struct DACCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Control This register controls the analog part of the DAC."]
pub mod dacctl;
#[doc = "Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A."]
pub struct LPMBIASCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A."]
pub mod lpmbiasctl;
#[doc = "DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform."]
pub struct DACSMPLCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform."]
pub mod dacsmplctl;
#[doc = "DAC Sample Configuration 0"]
pub struct DACSMPLCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Sample Configuration 0"]
pub mod dacsmplcfg0;
#[doc = "DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) < 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0."]
pub struct DACSMPLCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) < 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0."]
pub mod dacsmplcfg1;
#[doc = "DAC Value"]
pub struct DACVALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Value"]
pub mod dacvalue;
#[doc = "DAC Status"]
pub struct DACSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Status"]
pub mod dacstat;
