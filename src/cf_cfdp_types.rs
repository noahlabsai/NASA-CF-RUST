//! CF CFDP type definitions.
//!
//! Translated from: cf_cfdp_types.h
//!
//! Macros and data types used across the CF application.

use crate::common_types::*;
use crate::cf_platform_cfg::*;
use crate::cf_extern_typedefs::*;
use crate::cf_msg::*;
use crate::cf_cfdp_pdu::*;
pub use crate::cf_logical_pdu::*;
use crate::cf_clist_types::*;
use crate::cf_chunk_types::*;
use crate::cf_timer_types::*;
use crate::cf_crc_types::*;
use crate::cf_codec_types::*;

// =====================================================================
// Transaction state enums
// =====================================================================

/// High-level state of a transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum CF_TxnState_t {
    #[default]
    CF_TxnState_UNDEF   = 0,
    CF_TxnState_INIT    = 1,
    CF_TxnState_R1      = 2,
    CF_TxnState_S1      = 3,
    CF_TxnState_R2      = 4,
    CF_TxnState_S2      = 5,
    CF_TxnState_DROP    = 6,
    CF_TxnState_HOLD    = 7,
    CF_TxnState_INVALID = 8,
}

/// Sub-state of a send file transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum CF_TxSubState_t {
    #[default]
    CF_TxSubState_DATA_NORMAL = 0,
    CF_TxSubState_DATA_EOF    = 1,
    CF_TxSubState_FILESTORE   = 2,
    CF_TxSubState_COMPLETE    = 3,
    CF_TxSubState_NUM_STATES  = 4,
}

/// Sub-state of a receive file transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum CF_RxSubState_t {
    #[default]
    CF_RxSubState_DATA_NORMAL = 0,
    CF_RxSubState_DATA_EOF    = 1,
    CF_RxSubState_VALIDATE    = 2,
    CF_RxSubState_FILESTORE   = 3,
    CF_RxSubState_FINACK      = 4,
    CF_RxSubState_COMPLETE    = 5,
    CF_RxSubState_NUM_STATES  = 6,
}

/// Direction identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum CF_Direction_t {
    #[default]
    CF_Direction_RX  = 0,
    CF_Direction_TX  = 1,
}

/// Number of direction values
pub const CF_Direction_NUM: usize = 2;

// =====================================================================
// Const array-size aliases (used by dispatch tables)
// =====================================================================

/// Number of valid TxnState values (used as array size)
pub const CF_TxnState_INVALID: usize = CF_TxnState_t::CF_TxnState_INVALID as usize;

/// Number of file directive codes (used as array size)
pub const CF_CFDP_FileDirective_INVALID_MAX: usize = CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_INVALID_MAX as usize;

/// Number of RX sub-states (used as array size)
pub const CF_RxSubState_NUM_STATES: usize = CF_RxSubState_t::CF_RxSubState_NUM_STATES as usize;

/// Number of TX sub-states (used as array size)
pub const CF_TxSubState_NUM_STATES: usize = CF_TxSubState_t::CF_TxSubState_NUM_STATES as usize;

// =====================================================================
// Transaction status (extended condition codes)
// =====================================================================

/// Transaction status code
///
/// Superset of CFDP condition codes with additional local status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CF_TxnStatus_t {
    CF_TxnStatus_UNDEFINED = -1,

    // Status codes 0-15 share values with CFDP condition codes
    CF_TxnStatus_NO_ERROR                  = 0,
    CF_TxnStatus_POS_ACK_LIMIT_REACHED     = 1,
    CF_TxnStatus_KEEP_ALIVE_LIMIT_REACHED  = 2,
    CF_TxnStatus_INVALID_TRANSMISSION_MODE = 3,
    CF_TxnStatus_FILESTORE_REJECTION       = 4,
    CF_TxnStatus_FILE_CHECKSUM_FAILURE     = 5,
    CF_TxnStatus_FILE_SIZE_ERROR           = 6,
    CF_TxnStatus_NAK_LIMIT_REACHED         = 7,
    CF_TxnStatus_INACTIVITY_DETECTED       = 8,
    CF_TxnStatus_INVALID_FILE_STRUCTURE    = 9,
    CF_TxnStatus_CHECK_LIMIT_REACHED       = 10,
    CF_TxnStatus_UNSUPPORTED_CHECKSUM_TYPE = 11,
    CF_TxnStatus_SUSPEND_REQUEST_RECEIVED  = 14,
    CF_TxnStatus_CANCEL_REQUEST_RECEIVED   = 15,

    // Additional status codes (16+)
    CF_TxnStatus_PROTOCOL_ERROR     = 16,
    CF_TxnStatus_ACK_LIMIT_NO_FIN   = 17,
    CF_TxnStatus_ACK_LIMIT_NO_EOF   = 18,
    CF_TxnStatus_NAK_RESPONSE_ERROR = 19,
    CF_TxnStatus_SEND_EOF_FAILURE   = 20,
    CF_TxnStatus_EARLY_FIN          = 21,
    CF_TxnStatus_READ_FAILURE       = 22,
    CF_TxnStatus_NO_RESOURCE        = 23,
    CF_TxnStatus_MAX                = 24,
}

impl Default for CF_TxnStatus_t {
    fn default() -> Self {
        Self::CF_TxnStatus_UNDEFINED
    }
}

// =====================================================================
// History entry
// =====================================================================

/// CF History entry — records completed operations
#[repr(C)]
pub struct CF_History_t {
    pub fnames: CF_TxnFilenames_t,
    pub cl_node: CF_CListNode_t,
    pub dir: CF_Direction_t,
    pub txn_stat: CF_TxnStatus_t,
    pub src_eid: CF_EntityId_t,
    pub peer_eid: CF_EntityId_t,
    pub seq_num: CF_TransactionSeq_t,
}

// =====================================================================
// Chunk wrapper
// =====================================================================

/// Wrapper around CF_ChunkList_t for use in CList
#[repr(C)]
pub struct CF_ChunkWrapper_t {
    pub chunks: CF_ChunkList_t,
    pub cl_node: CF_CListNode_t,
}

// =====================================================================
// Playback / Poll
// =====================================================================

/// CF Playback entry
#[repr(C)]
pub struct CF_Playback_t {
    pub dir_id: osal_id_t,
    pub cfdp_class: CF_CFDP_Class_t,
    pub fnames: CF_TxnFilenames_t,
    pub num_ts: u16,
    pub priority: u8,
    pub dest_id: CF_EntityId_t,
    pub pending_file: [u8; OS_MAX_FILE_NAME],
    pub busy: bool,
    pub diropen: bool,
    pub keep: bool,
    pub counted: bool,
}

/// CF Poll entry
#[repr(C)]
pub struct CF_Poll_t {
    pub pb: CF_Playback_t,
    pub interval_timer: CF_Timer_t,
    pub timer_set: bool,
}

// =====================================================================
// Transaction flags
// =====================================================================

/// Flags common to all transaction types
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Flags_Common_t {
    pub q_index: u8,
    pub close_req: bool,
    pub ack_timer_armed: bool,
    pub suspended: bool,
    pub canceled: bool,
    pub is_complete: bool,
    pub crc_complete: bool,
    pub inactivity_fired: bool,
    pub keep_history: bool,
}

/// Flags for receive transactions
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Flags_Rx_t {
    pub com: CF_Flags_Common_t,
    pub tempfile_created: bool,
    pub md_recv: bool,
    pub eof_count: u8,
    pub eof_ack_count: u8,
    pub finack_recv: bool,
    pub send_nak: bool,
    pub send_fin: bool,
}

/// Flags for send transactions
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Flags_Tx_t {
    pub com: CF_Flags_Common_t,
    pub cmd_tx: bool,
    pub fd_nak_pending: bool,
    pub eof_ack_recv: bool,
    pub fin_count: u8,
    pub fin_ack_count: u8,
    pub send_md: bool,
    pub send_eof: bool,
}

/// Union of all possible transaction flags
#[derive(Clone, Copy)]
#[repr(C)]
pub union CF_StateFlags_t {
    pub com: CF_Flags_Common_t,
    pub rx: CF_Flags_Rx_t,
    pub tx: CF_Flags_Tx_t,
}

impl Default for CF_StateFlags_t {
    fn default() -> Self {
        Self {
            com: CF_Flags_Common_t::default(),
        }
    }
}

impl core::fmt::Debug for CF_StateFlags_t {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CF_StateFlags_t {{ ... }}")
    }
}

// =====================================================================
// Transaction state data
// =====================================================================

/// Summary of all possible transaction state information
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_StateData_t {
    pub sub_state: u8,
    pub acknak_count: u8,
    pub peer_cc: u8,
    pub fin_dc: u8,
    pub fin_fs: u8,
    pub cached_pos: CF_FileSize_t,
    pub eof_crc: u32,
    pub eof_size: CF_FileSize_t,
}

// =====================================================================
// Transaction
// =====================================================================

/// Transaction state object
#[repr(C)]
pub struct CF_Transaction_t {
    pub state: CF_TxnState_t,
    pub history: *mut CF_History_t,
    pub chunks: *mut CF_ChunkWrapper_t,
    pub inactivity_timer: CF_Timer_t,
    pub ack_timer: CF_Timer_t,
    pub fsize: CF_FileSize_t,
    pub foffs: CF_FileSize_t,
    pub fd: osal_id_t,
    pub crc: CF_Crc_t,
    pub reliable_mode: bool,
    pub keep: u8,
    pub chan_num: u8,
    pub priority: u8,
    pub cl_node: CF_CListNode_t,
    pub pb: *mut CF_Playback_t,
    pub state_data: CF_StateData_t,
    pub flags: CF_StateFlags_t,
}

// =====================================================================
// Tick state
// =====================================================================

/// Identifies the type of timer tick being processed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_TickState_t {
    CF_TickState_INIT         = 0,
    CF_TickState_RX_STATE     = 1,
    CF_TickState_TX_STATE     = 2,
    CF_TickState_TX_NAK       = 3,
    CF_TickState_TX_FILEDATA  = 4,
    CF_TickState_TX_PEND      = 5,
    CF_TickState_COMPLETE     = 6,
    CF_TickState_NUM_TYPES    = 7,
}

// =====================================================================
// Channel
// =====================================================================

/// Channel state object
#[repr(C)]
pub struct CF_Channel_t {
    pub qs: [*mut CF_CListNode_t; CF_QueueIdx_NUM],
    pub cs: [*mut CF_CListNode_t; CF_Direction_NUM],
    pub pipe: CFE_SB_PipeId_t,
    pub num_cmd_tx: u32,
    pub playback: [CF_Playback_t; CF_MAX_COMMANDED_PLAYBACK_DIRECTORIES_PER_CHAN],
    pub poll: [CF_Poll_t; CF_MAX_POLLING_DIR_PER_CHAN],
    pub sem_id: osal_id_t,
    pub outgoing_counter: u32,
    pub tick_resume: *const CF_Transaction_t,
    pub tx_blocked: bool,
}

// =====================================================================
// Engine I/O
// =====================================================================

/// CF engine output state
#[repr(C)]
pub struct CF_Output_t {
    pub msg: *mut CFE_SB_Buffer_t,
    pub encode: CF_EncoderState_t,
    pub tx_pdudata: CF_Logical_PduBuffer_t,
}

/// CF engine input state
#[repr(C)]
pub struct CF_Input_t {
    pub msg: *mut CFE_SB_Buffer_t,
    pub decode: CF_DecoderState_t,
    pub rx_pdudata: CF_Logical_PduBuffer_t,
}

// =====================================================================
// Engine
// =====================================================================

/// CF Engine — represents a pairing to a local EID
#[repr(C)]
pub struct CF_Engine_t {
    pub seq_num: CF_TransactionSeq_t,
    pub out: CF_Output_t,
    pub r#in: CF_Input_t,
    pub transactions: [CF_Transaction_t; CF_NUM_TRANSACTIONS],
    pub histories: [CF_History_t; CF_NUM_HISTORIES],
    pub channels: [CF_Channel_t; CF_NUM_CHANNELS],
    pub chunks: [CF_ChunkWrapper_t; CF_NUM_TRANSACTIONS * CF_Direction_NUM],
    pub chunk_mem: [CF_Chunk_t; CF_NUM_CHUNKS_ALL_CHANNELS],
    pub enabled: bool,
}

// =====================================================================
// Application Data
// =====================================================================

// CF_AppData_t is defined in cf_app_types.rs
// CFE_SB_PipeId_t and CFE_TBL_Handle_t are defined in common_types.rs
