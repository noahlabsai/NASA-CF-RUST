//! CF Codec type definitions.
//!
//! Translated from: cf_codec.h (type definitions only)
//!
//! These are the encoder/decoder state structures used by the codec.

use crate::common_types::CFE_Status_t;

/// Tracks the current state of an encode or decode operation
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CodecState_t {
    /// Whether decode is valid or not
    pub is_valid: bool,
    /// Offset of next byte to encode/decode
    pub next_offset: usize,
    /// Maximum number of bytes in the PDU
    pub max_size: usize,
}

/// Current state of an encode operation
#[derive(Debug)]
#[repr(C)]
pub struct CF_EncoderState_t {
    pub codec_state: CF_CodecState_t,
    /// Pointer to start of encoded PDU data
    pub base: *mut u8,
}

impl Default for CF_EncoderState_t {
    fn default() -> Self {
        Self {
            codec_state: CF_CodecState_t::default(),
            base: core::ptr::null_mut(),
        }
    }
}

/// Current state of a decode operation
#[derive(Debug)]
#[repr(C)]
pub struct CF_DecoderState_t {
    pub codec_state: CF_CodecState_t,
    /// Pointer to start of encoded PDU data (read-only)
    pub base: *const u8,
}

impl Default for CF_DecoderState_t {
    fn default() -> Self {
        Self {
            codec_state: CF_CodecState_t::default(),
            base: core::ptr::null(),
        }
    }
}

// =====================================================================
// Inline utility functions (from cf_codec.h)
// =====================================================================

/// Checks if the codec is currently valid
#[inline]
pub fn CF_CFDP_CodecIsOK(state: &CF_CodecState_t) -> bool {
    state.is_valid
}

/// Sets a codec to the "done" state
#[inline]
pub fn CF_CFDP_CodecSetDone(state: &mut CF_CodecState_t) {
    state.is_valid = false;
}

/// Obtains the current position/offset within the PDU
#[inline]
pub fn CF_CFDP_CodecGetPosition(state: &CF_CodecState_t) -> usize {
    state.next_offset
}

/// Obtains the maximum size of the PDU
#[inline]
pub fn CF_CFDP_CodecGetSize(state: &CF_CodecState_t) -> usize {
    state.max_size
}

/// Obtains the remaining size of the PDU
#[inline]
pub fn CF_CFDP_CodecGetRemain(state: &CF_CodecState_t) -> usize {
    state.max_size - state.next_offset
}

/// Resets a codec state
#[inline]
pub fn CF_CFDP_CodecReset(state: &mut CF_CodecState_t, max_size: usize) {
    state.is_valid = true;
    state.next_offset = 0;
    state.max_size = max_size;
}

// =====================================================================
// Macro-equivalent helper functions
// =====================================================================

/// CF_CODEC_IS_OK(s) — works with encoder or decoder
#[inline]
pub fn CF_CODEC_IS_OK_ENC(s: &CF_EncoderState_t) -> bool {
    CF_CFDP_CodecIsOK(&s.codec_state)
}

#[inline]
pub fn CF_CODEC_IS_OK_DEC(s: &CF_DecoderState_t) -> bool {
    CF_CFDP_CodecIsOK(&s.codec_state)
}

/// CF_CODEC_SET_DONE(s)
#[inline]
pub fn CF_CODEC_SET_DONE_ENC(s: &mut CF_EncoderState_t) {
    CF_CFDP_CodecSetDone(&mut s.codec_state);
}

#[inline]
pub fn CF_CODEC_SET_DONE_DEC(s: &mut CF_DecoderState_t) {
    CF_CFDP_CodecSetDone(&mut s.codec_state);
}

/// CF_CODEC_GET_POSITION(s)
#[inline]
pub fn CF_CODEC_GET_POSITION_ENC(s: &CF_EncoderState_t) -> usize {
    CF_CFDP_CodecGetPosition(&s.codec_state)
}

#[inline]
pub fn CF_CODEC_GET_POSITION_DEC(s: &CF_DecoderState_t) -> usize {
    CF_CFDP_CodecGetPosition(&s.codec_state)
}

/// CF_CODEC_GET_REMAIN(s)
#[inline]
pub fn CF_CODEC_GET_REMAIN_ENC(s: &CF_EncoderState_t) -> usize {
    CF_CFDP_CodecGetRemain(&s.codec_state)
}

#[inline]
pub fn CF_CODEC_GET_REMAIN_DEC(s: &CF_DecoderState_t) -> usize {
    CF_CFDP_CodecGetRemain(&s.codec_state)
}

/// CF_CODEC_GET_SIZE(s)
#[inline]
pub fn CF_CODEC_GET_SIZE_ENC(s: &CF_EncoderState_t) -> usize {
    CF_CFDP_CodecGetSize(&s.codec_state)
}

#[inline]
pub fn CF_CODEC_GET_SIZE_DEC(s: &CF_DecoderState_t) -> usize {
    CF_CFDP_CodecGetSize(&s.codec_state)
}
