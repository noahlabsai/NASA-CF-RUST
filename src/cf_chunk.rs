//! CF Application chunks (sparse gap tracking) logic.
//!
//! This class handles the complexity of sparse gap tracking so that
//! the CFDP engine doesn't need to worry about it.
//!
//! Translated from: cf_chunk.c / cf_chunk.h

use crate::cf_chunk_types::*;

/// Erase a range of chunks [start, end).
///
/// C original: `void CF_Chunks_EraseRange(CF_ChunkList_t *chunks, CF_ChunkIdx_t start, CF_ChunkIdx_t end)`
///
/// # Safety
/// `chunks` must have a valid `chunks` pointer with at least `count` elements.
pub fn CF_Chunks_EraseRange(chunks: &mut CF_ChunkList_t, start: CF_ChunkIdx_t, end: CF_ChunkIdx_t) {
    assert!(end <= chunks.count, "CF_Assert: end <= chunks->count");

    if start < end {
        let s = unsafe {
            std::slice::from_raw_parts_mut(chunks.chunks, chunks.count as usize)
        };
        s.copy_within(end as usize.., start as usize);
        chunks.count -= end - start;
    }
}

/// Erase a single chunk at `erase_index`.
///
/// C original: `void CF_Chunks_EraseChunk(CF_ChunkList_t *chunks, CF_ChunkIdx_t erase_index)`
pub fn CF_Chunks_EraseChunk(chunks: &mut CF_ChunkList_t, erase_index: CF_ChunkIdx_t) {
    assert!(chunks.count > 0, "CF_Assert: chunks->count > 0");
    assert!(erase_index < chunks.count, "CF_Assert: erase_index < chunks->count");

    let s = unsafe {
        std::slice::from_raw_parts_mut(chunks.chunks, chunks.count as usize)
    };
    s.copy_within((erase_index + 1) as usize.., erase_index as usize);
    chunks.count -= 1;
}

/// Insert a chunk before `index_before`.
///
/// C original: `void CF_Chunks_InsertChunk(CF_ChunkList_t *chunks, CF_ChunkIdx_t index_before, const CF_Chunk_t *chunk)`
pub fn CF_Chunks_InsertChunk(chunks: &mut CF_ChunkList_t, index_before: CF_ChunkIdx_t, chunk: &CF_Chunk_t) {
    assert!(chunks.count < chunks.max_chunks, "CF_Assert: count < max_chunks");
    assert!(index_before <= chunks.count, "CF_Assert: index_before <= count");

    // Use max_chunks capacity for the slice since we need room for the new element
    let s = unsafe {
        std::slice::from_raw_parts_mut(chunks.chunks, (chunks.count + 1) as usize)
    };

    if chunks.count > 0 && index_before != chunks.count {
        // Shift elements right to make room — must go from end to avoid overwrite
        let ib = index_before as usize;
        let cnt = chunks.count as usize;
        s.copy_within(ib..cnt, ib + 1);
    }

    s[index_before as usize] = *chunk;
    chunks.count += 1;
}

/// Find where a chunk should be inserted (lower_bound by offset).
///
/// C original: `CF_ChunkIdx_t CF_Chunks_FindInsertPosition(CF_ChunkList_t *chunks, const CF_Chunk_t *chunk)`
pub fn CF_Chunks_FindInsertPosition(chunks: &CF_ChunkList_t, chunk: &CF_Chunk_t) -> CF_ChunkIdx_t {
    let mut first: CF_ChunkIdx_t = 0;
    let mut count: CF_ChunkIdx_t = chunks.count;

    let s = unsafe {
        std::slice::from_raw_parts(chunks.chunks, chunks.count as usize)
    };

    while count > 0 {
        let step = count / 2;
        let i = first + step;

        if s[i as usize].offset < chunk.offset {
            first = i + 1;
            count -= step + 1;
        } else {
            count = step;
        }
    }

    first
}

/// Possibly combines the given chunk with the previous chunk.
///
/// C original: `int CF_Chunks_CombinePrevious(CF_ChunkList_t *chunks, CF_ChunkIdx_t i, const CF_Chunk_t *chunk)`
///
/// Returns 1 if combined, 0 if not.
pub fn CF_Chunks_CombinePrevious(chunks: &mut CF_ChunkList_t, i: CF_ChunkIdx_t, chunk: &CF_Chunk_t) -> i32 {
    assert!(i <= chunks.max_chunks, "CF_Assert: i <= max_chunks");

    if i > 0 {
        let chunk_end = chunk.offset + chunk.size;
        let prev = unsafe { &mut *chunks.chunks.add((i - 1) as usize) };
        let prev_end = prev.offset + prev.size;

        // Check if start of new chunk is less than or equal to end of previous (overlaps or adjacent)
        if chunk.offset <= prev_end {
            // When combining, use the bigger of the two endings
            if prev_end < chunk_end {
                prev.size = chunk_end - prev.offset;
            }
            return 1;
        }
    }
    0
}

/// Possibly combines the given chunk with the next chunk(s).
///
/// C original: `CFE_Status_t CF_Chunks_CombineNext(CF_ChunkList_t *chunks, CF_ChunkIdx_t i, const CF_Chunk_t *chunk)`
///
/// Returns 1 if combined, 0 if not.
pub fn CF_Chunks_CombineNext(chunks: &mut CF_ChunkList_t, i: CF_ChunkIdx_t, chunk: &CF_Chunk_t) -> i32 {
    let mut combined_i = i;
    let mut chunk_end = chunk.offset + chunk.size;

    // Assert no rollover
    assert!(chunk_end > chunk.offset, "CF_Assert: chunk_end > chunk->offset (no rollover)");

    let s = unsafe {
        std::slice::from_raw_parts(chunks.chunks, chunks.count as usize)
    };

    // Determine how many can be combined
    while combined_i < chunks.count {
        if chunk_end < s[combined_i as usize].offset {
            break;
        }
        combined_i += 1;
    }

    // If index advanced, the range of chunks can be combined
    if i != combined_i {
        let last = &s[(combined_i - 1) as usize];
        chunk_end = CF_Chunk_MAX(last.offset + last.size, chunk_end);

        // Use current slot as combined entry
        let slot = unsafe { &mut *chunks.chunks.add(i as usize) };
        slot.size = chunk_end - chunk.offset;
        slot.offset = chunk.offset;

        // Erase the rest of the combined chunks (if any)
        CF_Chunks_EraseRange(chunks, i + 1, combined_i);
        return 1;
    }

    0
}

/// Find the index of the chunk with the smallest size.
///
/// C original: `CF_ChunkIdx_t CF_Chunks_FindSmallestSize(const CF_ChunkList_t *chunks)`
pub fn CF_Chunks_FindSmallestSize(chunks: &CF_ChunkList_t) -> CF_ChunkIdx_t {
    let mut smallest: CF_ChunkIdx_t = 0;

    let s = unsafe {
        std::slice::from_raw_parts(chunks.chunks, chunks.count as usize)
    };

    for i in 1..chunks.count {
        if s[i as usize].size < s[smallest as usize].size {
            smallest = i;
        }
    }

    smallest
}

/// Insert a chunk, combining with neighbors if possible.
///
/// C original: `void CF_Chunks_Insert(CF_ChunkList_t *chunks, CF_ChunkIdx_t i, const CF_Chunk_t *chunk)`
pub fn CF_Chunks_Insert(chunks: &mut CF_ChunkList_t, i: CF_ChunkIdx_t, chunk: &CF_Chunk_t) {
    let n = CF_Chunks_CombineNext(chunks, i, chunk);

    if n != 0 {
        // CombineNext succeeded — now try to combine the result with previous
        let combined_chunk = unsafe { *chunks.chunks.add(i as usize) };
        let combined = CF_Chunks_CombinePrevious(chunks, i, &combined_chunk);
        if combined != 0 {
            CF_Chunks_EraseChunk(chunks, i);
        }
    } else {
        let combined = CF_Chunks_CombinePrevious(chunks, i, chunk);
        if combined == 0 {
            if chunks.count < chunks.max_chunks {
                CF_Chunks_InsertChunk(chunks, i, chunk);
            } else {
                let smallest_i = CF_Chunks_FindSmallestSize(chunks);
                let smallest_size = unsafe { (*chunks.chunks.add(smallest_i as usize)).size };
                if smallest_size < chunk.size {
                    CF_Chunks_EraseChunk(chunks, smallest_i);
                    let new_pos = CF_Chunks_FindInsertPosition(chunks, chunk);
                    CF_Chunks_InsertChunk(chunks, new_pos, chunk);
                }
            }
        }
    }
}

/// Public function to add a chunk.
///
/// C original: `void CF_ChunkListAdd(CF_ChunkList_t *chunks, CF_ChunkOffset_t offset, CF_ChunkSize_t size)`
pub fn CF_ChunkListAdd(chunks: &mut CF_ChunkList_t, offset: CF_ChunkOffset_t, size: CF_ChunkSize_t) {
    let chunk = CF_Chunk_t { offset, size };
    let i = CF_Chunks_FindInsertPosition(chunks, &chunk);

    assert!(offset.wrapping_add(size) >= offset, "CF_Assert: (offset + size) >= offset");

    CF_Chunks_Insert(chunks, i, &chunk);
}

/// Remove some amount of size from the first chunk.
///
/// C original: `void CF_ChunkList_RemoveFromFirst(CF_ChunkList_t *chunks, CF_ChunkSize_t size)`
pub fn CF_ChunkList_RemoveFromFirst(chunks: &mut CF_ChunkList_t, mut size: CF_ChunkSize_t) {
    let chunk = unsafe { &mut *chunks.chunks.add(0) };

    if size > chunk.size {
        size = chunk.size;
    }
    chunk.size -= size;

    if chunk.size == 0 {
        CF_Chunks_EraseChunk(chunks, 0);
    } else {
        chunk.offset += size;
    }
}

/// Get the first chunk from the list, or None if empty.
///
/// C original: `const CF_Chunk_t *CF_ChunkList_GetFirstChunk(const CF_ChunkList_t *chunks)`
pub fn CF_ChunkList_GetFirstChunk(chunks: &CF_ChunkList_t) -> Option<&CF_Chunk_t> {
    if chunks.count > 0 {
        unsafe { Some(&*chunks.chunks) }
    } else {
        None
    }
}

/// Initialize a CF_ChunkList_t structure.
///
/// C original: `void CF_ChunkListInit(CF_ChunkList_t *chunks, CF_ChunkIdx_t max_chunks, CF_Chunk_t *chunks_mem)`
pub fn CF_ChunkListInit(chunks: &mut CF_ChunkList_t, max_chunks: CF_ChunkIdx_t, chunks_mem: *mut CF_Chunk_t) {
    assert!(max_chunks > 0, "CF_Assert: max_chunks > 0");
    chunks.max_chunks = max_chunks;
    chunks.chunks = chunks_mem;
    CF_ChunkListReset(chunks);
}

/// Reset a chunks structure (clear all chunks, keep allocation).
///
/// C original: `void CF_ChunkListReset(CF_ChunkList_t *chunks)`
pub fn CF_ChunkListReset(chunks: &mut CF_ChunkList_t) {
    chunks.count = 0;
    let s = unsafe {
        std::slice::from_raw_parts_mut(chunks.chunks, chunks.max_chunks as usize)
    };
    s.fill(CF_Chunk_t::default());
}

/// Compute gaps between chunks, and call a callback for each.
///
/// C original: `uint32 CF_ChunkList_ComputeGaps(...)`
///
/// # Safety
/// `chunks` must have a valid `chunks` pointer. `compute_gap_fn` if Some must be valid.
pub fn CF_ChunkList_ComputeGaps(
    chunks: &CF_ChunkList_t,
    max_gaps: CF_ChunkIdx_t,
    total: CF_ChunkSize_t,
    start: CF_ChunkOffset_t,
    compute_gap_fn: CF_ChunkList_ComputeGapFn_t,
    opaque: *mut u8,
) -> u32 {
    let mut ret: u32 = 0;
    let mut i: CF_ChunkIdx_t = 0;

    assert!(start <= total, "CF_Assert: start <= total");

    if start == total {
        // Simple case: there cannot be a gap (this includes e.g. 0 byte file)
        ret = 0;
    } else if chunks.count == 0 {
        // Simple case: no chunk data, single gap of entire size
        let chunk = CF_Chunk_t { offset: 0, size: total };
        if let Some(gap_fn) = compute_gap_fn {
            unsafe { gap_fn(chunks, &chunk, opaque) };
        }
        ret = 1;
    } else {
        let s = unsafe {
            std::slice::from_raw_parts(chunks.chunks, chunks.count as usize)
        };

        // Handle initial gap if needed
        if start < s[0].offset {
            let chunk = CF_Chunk_t {
                offset: start,
                size: s[0].offset - start,
            };
            if let Some(gap_fn) = compute_gap_fn {
                unsafe { gap_fn(chunks, &chunk, opaque) };
            }
            ret = 1;
        }

        while ret < max_gaps && i < chunks.count {
            let next_off = if i == chunks.count - 1 {
                total
            } else {
                s[(i + 1) as usize].offset
            };

            let gap_start = s[i as usize].offset + s[i as usize].size;

            let chunk_offset = if gap_start > start { gap_start } else { start };
            let chunk_size = next_off - chunk_offset;

            if gap_start >= total {
                break;
            } else if start < next_off {
                let chunk = CF_Chunk_t {
                    offset: chunk_offset,
                    size: chunk_size,
                };
                if let Some(gap_fn) = compute_gap_fn {
                    unsafe { gap_fn(chunks, &chunk, opaque) };
                }
                ret += 1;
            }
            i += 1;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    fn make_chunk_list(max: u32) -> (Vec<CF_Chunk_t>, CF_ChunkList_t) {
        let mut mem = vec![CF_Chunk_t::default(); max as usize];
        let cl = CF_ChunkList_t {
            count: 0,
            max_chunks: max,
            chunks: mem.as_mut_ptr(),
        };
        (mem, cl)
    }

    #[test]
    fn test_add_single() {
        let (mut mem, mut cl) = make_chunk_list(10);
        cl.chunks = mem.as_mut_ptr();
        CF_ChunkListAdd(&mut cl, 0, 100);
        assert_eq!(cl.count, 1);
        assert_eq!(mem[0].offset, 0);
        assert_eq!(mem[0].size, 100);
    }

    #[test]
    fn test_add_combine_adjacent() {
        let (mut mem, mut cl) = make_chunk_list(10);
        cl.chunks = mem.as_mut_ptr();
        CF_ChunkListAdd(&mut cl, 0, 50);
        CF_ChunkListAdd(&mut cl, 50, 50);
        assert_eq!(cl.count, 1);
        assert_eq!(mem[0].offset, 0);
        assert_eq!(mem[0].size, 100);
    }

    #[test]
    fn test_add_combine_overlapping() {
        let (mut mem, mut cl) = make_chunk_list(10);
        cl.chunks = mem.as_mut_ptr();
        CF_ChunkListAdd(&mut cl, 0, 60);
        CF_ChunkListAdd(&mut cl, 40, 60);
        assert_eq!(cl.count, 1);
        assert_eq!(mem[0].offset, 0);
        assert_eq!(mem[0].size, 100);
    }

    #[test]
    fn test_add_gap_between() {
        let (mut mem, mut cl) = make_chunk_list(10);
        cl.chunks = mem.as_mut_ptr();
        CF_ChunkListAdd(&mut cl, 0, 30);
        CF_ChunkListAdd(&mut cl, 70, 30);
        assert_eq!(cl.count, 2);
        assert_eq!(mem[0].offset, 0);
        assert_eq!(mem[0].size, 30);
        assert_eq!(mem[1].offset, 70);
        assert_eq!(mem[1].size, 30);
    }

    #[test]
    fn test_remove_from_first() {
        let (mut mem, mut cl) = make_chunk_list(10);
        cl.chunks = mem.as_mut_ptr();
        CF_ChunkListAdd(&mut cl, 0, 100);
        CF_ChunkList_RemoveFromFirst(&mut cl, 30);
        assert_eq!(cl.count, 1);
        let first = unsafe { &*cl.chunks };
        assert_eq!(first.offset, 30);
        assert_eq!(first.size, 70);
    }

    #[test]
    fn test_remove_from_first_all() {
        let (mut mem, mut cl) = make_chunk_list(10);
        cl.chunks = mem.as_mut_ptr();
        CF_ChunkListAdd(&mut cl, 0, 100);
        CF_ChunkList_RemoveFromFirst(&mut cl, 100);
        assert_eq!(cl.count, 0);
    }

    unsafe fn gap_counter(_cs: &CF_ChunkList_t, _chunk: &CF_Chunk_t, opaque: *mut u8) {
        let count = &mut *(opaque as *mut u32);
        *count += 1;
    }

    #[test]
    fn test_compute_gaps_no_chunks() {
        let (mut mem, mut cl) = make_chunk_list(10);
        cl.chunks = mem.as_mut_ptr();
        let mut count: u32 = 0;
        let ret = CF_ChunkList_ComputeGaps(
            &cl, 10, 100, 0,
            Some(gap_counter),
            &mut count as *mut u32 as *mut u8,
        );
        assert_eq!(ret, 1);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_compute_gaps_full() {
        let (mut mem, mut cl) = make_chunk_list(10);
        cl.chunks = mem.as_mut_ptr();
        CF_ChunkListAdd(&mut cl, 0, 100);
        let mut count: u32 = 0;
        let ret = CF_ChunkList_ComputeGaps(
            &cl, 10, 100, 0,
            Some(gap_counter),
            &mut count as *mut u32 as *mut u8,
        );
        assert_eq!(ret, 0);
        assert_eq!(count, 0);
    }
}
