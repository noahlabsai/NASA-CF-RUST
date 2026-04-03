use crate::cf_chunk::*;

#[cfg(test)]
mod stubs {
    use super::*;

    pub fn ut_default_handler_cf_chunk_list_get_first_chunk() {
        // Default handler stub
    }

    pub fn cf_chunk_list_add(chunks: &mut CfChunkList, offset: CfChunkOffset, size: CfChunkSize) {
        // Stub implementation
    }

    pub fn cf_chunk_list_init(chunks: &mut CfChunkList, max_chunks: CfChunkIdx, chunks_mem: &mut [CfChunk]) {
        // Stub implementation
    }

    pub fn cf_chunk_list_reset(chunks: &mut CfChunkList) {
        // Stub implementation
    }

    pub fn cf_chunk_list_compute_gaps(
        chunks: &CfChunkList,
        max_gaps: CfChunkIdx,
        total: CfChunkSize,
        start: CfChunkOffset,
        compute_gap_fn: CfChunkListComputeGapFn,
        opaque: *mut std::ffi::c_void,
    ) -> u32 {
        u32::default()
    }

    pub fn cf_chunk_list_get_first_chunk(chunks: &CfChunkList) -> Option<&CfChunk> {
        None
    }

    pub fn cf_chunk_list_remove_from_first(chunks: &mut CfChunkList, size: CfChunkSize) {
        // Stub implementation
    }

    pub fn cf_chunks_combine_next(chunks: &mut CfChunkList, i: CfChunkIdx, chunk: &CfChunk) -> CfeStatus {
        CfeStatus::default()
    }

    pub fn cf_chunks_combine_previous(chunks: &mut CfChunkList, i: CfChunkIdx, chunk: &CfChunk) -> i32 {
        i32::default()
    }

    pub fn cf_chunks_erase_chunk(chunks: &mut CfChunkList, erase_index: CfChunkIdx) {
        // Stub implementation
    }

    pub fn cf_chunks_erase_range(chunks: &mut CfChunkList, start: CfChunkIdx, end: CfChunkIdx) {
        // Stub implementation
    }

    pub fn cf_chunks_find_insert_position(chunks: &mut CfChunkList, chunk: &CfChunk) -> CfChunkIdx {
        CfChunkIdx::default()
    }

    pub fn cf_chunks_find_smallest_size(chunks: &CfChunkList) -> CfChunkIdx {
        CfChunkIdx::default()
    }

    pub fn cf_chunks_insert(chunks: &mut CfChunkList, i: CfChunkIdx, chunk: &CfChunk) {
        // Stub implementation
    }

    pub fn cf_chunks_insert_chunk(chunks: &mut CfChunkList, index_before: CfChunkIdx, chunk: &CfChunk) {
        // Stub implementation
    }
}