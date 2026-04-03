use std::os::raw::{c_char, c_uint};

pub const CF_FILENAME_MAX_PATH: usize = 64; // Assuming typical value, should match cf_mission_cfg.h
pub const CF_NUM_CHANNELS: usize = 2; // Assuming typical value, should match cf_mission_cfg.h

pub type CF_EntityId_t = u32;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ChannelConfig_t {
    // This would need to be defined based on cf_tbldefs.h
    // Placeholder structure - actual fields depend on the header
    pub placeholder: u32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ConfigTable_t {
    pub ticks_per_second: u32,
    pub rx_crc_calc_bytes_per_wakeup: u32,
    pub local_eid: CF_EntityId_t,
    pub chan: [CF_ChannelConfig_t; CF_NUM_CHANNELS],
    pub outgoing_file_chunk_size: u16,
    pub tmp_dir: [c_char; CF_FILENAME_MAX_PATH],
    pub fail_dir: [c_char; CF_FILENAME_MAX_PATH],
}

pub type CF_ConfigTable = CF_ConfigTable_t;