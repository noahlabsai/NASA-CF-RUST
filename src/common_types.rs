//! Common type definitions matching cFE/OSAL common_types.h
//!
//! These are the basic integer type aliases and OSAL types used
//! throughout the CF application.

// =====================================================================
// OSAL types
// =====================================================================

/// OSAL object identifier (opaque handle)
pub type osal_id_t = u32;

/// OSAL invalid ID sentinel
pub const OS_OBJECT_ID_UNDEFINED: osal_id_t = 0;

/// OSAL success code
pub const OS_SUCCESS: i32 = 0;

// OS_MAX_FILE_NAME and OS_MAX_PATH_LEN are defined in cf_platform_cfg.rs

// =====================================================================
// CFE Status
// =====================================================================

/// CFE status return type
pub type CFE_Status_t = i32;

/// CFE success code
pub const CFE_SUCCESS: CFE_Status_t = 0;

/// CF application error code
pub const CF_ERROR: CFE_Status_t = -1;

// =====================================================================
// CFE Software Bus types
// =====================================================================

/// CFE SB message ID atom (integer form)
pub type CFE_SB_MsgId_Atom_t = u32;

/// CFE SB message ID type
pub type CFE_SB_MsgId_t = CFE_SB_MsgId_Atom_t;

/// CFE SB invalid message ID
pub const CFE_SB_INVALID_MSG_ID: CFE_SB_MsgId_t = 0xFFFFFFFF;

/// CFE SB reserved message ID
pub const CFE_SB_MSGID_RESERVED: CFE_SB_MsgId_t = 0xFFFFFFFE;

/// CFE Software Bus pipe ID
pub type CFE_SB_PipeId_t = u32;

/// CFE Software Bus buffer (opaque, used as pointer target)
/// In C this is a union with `Msg` and `Byte[]` members.
/// We represent `Msg` as a CFE_MSG_Message_t at offset 0.
#[repr(C)]
pub struct CFE_SB_Buffer_t {
    pub Msg: CFE_MSG_Message_t,
}

/// CFE message function code type
pub type CFE_MSG_FcnCode_t = u16;

/// CFE message type (opaque)
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CFE_MSG_Message_t {
    pub data: [u8; 0],
}

/// CFE message command header (simplified)
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CFE_MSG_CommandHeader_t {
    pub bytes: [u8; 8], // typical cFS command header size
}

/// CFE message telemetry header (simplified)
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CFE_MSG_TelemetryHeader_t {
    pub bytes: [u8; 12], // typical cFS telemetry header size
}

// =====================================================================
// CFE Table types
// =====================================================================

/// CFE Table handle
pub type CFE_TBL_Handle_t = i16;

// =====================================================================
// CFE Time types
// =====================================================================

/// CFE time type
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CFE_TIME_SysTime_t {
    pub seconds: u32,
    pub subseconds: u32,
}

// =====================================================================
// CFE Event Services
// =====================================================================

/// CFE Event Services event type
pub type CFE_EVS_EventType_t = u16;

pub const CFE_EVS_EventType_DEBUG: CFE_EVS_EventType_t = 1;
pub const CFE_EVS_EventType_INFORMATION: CFE_EVS_EventType_t = 2;
pub const CFE_EVS_EventType_ERROR: CFE_EVS_EventType_t = 3;
pub const CFE_EVS_EventType_CRITICAL: CFE_EVS_EventType_t = 4;

// =====================================================================
// CFE ES types
// =====================================================================

/// CFE ES run status
pub const CFE_ES_RunStatus_APP_RUN: u32 = 1;
pub const CFE_ES_RunStatus_APP_EXIT: u32 = 2;
pub const CFE_ES_RunStatus_APP_ERROR: u32 = 3;

// =====================================================================
// CFE stub functions (no-op in Rust-only build)
// =====================================================================

// CFE_EVS_SendEvent — stub. Real cFE is variadic printf-style.
// We use a macro to accept any number of arguments on stable Rust.
// All call sites must use CFE_EVS_SendEvent!(...) syntax.
macro_rules! CFE_EVS_SendEvent {
    ($($arg:expr),* $(,)?) => { { let _ = ($($arg,)*); } };
}
pub(crate) use CFE_EVS_SendEvent;
pub unsafe fn CFE_ES_PerfLogEntry(_id: u32) {}
pub unsafe fn CFE_ES_PerfLogExit(_id: u32) {}
pub unsafe fn CFE_ES_RunLoop(_run_status: *mut u32) -> bool { false }
pub unsafe fn CFE_ES_ExitApp(_run_status: u32) {}
pub unsafe fn CFE_SB_ReceiveBuffer(
    _buf: *mut *mut CFE_SB_Buffer_t,
    _pipe: CFE_SB_PipeId_t,
    _timeout: i32,
) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_SB_AllocateMessageBuffer(_size: usize) -> *mut CFE_SB_Buffer_t {
    std::ptr::null_mut()
}
pub unsafe fn CFE_SB_ReleaseMessageBuffer(_buf: *mut CFE_SB_Buffer_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_SB_TransmitBuffer(_buf: *mut CFE_SB_Buffer_t, _inc: bool) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_MSG_GetMsgId(_msg: *const CFE_MSG_Message_t, _id: *mut CFE_SB_MsgId_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_MSG_GetFcnCode(_msg: *const CFE_MSG_Message_t, _fc: *mut CFE_MSG_FcnCode_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_MSG_GetSize(_msg: *const CFE_MSG_Message_t, _size: *mut usize) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_MSG_SetMsgId(_msg: *mut CFE_MSG_Message_t, _id: CFE_SB_MsgId_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_MSG_GetMsgTime(_msg: *const CFE_MSG_Message_t, _time: *mut CFE_TIME_SysTime_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_SB_CreatePipe(_pipe: *mut CFE_SB_PipeId_t, _depth: u16, _name: *const std::os::raw::c_char) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_SB_Subscribe(_mid: CFE_SB_MsgId_t, _pipe: CFE_SB_PipeId_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_TBL_Register(_handle: *mut CFE_TBL_Handle_t, _name: *const std::os::raw::c_char, _size: usize, _opts: u16, _val: Option<unsafe extern "C" fn(*mut std::os::raw::c_void) -> CFE_Status_t>) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_TBL_Load(_handle: CFE_TBL_Handle_t, _src_type: u8, _src: *const std::os::raw::c_void) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_TBL_Manage(_handle: CFE_TBL_Handle_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_TBL_GetAddress(_addr: *mut *mut std::os::raw::c_void, _handle: CFE_TBL_Handle_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_TBL_ReleaseAddress(_handle: CFE_TBL_Handle_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_SB_ValueToMsgId(val: CFE_SB_MsgId_Atom_t) -> CFE_SB_MsgId_t { val }
pub unsafe fn CFE_SB_MsgIdToValue(mid: CFE_SB_MsgId_t) -> CFE_SB_MsgId_Atom_t { mid }
pub unsafe fn CFE_MSG_SetMsgTime(_msg: *mut CFE_MSG_Message_t, _time: CFE_TIME_SysTime_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_MSG_GenerateChecksum(_msg: *mut CFE_MSG_Message_t) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn CFE_MSG_SetSize(_msg: *mut CFE_MSG_Message_t, _size: usize) -> CFE_Status_t { CFE_SUCCESS }
pub unsafe fn OS_OpenCreate(_fd: *mut osal_id_t, _path: *const std::os::raw::c_char, _flags: i32, _access: i32) -> i32 { OS_SUCCESS }
pub unsafe fn OS_close(_fd: osal_id_t) -> i32 { OS_SUCCESS }
pub unsafe fn OS_read(_fd: osal_id_t, _buf: *mut std::os::raw::c_void, _nbytes: usize) -> i32 { 0 }
pub unsafe fn OS_write(_fd: osal_id_t, _buf: *const std::os::raw::c_void, _nbytes: usize) -> i32 { 0 }
pub unsafe fn OS_lseek(_fd: osal_id_t, _offset: i64, _whence: u32) -> i64 { 0 }
pub unsafe fn OS_rename(_old: *const std::os::raw::c_char, _new: *const std::os::raw::c_char) -> i32 { OS_SUCCESS }
pub unsafe fn OS_remove(_path: *const std::os::raw::c_char) -> i32 { OS_SUCCESS }
pub unsafe fn OS_DirectoryOpen(_dir: *mut osal_id_t, _path: *const std::os::raw::c_char) -> i32 { OS_SUCCESS }
pub unsafe fn OS_DirectoryClose(_dir: osal_id_t) -> i32 { OS_SUCCESS }
pub unsafe fn OS_DirectoryRead(_dir: osal_id_t, _entry: *mut std::os::raw::c_void) -> i32 { OS_SUCCESS }
pub unsafe fn OS_CountSemCreate(_sem: *mut osal_id_t, _name: *const std::os::raw::c_char, _init: u32, _opts: u32) -> i32 { OS_SUCCESS }
pub unsafe fn OS_CountSemGive(_sem: osal_id_t) -> i32 { OS_SUCCESS }
pub unsafe fn OS_CountSemTake(_sem: osal_id_t) -> i32 { OS_SUCCESS }

// =====================================================================
// Additional CFE status codes used by CF
// =====================================================================

/// CFE status validation failure
pub const CFE_STATUS_VALIDATION_FAILURE: CFE_Status_t = -70;

/// CFE TBL info updated (positive alt-success code)
pub const CFE_TBL_INFO_UPDATED: CFE_Status_t = 0x4C000001_u32 as CFE_Status_t;

/// CFE SB timeout status
pub const CFE_SB_TIME_OUT: CFE_Status_t = 0x4A000008_u32 as CFE_Status_t;

/// CFE SB no message status
pub const CFE_SB_NO_MESSAGE: CFE_Status_t = 0x4A000009_u32 as CFE_Status_t;

/// CFE EVS event filter type
pub const CFE_EVS_EventFilter_BINARY: u16 = 1;

/// CFE TBL options
pub const CFE_TBL_OPT_SNGL_BUFFER: u16 = 0;
pub const CFE_TBL_OPT_LOAD_DUMP: u16 = 0;

/// CFE TBL source type
pub const CFE_TBL_SRC_FILE: u8 = 1;

// =====================================================================
// Additional CFE stub functions
// =====================================================================

pub unsafe fn CFE_EVS_Register(
    _filters: *const std::os::raw::c_void,
    _num_filters: u16,
    _filter_scheme: u16,
) -> CFE_Status_t {
    CFE_SUCCESS
}

macro_rules! CFE_ES_WriteToSysLog {
    ($($arg:expr),* $(,)?) => {
        { let _ = ($($arg,)*); }
    };
}
pub(crate) use CFE_ES_WriteToSysLog;

pub unsafe fn CFE_MSG_Init(
    _msg: *mut CFE_MSG_Message_t,
    _mid: CFE_SB_MsgId_t,
    _size: usize,
) {}

/// Macro-like helper: in C this is `CFE_MSG_PTR(x)` which casts to `CFE_MSG_Message_t*`.
/// In Rust we just return a raw pointer to the field.
#[inline]
pub unsafe fn CFE_MSG_PTR<T>(header: &mut T) -> *mut CFE_MSG_Message_t {
    header as *mut T as *mut CFE_MSG_Message_t
}

// =====================================================================
// =====================================================================
// Missing constants, types, and shim functions
// =====================================================================

pub const CF_Direction_TX: u8 = 0;
pub const CF_Direction_RX: u8 = 1;
pub const CF_Direction_NUM: u8 = 2;

pub const CF_CFDP_CLASS_1: u8 = 0;
pub const CF_CFDP_CLASS_2: u8 = 1;

pub const CF_QueueIdx_FREE: u8 = 0;
pub const CF_QueueIdx_PEND: u8 = 1;
pub const CF_QueueIdx_TX: u8 = 2;
pub const CF_QueueIdx_TXA: u8 = 2;
pub const CF_QueueIdx_TXW: u8 = 3;
pub const CF_QueueIdx_RX: u8 = 4;
pub const CF_QueueIdx_HIST: u8 = 5;
pub const CF_QueueIdx_HIST_FREE: u8 = 6;
pub const CF_QueueIdx_NUM: usize = 7;

pub const CF_TickState_INIT: u8 = 0;
pub const CF_TickState_RX_STATE: u8 = 1;
pub const CF_TickState_TX_STATE: u8 = 2;
pub const CF_TickState_NAK: u8 = 3;
pub const CF_TickState_NEW_DATA: u8 = 4;
pub const CF_TickState_COMPLETE: u8 = 5;

pub const CF_TxnState_INIT: u8 = 0;

pub const CF_Reset_all: u8 = 0;
pub const CF_Reset_cmd: u8 = 1;
pub const CF_Reset_fault: u8 = 2;
pub const CF_Reset_up: u8 = 3;
pub const CF_Reset_down: u8 = 4;

pub const CF_TxnStatus_UNDEFINED: i32 = 0;
pub const CF_TxnStatus_FILESTORE_REJECTION: i32 = 1;
pub const CF_TxnStatus_READ_FAILURE: i32 = 2;
pub const CF_TxnStatus_INACTIVITY_DETECTED: i32 = 3;
pub const CF_TxnStatus_NAK_LIMIT_REACHED: i32 = 4;
pub const CF_TxnStatus_POS_ACK_LIMIT_REACHED: i32 = 5;
pub const CF_TxnStatus_EARLY_FIN: i32 = 6;
pub const CF_TxnStatus_NO_ERROR: i32 = 7;

pub const CF_CFDP_AckTxnStatus_ACTIVE: u8 = 1;
pub const CF_CFDP_AckTxnStatus_TERMINATED: u8 = 2;
pub const CF_CFDP_AckTxnStatus_UNRECOGNIZED: u8 = 3;

pub const CF_NUM_CFG_PACKET_CONDITIONS: usize = 1;

pub const OS_ERR_NAME_NOT_FOUND: i32 = -4;
pub const OS_FILE_FLAG_TRUNCATE: i32 = 0x0200;

pub const CF_DIR_MAX_CHUNKS: u32 = 1;

/// CF_Assert — panics in debug, no-op in release
#[cfg(debug_assertions)]
macro_rules! CF_Assert {
    ($cond:expr) => {
        if !($cond) {
            panic!("CF_Assert failed: {}", stringify!($cond));
        }
    };
}
#[cfg(not(debug_assertions))]
macro_rules! CF_Assert {
    ($cond:expr) => {};
}
pub(crate) use CF_Assert;

/// OS_ObjectIdDefined — checks if an OSAL ID is valid (non-zero)
#[inline]
pub fn OS_ObjectIdDefined(id: osal_id_t) -> bool {
    id != 0
}

/// OS_strnlen shim
#[inline]
pub unsafe fn OS_strnlen(s: *const std::os::raw::c_char, maxlen: usize) -> usize {
    let mut i = 0usize;
    while i < maxlen && *s.add(i) != 0 {
        i += 1;
    }
    i
}

/// OS_mv shim
pub unsafe fn OS_mv(_src: *const std::os::raw::c_char, _dst: *const std::os::raw::c_char) -> i32 {
    OS_SUCCESS
}

/// libc_strcmp shim
pub unsafe fn libc_strcmp(a: *const std::os::raw::c_char, b: *const std::os::raw::c_char) -> i32 {
    let mut i = 0usize;
    loop {
        let ca = *a.add(i) as u8;
        let cb = *b.add(i) as u8;
        if ca != cb { return (ca as i32) - (cb as i32); }
        if ca == 0 { return 0; }
        i += 1;
    }
}

/// libc_strncpy shim
pub unsafe fn libc_strncpy(dst: *mut std::os::raw::c_char, src: *const std::os::raw::c_char, n: usize) -> *mut std::os::raw::c_char {
    let mut i = 0usize;
    while i < n {
        let c = *src.add(i);
        *dst.add(i) = c;
        if c == 0 { break; }
        i += 1;
    }
    while i < n {
        *dst.add(i) = 0;
        i += 1;
    }
    dst
}

/// libc_strrchr shim
pub unsafe fn libc_strrchr(s: *const std::os::raw::c_char, c: i32) -> *const std::os::raw::c_char {
    let mut last: *const std::os::raw::c_char = std::ptr::null();
    let mut p = s;
    loop {
        if *p as i32 == c { last = p; }
        if *p == 0 { break; }
        p = p.add(1);
    }
    last
}

/// libc_strlen shim
pub unsafe fn libc_strlen(s: *const std::os::raw::c_char) -> usize {
    let mut i = 0usize;
    while *s.add(i) != 0 { i += 1; }
    i
}

// libc_snprintf shim — variadic snprintf for C-style format strings
// =====================================================================

/// Minimal snprintf shim. In a real cFE integration this would call libc::snprintf.
/// Here we just zero the buffer to avoid UB.
macro_rules! libc_snprintf {
    ($buf:expr, $size:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        {
            let _ = ($fmt $(, $arg)*);
            if !($buf as *mut std::os::raw::c_char).is_null() && $size > 0 {
                *($buf as *mut u8) = 0;
            }
        }
    };
}
pub(crate) use libc_snprintf;

// =====================================================================
// Additional OSAL / cFE stubs needed by implementation modules
// =====================================================================

pub unsafe fn OS_FileOpenCheck(_path: *const std::os::raw::c_char) -> i32 { -1 /* not open */ }
pub unsafe fn OS_CountSemGetIdByName(_sem: *mut osal_id_t, _name: *const std::os::raw::c_char) -> i32 { OS_SUCCESS }
pub unsafe fn OS_TaskDelay(_ms: u32) -> i32 { OS_SUCCESS }

/// os_dirent_t — matches OSAL directory entry
#[repr(C)]
#[derive(Clone)]
pub struct os_dirent_t {
    pub FileName: [std::os::raw::c_char; 64],
}

/// OS_DIRENTRY_NAME macro equivalent
#[inline]
pub unsafe fn OS_DIRENTRY_NAME(d: &os_dirent_t) -> *const std::os::raw::c_char {
    d.FileName.as_ptr()
}

pub const CF_STARTUP_SEM_TASK_DELAY: u32 = 100;
pub const CF_STARTUP_SEM_MAX_RETRIES: u32 = 25;

// NOTE: CFE_SB_TransmitBuffer, CFE_SB_AllocateMessageBuffer, and CF_QueueIdx_*
// are already defined earlier in this file. Do not duplicate.

// Error codes used in CFDP engine
pub const CF_SEND_PDU_NO_BUF_AVAIL_ERROR: CFE_Status_t = -10;
pub const CF_SHORT_PDU_ERROR: CFE_Status_t = -11;

// OS file flags
pub const OS_FILE_FLAG_NONE: i32 = 0;
pub const OS_FILE_FLAG_CREATE: i32 = 1;
pub const OS_READ_ONLY: i32 = 0;
pub const OS_READ_WRITE: i32 = 1;
pub const OS_WRITE_ONLY: i32 = 2;
pub const OS_SEEK_SET: u32 = 0;
pub const OS_SEEK_CUR: u32 = 1;
pub const OS_SEEK_END: u32 = 2;
