//! CF EDS-based dispatch (alternative to cf_dispatch.rs).
//!
//! Translated from: cf_eds_dispatch.c
//!
//! NOTE: The EDS dispatch variant is only used when the cFS build system
//! selects EDS-based message routing. For the default (non-EDS) build,
//! cf_dispatch.rs is used instead. This module is compiled but the
//! functions are not called unless EDS is active.
//!
//! This is a placeholder — the real EDS dispatch requires the EDS
//! code generator output which is build-system specific.

use crate::common_types::*;
use crate::cf_cfdp_types::*;

/// Placeholder for EDS-based command pipe processing.
///
/// In the real EDS build, this would use auto-generated dispatch tables
/// from the EDS toolchain. For the default build, cf_dispatch::CF_AppPipe
/// is used instead.
pub fn CF_AppPipe_EDS(_buf: *const CFE_SB_Buffer_t) {
    // EDS dispatch not active in this build configuration
}
