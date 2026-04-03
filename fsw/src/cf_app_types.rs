//! CF Application types.
//!
//! Translated from: cf_app.h

use crate::common_types::{CFE_Status_t, CFE_SB_PipeId_t, CFE_TBL_Handle_t};
use crate::cf_msg::{CF_HkPacket_t, CF_EotPacket_t, CF_ConfigTable_t};
use crate::cf_cfdp_types::CF_Engine_t;

// =====================================================================
// Error codes
// =====================================================================

pub const CF_PDU_METADATA_ERROR: CFE_Status_t = -2;
pub const CF_SHORT_PDU_ERROR: CFE_Status_t = -3;
pub const CF_REC_PDU_FSIZE_MISMATCH_ERROR: CFE_Status_t = -4;
pub const CF_REC_PDU_BAD_EOF_ERROR: CFE_Status_t = -5;
pub const CF_SEND_PDU_NO_BUF_AVAIL_ERROR: CFE_Status_t = -6;
pub const CF_SEND_PDU_ERROR: CFE_Status_t = -7;

// =====================================================================
// String constants
// =====================================================================

pub const CF_PIPE_NAME: &[u8] = b"CF_CMD_PIPE\0";
pub const CF_CHANNEL_PIPE_PREFIX: &[u8] = b"CF_CHAN_\0";
pub const CF_FILENAME_TRUNCATED: u8 = b'$';

/// Pipe depth for the CF command pipe
pub const CF_PIPE_DEPTH: u16 = 32;

/// Timeout for CFE_SB_ReceiveBuffer (in milliseconds)
/// CFE_SB_PEND_FOREVER = -1 in C; we use a wakeup-driven timeout
pub const CF_RCVMSG_TIMEOUT: i32 = 100;

/// Configuration table name
pub const CF_CONFIG_TABLE_NAME: &[u8] = b"CF_CONFIG_TABLE\0";

/// Configuration table default filename
pub const CF_CONFIG_TABLE_FILENAME: &[u8] = b"/cf/cf_def_config.tbl\0";

/// Housekeeping telemetry MID (from cf_msgids)
pub const CF_HK_TLM_MID: u32 = 0x08B0;

/// End-of-transaction telemetry MID
pub const CF_EOT_TLM_MID: u32 = 0x08B1;

// =====================================================================
// Application global state
// =====================================================================

/// The CF application global state structure
///
/// Matches C `CF_AppData_t` from cf_app.h
#[repr(C)]
pub struct CF_AppData_t {
    pub hk: CF_HkPacket_t,
    pub RunStatus: u32,
    pub CmdPipe: CFE_SB_PipeId_t,
    pub config_handle: CFE_TBL_Handle_t,
    pub config_table: *mut CF_ConfigTable_t,
    pub engine: CF_Engine_t,
}
