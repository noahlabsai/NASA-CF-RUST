use std::mem;
use std::ptr;

// cf testing includes
use crate::cf_test_utils::*;
use crate::cf_chunk::*;

// Gap function test function and context
const TEST_CF_MAX_GAPS: usize = 3;

#[derive(Debug, Clone, Default)]
struct TestCfComputeGapContext {
    count: u32,
    chunks: [CfChunk; TEST_CF_MAX_GAPS],
}

static mut TEST_CF_COMPUTE_GAP_CONTEXT: TestCfComputeGapContext = TestCfComputeGapContext {
    count: 0,
    chunks: [CfChunk { size: 0, offset: 0 }; TEST_CF_MAX_GAPS],
};

fn test_cf_compute_gap_fn(clist: &CfChunkList, chunk: &CfChunk, opaque: *const CfChunkList) {
    ut_assert_address_eq(clist as *const CfChunkList, opaque);

    unsafe {
        if TEST_CF_COMPUTE_GAP_CONTEXT.count < TEST_CF_MAX_GAPS as u32 {
            let idx = TEST_CF_COMPUTE_GAP_CONTEXT.count as usize;
            TEST_CF_COMPUTE_GAP_CONTEXT.chunks[idx].size = chunk.size;
            TEST_CF_COMPUTE_GAP_CONTEXT.chunks[idx].offset = chunk.offset;
        }
        TEST_CF_COMPUTE_GAP_CONTEXT.count += 1;
    }
}

// Fill a chunk list
fn ut_cf_chunk_setup_full(clist: &mut CfChunkList) {
    /*
     * Set up nonzero values for size and calculate a "realistic" offsets w/ size
     *   Size: just set to index+1 so it's uniquely identifiable
     *   Offset: calculated using size and gap of 10
     *
     *  1    2     3     4     5
     * 0-1 11-13 23-26 36-40 50-55
     */
    clist.chunks[0].offset = 0;
    for cidx in 0..clist.max_chunks {
        clist.chunks[cidx].size = cidx as u32 + 1;

        if cidx > 0 {
            clist.chunks[cidx].offset = clist.chunks[cidx - 1].offset + clist.chunks[cidx - 1].size + 10;
        }
    }

    // Set count to max since list is now full
    clist.count = clist.max_chunks as u32;
}

// Print the chunk list to the UT log (test debug helper)
fn ut_cf_chunk_print(clist: &CfChunkList) {
    ut_printf("Chunk list: index{offset, size}");
    for cidx in 0..clist.count as usize {
        ut_printf(
            &format!("{}{{}, {}}", cidx, clist.chunks[cidx].offset, clist.chunks[cidx].size)
        );
    }

    ut_printf("Chunk list: index{start-end}");
    for cidx in 0..clist.count as usize {
        ut_printf(
            &format!(
                "{}{{}-{}}",
                cidx,
                clist.chunks[cidx].offset,
                clist.chunks[cidx].offset + clist.chunks[cidx].size
            )
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * Cover nominal create (which resets), add cases (empty, front, end, replace smallest)
     * Note out of order sizes are to fully cover CF_Chunks_FindSmallestSize
     */
    #[test]
    fn test_cf_chunk_create_add_reset() {
        let mut clist = CfChunkList::default();
        let mut chunks = [CfChunk::default(); 3];

        // Set nonzero values and test CF_ChunkListInit
        unsafe {
            ptr::write_bytes(&mut clist as *mut CfChunkList, 0xFF, 1);
            ptr::write_bytes(chunks.as_mut_ptr(), 0xFF, chunks.len());
        }
        cf_chunk_list_init(&mut clist, chunks.len(), &mut chunks);
        assert_eq!(clist.count, 0);
        assert_eq!(clist.max_chunks, chunks.len() as u32);
        // Spot check chunks clear
        assert_eq!(chunks[1].size, 0);
        assert_eq!(chunks[1].offset, 0);

        // Add to empty list
        cf_chunk_list_add(&mut clist, 5, 1);
        assert_eq!(clist.chunks[0].offset, 5);
        assert_eq!(clist.chunks[0].size, 1);
        assert_eq!(clist.count, 1);

        // Add to end of list
        cf_chunk_list_add(&mut clist, 10, 1);
        assert_eq!(clist.chunks[0].offset, 5);
        assert_eq!(clist.chunks[0].size, 1);
        assert_eq!(clist.chunks[1].offset, 10);
        assert_eq!(clist.chunks[1].size, 1);
        assert_eq!(clist.count, 2);

        // Add to front of list
        cf_chunk_list_add(&mut clist, 0, 2);
        assert_eq!(clist.chunks[0].offset, 0);
        assert_eq!(clist.chunks[0].size, 2);
        assert_eq!(clist.chunks[1].offset, 5);
        assert_eq!(clist.chunks[1].size, 1);
        assert_eq!(clist.chunks[2].offset, 10);
        assert_eq!(clist.chunks[2].size, 1);
        assert_eq!(clist.count, 3);

        // Force 1 to drop (first smallest), with new at the end
        cf_chunk_list_add(&mut clist, 20, 2);
        assert_eq!(clist.chunks[0].offset, 0);
        assert_eq!(clist.chunks[0].size, 2);
        assert_eq!(clist.chunks[1].offset, 10);
        assert_eq!(clist.chunks[1].size, 1);
        assert_eq!(clist.chunks[2].offset, 20);
        assert_eq!(clist.chunks[2].size, 2);
        assert_eq!(clist.count, 3);

        // Nominal combine previous (no overlap, at the end)
        cf_chunk_list_add(&mut clist, 22, 2);
        assert_eq!(clist.chunks[0].offset, 0);
        assert_eq!(clist.chunks[0].size, 2);
        assert_eq!(clist.chunks[1].offset, 10);
        assert_eq!(clist.chunks[1].size, 1);
        assert_eq!(clist.chunks[2].offset, 20);
        assert_eq!(clist.chunks[2].size, 4);
        assert_eq!(clist.count, 3);
    }

    // Cover combination cases
    #[test]
    fn test_cf_chunk_combine() {
        let mut clist = CfChunkList::default();
        let mut chunks = [CfChunk::default(); 5];

        // Initialize list (note already tested)
        cf_chunk_list_init(&mut clist, chunks.len(), &mut chunks);

        ut_printf("Initial chunk list state for reference");
        ut_cf_chunk_setup_full(&mut clist);
        ut_cf_chunk_print(&clist);

        ut_printf("Add chunk that won't add since list full and new chunk is smallest");
        cf_chunk_list_add(&mut clist, 2, 1); // 2-3
        ut_cf_chunk_print(&clist);
        // Confirm 0 and 1 didn't change
        assert_eq!(clist.chunks[0].offset, 0);
        assert_eq!(clist.chunks[0].size, 1);
        assert_eq!(clist.chunks[1].offset, 11);
        assert_eq!(clist.chunks[1].size, 2);
        assert_eq!(clist.count, 5);

        ut_cf_chunk_setup_full(&mut clist);
        ut_printf("Add chunk that replaces chunk 0 as the smallest chunk");
        cf_chunk_list_add(&mut clist, 2, 2); // 2-4
        ut_cf_chunk_print(&clist);
        // Confirm 0 replaced and 1 didn't change
        assert_eq!(clist.chunks[0].offset, 2);
        assert_eq!(clist.chunks[0].size, 2);
        assert_eq!(clist.chunks[1].offset, 11);
        assert_eq!(clist.chunks[1].size, 2);
        assert_eq!(clist.count, 5);

        ut_cf_chunk_setup_full(&mut clist);
        ut_printf("Add chunk that combines with chunk 1 w/ no overlap");
        cf_chunk_list_add(&mut clist, 10, 1); // 10-11
        ut_cf_chunk_print(&clist);
        // 0 and 2 unchanged, 1 combined
        assert_eq!(clist.chunks[0].offset, 0);
        assert_eq!(clist.chunks[0].size, 1);
        assert_eq!(clist.chunks[1].offset, 10);
        assert_eq!(clist.chunks[1].size, 3);
        assert_eq!(clist.chunks[2].offset, 23);
        assert_eq!(clist.chunks[2].size, 3);
        assert_eq!(clist.count, 5);

        ut_cf_chunk_setup_full(&mut clist);
        ut_printf("Add chunk that should completely replace chunk 2 and 3, both as Next");
        cf_chunk_list_add(&mut clist, 20, 21); // 20-41
        ut_cf_chunk_print(&clist);
        // 1 unchanged, 2 combined, 4 in slot 3
        assert_eq!(clist.chunks[1].offset, 11);
        assert_eq!(clist.chunks[1].size, 2);
        assert_eq!(clist.chunks[2].offset, 20);
        assert_eq!(clist.chunks[2].size, 21);
        assert_eq!(clist.chunks[3].offset, 50);
        assert_eq!(clist.chunks[3].size, 5);
        assert_eq!(clist.count, 4);

        ut_cf_chunk_setup_full(&mut clist);
        ut_printf("Add chunk that combines with chunk 1, 2 and 3, (prev, next, next)");
        cf_chunk_list_add(&mut clist, 12, 25); // 12-37
        ut_cf_chunk_print(&clist);
        // 0 unchanged, 1 combined, 4 in slot 2
        assert_eq!(clist.chunks[0].offset, 0);
        assert_eq!(clist.chunks[0].size, 1);
        assert_eq!(clist.chunks[1].offset, 11);
        assert_eq!(clist.chunks[1].size, 29);
        assert_eq!(clist.chunks[2].offset, 50);
        assert_eq!(clist.chunks[2].size, 5);
        assert_eq!(clist.count, 3);

        ut_cf_chunk_setup_full(&mut clist);
        ut_printf("Add chunk that is a subset of 3 (should just drop)");
        cf_chunk_list_add(&mut clist, 37, 2); // 37-39
        ut_cf_chunk_print(&clist);
        // 1, 2, and 3 unchanged
        assert_eq!(clist.chunks[1].offset, 11);
        assert_eq!(clist.chunks[1].size, 2);
        assert_eq!(clist.chunks[2].offset, 23);
        assert_eq!(clist.chunks[2].size, 3);
        assert_eq!(clist.chunks[3].offset, 36);
        assert_eq!(clist.chunks[3].size, 4);
        assert_eq!(clist.count, 5);
    }

    #[test]
    fn test_cf_chunk_get_rm_first() {
        let mut clist = CfChunkList::default();
        let mut chunks = [CfChunk::default(); 2];

        // Initialize list (note already tested)
        cf_chunk_list_init(&mut clist, chunks.len(), &mut chunks);

        // Get first with empty list
        assert!(cf_chunk_list_get_first_chunk(&clist).is_none());

        // Note CF_ChunkList_RemoveFromFirst can not be called on empty list (as documented)

        // Add two (already tested)
        cf_chunk_list_add(&mut clist, 0, 10);
        cf_chunk_list_add(&mut clist, 20, 10);

        // Get first with non-empty list
        assert_eq!(cf_chunk_list_get_first_chunk(&clist).unwrap() as *const CfChunk, chunks.as_ptr());

        // Remove part from first non-empty list
        cf_chunk_list_remove_from_first(&mut clist, 5);
        assert_eq!(clist.chunks[0].offset, 5);
        assert_eq!(clist.chunks[0].size, 5);
        assert_eq!(clist.chunks[1].offset, 20);
        assert_eq!(clist.chunks[1].size, 10);
        assert_eq!(clist.count, 2);

        // Remove the rest of first from non-empty list
        cf_chunk_list_remove_from_first(&mut clist, 5);
        assert_eq!(clist.chunks[0].offset, 20);
        assert_eq!(clist.chunks[0].size, 10);
        assert_eq!(clist.count, 1);

        // Add back in, do large remove, confirm only first chunk removed
        cf_chunk_list_add(&mut clist, 0, 10);
        cf_chunk_list_remove_from_first(&mut clist, 50);
        assert_eq!(clist.chunks[0].offset, 20);
        assert_eq!(clist.chunks[0].size, 10);
        assert_eq!(clist.count, 1);
    }

    #[test]
    fn test_cf_chunk_compute_gaps() {
        let mut clist = CfChunkList::default();
        let mut chunks = [CfChunk::default(); 5];

        // Initialize list (note already tested)
        cf_chunk_list_init(&mut clist, chunks.len(), &mut chunks);

        // Zero byte file
        assert_eq!(cf_chunk_list_compute_gaps(&clist, TEST_CF_MAX_GAPS as u32, 0, 0, None, ptr::null()), 0);

        // Empty list with function callback
        let total = 10;
        unsafe {
            TEST_CF_COMPUTE_GAP_CONTEXT = TestCfComputeGapContext::default();
        }
        assert_eq!(
            cf_chunk_list_compute_gaps(
                &clist,
                TEST_CF_MAX_GAPS as u32,
                total,
                0,
                Some(test_cf_compute_gap_fn),
                &clist as *const CfChunkList
            ),
            1
        );
        unsafe {
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[0].size, total);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[0].offset, 0);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.count, 1);
        }

        // Empty list no callback
        unsafe {
            TEST_CF_COMPUTE_GAP_CONTEXT = TestCfComputeGapContext::default();
        }
        assert_eq!(cf_chunk_list_compute_gaps(&clist, TEST_CF_MAX_GAPS as u32, total, 0, None, ptr::null()), 1);

        // Add three with gaps 0-4, 10-19, 30-49
        cf_chunk_list_add(&mut clist, 5, 5);
        cf_chunk_list_add(&mut clist, 20, 10);
        cf_chunk_list_add(&mut clist, 50, 10);

        // Check 0-45, reports 3 gaps and breaks on total limit
        unsafe {
            TEST_CF_COMPUTE_GAP_CONTEXT = TestCfComputeGapContext::default();
        }
        assert_eq!(
            cf_chunk_list_compute_gaps(
                &clist,
                TEST_CF_MAX_GAPS as u32,
                25,
                0,
                Some(test_cf_compute_gap_fn),
                &clist as *const CfChunkList
            ),
            2
        );
        unsafe {
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[0].size, 5);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[0].offset, 0);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[1].size, 10);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[1].offset, 10);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.count, 2);
        }

        // Same with no callback
        unsafe {
            TEST_CF_COMPUTE_GAP_CONTEXT = TestCfComputeGapContext::default();
        }
        assert_eq!(cf_chunk_list_compute_gaps(&clist, TEST_CF_MAX_GAPS as u32, 45, 0, None, ptr::null()), 3);

        // Check 25-75, end while loop at end of chunk list
        unsafe {
            TEST_CF_COMPUTE_GAP_CONTEXT = TestCfComputeGapContext::default();
        }
        assert_eq!(
            cf_chunk_list_compute_gaps(
                &clist,
                TEST_CF_MAX_GAPS as u32,
                75,
                25,
                Some(test_cf_compute_gap_fn),
                &clist as *const CfChunkList
            ),
            2
        );
        unsafe {
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[0].size, 20);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[0].offset, 30);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[1].size, 15);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[1].offset, 60);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.count, 2);
        }

        // Check 0-75, limit by TEST_CF_MAX_GAPS
        unsafe {
            TEST_CF_COMPUTE_GAP_CONTEXT = TestCfComputeGapContext::default();
        }
        assert_eq!(
            cf_chunk_list_compute_gaps(
                &clist,
                TEST_CF_MAX_GAPS as u32,
                75,
                0,
                Some(test_cf_compute_gap_fn),
                &clist as *const CfChunkList
            ),
            3
        );
        unsafe {
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[0].size, 5);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[0].offset, 0);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[1].size, 10);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[1].offset, 10);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[2].size, 20);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.chunks[2].offset, 30);
            assert_eq!(TEST_CF_COMPUTE_GAP_CONTEXT.count, 3);
        }
    }

    // Add tests
    #[test]
    fn ut_test_setup() {
        // Full coverage with just this section of tests
        test_cf_chunk_create_add_reset();
        test_cf_chunk_combine();
        test_cf_chunk_get_rm_first();
        test_cf_chunk_compute_gaps();
    }
}