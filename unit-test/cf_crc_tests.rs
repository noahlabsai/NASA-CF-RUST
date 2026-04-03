use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cf_crc_start() {
        /* Arrange */
        let mut crc = CF_Crc_t {
            working: 0xFFFFFFFF,
            result: 0xFFFFFFFF,
            index: 0xFFFFFFFF,
        };

        /* Act */
        CF_CRC_Start(&mut crc);

        /* Assert */
        assert_eq!(crc.working, 0);
        assert_eq!(crc.result, 0);
        assert_eq!(crc.index, 0);
    }

    #[test]
    fn test_cf_crc_digest() {
        let mut crc = CF_Crc_t::default();
        let data: [u8; 5] = [1, 2, 3, 4, 5];

        /* Already tested, so OK to use */
        CF_CRC_Start(&mut crc);

        /* Zero length should leave crc as zeros */
        CF_CRC_Digest(&mut crc, &[], 0);
        assert_eq!(crc.working, 0);
        assert_eq!(crc.result, 0);
        assert_eq!(crc.index, 0);

        /* Digest data and confirm */
        CF_CRC_Digest(&mut crc, &data, data.len());
        assert_eq!(crc.working, (data[1] as u32) << 24 | (data[2] as u32) << 16 | (data[3] as u32) << 8 | data[4] as u32);
        assert_eq!(crc.result, (data[0] as u32) << 24 | (data[1] as u32) << 16 | (data[2] as u32) << 8 | data[3] as u32);
        assert_eq!(crc.index, 1);
    }

    #[test]
    fn test_cf_crc_finalize() {
        let mut crc = CF_Crc_t::default();
        let data: [u8; 5] = [1, 2, 3, 4, 5];

        /* Already tested, so OK to use */
        CF_CRC_Start(&mut crc);

        /* Test with clear crc */
        CF_CRC_Finalize(&mut crc);
        assert_eq!(crc.working, 0);
        assert_eq!(crc.result, 0);
        assert_eq!(crc.index, 0);

        /* Already tested, so OK to use */
        CF_CRC_Digest(&mut crc, &data, data.len());

        /* Test with filled in crc */
        CF_CRC_Finalize(&mut crc);
        assert_eq!(crc.working, 0);
        assert_eq!(crc.result, ((data[0] as u32 + data[4] as u32) << 24) | (data[1] as u32) << 16 | (data[2] as u32) << 8 | data[3] as u32);
        assert_eq!(crc.index, 0);
    }
}