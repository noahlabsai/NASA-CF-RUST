//! CF Application message definitions.
//!
//! Translated from: default_cf_msgdefs.h, default_cf_msgstruct.h,
//!                   default_cf_tbldefs.h, default_cf_tblstruct.h
//!
//! Contains all command/telemetry payload types and table definitions.

use crate::cf_platform_cfg::*;
use crate::cf_extern_typedefs::*;
use crate::common_types::*;

// =====================================================================
// Housekeeping telemetry structures (from cf_msgdefs.h)
// =====================================================================

/// Housekeeping command counters
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkCmdCounters_t {
    pub cmd: u16,
    pub err: u16,
}

/// Housekeeping sent counters
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkSent_t {
    pub file_data_bytes: u64,
    pub pdu: u32,
    pub nak_segment_requests: u32,
}

/// Housekeeping received counters
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkRecv_t {
    pub file_data_bytes: u64,
    pub pdu: u32,
    pub error: u32,
    pub spurious: u16,
    pub dropped: u16,
    pub nak_segment_requests: u32,
}

/// Housekeeping fault counters
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkFault_t {
    pub file_open: u16,
    pub file_read: u16,
    pub file_seek: u16,
    pub file_write: u16,
    pub file_rename: u16,
    pub directory_read: u16,
    pub crc_mismatch: u16,
    pub file_size_mismatch: u16,
    pub nak_limit: u16,
    pub ack_limit: u16,
    pub inactivity_timer: u16,
    pub spare: u16,
}

/// Housekeeping counters
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkCounters_t {
    pub sent: CF_HkSent_t,
    pub recv: CF_HkRecv_t,
    pub fault: CF_HkFault_t,
}

/// Housekeeping channel data
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkChannel_Data_t {
    pub counters: CF_HkCounters_t,
    pub q_size: [u16; CF_QueueIdx_NUM],
    pub poll_counter: u8,
    pub playback_counter: u8,
    pub frozen: u8,
    pub spare: [u8; 7],
}

/// Housekeeping packet payload
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_HkPacket_Payload_t {
    pub counters: CF_HkCmdCounters_t,
    pub spare: [u8; 4],
    pub channel_hk: [CF_HkChannel_Data_t; CF_NUM_CHANNELS],
}

impl Default for CF_HkPacket_Payload_t {
    fn default() -> Self {
        Self {
            counters: CF_HkCmdCounters_t::default(),
            spare: [0u8; 4],
            channel_hk: [CF_HkChannel_Data_t::default(); CF_NUM_CHANNELS],
        }
    }
}

/// End of transaction packet payload
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CF_EotPacket_Payload_t {
    pub seq_num: CF_TransactionSeq_t,
    pub channel: u32,
    pub direction: u32,
    pub state: u32,
    pub txn_stat: u32,
    pub src_eid: CF_EntityId_t,
    pub peer_eid: CF_EntityId_t,
    pub fsize: u32,
    pub crc_result: u32,
    pub fnames: CF_TxnFilenames_t,
}

// =====================================================================
// Housekeeping packet (from cf_msgstruct.h)
// =====================================================================

/// Housekeeping packet
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CF_HkPacket_t {
    pub TelemetryHeader: CFE_MSG_TelemetryHeader_t,
    pub Payload: CF_HkPacket_Payload_t,
}

impl Default for CF_HkPacket_t {
    fn default() -> Self {
        Self {
            TelemetryHeader: CFE_MSG_TelemetryHeader_t::default(),
            Payload: CF_HkPacket_Payload_t::default(),
        }
    }
}

/// End of transaction packet
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CF_EotPacket_t {
    pub TelemetryHeader: CFE_MSG_TelemetryHeader_t,
    pub Payload: CF_EotPacket_Payload_t,
}

// =====================================================================
// Command payload types (from cf_msgdefs.h)
// =====================================================================

/// Command payload argument union
#[derive(Clone, Copy)]
#[repr(C)]
pub union CF_UnionArgs_Payload_t {
    pub dword: u32,
    pub hword: [u16; 2],
    pub byte: [u8; 4],
}

impl Default for CF_UnionArgs_Payload_t {
    fn default() -> Self {
        Self { dword: 0 }
    }
}

impl core::fmt::Debug for CF_UnionArgs_Payload_t {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Safety: reading dword is always valid for a 4-byte union
        write!(f, "CF_UnionArgs_Payload_t {{ dword: {} }}", unsafe { self.dword })
    }
}

/// IDs for Reset command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_Reset_t {
    CF_Reset_all     = 0,
    CF_Reset_command = 1,
    CF_Reset_fault   = 2,
    CF_Reset_up      = 3,
    CF_Reset_down    = 4,
}

/// Type IDs for Write Queue command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_Type_t {
    CF_Type_all  = 0,
    CF_Type_up   = 1,
    CF_Type_down = 2,
}

/// Queue IDs for Write Queue command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_Queue_t {
    CF_Queue_pend    = 0,
    CF_Queue_active  = 1,
    CF_Queue_history = 2,
    CF_Queue_all     = 3,
}

/// Parameter IDs for Get/Set parameter messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_GetSet_ValueID_t {
    CF_GetSet_ValueID_ticks_per_second = 0,
    CF_GetSet_ValueID_rx_crc_calc_bytes_per_wakeup = 1,
    CF_GetSet_ValueID_ack_timer_s = 2,
    CF_GetSet_ValueID_nak_timer_s = 3,
    CF_GetSet_ValueID_inactivity_timer_s = 4,
    CF_GetSet_ValueID_outgoing_file_chunk_size = 5,
    CF_GetSet_ValueID_ack_limit = 6,
    CF_GetSet_ValueID_nak_limit = 7,
    CF_GetSet_ValueID_local_eid = 8,
    CF_GetSet_ValueID_chan_max_outgoing_messages_per_wakeup = 9,
    CF_GetSet_ValueID_MAX = 10,
}

/// Get parameter command payload
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_GetParam_Payload_t {
    pub key: u8,
    pub chan_num: u8,
}

/// Set parameter command payload
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_SetParam_Payload_t {
    pub value: u32,
    pub key: u8,
    pub chan_num: u8,
    pub spare: [u8; 2],
}

/// Transmit file command payload
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CF_TxFile_Payload_t {
    pub cfdp_class: u8,
    pub keep: u8,
    pub chan_num: u8,
    pub priority: u8,
    pub dest_id: CF_EntityId_t,
    pub src_filename: [u8; CF_FILENAME_MAX_LEN],
    pub dst_filename: [u8; CF_FILENAME_MAX_LEN],
}

impl Default for CF_TxFile_Payload_t {
    fn default() -> Self {
        Self {
            cfdp_class: 0,
            keep: 0,
            chan_num: 0,
            priority: 0,
            dest_id: 0,
            src_filename: [0u8; CF_FILENAME_MAX_LEN],
            dst_filename: [0u8; CF_FILENAME_MAX_LEN],
        }
    }
}

/// Write Queue command payload
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CF_WriteQueue_Payload_t {
    pub r#type: u8,
    pub chan: u8,
    pub queue: u8,
    pub spare: u8,
    pub filename: [u8; CF_FILENAME_MAX_LEN],
}

impl Default for CF_WriteQueue_Payload_t {
    fn default() -> Self {
        Self {
            r#type: 0,
            chan: 0,
            queue: 0,
            spare: 0,
            filename: [0u8; CF_FILENAME_MAX_LEN],
        }
    }
}

/// Transaction command payload
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Transaction_Payload_t {
    pub ts: CF_TransactionSeq_t,
    pub eid: CF_EntityId_t,
    pub chan: u8,
    pub spare: [u8; 3],
}

// =====================================================================
// Command structures (from cf_msgstruct.h)
// =====================================================================

macro_rules! define_cmd {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Default)]
        #[repr(C)]
        pub struct $name {
            pub CommandHeader: CFE_MSG_CommandHeader_t,
        }
    };
    ($name:ident, $payload_type:ty) => {
        #[derive(Debug, Clone)]
        #[repr(C)]
        pub struct $name {
            pub CommandHeader: CFE_MSG_CommandHeader_t,
            pub Payload: $payload_type,
        }
    };
}

define_cmd!(CF_NoopCmd_t);
define_cmd!(CF_EnableEngineCmd_t);
define_cmd!(CF_DisableEngineCmd_t);
define_cmd!(CF_SendHkCmd_t);
define_cmd!(CF_WakeupCmd_t);

// Commands with CF_UnionArgs_Payload_t
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_ResetCountersCmd_t {
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_UnionArgs_Payload_t,
}

pub type CF_FreezeCmd_t = CF_ResetCountersCmd_t;
pub type CF_ThawCmd_t = CF_ResetCountersCmd_t;
pub type CF_EnableDequeueCmd_t = CF_ResetCountersCmd_t;
pub type CF_DisableDequeueCmd_t = CF_ResetCountersCmd_t;
pub type CF_EnableDirPollingCmd_t = CF_ResetCountersCmd_t;
pub type CF_DisableDirPollingCmd_t = CF_ResetCountersCmd_t;
pub type CF_PurgeQueueCmd_t = CF_ResetCountersCmd_t;

/// Generic union-args command (used for sizing in dispatch tables)
pub type CF_UnionArgsCmd_t = CF_ResetCountersCmd_t;

define_cmd!(CF_GetParamCmd_t, CF_GetParam_Payload_t);
define_cmd!(CF_SetParamCmd_t, CF_SetParam_Payload_t);
define_cmd!(CF_TxFileCmd_t, CF_TxFile_Payload_t);
define_cmd!(CF_WriteQueueCmd_t, CF_WriteQueue_Payload_t);
define_cmd!(CF_PlaybackDirCmd_t, CF_TxFile_Payload_t);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_SuspendCmd_t {
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_Transaction_Payload_t,
}

pub type CF_ResumeCmd_t = CF_SuspendCmd_t;
pub type CF_CancelCmd_t = CF_SuspendCmd_t;
pub type CF_AbandonCmd_t = CF_SuspendCmd_t;

// =====================================================================
// Table definitions (from cf_tbldefs.h / cf_tblstruct.h)
// =====================================================================

/// Configuration entry for directory polling
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CF_PollDir_t {
    pub interval_sec: u32,
    pub priority: u8,
    pub cfdp_class: CF_CFDP_Class_t,
    pub dest_eid: CF_EntityId_t,
    pub src_dir: [u8; CF_FILENAME_MAX_PATH],
    pub dst_dir: [u8; CF_FILENAME_MAX_PATH],
    pub enabled: u8,
}

impl Default for CF_PollDir_t {
    fn default() -> Self {
        Self {
            interval_sec: 0,
            priority: 0,
            cfdp_class: CF_CFDP_Class_t::CF_CFDP_CLASS_1,
            dest_eid: 0,
            src_dir: [0u8; CF_FILENAME_MAX_PATH],
            dst_dir: [0u8; CF_FILENAME_MAX_PATH],
            enabled: 0,
        }
    }
}

/// Configuration entry for CFDP channel
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CF_ChannelConfig_t {
    pub max_outgoing_messages_per_wakeup: u32,
    pub rx_max_messages_per_wakeup: u32,
    pub ack_timer_s: u32,
    pub nak_timer_s: u32,
    pub inactivity_timer_s: u32,
    pub ack_limit: u8,
    pub nak_limit: u8,
    pub mid_input: CFE_SB_MsgId_Atom_t,
    pub mid_output: CFE_SB_MsgId_Atom_t,
    pub pipe_depth_input: u16,
    pub polldir: [CF_PollDir_t; CF_MAX_POLLING_DIR_PER_CHAN],
    pub sem_name: [u8; OS_MAX_API_NAME],
    pub dequeue_enabled: u8,
    pub move_dir: [u8; OS_MAX_PATH_LEN],
}

impl Default for CF_ChannelConfig_t {
    fn default() -> Self {
        Self {
            max_outgoing_messages_per_wakeup: 0,
            rx_max_messages_per_wakeup: 0,
            ack_timer_s: 0,
            nak_timer_s: 0,
            inactivity_timer_s: 0,
            ack_limit: 0,
            nak_limit: 0,
            mid_input: 0,
            mid_output: 0,
            pipe_depth_input: 0,
            polldir: core::array::from_fn(|_| CF_PollDir_t::default()),
            sem_name: [0u8; OS_MAX_API_NAME],
            dequeue_enabled: 0,
            move_dir: [0u8; OS_MAX_PATH_LEN],
        }
    }
}

/// Top-level CFDP configuration table
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CF_ConfigTable_t {
    pub ticks_per_second: u32,
    pub rx_crc_calc_bytes_per_wakeup: u32,
    pub local_eid: CF_EntityId_t,
    pub chan: [CF_ChannelConfig_t; CF_NUM_CHANNELS],
    pub outgoing_file_chunk_size: u16,
    pub tmp_dir: [u8; CF_FILENAME_MAX_PATH],
    pub fail_dir: [u8; CF_FILENAME_MAX_PATH],
}

impl Default for CF_ConfigTable_t {
    fn default() -> Self {
        Self {
            ticks_per_second: 0,
            rx_crc_calc_bytes_per_wakeup: 0,
            local_eid: 0,
            chan: core::array::from_fn(|_| CF_ChannelConfig_t::default()),
            outgoing_file_chunk_size: 0,
            tmp_dir: [0u8; CF_FILENAME_MAX_PATH],
            fail_dir: [0u8; CF_FILENAME_MAX_PATH],
        }
    }
}
