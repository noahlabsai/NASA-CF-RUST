//! CF Application CRC calculation module.
//!
//! Streaming CRC calculator. Data can all be given at once for a result
//! or it can trickle in.
//!
//! Translated from: cf_crc.c / cf_crc.h

use crate::cf_crc_types::CF_Crc_t;

/// Start a CRC streamable digest.
///
/// C original: `void CF_CRC_Start(CF_Crc_t *crc)` — does `memset(crc, 0, sizeof(*crc))`.
///
/// # Panics
/// Never (infallible).
pub fn CF_CRC_Start(crc: &mut CF_Crc_t) {
    crc.working = 0;
    crc.result = 0;
    crc.index = 0;
}

/// Digest a chunk for CRC calculation.
///
/// Does the CRC calculation, and stores an index into the given 4-byte word
/// in case the input was not evenly divisible by 4.
///
/// C original: `void CF_CRC_Digest(CF_Crc_t *crc, const uint8 *data, size_t len)`
///
/// # Panics
/// Never (infallible).
pub fn CF_CRC_Digest(crc: &mut CF_Crc_t, data: &[u8]) {
    for &byte in data {
        crc.working <<= 8;
        crc.working |= byte as u32;

        crc.index += 1;

        if crc.index == 4 {
            crc.result = crc.result.wrapping_add(crc.working);
            crc.index = 0;
        }
    }
}

/// Finalize a CRC calculation.
///
/// Checks the index and if it isn't 0, does the final calculations on the
/// bytes in the shift register. After this call, the `result` field holds
/// the CRC result.
///
/// The index and working fields are reset so the user may call `CF_CRC_Digest`
/// again to continue accumulating into the same result.
///
/// C original: `void CF_CRC_Finalize(CF_Crc_t *crc)`
///
/// # Panics
/// Never (infallible).
pub fn CF_CRC_Finalize(crc: &mut CF_Crc_t) {
    if crc.index != 0 {
        crc.result = crc
            .result
            .wrapping_add(crc.working << (8 * (4 - crc.index as u32)));

        // Reset so the user can call CF_CRC_Digest() again and it will
        // add new data to the CRC result (lets user get intermediate results).
        crc.index = 0;
        crc.working = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_empty() {
        let mut crc = CF_Crc_t::default();
        CF_CRC_Start(&mut crc);
        CF_CRC_Finalize(&mut crc);
        assert_eq!(crc.result, 0);
    }

    #[test]
    fn test_crc_four_bytes() {
        let mut crc = CF_Crc_t::default();
        CF_CRC_Start(&mut crc);
        CF_CRC_Digest(&mut crc, &[0x01, 0x02, 0x03, 0x04]);
        CF_CRC_Finalize(&mut crc);
        // working = 0x01020304, result = 0x01020304
        assert_eq!(crc.result, 0x01020304);
    }

    #[test]
    fn test_crc_partial_finalize() {
        let mut crc = CF_Crc_t::default();
        CF_CRC_Start(&mut crc);
        CF_CRC_Digest(&mut crc, &[0xAA, 0xBB]);
        // index=2, working=0x0000AABB
        CF_CRC_Finalize(&mut crc);
        // shift left by 8*(4-2)=16 → 0xAABB0000
        assert_eq!(crc.result, 0xAABB0000);
    }

    #[test]
    fn test_crc_streaming() {
        let mut crc = CF_Crc_t::default();
        CF_CRC_Start(&mut crc);
        CF_CRC_Digest(&mut crc, &[0x01, 0x02]);
        CF_CRC_Digest(&mut crc, &[0x03, 0x04]);
        CF_CRC_Finalize(&mut crc);
        assert_eq!(crc.result, 0x01020304);
    }

    #[test]
    fn test_crc_wrapping() {
        let mut crc = CF_Crc_t::default();
        CF_CRC_Start(&mut crc);
        CF_CRC_Digest(&mut crc, &[0xFF, 0xFF, 0xFF, 0xFF]);
        CF_CRC_Digest(&mut crc, &[0x00, 0x00, 0x00, 0x01]);
        CF_CRC_Finalize(&mut crc);
        // 0xFFFFFFFF + 0x00000001 = 0x00000000 (wrapping)
        assert_eq!(crc.result, 0x00000000);
    }
}