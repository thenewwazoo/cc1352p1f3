#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub mux0: MUX0,
    #[doc = "0x01 - Internal. Only to be used through TI provided API."]
    pub mux1: MUX1,
    #[doc = "0x02 - Internal. Only to be used through TI provided API."]
    pub mux2: MUX2,
    #[doc = "0x03 - Internal. Only to be used through TI provided API."]
    pub mux3: MUX3,
    #[doc = "0x04 - Current Source Strength and trim control for current source. Only to be used through TI provided API."]
    pub isrc: ISRC,
    #[doc = "0x05 - Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
    pub comp: COMP,
    _reserved6: [u8; 1usize],
    #[doc = "0x07 - Internal. Only to be used through TI provided API."]
    pub mux4: MUX4,
    #[doc = "0x08 - ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
    pub adc0: ADC0,
    #[doc = "0x09 - ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
    pub adc1: ADC1,
    #[doc = "0x0a - ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
    pub adcref0: ADCREF0,
    #[doc = "0x0b - ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
    pub adcref1: ADCREF1,
    _reserved11: [u8; 2usize],
    #[doc = "0x0e - Internal. Only to be used through TI provided API."]
    pub lpmbias: LPMBIAS,
    #[doc = "0x0f - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub stat: STAT,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub struct MUX0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct MUX1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct MUX2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct MUX3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux3;
#[doc = "Current Source Strength and trim control for current source. Only to be used through TI provided API."]
pub struct ISRC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Current Source Strength and trim control for current source. Only to be used through TI provided API."]
pub mod isrc;
#[doc = "Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
pub struct COMP {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
pub mod comp;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct MUX4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux4;
#[doc = "ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
pub struct ADC0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
pub mod adc0;
#[doc = "ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
pub struct ADC1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
pub mod adc1;
#[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
pub struct ADCREF0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
pub mod adcref0;
#[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
pub struct ADCREF1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
pub mod adcref1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct LPMBIAS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod lpmbias;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct STAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod stat;
