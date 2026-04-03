//! Type declarations for CF Application dispatch module.
//!
//! This corresponds to cf_dispatch.h in the C source.
//! The actual function implementations are in cf_dispatch.rs.
//!
//! No types are defined in cf_dispatch.h — it only declares function prototypes
//! (`CF_ProcessGroundCommand` and `CF_AppPipe`).
//! In Rust, the functions are simply `pub fn` in cf_dispatch.rs, so this file
//! is intentionally minimal.
