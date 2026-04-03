use std::ffi::CString;

// Constants that would typically be defined elsewhere
const CF_NUM_CHANNELS: usize = 2;
const CF_MAX_POLLING_DIR_PER_CHAN: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum CfCfdpClass {
    Class1 = 1,
    Class2 = 2,
}

const CF_CFDP_CLASS_2: CfCfdpClass = CfCfdpClass::Class2;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CfPolldirConfig {
    pub interval_sec: u32,
    pub priority: u32,
    pub cfdp_class: u32,
    pub dest_eid: u32,
    pub src_dir: String,
    pub dst_dir: String,
    pub enabled: u32,
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CfChannelConfig {
    pub max_outgoing_messages_per_wakeup: u32,
    pub max_rx_messages_per_wakeup: u32,
    pub ack_timer_s: u32,
    pub nak_timer_s: u32,
    pub inactivity_timer_s: u32,
    pub ack_limit: u32,
    pub nak_limit: u32,
    pub input_msg_id: u32,
    pub output_msg_id: u32,
    pub pipe_depth_input: u32,
    pub polldir: [CfPolldirConfig; CF_MAX_POLLING_DIR_PER_CHAN],
    pub throttle_sem_name: String,
    pub dequeue_enabled: u32,
    pub move_dir: String,
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CfConfigTable {
    pub ticks_per_second: u32,
    pub rx_crc_calc_bytes_per_wakeup: u32,
    pub local_eid: u32,
    pub chan: [CfChannelConfig; CF_NUM_CHANNELS],
    pub outgoing_file_chunk_size: u32,
    pub tmp_dir: String,
    pub failed_dir: String,
}

pub static CF_CONFIG_TABLE: CfConfigTable = CfConfigTable {
    ticks_per_second: 10,
    rx_crc_calc_bytes_per_wakeup: 16384,
    local_eid: 25,
    chan: [
        CfChannelConfig {
            max_outgoing_messages_per_wakeup: 5,
            max_rx_messages_per_wakeup: 5,
            ack_timer_s: 3,
            nak_timer_s: 3,
            inactivity_timer_s: 30,
            ack_limit: 4,
            nak_limit: 4,
            input_msg_id: 0x18c8,
            output_msg_id: 0x08c2,
            pipe_depth_input: 16,
            polldir: [
                CfPolldirConfig {
                    interval_sec: 5,
                    priority: 25,
                    cfdp_class: CF_CFDP_CLASS_2 as u32,
                    dest_eid: 23,
                    src_dir: String::from("/cf/poll_dir"),
                    dst_dir: String::from("./poll_dir"),
                    enabled: 0,
                },
                CfPolldirConfig::default(),
            ],
            throttle_sem_name: String::new(),
            dequeue_enabled: 1,
            move_dir: String::new(),
        },
        CfChannelConfig {
            max_outgoing_messages_per_wakeup: 5,
            max_rx_messages_per_wakeup: 5,
            ack_timer_s: 3,
            nak_timer_s: 3,
            inactivity_timer_s: 30,
            ack_limit: 4,
            nak_limit: 4,
            input_msg_id: 0x18c9,
            output_msg_id: 0x08c3,
            pipe_depth_input: 16,
            polldir: [CfPolldirConfig::default(); CF_MAX_POLLING_DIR_PER_CHAN],
            throttle_sem_name: String::new(),
            dequeue_enabled: 1,
            move_dir: String::new(),
        },
    ],
    outgoing_file_chunk_size: 480,
    tmp_dir: String::from("/cf/tmp"),
    failed_dir: String::from("/cf/fail"),
};

// CFE table file definition would be handled by build system or separate macro
// CFE_TBL_FILEDEF equivalent would be implemented as needed for the specific CFE binding