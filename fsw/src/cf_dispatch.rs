//! CF Application command dispatch module.
//!
//! Translated from: cf_dispatch.c / cf_dispatch.h
//!
//! Routes incoming SB messages to the appropriate handler based on
//! message ID and function code.

use crate::common_types::*;
use crate::cf_platform_cfg::*;
use crate::cf_msg::*;
use crate::cf_cfdp_types::*;
use crate::cf_eventids::*;
use crate::cf_app_types::CF_AppData_t;

// =====================================================================
// Function code constants (from default_cf_fcncode_values.h)
// =====================================================================

pub const CF_NOOP_CC: u16                = 0;
pub const CF_RESET_CC: u16              = 1;
pub const CF_TX_FILE_CC: u16            = 2;
pub const CF_PLAYBACK_DIR_CC: u16       = 3;
pub const CF_FREEZE_CC: u16             = 4;
pub const CF_THAW_CC: u16               = 5;
pub const CF_SUSPEND_CC: u16            = 6;
pub const CF_RESUME_CC: u16             = 7;
pub const CF_CANCEL_CC: u16             = 8;
pub const CF_ABANDON_CC: u16            = 9;
pub const CF_SET_PARAM_CC: u16          = 10;
pub const CF_GET_PARAM_CC: u16          = 11;
pub const CF_WRITE_QUEUE_CC: u16        = 15;
pub const CF_ENABLE_DEQUEUE_CC: u16     = 16;
pub const CF_DISABLE_DEQUEUE_CC: u16    = 17;
pub const CF_ENABLE_DIR_POLLING_CC: u16 = 18;
pub const CF_DISABLE_DIR_POLLING_CC: u16 = 19;
pub const CF_PURGE_QUEUE_CC: u16        = 21;
pub const CF_ENABLE_ENGINE_CC: u16      = 22;
pub const CF_DISABLE_ENGINE_CC: u16     = 23;

/// Number of entries in the dispatch table (max CC + 1)
const NUM_DISPATCH_ENTRIES: usize = 24;

/// Command handler function pointer type.
/// C original: `typedef void (*const handler_fn_t)(const void *)`
pub type CF_CmdHandlerFunc_t = Option<unsafe fn(*const CFE_SB_Buffer_t)>;

// =====================================================================
// CF_ProcessGroundCommand
// =====================================================================

/// Process a ground command message.
///
/// C original: `void CF_ProcessGroundCommand(const CFE_SB_Buffer_t *BufPtr)`
///
/// Validates the function code and message length, then dispatches to
/// the appropriate command handler.
///
/// # Safety
/// `buf_ptr` must be a valid pointer to a CFE_SB_Buffer_t.
pub unsafe fn CF_ProcessGroundCommand(buf_ptr: *const CFE_SB_Buffer_t) {
    // Handler function table indexed by function code
    static FNS: [CF_CmdHandlerFunc_t; NUM_DISPATCH_ENTRIES] = {
        let mut table: [CF_CmdHandlerFunc_t; NUM_DISPATCH_ENTRIES] = [None; NUM_DISPATCH_ENTRIES];
        // These will be filled in by cf_cmd.rs functions.
        // For now they are None (no-op). In the real build, each entry
        // would be Some(CF_NoopCmd), Some(CF_ResetCountersCmd), etc.
        table
    };

    // Expected message lengths indexed by function code
    static EXPECTED_LENGTHS: [u16; NUM_DISPATCH_ENTRIES] = {
        let mut table = [0u16; NUM_DISPATCH_ENTRIES];
        table[CF_NOOP_CC as usize]                = core::mem::size_of::<CF_NoopCmd_t>() as u16;
        table[CF_RESET_CC as usize]               = core::mem::size_of::<CF_ResetCountersCmd_t>() as u16;
        table[CF_TX_FILE_CC as usize]             = core::mem::size_of::<CF_TxFileCmd_t>() as u16;
        table[CF_PLAYBACK_DIR_CC as usize]        = core::mem::size_of::<CF_PlaybackDirCmd_t>() as u16;
        table[CF_FREEZE_CC as usize]              = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_THAW_CC as usize]                = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_SUSPEND_CC as usize]             = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_RESUME_CC as usize]              = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_CANCEL_CC as usize]              = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_ABANDON_CC as usize]             = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_SET_PARAM_CC as usize]           = core::mem::size_of::<CF_SetParamCmd_t>() as u16;
        table[CF_GET_PARAM_CC as usize]           = core::mem::size_of::<CF_GetParamCmd_t>() as u16;
        table[CF_WRITE_QUEUE_CC as usize]         = core::mem::size_of::<CF_WriteQueueCmd_t>() as u16;
        table[CF_ENABLE_DEQUEUE_CC as usize]      = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_DISABLE_DEQUEUE_CC as usize]     = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_ENABLE_DIR_POLLING_CC as usize]  = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_DISABLE_DIR_POLLING_CC as usize] = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_PURGE_QUEUE_CC as usize]         = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_ENABLE_ENGINE_CC as usize]       = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table[CF_DISABLE_ENGINE_CC as usize]      = core::mem::size_of::<CF_UnionArgsCmd_t>() as u16;
        table
    };

    let mut cmd: CFE_MSG_FcnCode_t = 0;
    let mut len: usize = 0;

    CFE_MSG_GetFcnCode(buf_ptr, &mut cmd);

    if (cmd as usize) < NUM_DISPATCH_ENTRIES {
        CFE_MSG_GetSize(buf_ptr, &mut len);

        if len == EXPECTED_LENGTHS[cmd as usize] as usize {
            if let Some(handler) = FNS[cmd as usize] {
                handler(buf_ptr);
            }
        } else {
            CFE_EVS_SendEvent!(
                CF_CMD_LEN_ERR_EID,
                CFE_EVS_EventType_ERROR,
                cmd,
                EXPECTED_LENGTHS[cmd as usize],
                len as u16,
            );
            (*CF_AppData_ptr()).hk.Payload.counters.err += 1;
        }
    } else {
        CFE_EVS_SendEvent!(
            CF_CC_ERR_EID,
            CFE_EVS_EventType_ERROR,
            cmd,
            0,
            0,
        );
        (*CF_AppData_ptr()).hk.Payload.counters.err += 1;
    }
}

// =====================================================================
// CF_AppPipe
// =====================================================================

/// Main application message pipe handler.
///
/// C original: `void CF_AppPipe(const CFE_SB_Buffer_t *BufPtr)`
///
/// Routes incoming SB messages to the appropriate handler based on
/// message ID.
///
/// # Safety
/// `buf_ptr` must be a valid pointer to a CFE_SB_Buffer_t.
pub unsafe fn CF_AppPipe(buf_ptr: *const CFE_SB_Buffer_t) {
    static mut CMD_MID: CFE_SB_MsgId_t = CFE_SB_MSGID_RESERVED;
    static mut SEND_HK_MID: CFE_SB_MsgId_t = CFE_SB_MSGID_RESERVED;
    static mut WAKE_UP_MID: CFE_SB_MsgId_t = CFE_SB_MSGID_RESERVED;

    let mut msg_id: CFE_SB_MsgId_t = CFE_SB_INVALID_MSG_ID;

    // Cache the local MID values here, this avoids repeat lookups
    if !CFE_SB_IsValidMsgId(CMD_MID) {
        CMD_MID     = CFE_SB_ValueToMsgId(CF_CMD_MID);
        SEND_HK_MID = CFE_SB_ValueToMsgId(CF_SEND_HK_MID);
        WAKE_UP_MID = CFE_SB_ValueToMsgId(CF_WAKE_UP_MID);
    }

    CFE_MSG_GetMsgId(buf_ptr, &mut msg_id);

    if CFE_SB_MsgId_Equal(msg_id, WAKE_UP_MID) {
        CF_WakeupCmd(buf_ptr);
    } else if CFE_SB_MsgId_Equal(msg_id, SEND_HK_MID) {
        CF_SendHkCmd(buf_ptr);
    } else if CFE_SB_MsgId_Equal(msg_id, CMD_MID) {
        CF_ProcessGroundCommand(buf_ptr);
    } else {
        (*CF_AppData_ptr()).hk.Payload.counters.err += 1;
        CFE_EVS_SendEvent!(
            CF_MID_ERR_EID,
            CFE_EVS_EventType_ERROR,
            CFE_SB_MsgIdToValue(msg_id) as u16,
            0,
            0,
        );
    }
}

// =====================================================================
// cFE API stubs (to be replaced with real FFI bindings)
// =====================================================================

// Message ID values (from cf_topicids / cf_msgids)
pub const CF_CMD_MID: u32     = 0x18B3;
pub const CF_SEND_HK_MID: u32 = 0x18B4;
pub const CF_WAKE_UP_MID: u32 = 0x18B5;

pub const CFE_EVS_EventType_ERROR: u16 = 1;

/// Extract the function code from a cFE SB message buffer.
///
/// In the real cFE the function code is at a fixed offset inside the
/// CCSDS secondary header.  We replicate that layout here:
///   primary header  = 6 bytes (3 × u16)
///   sec hdr byte[0] = function code
#[inline]
pub unsafe fn CFE_MSG_GetFcnCode(buf: *const CFE_SB_Buffer_t, cmd: &mut CFE_MSG_FcnCode_t) {
    /* CCSDS v1 command secondary header: fc is first byte after 6-byte primary */
    let base = buf as *const u8;
    let fc_byte = *base.add(6);
    /* per CCSDS, function code is bits 6..0 (mask 0x7F) */
    *cmd = (fc_byte & 0x7F) as CFE_MSG_FcnCode_t;
}

/// Extract the total message size from a cFE SB message buffer.
///
/// CCSDS primary header stores (length - 7) in bytes 4..5 (big-endian).
#[inline]
pub unsafe fn CFE_MSG_GetSize(buf: *const CFE_SB_Buffer_t, len: &mut usize) {
    let base = buf as *const u8;
    let len_field = ((*base.add(4) as u16) << 8) | (*base.add(5) as u16);
    *len = (len_field as usize) + 7;
}

/// Extract the message ID from a cFE SB message buffer.
///
/// CCSDS v1 message ID is the first 2 bytes (big-endian).
#[inline]
pub unsafe fn CFE_MSG_GetMsgId(buf: *const CFE_SB_Buffer_t, msg_id: &mut CFE_SB_MsgId_t) {
    let base = buf as *const u8;
    *msg_id = ((*base.add(0) as u32) << 8) | (*base.add(1) as u32);
}

#[inline]
pub fn CFE_SB_IsValidMsgId(mid: CFE_SB_MsgId_t) -> bool {
    mid != CFE_SB_MSGID_RESERVED && mid != CFE_SB_INVALID_MSG_ID
}

#[inline]
pub fn CFE_SB_ValueToMsgId(val: u32) -> CFE_SB_MsgId_t { val }

#[inline]
pub fn CFE_SB_MsgIdToValue(mid: CFE_SB_MsgId_t) -> u32 { mid }

#[inline]
pub fn CFE_SB_MsgId_Equal(a: CFE_SB_MsgId_t, b: CFE_SB_MsgId_t) -> bool { a == b }

/// Stub command handlers — these are implemented in cf_cmd.rs
pub unsafe fn CF_WakeupCmd(_msg: *const CFE_SB_Buffer_t) {}
pub unsafe fn CF_SendHkCmd(_msg: *const CFE_SB_Buffer_t) {}

/// Returns a mutable pointer to the global CF_AppData singleton.
/// In the real build this is `extern CF_AppData_t CF_AppData;`
pub unsafe fn CF_AppData_ptr() -> *mut CF_AppData_t {
    &raw mut crate::cf_app::CF_AppData as *mut CF_AppData_t
}
