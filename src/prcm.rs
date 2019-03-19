#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Infrastructure Clock Division Factor For Run Mode"]
    pub infrclkdivr: INFRCLKDIVR,
    #[doc = "0x04 - Infrastructure Clock Division Factor For Sleep Mode"]
    pub infrclkdivs: INFRCLKDIVS,
    #[doc = "0x08 - Infrastructure Clock Division Factor For DeepSleep Mode"]
    pub infrclkdivds: INFRCLKDIVDS,
    #[doc = "0x0c - MCU Voltage Domain Control"]
    pub vdctl: VDCTL,
    _reserved4: [u8; 24usize],
    #[doc = "0x28 - Load PRCM Settings To CLKCTRL Power Domain"]
    pub clkloadctl: CLKLOADCTL,
    #[doc = "0x2c - RFC Clock Gate"]
    pub rfcclkg: RFCCLKG,
    #[doc = "0x30 - VIMS Clock Gate"]
    pub vimsclkg: VIMSCLKG,
    _reserved7: [u8; 8usize],
    #[doc = "0x3c - SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes"]
    pub secdmaclkgr: SECDMACLKGR,
    #[doc = "0x40 - SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode"]
    pub secdmaclkgs: SECDMACLKGS,
    #[doc = "0x44 - SEC (PKA And TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode"]
    pub secdmaclkgds: SECDMACLKGDS,
    #[doc = "0x48 - GPIO Clock Gate For Run And All Modes"]
    pub gpioclkgr: GPIOCLKGR,
    #[doc = "0x4c - GPIO Clock Gate For Sleep Mode"]
    pub gpioclkgs: GPIOCLKGS,
    #[doc = "0x50 - GPIO Clock Gate For Deep Sleep Mode"]
    pub gpioclkgds: GPIOCLKGDS,
    #[doc = "0x54 - GPT Clock Gate For Run And All Modes"]
    pub gptclkgr: GPTCLKGR,
    #[doc = "0x58 - GPT Clock Gate For Sleep Mode"]
    pub gptclkgs: GPTCLKGS,
    #[doc = "0x5c - GPT Clock Gate For Deep Sleep Mode"]
    pub gptclkgds: GPTCLKGDS,
    #[doc = "0x60 - I2C Clock Gate For Run And All Modes"]
    pub i2cclkgr: I2CCLKGR,
    #[doc = "0x64 - I2C Clock Gate For Sleep Mode"]
    pub i2cclkgs: I2CCLKGS,
    #[doc = "0x68 - I2C Clock Gate For Deep Sleep Mode"]
    pub i2cclkgds: I2CCLKGDS,
    #[doc = "0x6c - UART Clock Gate For Run And All Modes"]
    pub uartclkgr: UARTCLKGR,
    #[doc = "0x70 - UART Clock Gate For Sleep Mode"]
    pub uartclkgs: UARTCLKGS,
    #[doc = "0x74 - UART Clock Gate For Deep Sleep Mode"]
    pub uartclkgds: UARTCLKGDS,
    #[doc = "0x78 - SSI Clock Gate For Run And All Modes"]
    pub ssiclkgr: SSICLKGR,
    #[doc = "0x7c - SSI Clock Gate For Sleep Mode"]
    pub ssiclkgs: SSICLKGS,
    #[doc = "0x80 - SSI Clock Gate For Deep Sleep Mode"]
    pub ssiclkgds: SSICLKGDS,
    #[doc = "0x84 - I2S Clock Gate For Run And All Modes"]
    pub i2sclkgr: I2SCLKGR,
    #[doc = "0x88 - I2S Clock Gate For Sleep Mode"]
    pub i2sclkgs: I2SCLKGS,
    #[doc = "0x8c - I2S Clock Gate For Deep Sleep Mode"]
    pub i2sclkgds: I2SCLKGDS,
    _reserved28: [u8; 36usize],
    #[doc = "0xb4 - Internal. Only to be used through TI provided API."]
    pub sysbusclkdiv: SYSBUSCLKDIV,
    #[doc = "0xb8 - Internal. Only to be used through TI provided API."]
    pub cpuclkdiv: CPUCLKDIV,
    #[doc = "0xbc - Internal. Only to be used through TI provided API."]
    pub perbuscpuclkdiv: PERBUSCPUCLKDIV,
    #[doc = "0xc0 - Internal. Only to be used through TI provided API."]
    pub perbusdmaclkdiv: PERBUSDMACLKDIV,
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    pub perdmaclkdiv: PERDMACLKDIV,
    #[doc = "0xc8 - I2S Clock Control"]
    pub i2sbclksel: I2SBCLKSEL,
    #[doc = "0xcc - GPT Scalar"]
    pub gptclkdiv: GPTCLKDIV,
    #[doc = "0xd0 - I2S Clock Control"]
    pub i2sclkctl: I2SCLKCTL,
    #[doc = "0xd4 - MCLK Division Ratio"]
    pub i2smclkdiv: I2SMCLKDIV,
    #[doc = "0xd8 - BCLK Division Ratio"]
    pub i2sbclkdiv: I2SBCLKDIV,
    #[doc = "0xdc - WCLK Division Ratio"]
    pub i2swclkdiv: I2SWCLKDIV,
    _reserved39: [u8; 16usize],
    #[doc = "0xf0 - RESET For SEC (PKA And TRNG And CRYPTO) And UDMA"]
    pub resetsecdma: RESETSECDMA,
    #[doc = "0xf4 - RESET For GPIO IPs"]
    pub resetgpio: RESETGPIO,
    #[doc = "0xf8 - RESET For GPT Ips"]
    pub resetgpt: RESETGPT,
    #[doc = "0xfc - RESET For I2C IPs"]
    pub reseti2c: RESETI2C,
    #[doc = "0x100 - RESET For UART IPs"]
    pub resetuart: RESETUART,
    #[doc = "0x104 - RESET For SSI IPs"]
    pub resetssi: RESETSSI,
    #[doc = "0x108 - RESET For I2S IP"]
    pub reseti2s: RESETI2S,
    _reserved46: [u8; 32usize],
    #[doc = "0x12c - Power Domain Control"]
    pub pdctl0: PDCTL0,
    #[doc = "0x130 - RFC Power Domain Control"]
    pub pdctl0rfc: PDCTL0RFC,
    #[doc = "0x134 - SERIAL Power Domain Control"]
    pub pdctl0serial: PDCTL0SERIAL,
    #[doc = "0x138 - PERIPH Power Domain Control"]
    pub pdctl0periph: PDCTL0PERIPH,
    _reserved50: [u8; 4usize],
    #[doc = "0x140 - Power Domain Status"]
    pub pdstat0: PDSTAT0,
    #[doc = "0x144 - RFC Power Domain Status"]
    pub pdstat0rfc: PDSTAT0RFC,
    #[doc = "0x148 - SERIAL Power Domain Status"]
    pub pdstat0serial: PDSTAT0SERIAL,
    #[doc = "0x14c - PERIPH Power Domain Status"]
    pub pdstat0periph: PDSTAT0PERIPH,
    _reserved54: [u8; 44usize],
    #[doc = "0x17c - Power Domain Control"]
    pub pdctl1: PDCTL1,
    _reserved55: [u8; 4usize],
    #[doc = "0x184 - CPU Power Domain Direct Control"]
    pub pdctl1cpu: PDCTL1CPU,
    #[doc = "0x188 - RFC Power Domain Direct Control"]
    pub pdctl1rfc: PDCTL1RFC,
    #[doc = "0x18c - VIMS Mode Direct Control"]
    pub pdctl1vims: PDCTL1VIMS,
    _reserved58: [u8; 4usize],
    #[doc = "0x194 - Power Manager Status"]
    pub pdstat1: PDSTAT1,
    #[doc = "0x198 - BUS Power Domain Direct Read Status"]
    pub pdstat1bus: PDSTAT1BUS,
    #[doc = "0x19c - RFC Power Domain Direct Read Status"]
    pub pdstat1rfc: PDSTAT1RFC,
    #[doc = "0x1a0 - CPU Power Domain Direct Read Status"]
    pub pdstat1cpu: PDSTAT1CPU,
    #[doc = "0x1a4 - VIMS Mode Direct Read Status"]
    pub pdstat1vims: PDSTAT1VIMS,
    _reserved63: [u8; 36usize],
    #[doc = "0x1cc - Control To RFC"]
    pub rfcbits: RFCBITS,
    #[doc = "0x1d0 - Selected RFC Mode"]
    pub rfcmodesel: RFCMODESEL,
    #[doc = "0x1d4 - Allowed RFC Modes"]
    pub rfcmodehwopt: RFCMODEHWOPT,
    _reserved66: [u8; 8usize],
    #[doc = "0x1e0 - Power Profiler Register"]
    pub pwrprofstat: PWRPROFSTAT,
    _reserved67: [u8; 56usize],
    #[doc = "0x21c - MCU SRAM configuration"]
    pub mcusramcfg: MCUSRAMCFG,
    _reserved68: [u8; 4usize],
    #[doc = "0x224 - Memory Retention Control"]
    pub ramreten: RAMRETEN,
    _reserved69: [u8; 104usize],
    #[doc = "0x290 - Oscillator Interrupt Mask"]
    pub oscimsc: OSCIMSC,
    #[doc = "0x294 - Oscillator Raw Interrupt Status"]
    pub oscris: OSCRIS,
    #[doc = "0x298 - Oscillator Raw Interrupt Clear"]
    pub oscicr: OSCICR,
}
#[doc = "Infrastructure Clock Division Factor For Run Mode"]
pub struct INFRCLKDIVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Infrastructure Clock Division Factor For Run Mode"]
pub mod infrclkdivr;
#[doc = "Infrastructure Clock Division Factor For Sleep Mode"]
pub struct INFRCLKDIVS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Infrastructure Clock Division Factor For Sleep Mode"]
pub mod infrclkdivs;
#[doc = "Infrastructure Clock Division Factor For DeepSleep Mode"]
pub struct INFRCLKDIVDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Infrastructure Clock Division Factor For DeepSleep Mode"]
pub mod infrclkdivds;
#[doc = "MCU Voltage Domain Control"]
pub struct VDCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU Voltage Domain Control"]
pub mod vdctl;
#[doc = "Load PRCM Settings To CLKCTRL Power Domain"]
pub struct CLKLOADCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Load PRCM Settings To CLKCTRL Power Domain"]
pub mod clkloadctl;
#[doc = "RFC Clock Gate"]
pub struct RFCCLKG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFC Clock Gate"]
pub mod rfcclkg;
#[doc = "VIMS Clock Gate"]
pub struct VIMSCLKG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIMS Clock Gate"]
pub mod vimsclkg;
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes"]
pub struct SECDMACLKGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes"]
pub mod secdmaclkgr;
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode"]
pub struct SECDMACLKGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode"]
pub mod secdmaclkgs;
#[doc = "SEC (PKA And TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode"]
pub struct SECDMACLKGDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SEC (PKA And TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode"]
pub mod secdmaclkgds;
#[doc = "GPIO Clock Gate For Run And All Modes"]
pub struct GPIOCLKGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Clock Gate For Run And All Modes"]
pub mod gpioclkgr;
#[doc = "GPIO Clock Gate For Sleep Mode"]
pub struct GPIOCLKGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Clock Gate For Sleep Mode"]
pub mod gpioclkgs;
#[doc = "GPIO Clock Gate For Deep Sleep Mode"]
pub struct GPIOCLKGDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Clock Gate For Deep Sleep Mode"]
pub mod gpioclkgds;
#[doc = "GPT Clock Gate For Run And All Modes"]
pub struct GPTCLKGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Clock Gate For Run And All Modes"]
pub mod gptclkgr;
#[doc = "GPT Clock Gate For Sleep Mode"]
pub struct GPTCLKGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Clock Gate For Sleep Mode"]
pub mod gptclkgs;
#[doc = "GPT Clock Gate For Deep Sleep Mode"]
pub struct GPTCLKGDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Clock Gate For Deep Sleep Mode"]
pub mod gptclkgds;
#[doc = "I2C Clock Gate For Run And All Modes"]
pub struct I2CCLKGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Clock Gate For Run And All Modes"]
pub mod i2cclkgr;
#[doc = "I2C Clock Gate For Sleep Mode"]
pub struct I2CCLKGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Clock Gate For Sleep Mode"]
pub mod i2cclkgs;
#[doc = "I2C Clock Gate For Deep Sleep Mode"]
pub struct I2CCLKGDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Clock Gate For Deep Sleep Mode"]
pub mod i2cclkgds;
#[doc = "UART Clock Gate For Run And All Modes"]
pub struct UARTCLKGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Clock Gate For Run And All Modes"]
pub mod uartclkgr;
#[doc = "UART Clock Gate For Sleep Mode"]
pub struct UARTCLKGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Clock Gate For Sleep Mode"]
pub mod uartclkgs;
#[doc = "UART Clock Gate For Deep Sleep Mode"]
pub struct UARTCLKGDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Clock Gate For Deep Sleep Mode"]
pub mod uartclkgds;
#[doc = "SSI Clock Gate For Run And All Modes"]
pub struct SSICLKGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Clock Gate For Run And All Modes"]
pub mod ssiclkgr;
#[doc = "SSI Clock Gate For Sleep Mode"]
pub struct SSICLKGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Clock Gate For Sleep Mode"]
pub mod ssiclkgs;
#[doc = "SSI Clock Gate For Deep Sleep Mode"]
pub struct SSICLKGDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSI Clock Gate For Deep Sleep Mode"]
pub mod ssiclkgds;
#[doc = "I2S Clock Gate For Run And All Modes"]
pub struct I2SCLKGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Clock Gate For Run And All Modes"]
pub mod i2sclkgr;
#[doc = "I2S Clock Gate For Sleep Mode"]
pub struct I2SCLKGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Clock Gate For Sleep Mode"]
pub mod i2sclkgs;
#[doc = "I2S Clock Gate For Deep Sleep Mode"]
pub struct I2SCLKGDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Clock Gate For Deep Sleep Mode"]
pub mod i2sclkgds;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct SYSBUSCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod sysbusclkdiv;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CPUCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpuclkdiv;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct PERBUSCPUCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perbuscpuclkdiv;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct PERBUSDMACLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perbusdmaclkdiv;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct PERDMACLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perdmaclkdiv;
#[doc = "I2S Clock Control"]
pub struct I2SBCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Clock Control"]
pub mod i2sbclksel;
#[doc = "GPT Scalar"]
pub struct GPTCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Scalar"]
pub mod gptclkdiv;
#[doc = "I2S Clock Control"]
pub struct I2SCLKCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S Clock Control"]
pub mod i2sclkctl;
#[doc = "MCLK Division Ratio"]
pub struct I2SMCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCLK Division Ratio"]
pub mod i2smclkdiv;
#[doc = "BCLK Division Ratio"]
pub struct I2SBCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BCLK Division Ratio"]
pub mod i2sbclkdiv;
#[doc = "WCLK Division Ratio"]
pub struct I2SWCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WCLK Division Ratio"]
pub mod i2swclkdiv;
#[doc = "RESET For SEC (PKA And TRNG And CRYPTO) And UDMA"]
pub struct RESETSECDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RESET For SEC (PKA And TRNG And CRYPTO) And UDMA"]
pub mod resetsecdma;
#[doc = "RESET For GPIO IPs"]
pub struct RESETGPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RESET For GPIO IPs"]
pub mod resetgpio;
#[doc = "RESET For GPT Ips"]
pub struct RESETGPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RESET For GPT Ips"]
pub mod resetgpt;
#[doc = "RESET For I2C IPs"]
pub struct RESETI2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RESET For I2C IPs"]
pub mod reseti2c;
#[doc = "RESET For UART IPs"]
pub struct RESETUART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RESET For UART IPs"]
pub mod resetuart;
#[doc = "RESET For SSI IPs"]
pub struct RESETSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RESET For SSI IPs"]
pub mod resetssi;
#[doc = "RESET For I2S IP"]
pub struct RESETI2S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RESET For I2S IP"]
pub mod reseti2s;
#[doc = "Power Domain Control"]
pub struct PDCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Domain Control"]
pub mod pdctl0;
#[doc = "RFC Power Domain Control"]
pub struct PDCTL0RFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFC Power Domain Control"]
pub mod pdctl0rfc;
#[doc = "SERIAL Power Domain Control"]
pub struct PDCTL0SERIAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SERIAL Power Domain Control"]
pub mod pdctl0serial;
#[doc = "PERIPH Power Domain Control"]
pub struct PDCTL0PERIPH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PERIPH Power Domain Control"]
pub mod pdctl0periph;
#[doc = "Power Domain Status"]
pub struct PDSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Domain Status"]
pub mod pdstat0;
#[doc = "RFC Power Domain Status"]
pub struct PDSTAT0RFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFC Power Domain Status"]
pub mod pdstat0rfc;
#[doc = "SERIAL Power Domain Status"]
pub struct PDSTAT0SERIAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SERIAL Power Domain Status"]
pub mod pdstat0serial;
#[doc = "PERIPH Power Domain Status"]
pub struct PDSTAT0PERIPH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PERIPH Power Domain Status"]
pub mod pdstat0periph;
#[doc = "Power Domain Control"]
pub struct PDCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Domain Control"]
pub mod pdctl1;
#[doc = "CPU Power Domain Direct Control"]
pub struct PDCTL1CPU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Power Domain Direct Control"]
pub mod pdctl1cpu;
#[doc = "RFC Power Domain Direct Control"]
pub struct PDCTL1RFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFC Power Domain Direct Control"]
pub mod pdctl1rfc;
#[doc = "VIMS Mode Direct Control"]
pub struct PDCTL1VIMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIMS Mode Direct Control"]
pub mod pdctl1vims;
#[doc = "Power Manager Status"]
pub struct PDSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Manager Status"]
pub mod pdstat1;
#[doc = "BUS Power Domain Direct Read Status"]
pub struct PDSTAT1BUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BUS Power Domain Direct Read Status"]
pub mod pdstat1bus;
#[doc = "RFC Power Domain Direct Read Status"]
pub struct PDSTAT1RFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFC Power Domain Direct Read Status"]
pub mod pdstat1rfc;
#[doc = "CPU Power Domain Direct Read Status"]
pub struct PDSTAT1CPU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Power Domain Direct Read Status"]
pub mod pdstat1cpu;
#[doc = "VIMS Mode Direct Read Status"]
pub struct PDSTAT1VIMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIMS Mode Direct Read Status"]
pub mod pdstat1vims;
#[doc = "Control To RFC"]
pub struct RFCBITS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control To RFC"]
pub mod rfcbits;
#[doc = "Selected RFC Mode"]
pub struct RFCMODESEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selected RFC Mode"]
pub mod rfcmodesel;
#[doc = "Allowed RFC Modes"]
pub struct RFCMODEHWOPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Allowed RFC Modes"]
pub mod rfcmodehwopt;
#[doc = "Power Profiler Register"]
pub struct PWRPROFSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Profiler Register"]
pub mod pwrprofstat;
#[doc = "MCU SRAM configuration"]
pub struct MCUSRAMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SRAM configuration"]
pub mod mcusramcfg;
#[doc = "Memory Retention Control"]
pub struct RAMRETEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Retention Control"]
pub mod ramreten;
#[doc = "Oscillator Interrupt Mask"]
pub struct OSCIMSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Interrupt Mask"]
pub mod oscimsc;
#[doc = "Oscillator Raw Interrupt Status"]
pub struct OSCRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Raw Interrupt Status"]
pub mod oscris;
#[doc = "Oscillator Raw Interrupt Clear"]
pub struct OSCICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Raw Interrupt Clear"]
pub mod oscicr;
