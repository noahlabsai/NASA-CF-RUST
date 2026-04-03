//! CF Application chunks (sparse gap tracking) type definitions.
//!
//! Translated from: cf_chunk.h

/// Index type for chunk arrays.
pub type CF_ChunkIdx_t = u32;

/// Offset type within a file.
pub type CF_ChunkOffset_t = u32;

/// Size type for chunks.
pub type CF_ChunkSize_t = u32;

/// Pairs an offset with a size to identify a specific piece of a file.
///
/// Matches C `CF_Chunk_t`.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub struct CF_Chunk_t {
    /// The start offset of the chunk within the file.
    pub offset: CF_ChunkOffset_t,
    /// The size of the chunk.
    pub size: CF_ChunkSize_t,
}

/// A list of CF_Chunk_t pairs.
///
/// This list is ordered by chunk offset, from lowest to highest.
/// Matches C `CF_ChunkList_t` — uses a raw pointer to externally-owned memory.
#[repr(C)]
pub struct CF_ChunkList_t {
    /// Number of chunks currently in the array.
    pub count: CF_ChunkIdx_t,
    /// Maximum number of chunks allowed in the list (allocation size).
    pub max_chunks: CF_ChunkIdx_t,
    /// Chunk list array (raw pointer to externally-owned memory).
    pub chunks: *mut CF_Chunk_t,
}

/// Callback function type for use with `CF_ChunkList_ComputeGaps`.
///
/// Matches C: `typedef void (*CF_ChunkList_ComputeGapFn_t)(const CF_ChunkList_t *cs, const CF_Chunk_t *chunk, void *opaque);`
pub type CF_ChunkList_ComputeGapFn_t =
    Option<unsafe fn(cs: &CF_ChunkList_t, chunk: &CF_Chunk_t, opaque: *mut u8)>;

/// Selects the larger of the two passed-in offsets.
///
/// Matches C inline: `CF_Chunk_MAX`.
#[inline]
pub fn CF_Chunk_MAX(a: CF_ChunkOffset_t, b: CF_ChunkOffset_t) -> CF_ChunkOffset_t {
    if a > b { a } else { b }
}
