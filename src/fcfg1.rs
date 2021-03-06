#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 160usize],
    #[doc = "0xa0 - Misc configurations"]
    pub misc_conf_1: MISC_CONF_1,
    #[doc = "0xa4 - Internal. Only to be used through TI provided API."]
    pub misc_conf_2: MISC_CONF_2,
    _reserved2: [u8; 28usize],
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    pub config_cc26_fe: CONFIG_CC26_FE,
    #[doc = "0xc8 - Internal. Only to be used through TI provided API."]
    pub config_cc13_fe: CONFIG_CC13_FE,
    #[doc = "0xcc - Internal. Only to be used through TI provided API."]
    pub config_rf_common: CONFIG_RF_COMMON,
    #[doc = "0xd0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc26_2g4: CONFIG_SYNTH_DIV2_CC26_2G4,
    #[doc = "0xd4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc13_2g4: CONFIG_SYNTH_DIV2_CC13_2G4,
    #[doc = "0xd8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc26_1g: CONFIG_SYNTH_DIV2_CC26_1G,
    #[doc = "0xdc - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc13_1g: CONFIG_SYNTH_DIV2_CC13_1G,
    #[doc = "0xe0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div4_cc26: CONFIG_SYNTH_DIV4_CC26,
    #[doc = "0xe4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div4_cc13: CONFIG_SYNTH_DIV4_CC13,
    #[doc = "0xe8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div5: CONFIG_SYNTH_DIV5,
    #[doc = "0xec - Internal. Only to be used through TI provided API."]
    pub config_synth_div6_cc26: CONFIG_SYNTH_DIV6_CC26,
    #[doc = "0xf0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div6_cc13: CONFIG_SYNTH_DIV6_CC13,
    #[doc = "0xf4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div10: CONFIG_SYNTH_DIV10,
    #[doc = "0xf8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div12_cc26: CONFIG_SYNTH_DIV12_CC26,
    #[doc = "0xfc - Internal. Only to be used through TI provided API."]
    pub config_synth_div12_cc13: CONFIG_SYNTH_DIV12_CC13,
    #[doc = "0x100 - Internal. Only to be used through TI provided API."]
    pub config_synth_div15: CONFIG_SYNTH_DIV15,
    #[doc = "0x104 - Internal. Only to be used through TI provided API."]
    pub config_synth_div30: CONFIG_SYNTH_DIV30,
    _reserved19: [u8; 92usize],
    #[doc = "0x164 - Flash information"]
    pub flash_number: FLASH_NUMBER,
    _reserved20: [u8; 4usize],
    #[doc = "0x16c - Flash information"]
    pub flash_coordinate: FLASH_COORDINATE,
    #[doc = "0x170 - Internal. Only to be used through TI provided API."]
    pub flash_e_p: FLASH_E_P,
    #[doc = "0x174 - Internal. Only to be used through TI provided API."]
    pub flash_c_e_p_r: FLASH_C_E_P_R,
    #[doc = "0x178 - Internal. Only to be used through TI provided API."]
    pub flash_p_r_pv: FLASH_P_R_PV,
    #[doc = "0x17c - Internal. Only to be used through TI provided API."]
    pub flash_eh_seq: FLASH_EH_SEQ,
    #[doc = "0x180 - Internal. Only to be used through TI provided API."]
    pub flash_vhv_e: FLASH_VHV_E,
    #[doc = "0x184 - Internal. Only to be used through TI provided API."]
    pub flash_pp: FLASH_PP,
    #[doc = "0x188 - Internal. Only to be used through TI provided API."]
    pub flash_prog_ep: FLASH_PROG_EP,
    #[doc = "0x18c - Internal. Only to be used through TI provided API."]
    pub flash_era_pw: FLASH_ERA_PW,
    #[doc = "0x190 - Internal. Only to be used through TI provided API."]
    pub flash_vhv: FLASH_VHV,
    #[doc = "0x194 - Internal. Only to be used through TI provided API."]
    pub flash_vhv_pv: FLASH_VHV_PV,
    #[doc = "0x198 - Internal. Only to be used through TI provided API."]
    pub flash_v: FLASH_V,
    _reserved32: [u8; 248usize],
    #[doc = "0x294 - User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone."]
    pub user_id: USER_ID,
    _reserved33: [u8; 24usize],
    #[doc = "0x2b0 - Internal. Only to be used through TI provided API."]
    pub flash_otp_data3: FLASH_OTP_DATA3,
    #[doc = "0x2b4 - Internal. Only to be used through TI provided API."]
    pub ana2_trim: ANA2_TRIM,
    #[doc = "0x2b8 - Internal. Only to be used through TI provided API."]
    pub ldo_trim: LDO_TRIM,
    _reserved36: [u8; 44usize],
    #[doc = "0x2e8 - MAC BLE Address 0"]
    pub mac_ble_0: MAC_BLE_0,
    #[doc = "0x2ec - MAC BLE Address 1"]
    pub mac_ble_1: MAC_BLE_1,
    #[doc = "0x2f0 - MAC IEEE 802.15.4 Address 0"]
    pub mac_15_4_0: MAC_15_4_0,
    #[doc = "0x2f4 - MAC IEEE 802.15.4 Address 1"]
    pub mac_15_4_1: MAC_15_4_1,
    _reserved40: [u8; 16usize],
    #[doc = "0x308 - Internal. Only to be used through TI provided API."]
    pub flash_otp_data4: FLASH_OTP_DATA4,
    #[doc = "0x30c - Miscellaneous Trim Parameters"]
    pub misc_trim: MISC_TRIM,
    #[doc = "0x310 - Internal. Only to be used through TI provided API."]
    pub rcosc_hf_tempcomp: RCOSC_HF_TEMPCOMP,
    _reserved43: [u8; 4usize],
    #[doc = "0x318 - IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device."]
    pub icepick_device_id: ICEPICK_DEVICE_ID,
    #[doc = "0x31c - Factory Configuration (FCFG1) Revision"]
    pub fcfg1_revision: FCFG1_REVISION,
    #[doc = "0x320 - Misc OTP Data"]
    pub misc_otp_data: MISC_OTP_DATA,
    _reserved46: [u8; 32usize],
    #[doc = "0x344 - IO Configuration"]
    pub ioconf: IOCONF,
    _reserved47: [u8; 4usize],
    #[doc = "0x34c - Internal. Only to be used through TI provided API."]
    pub config_if_adc: CONFIG_IF_ADC,
    #[doc = "0x350 - Internal. Only to be used through TI provided API."]
    pub config_osc_top: CONFIG_OSC_TOP,
    _reserved49: [u8; 8usize],
    #[doc = "0x35c - AUX_ADC Gain in Absolute Reference Mode"]
    pub soc_adc_abs_gain: SOC_ADC_ABS_GAIN,
    #[doc = "0x360 - AUX_ADC Gain in Relative Reference Mode"]
    pub soc_adc_rel_gain: SOC_ADC_REL_GAIN,
    _reserved51: [u8; 4usize],
    #[doc = "0x368 - AUX_ADC Temperature Offsets in Absolute Reference Mode"]
    pub soc_adc_offset_int: SOC_ADC_OFFSET_INT,
    #[doc = "0x36c - Internal. Only to be used through TI provided API."]
    pub soc_adc_ref_trim_and_offset_ext: SOC_ADC_REF_TRIM_AND_OFFSET_EXT,
    #[doc = "0x370 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th1: AMPCOMP_TH1,
    #[doc = "0x374 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th2: AMPCOMP_TH2,
    #[doc = "0x378 - Internal. Only to be used through TI provided API."]
    pub ampcomp_ctrl1: AMPCOMP_CTRL1,
    #[doc = "0x37c - Internal. Only to be used through TI provided API."]
    pub anabypass_value2: ANABYPASS_VALUE2,
    _reserved57: [u8; 8usize],
    #[doc = "0x388 - Internal. Only to be used through TI provided API."]
    pub volt_trim: VOLT_TRIM,
    #[doc = "0x38c - OSC Configuration"]
    pub osc_conf: OSC_CONF,
    #[doc = "0x390 - Internal. Only to be used through TI provided API."]
    pub freq_offset: FREQ_OFFSET,
    _reserved60: [u8; 4usize],
    #[doc = "0x398 - Internal. Only to be used through TI provided API."]
    pub misc_otp_data_1: MISC_OTP_DATA_1,
    _reserved61: [u8; 52usize],
    #[doc = "0x3d0 - Shadow of DIE_ID_0 register in eFuse"]
    pub shdw_die_id_0: SHDW_DIE_ID_0,
    #[doc = "0x3d4 - Shadow of DIE_ID_1 register in eFuse"]
    pub shdw_die_id_1: SHDW_DIE_ID_1,
    #[doc = "0x3d8 - Shadow of DIE_ID_2 register in eFuse"]
    pub shdw_die_id_2: SHDW_DIE_ID_2,
    #[doc = "0x3dc - Shadow of DIE_ID_3 register in eFuse"]
    pub shdw_die_id_3: SHDW_DIE_ID_3,
    _reserved65: [u8; 24usize],
    #[doc = "0x3f8 - Internal. Only to be used through TI provided API."]
    pub shdw_osc_bias_ldo_trim: SHDW_OSC_BIAS_LDO_TRIM,
    #[doc = "0x3fc - Internal. Only to be used through TI provided API."]
    pub shdw_ana_trim: SHDW_ANA_TRIM,
    _reserved67: [u8; 12usize],
    #[doc = "0x40c - Internal. Only to be used through TI provided API."]
    pub dac_bias_cnf: DAC_BIAS_CNF,
    _reserved68: [u8; 8usize],
    #[doc = "0x418 - Internal. Only to be used through TI provided API."]
    pub tfw_probe: TFW_PROBE,
    #[doc = "0x41c - Internal. Only to be used through TI provided API."]
    pub tfw_ft: TFW_FT,
    #[doc = "0x420 - Internal. Only to be used through TI provided API."]
    pub dac_cal0: DAC_CAL0,
    #[doc = "0x424 - Internal. Only to be used through TI provided API."]
    pub dac_cal1: DAC_CAL1,
    #[doc = "0x428 - Internal. Only to be used through TI provided API."]
    pub dac_cal2: DAC_CAL2,
    #[doc = "0x42c - Internal. Only to be used through TI provided API."]
    pub dac_cal3: DAC_CAL3,
    #[doc = "0x430 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_n: RESERVED_N,
}
#[doc = "Misc configurations"]
pub struct MISC_CONF_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Misc configurations"]
pub mod misc_conf_1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct MISC_CONF_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_conf_2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_CC26_FE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_cc26_fe;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_CC13_FE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_cc13_fe;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_RF_COMMON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_common;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV2_CC26_2G4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc26_2g4;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV2_CC13_2G4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc13_2g4;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV2_CC26_1G {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc26_1g;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV2_CC13_1G {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc13_1g;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV4_CC26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div4_cc26;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV4_CC13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div4_cc13;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div5;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV6_CC26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6_cc26;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV6_CC13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6_cc13;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div10;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV12_CC26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12_cc26;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV12_CC13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12_cc13;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div15;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_SYNTH_DIV30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div30;
#[doc = "Flash information"]
pub struct FLASH_NUMBER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash information"]
pub mod flash_number;
#[doc = "Flash information"]
pub struct FLASH_COORDINATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash information"]
pub mod flash_coordinate;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_E_P {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_e_p;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_C_E_P_R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_c_e_p_r;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_P_R_PV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_p_r_pv;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_EH_SEQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_eh_seq;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_VHV_E {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_e;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_pp;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_PROG_EP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_prog_ep;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_ERA_PW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_era_pw;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_VHV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_VHV_PV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_pv;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_v;
#[doc = "User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone."]
pub struct USER_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone."]
pub mod user_id;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_OTP_DATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct ANA2_TRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ana2_trim;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct LDO_TRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ldo_trim;
#[doc = "MAC BLE Address 0"]
pub struct MAC_BLE_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC BLE Address 0"]
pub mod mac_ble_0;
#[doc = "MAC BLE Address 1"]
pub struct MAC_BLE_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC BLE Address 1"]
pub mod mac_ble_1;
#[doc = "MAC IEEE 802.15.4 Address 0"]
pub struct MAC_15_4_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC IEEE 802.15.4 Address 0"]
pub mod mac_15_4_0;
#[doc = "MAC IEEE 802.15.4 Address 1"]
pub struct MAC_15_4_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC IEEE 802.15.4 Address 1"]
pub mod mac_15_4_1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_OTP_DATA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data4;
#[doc = "Miscellaneous Trim Parameters"]
pub struct MISC_TRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Trim Parameters"]
pub mod misc_trim;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct RCOSC_HF_TEMPCOMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod rcosc_hf_tempcomp;
#[doc = "IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device."]
pub struct ICEPICK_DEVICE_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device."]
pub mod icepick_device_id;
#[doc = "Factory Configuration (FCFG1) Revision"]
pub struct FCFG1_REVISION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Factory Configuration (FCFG1) Revision"]
pub mod fcfg1_revision;
#[doc = "Misc OTP Data"]
pub struct MISC_OTP_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Misc OTP Data"]
pub mod misc_otp_data;
#[doc = "IO Configuration"]
pub struct IOCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Configuration"]
pub mod ioconf;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_IF_ADC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_if_adc;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CONFIG_OSC_TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_osc_top;
#[doc = "AUX_ADC Gain in Absolute Reference Mode"]
pub struct SOC_ADC_ABS_GAIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX_ADC Gain in Absolute Reference Mode"]
pub mod soc_adc_abs_gain;
#[doc = "AUX_ADC Gain in Relative Reference Mode"]
pub struct SOC_ADC_REL_GAIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX_ADC Gain in Relative Reference Mode"]
pub mod soc_adc_rel_gain;
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode"]
pub struct SOC_ADC_OFFSET_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode"]
pub mod soc_adc_offset_int;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct SOC_ADC_REF_TRIM_AND_OFFSET_EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod soc_adc_ref_trim_and_offset_ext;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_TH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_TH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_ctrl1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct ANABYPASS_VALUE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypass_value2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct VOLT_TRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod volt_trim;
#[doc = "OSC Configuration"]
pub struct OSC_CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC Configuration"]
pub mod osc_conf;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FREQ_OFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod freq_offset;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct MISC_OTP_DATA_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_otp_data_1;
#[doc = "Shadow of DIE_ID_0 register in eFuse"]
pub struct SHDW_DIE_ID_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow of DIE_ID_0 register in eFuse"]
pub mod shdw_die_id_0;
#[doc = "Shadow of DIE_ID_1 register in eFuse"]
pub struct SHDW_DIE_ID_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow of DIE_ID_1 register in eFuse"]
pub mod shdw_die_id_1;
#[doc = "Shadow of DIE_ID_2 register in eFuse"]
pub struct SHDW_DIE_ID_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow of DIE_ID_2 register in eFuse"]
pub mod shdw_die_id_2;
#[doc = "Shadow of DIE_ID_3 register in eFuse"]
pub struct SHDW_DIE_ID_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow of DIE_ID_3 register in eFuse"]
pub mod shdw_die_id_3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct SHDW_OSC_BIAS_LDO_TRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_osc_bias_ldo_trim;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct SHDW_ANA_TRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_ana_trim;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct DAC_BIAS_CNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_bias_cnf;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct TFW_PROBE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tfw_probe;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct TFW_FT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tfw_ft;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct DAC_CAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct DAC_CAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct DAC_CAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct DAC_CAL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal3;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED_N {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_n;
