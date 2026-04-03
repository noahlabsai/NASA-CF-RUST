//! NASA CF (CFDP) Application - Rust Translation
//!
//! This is a 1:1 translation of the NASA CFS CFDP (CF) Application
//! from C to Rust.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::all)]
#![allow(unused_unsafe)]

#![allow(unused_assignments)]
#![allow(unused_parens)]

/// `container_of` macro — given a pointer to a struct member, returns a pointer
/// to the containing struct. Matches the C `container_of` macro.
macro_rules! container_of {
    ($ptr:expr, $type:ty, $field:ident) => {{
        let __ptr = $ptr as *const u8;
        let __offset = core::mem::offset_of!($type, $field);
        (__ptr.sub(__offset)) as *mut $type
    }};
}
pub(crate) use container_of;

// ============================================================
// Layer 0: Foundation (no CF dependencies)
// ============================================================

pub mod common_types;
pub mod cf_platform_cfg;
pub mod cf_extern_typedefs;
pub mod cf_assert;

// ============================================================
// Layer 1: Core type definitions
// ============================================================

pub mod cf_crc_types;
pub mod cf_timer_types;
pub mod cf_clist_types;
pub mod cf_chunk_types;
pub mod cf_msg;

// ============================================================
// Layer 2: Core logic modules
// ============================================================

pub mod cf_crc;
pub mod cf_timer;
pub mod cf_clist;
pub mod cf_chunk;

// ============================================================
// Layer 3: PDU definitions
// ============================================================

pub mod cf_cfdp_pdu;
pub mod cf_logical_pdu;
pub mod cf_eventids;
pub mod cf_perfids;

// ============================================================
// Layer 4: Codec
// ============================================================

pub mod cf_codec_types;
pub mod cf_codec;

// ============================================================
// Layer 5: CFDP types (depends on everything above)
// ============================================================

pub mod cf_cfdp_types;

// ============================================================
// Layer 6: Application types and utilities
// ============================================================

pub mod cf_app_types;
pub mod cf_cmd_types;
pub mod cf_utils_types;
pub mod cf_cfdp_r_types;
pub mod cf_cfdp_s_types;
pub mod cf_cfdp_dispatch_types;
pub mod cf_cfdp_sbintf_types;
pub mod cf_dispatch_types;

// ============================================================
// Layer 7: Implementation modules
// ============================================================

pub mod cf_utils;
pub mod cf_cfdp;
pub mod cf_cfdp_r;
pub mod cf_cfdp_s;
pub mod cf_cfdp_dispatch;
pub mod cf_cfdp_sbintf;
pub mod cf_cmd;
pub mod cf_dispatch;
pub mod cf_app;
pub mod cf_eds_dispatch;
pub mod cf_verify;
pub mod cf_version;
