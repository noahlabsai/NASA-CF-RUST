//! CF Application platform configuration constants.
//!
//! Translated from: default_cf_platform_cfg.h, default_cf_mission_cfg.h,
//!                   default_cf_extern_typedefs.h, cf_interface_cfg.h
//!
//! These are compile-time constants that define the sizing of all CF
//! data structures. In the C code these come from multiple header files
//! and the cFS build system. Here we consolidate them.

// =====================================================================
// cFE / OSAL constants (normally from cfe_mission_cfg.h / osconfig.h)
// =====================================================================

/// Maximum file name length (from OS_MAX_FILE_NAME)
pub const OS_MAX_FILE_NAME: usize = 64;

/// Maximum path length (from OS_MAX_PATH_LEN)
pub const OS_MAX_PATH_LEN: usize = 64;

/// Maximum API name length (from OS_MAX_API_NAME)
pub const OS_MAX_API_NAME: usize = 20;

/// CFE mission max path len
pub const CFE_MISSION_MAX_PATH_LEN: usize = 64;

/// CFE mission max file len
pub const CFE_MISSION_MAX_FILE_LEN: usize = 20;

// =====================================================================
// CF_FILENAME_MAX_LEN — from cf_interface_cfg.h
// =====================================================================

/// Maximum length of a filename (including path + name + NUL).
/// In C: CF_FILENAME_MAX_LEN from cf_interface_cfg.h
pub const CF_FILENAME_MAX_LEN: usize = 64;

/// Maximum file path (not including file name)
/// In C: CF_FILENAME_MAX_PATH = CFE_MISSION_MAX_PATH_LEN - CFE_MISSION_MAX_FILE_LEN
pub const CF_FILENAME_MAX_PATH: usize = CFE_MISSION_MAX_PATH_LEN - CFE_MISSION_MAX_FILE_LEN;

// =====================================================================
// Interface configuration values (from cf_interface_cfg.h)
// =====================================================================

/// Number of CF channels
pub const CF_NUM_CHANNELS: usize = 2;

/// Maximum number of NAK segments per NAK PDU
pub const CF_NAK_MAX_SEGMENTS: usize = 58;

/// Maximum number of polling directories per channel
pub const CF_MAX_POLLING_DIR_PER_CHAN: usize = 5;

/// Maximum commanded playback directories per channel
pub const CF_MAX_COMMANDED_PLAYBACK_DIRECTORIES_PER_CHAN: usize = 5;

/// Maximum commanded playback files per channel
pub const CF_MAX_COMMANDED_PLAYBACK_FILES_PER_CHAN: usize = 10;

/// Maximum simultaneous receive transactions
pub const CF_MAX_SIMULTANEOUS_RX: usize = 5;

/// Number of transactions per playback directory
pub const CF_NUM_TRANSACTIONS_PER_PLAYBACK: usize = 5;

/// Number of history entries per channel
pub const CF_NUM_HISTORIES_PER_CHANNEL: usize = 256;

/// Maximum PDU size
pub const CF_MAX_PDU_SIZE: usize = 512;

// =====================================================================
// Platform configuration (from default_cf_platform_cfg.h)
// =====================================================================

/// Total number of chunks (tx, rx, all channels)
/// In C: CF_TOTAL_CHUNKS = CF_NAK_MAX_SEGMENTS * 4
pub const CF_TOTAL_CHUNKS: usize = CF_NAK_MAX_SEGMENTS * 4;

/// Mission specific revision number
pub const CF_MISSION_REV: u32 = 0;

// =====================================================================
// Derived constants (from cf_cfdp_types.h)
// =====================================================================

/// Maximum possible number of transactions per channel
/// In C: CF_NUM_TRANSACTIONS_PER_CHANNEL
pub const CF_NUM_TRANSACTIONS_PER_CHANNEL: usize =
    CF_MAX_COMMANDED_PLAYBACK_FILES_PER_CHAN
        + CF_MAX_SIMULTANEOUS_RX
        + ((CF_MAX_POLLING_DIR_PER_CHAN + CF_MAX_COMMANDED_PLAYBACK_DIRECTORIES_PER_CHAN)
            * CF_NUM_TRANSACTIONS_PER_PLAYBACK);

/// Maximum possible number of transactions in the CF application
pub const CF_NUM_TRANSACTIONS: usize = CF_NUM_CHANNELS * CF_NUM_TRANSACTIONS_PER_CHANNEL;

/// Maximum possible number of history entries
pub const CF_NUM_HISTORIES: usize = CF_NUM_CHANNELS * CF_NUM_HISTORIES_PER_CHANNEL;

/// Maximum possible number of chunk entries
pub const CF_NUM_CHUNKS_ALL_CHANNELS: usize = CF_TOTAL_CHUNKS * CF_NUM_TRANSACTIONS_PER_CHANNEL;

/// Extra trailing bytes in PDU encapsulation (for CCSDS secondary header, etc.)
pub const CF_PDU_ENCAPSULATION_EXTRA_TRAILING_BYTES: usize = 0;
