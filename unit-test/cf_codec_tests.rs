use std::mem;

// Mock types and constants - these would normally come from other modules
type CF_EncoderState_t = EncoderState;
type CF_DecoderState_t = DecoderState;
type CF_Logical_PduHeader_t = LogicalPduHeader;
type CF_Logical_PduFileDirectiveHeader_t = LogicalPduFileDirectiveHeader;
type CF_Logical_Lv_t = LogicalLv;
type CF_Logical_Tlv_t = LogicalTlv;
type CF_Logical_SegmentRequest_t = LogicalSegmentRequest;
type CF_Logical_TlvList_t = LogicalTlvList;
type CF_Logical_SegmentList_t = LogicalSegmentList;
type CF_Logical_PduMd_t = LogicalPduMd;
type CF_Logical_PduFileDataHeader_t = LogicalPduFileDataHeader;
type CF_Logical_PduEof_t = LogicalPduEof;
type CF_Logical_PduFin_t = LogicalPduFin;
type CF_Logical_PduAck_t = LogicalPduAck;
type CF_Logical_PduNak_t = LogicalPduNak;

const CF_CFDP_TLV_TYPE_ENTITY_ID: u8 = 6;
const CF_CFDP_FileDirective_NAK: u8 = 8;
const CF_PDU_MAX_TLV: u8 = 10;
const CF_PDU_MAX_SEGMENTS: u8 = 10;

#[derive(Debug, Default)]
struct CodecState {
    next_offset: usize,
}

#[derive(Debug, Default)]
struct EncoderState {
    base: *mut u8,
    codec_state: CodecState,
}

#[derive(Debug, Default)]
struct DecoderState {
    base: *const u8,
    codec_state: CodecState,
}

#[derive(Debug, Default)]
struct LogicalPduHeader {
    version: u8,
    direction: u8,
    pdu_type: u8,
    txm_mode: u8,
    txn_seq_length: u8,
    eid_length: u8,
    source_eid: u64,
    sequence_num: u64,
    destination_eid: u64,
    data_encoded_length: u16,
    header_encoded_length: usize,
}

#[derive(Debug, Default)]
struct LogicalPduFileDirectiveHeader {
    directive_code: u8,
}

#[derive(Debug, Default)]
struct LogicalLv {
    length: u8,
    data_ptr: *const u8,
}

#[derive(Debug, Default)]
struct LogicalTlv {
    type_: u8,
    length: u8,
    data: TlvData,
}

#[derive(Debug)]
union TlvData {
    eid: u64,
    data_ptr: *const u8,
}

impl Default for TlvData {
    fn default() -> Self {
        Self { eid: 0 }
    }
}

#[derive(Debug, Default)]
struct LogicalSegmentRequest {
    offset_start: u32,
    offset_end: u32,
}

#[derive(Debug, Default)]
struct LogicalTlvList {
    num_tlv: u8,
    tlv: [LogicalTlv; 16],
}

#[derive(Debug, Default)]
struct LogicalSegmentList {
    num_segments: u8,
    segments: [LogicalSegmentRequest; 16],
}

#[derive(Debug, Default)]
struct LogicalPduMd {
    size: u32,
    source_filename: LogicalLv,
    dest_filename: LogicalLv,
}

#[derive(Debug, Default)]
struct LogicalPduFileDataHeader {
    offset: u32,
    data_len: usize,
    data_ptr: *const u8,
    continuation_state: u8,
    segment_list: LogicalSegmentList,
}

#[derive(Debug, Default)]
struct LogicalPduEof {
    crc: u32,
    size: u32,
    cc: u8,
    tlv_list: LogicalTlvList,
}

#[derive(Debug, Default)]
struct LogicalPduFin {
    cc: u8,
    delivery_code: u8,
    file_status: u8,
}

#[derive(Debug, Default)]
struct LogicalPduAck {
    ack_directive_code: u8,
    ack_subtype_code: u8,
    cc: u8,
    txn_status: u8,
}

#[derive(Debug, Default)]
struct LogicalPduNak {
    scope_start: u32,
    scope_end: u32,
    segment_list: LogicalSegmentList,
}

// Mock functions
fn CF_CFDP_GetValueEncodedSize(_value: u64) -> u8 { 1 }
fn CF_EncodeIntegerInSize(_state: &mut CF_EncoderState_t, _value: u64, _encode_size: u8) {}
fn CF_CFDP_EncodeHeaderWithoutSize(_state: &mut CF_EncoderState_t, _plh: &CF_Logical_PduHeader_t) {}
fn CF_CFDP_EncodeHeaderFinalSize(_state: &mut CF_EncoderState_t, _plh: &CF_Logical_PduHeader_t) {}
fn CF_CFDP_EncodeFileDirectiveHeader(_state: &mut CF_EncoderState_t, _pfdir: &CF_Logical_PduFileDirectiveHeader_t) {}
fn CF_CFDP_EncodeLV(_state: &mut CF_EncoderState_t, _pllv: &CF_Logical_Lv_t) {}
fn CF_CFDP_EncodeTLV(_state: &mut CF_EncoderState_t, _pltlv: &CF_Logical_Tlv_t) {}
fn CF_CFDP_EncodeSegmentRequest(_state: &mut CF_EncoderState_t, _plseg: &CF_Logical_SegmentRequest_t) {}
fn CF_CFDP_EncodeAllTlv(_state: &mut CF_EncoderState_t, _pltlv: &CF_Logical_TlvList_t) {}
fn CF_CFDP_EncodeAllSegments(_state: &mut CF_EncoderState_t, _plseg: &CF_Logical_SegmentList_t) {}
fn CF_CFDP_EncodeMd(_state: &mut CF_EncoderState_t, _plmd: &CF_Logical_PduMd_t) {}
fn CF_CFDP_EncodeFileDataHeader(_state: &mut CF_EncoderState_t, _with_meta: bool, _plfd: &CF_Logical_PduFileDataHeader_t) {}
fn CF_CFDP_EncodeEof(_state: &mut CF_EncoderState_t, _pleof: &CF_Logical_PduEof_t) {}
fn CF_CFDP_EncodeFin(_state: &mut CF_EncoderState_t, _plfin: &CF_Logical_PduFin_t) {}
fn CF_CFDP_EncodeAck(_state: &mut CF_EncoderState_t, _plack: &CF_Logical_PduAck_t) {}
fn CF_CFDP_EncodeNak(_state: &mut CF_EncoderState_t, _plnak: &CF_Logical_PduNak_t) {}
fn CF_CFDP_EncodeCrc(_state: &mut CF_EncoderState_t, _pcrc: &u32) {}
fn CF_DecodeIntegerInSize(_state: &mut CF_DecoderState_t, _decode_size: u8) -> u64 { 0 }
fn CF_CFDP_DecodeHeader(_state: &mut CF_DecoderState_t, _plh: &mut CF_Logical_PduHeader_t) -> i32 { 0 }
fn CF_CFDP_DecodeFileDirectiveHeader(_state: &mut CF_DecoderState_t, _pfdir: &mut CF_Logical_PduFileDirectiveHeader_t) {}
fn CF_CFDP_DecodeLV(_state: &mut CF_DecoderState_t, _pllv: &mut CF_Logical_Lv_t) {}
fn CF_CFDP_DecodeTLV(_state: &mut CF_DecoderState_t, _pltlv: &mut CF_Logical_Tlv_t) {}
fn CF_CFDP_DecodeSegmentRequest(_state: &mut CF_DecoderState_t, _plseg: &mut CF_Logical_SegmentRequest_t) {}
fn CF_CFDP_DecodeAllTlv(_state: &mut CF_DecoderState_t, _pltlv: &mut CF_Logical_TlvList_t, _limit: u8) {}
fn CF_CFDP_DecodeAllSegments(_state: &mut CF_DecoderState_t, _plseg: &mut CF_Logical_SegmentList_t, _limit: u8) {}
fn CF_CFDP_DecodeMd(_state: &mut CF_DecoderState_t, _plmd: &mut CF_Logical_PduMd_t) {}
fn CF_CFDP_DecodeFileDataHeader(_state: &mut CF_DecoderState_t, _with_meta: bool, _plfd: &mut CF_Logical_PduFileDataHeader_t) {}
fn CF_CFDP_DecodeEof(_state: &mut CF_DecoderState_t, _pleof: &mut CF_Logical_PduEof_t) {}
fn CF_CFDP_DecodeFin(_state: &mut CF_DecoderState_t, _plfin: &mut CF_Logical_PduFin_t) {}
fn CF_CFDP_DecodeAck(_state: &mut CF_DecoderState_t, _plack: &mut CF_Logical_PduAck_t) {}
fn CF_CFDP_DecodeNak(_state: &mut CF_DecoderState_t, _plnak: &mut CF_Logical_PduNak_t) {}
fn CF_CFDP_DecodeCrc(_state: &mut CF_DecoderState_t, _pcrc: &mut u32) {}
fn CF_CFDP_CodecReset(_codec_state: &mut CodecState, _sz: usize) {}

macro_rules! CF_CODEC_IS_OK {
    ($state:expr) => { true };
}

macro_rules! CF_CODEC_GET_POSITION {
    ($state:expr) => { 0 };
}

macro_rules! CF_CODEC_GET_REMAIN {
    ($state:expr) => { 0 };
}

fn ut_cf_setup_encode_state(state: &mut CF_EncoderState_t, bytes: &mut [u8], sz: usize) {
    *state = CF_EncoderState_t::default();
    state.base = bytes.as_mut_ptr();
    CF_CFDP_CodecReset(&mut state.codec_state, sz);
}

fn ut_cf_setup_decode_state(state: &mut CF_DecoderState_t, bytes: &[u8], sz: usize) {
    *state = CF_DecoderState_t::default();
    state.base = bytes.as_ptr();
    CF_CFDP_CodecReset(&mut state.codec_state, sz);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cf_cfdp_get_value_encoded_size() {
        assert_eq!(CF_CFDP_GetValueEncodedSize(0), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(1), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(126), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(u8::MAX as u64), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(u8::MAX as u64 + 1), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(u16::MAX as u64), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(u16::MAX as u64 + 1), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(16777215), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(16777216), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(u32::MAX as u64), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(u32::MAX as u64 + 1), 1);
        assert_eq!(CF_CFDP_GetValueEncodedSize(u64::MAX), 1);
    }

    #[test]
    fn test_cf_encode_integer_in_size() {
        let mut state = CF_EncoderState_t::default();
        let mut bytes = [0xEE; 10];
        let expected_2 = [0x12, 0x34];
        let expected_4 = [0x00, 0x00, 0x12, 0x34];

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_EncodeIntegerInSize(&mut state, 0x1234, 2);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_EncodeIntegerInSize(&mut state, 0x1234, expected_2.len() as u8);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected_2.len());
        assert_eq!(&bytes[..expected_2.len()], &expected_2);
        assert_eq!(&bytes[expected_2.len()..], &[0xEE; 10 - 2]);

        bytes.fill(0xEE);
        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_EncodeIntegerInSize(&mut state, 0x1234, expected_4.len() as u8);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected_4.len());
        assert_eq!(&bytes[..expected_4.len()], &expected_4);
        assert_eq!(&bytes[expected_4.len()..], &[0xEE; 10 - 4]);
    }

    #[test]
    fn test_cf_cfdp_encode_header_without_size() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_PduHeader_t::default();
        let mut bytes = [0xEE; 10];
        let expected = [0x3c, 0xEE, 0xEE, 0x00, 0x44, 0x55, 0x66];

        input.version = 1;
        input.direction = 1;
        input.pdu_type = 1;
        input.txm_mode = 1;
        input.txn_seq_length = 1;
        input.eid_length = 1;
        input.source_eid = 0x44;
        input.sequence_num = 0x55;
        input.destination_eid = 0x66;

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeHeaderWithoutSize(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeHeaderWithoutSize(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 10 - 7]);
    }

    #[test]
    fn test_cf_cfdp_encode_header_final_size() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_PduHeader_t::default();
        let mut bytes = [0xEE; 10];
        let expected = [0xEE, 0x12, 0x34, 0xEE];

        input.data_encoded_length = 0x1234;

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeHeaderFinalSize(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        state.codec_state.next_offset = mem::size_of::<u32>();
        CF_CFDP_EncodeHeaderFinalSize(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(&bytes[..expected.len()], &expected);

        CF_CFDP_EncodeHeaderFinalSize(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
    }

    #[test]
    fn test_cf_cfdp_encode_file_directive_header() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_PduFileDirectiveHeader_t::default();
        let mut bytes = [0xEE; 10];
        let expected = [0x07];

        input.directive_code = 7;

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeFileDirectiveHeader(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeFileDirectiveHeader(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 10 - 1]);
    }

    #[test]
    fn test_cf_cfdp_encode_lv() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_Lv_t::default();
        let mut bytes = [0xEE; 10];
        let ref_data = [0x45, 0x67, 0x89];
        let expected = [0x03, 0x45, 0x67, 0x89];
        let expected_nodata = [0x00];

        input.length = ref_data.len() as u8;
        input.data_ptr = ref_data.as_ptr();

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeLV(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeLV(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 10 - 4]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, expected.len() - 1);
        CF_CFDP_EncodeLV(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));

        input.data_ptr = std::ptr::null();
        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeLV(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));

        input.length = 0;
        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeLV(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected_nodata.len());
        assert_eq!(&bytes[..expected_nodata.len()], &expected_nodata);
    }

    #[test]
    fn test_cf_cfdp_encode_tlv() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_Tlv_t::default();
        let mut bytes = [0xEE; 10];
        let expected_tlv = [0x06, 0x01, 0x77];
        let expected_other = [0x01, 0x03, b'a', b'b', b'c'];
        let expected_nodata = [0x01, 0x00];

        input.type_ = CF_CFDP_TLV_TYPE_ENTITY_ID;
        input.length = 1;
        input.data.eid = 0x77;

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeTLV(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeTLV(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected_tlv.len());
        assert_eq!(&bytes[..expected_tlv.len()], &expected_tlv);
        assert_eq!(&bytes[expected_tlv.len()..], &[0xEE; 10 - 3]);

        input.type_ = 1;
        input.length = 3;
        input.data.data_ptr = b"abc".as_ptr();
        bytes.fill(0xEE);
        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeTLV(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected_other.len());
        assert_eq!(&bytes[..expected_other.len()], &expected_other);

        ut_cf_setup_encode_state(&mut state, &mut bytes, expected_other.len() - 1);
        CF_CFDP_EncodeTLV(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));

        input.data.data_ptr = std::ptr::null();
        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeTLV(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));

        input.length = 0;
        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeTLV(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected_nodata.len());
        assert_eq!(&bytes[..expected_nodata.len()], &expected_nodata);
    }

    #[test]
    fn test_cf_cfdp_encode_segment_request() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_SegmentRequest_t::default();
        let mut bytes = [0xEE; 10];
        let expected = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];

        input.offset_start = 0x11223344;
        input.offset_end = 0x55667788;

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeSegmentRequest(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeSegmentRequest(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 10 - 8]);
    }

    #[test]
    fn test_cf_cfdp_encode_all_tlv() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_TlvList_t::default();
        let mut bytes = [0xEE; 10];
        let expected = [0x06, 0x01, 0x88, 0x06, 0x01, 0x99];

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeAllTlv(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        input.num_tlv = 2;
        input.tlv[0].type_ = CF_CFDP_TLV_TYPE_ENTITY_ID;
        input.tlv[0].length = 1;
        input.tlv[0].data.eid = 0x88;
        input.tlv[1].type_ = CF_CFDP_TLV_TYPE_ENTITY_ID;
        input.tlv[1].length = 1;
        input.tlv[1].data.eid = 0x99;
        CF_CFDP_EncodeAllTlv(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeAllTlv(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 10 - 6]);
    }

    #[test]
    fn test_cf_cfdp_encode_all_segments() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_SegmentList_t::default();
        let mut bytes = [0xEE; 20];
        let expected = [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
                       0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04];

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeAllSegments(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 20]);

        input.num_segments = 2;
        input.segments[0].offset_start = 0x1;
        input.segments[0].offset_end = 0x2;
        input.segments[1].offset_start = 0x3;
        input.segments[1].offset_end = 0x4;
        CF_CFDP_EncodeAllSegments(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 20]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeAllSegments(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 20 - 16]);
    }

    #[test]
    fn test_cf_cfdp_encode_md() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_PduMd_t::default();
        let mut bytes = [0xEE; 20];
        let expected = [0x00, 0x00, 0x00, 0x12, 0x34, 0x03, b's', b'r', b'c', 0x04, b'd', b'e', b's', b't'];

        input.size = 0x1234;
        input.dest_filename.length = 4;
        input.dest_filename.data_ptr = b"dest".as_ptr();
        input.source_filename.length = 3;
        input.source_filename.data_ptr = b"src".as_ptr();

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeMd(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 20]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeMd(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 20 - 14]);
    }

    #[test]
    fn test_cf_cfdp_encode_file_data_header() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_PduFileDataHeader_t::default();
        let mut bytes = [0xEE; 20];
        let expected_basic = [0x00, 0x00, 0x00, 0x13];
        let expected_meta = [0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x13];

        input.offset = 0x13;
        input.data_len = 4;
        input.data_ptr = b"data".as_ptr();

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeFileDataHeader(&mut state, false, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 20]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeFileDataHeader(&mut state, false, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected_basic.len());
        assert_eq!(&bytes[..expected_basic.len()], &expected_basic);
        assert_eq!(&bytes[expected_basic.len()..], &[0xEE; 20 - 4]);

        input.continuation_state = 1;
        input.segment_list.num_segments = 1;
        input.segment_list.segments[0].offset_start = 0;
        input.segment_list.segments[0].offset_end = 0x11;
        bytes.fill(0xEE);
        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeFileDataHeader(&mut state, true, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected_meta.len());
        assert_eq!(&bytes[..expected_meta.len()], &expected_meta);
    }

    #[test]
    fn test_cf_cfdp_encode_eof() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_PduEof_t::default();
        let mut bytes = [0xEE; 20];
        let expected = [0x10, 0x12, 0x34, 0x56, 0x78, 0x00, 0x00, 0x45, 0x67, 0x06, 0x01, 0xaa];

        input.crc = 0x12345678;
        input.size = 0x4567;
        input.cc = 1;
        input.tlv_list.num_tlv = 1;
        input.tlv_list.tlv[0].type_ = CF_CFDP_TLV_TYPE_ENTITY_ID;
        input.tlv_list.tlv[0].length = 1;
        input.tlv_list.tlv[0].data.eid = 0xaa;

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeEof(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 20]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeEof(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 20 - 12]);
    }

    #[test]
    fn test_cf_cfdp_encode_fin() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_PduFin_t::default();
        let mut bytes = [0xEE; 10];
        let expected = [0x16];

        input.cc = 1;
        input.delivery_code = 1;
        input.file_status = 2;

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeFin(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeFin(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 10 - 1]);
    }

    #[test]
    fn test_cf_cfdp_encode_ack() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_PduAck_t::default();
        let mut bytes = [0xEE; 10];
        let expected = [0x51, 0x23];

        input.ack_directive_code = 5;
        input.ack_subtype_code = 1;
        input.cc = 2;
        input.txn_status = 3;

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeAck(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeAck(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 10 - 2]);
    }

    #[test]
    fn test_cf_cfdp_encode_nak() {
        let mut state = CF_EncoderState_t::default();
        let mut input = CF_Logical_PduNak_t::default();
        let mut bytes = [0xEE; 30];
        let expected = [0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x03, 0x04, 0x00, 0x00, 0x00, 0x05,
                       0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08];

        input.scope_start = 0x0102;
        input.scope_end = 0x0304;
        input.segment_list.num_segments = 2;
        input.segment_list.segments[0].offset_start = 0x5;
        input.segment_list.segments[0].offset_end = 0x6;
        input.segment_list.segments[1].offset_start = 0x7;
        input.segment_list.segments[1].offset_end = 0x8;

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeNak(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 30]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeNak(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 30 - 24]);
    }

    #[test]
    fn test_cf_cfdp_encode_crc() {
        let mut state = CF_EncoderState_t::default();
        let input = 0xdeadbeef;
        let mut bytes = [0xEE; 10];
        let expected = [0xde, 0xad, 0xbe, 0xef];

        ut_cf_setup_encode_state(&mut state, &mut bytes, 0);
        CF_CFDP_EncodeCrc(&mut state, &input);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(bytes, [0xEE; 10]);

        ut_cf_setup_encode_state(&mut state, &mut bytes, bytes.len());
        CF_CFDP_EncodeCrc(&mut state, &input);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), expected.len());
        assert_eq!(&bytes[..expected.len()], &expected);
        assert_eq!(&bytes[expected.len()..], &[0xEE; 10 - 4]);
    }

    #[test]
    fn test_cf_decode_integer_in_size() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 16];
        let bytes_2 = [0x12, 0x34];
        let bytes_4 = [0x00, 0x56, 0x78, 0x9a];

        ut_cf_setup_decode_state(&mut state, &bytes_2, 0);
        assert_eq!(CF_DecodeIntegerInSize(&mut state, 2), 0);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 16]);

        ut_cf_setup_decode_state(&mut state, &bytes_2, bytes_2.len());
        assert_eq!(CF_DecodeIntegerInSize(&mut state, bytes_2.len() as u8), 0x1234);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes_2.len());

        ut_cf_setup_decode_state(&mut state, &bytes_4, bytes_4.len());
        assert_eq!(CF_DecodeIntegerInSize(&mut state, bytes_4.len() as u8), 0x56789a);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes_4.len());
    }

    #[test]
    fn test_cf_cfdp_decode_header() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 64];
        let bytes = [0x3c, 0x01, 0x02, 0x00, 0x44, 0x55, 0x66];
        let bad_eid = [0x3c, 0x01, 0x02, 0x73, 0x44, 0x55, 0x66];
        let bad_tsn = [0x3c, 0x01, 0x02, 0x37, 0x44, 0x55, 0x66];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        assert_eq!(CF_CFDP_DecodeHeader(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduHeader_t), 0);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 64]);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        assert_eq!(CF_CFDP_DecodeHeader(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduHeader_t), 0);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());

        ut_cf_setup_decode_state(&mut state, &bad_eid, bad_eid.len());
        assert_eq!(CF_CFDP_DecodeHeader(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduHeader_t), -1);

        ut_cf_setup_decode_state(&mut state, &bad_tsn, bad_tsn.len());
        assert_eq!(CF_CFDP_DecodeHeader(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduHeader_t), -1);
    }

    #[test]
    fn test_cf_cfdp_decode_file_directive_header() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 16];
        let bytes = [0x08];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeFileDirectiveHeader(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduFileDirectiveHeader_t);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 16]);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeFileDirectiveHeader(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduFileDirectiveHeader_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());
    }

    #[test]
    fn test_cf_cfdp_decode_lv() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 16];
        let bytes = [0x03, 0x45, 0x67, 0x89];
        let bad_input = [0x32, 0x45, 0x67, 0x89, 0xaa];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeLV(&mut state, out.as_mut_ptr() as *mut CF_Logical_Lv_t);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 16]);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeLV(&mut state, out.as_mut_ptr() as *mut CF_Logical_Lv_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());

        ut_cf_setup_decode_state(&mut state, &bad_input, bad_input.len());
        CF_CFDP_DecodeLV(&mut state, out.as_mut_ptr() as *mut CF_Logical_Lv_t);
        assert!(!CF_CODEC_IS_OK!(&state));
    }

    #[test]
    fn test_cf_cfdp_decode_tlv() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 32];
        let bytes_tlv = [0x06, 0x01, 0x77];
        let bytes_other = [0x01, 0x02, 0x88, 0x99];
        let bad_input = [0x06, 0x21, 0x88, 0x99];

        ut_cf_setup_decode_state(&mut state, &bytes_tlv, 0);
        CF_CFDP_DecodeTLV(&mut state, out.as_mut_ptr() as *mut CF_Logical_Tlv_t);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 32]);

        ut_cf_setup_decode_state(&mut state, &bytes_tlv, bytes_tlv.len());
        CF_CFDP_DecodeTLV(&mut state, out.as_mut_ptr() as *mut CF_Logical_Tlv_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes_tlv.len());

        ut_cf_setup_decode_state(&mut state, &bytes_other, bytes_other.len());
        CF_CFDP_DecodeTLV(&mut state, out.as_mut_ptr() as *mut CF_Logical_Tlv_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes_other.len());

        ut_cf_setup_decode_state(&mut state, &bad_input, bad_input.len());
        CF_CFDP_DecodeTLV(&mut state, out.as_mut_ptr() as *mut CF_Logical_Tlv_t);
        assert!(!CF_CODEC_IS_OK!(&state));
    }

    #[test]
    fn test_cf_cfdp_decode_segment_request() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 16];
        let bytes = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeSegmentRequest(&mut state, out.as_mut_ptr() as *mut CF_Logical_SegmentRequest_t);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 16]);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeSegmentRequest(&mut state, out.as_mut_ptr() as *mut CF_Logical_SegmentRequest_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());
    }

    #[test]
    fn test_cf_cfdp_decode_all_tlv() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 128];
        let bytes = [0x06, 0x01, 0x88, 0x06, 0x01, 0x99];
        let bad_input = [0x06, 0x07, 0x88, 0x06, 0x03, 0x99, 0xaa];
        let long_input = [0u8; 2 * (CF_PDU_MAX_TLV as usize + 1)];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeAllTlv(&mut state, out.as_mut_ptr() as *mut CF_Logical_TlvList_t, CF_PDU_MAX_TLV);
        assert!(CF_CODEC_IS_OK!(&state));

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeAllTlv(&mut state, out.as_mut_ptr() as *mut CF_Logical_TlvList_t, 1);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), 3);
        assert_eq!(CF_CODEC_GET_REMAIN!(&state), 3);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeAllTlv(&mut state, out.as_mut_ptr() as *mut CF_Logical_TlvList_t, CF_PDU_MAX_TLV);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());

        ut_cf_setup_decode_state(&mut state, &bad_input, bad_input.len());
        CF_CFDP_DecodeAllTlv(&mut state, out.as_mut_ptr() as *mut CF_Logical_TlvList_t, CF_PDU_MAX_TLV);
        assert!(!CF_CODEC_IS_OK!(&state));

        ut_cf_setup_decode_state(&mut state, &long_input, long_input.len());
        CF_CFDP_DecodeAllTlv(&mut state, out.as_mut_ptr() as *mut CF_Logical_TlvList_t, 1 + CF_PDU_MAX_TLV);
        assert!(!CF_CODEC_IS_OK!(&state));
    }

    #[test]
    fn test_cf_cfdp_decode_all_segments() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 128];
        let bytes = [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
                    0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04];
        let long_input = [0u8; 8 * (CF_PDU_MAX_SEGMENTS as usize + 1)];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeAllSegments(&mut state, out.as_mut_ptr() as *mut CF_Logical_SegmentList_t, CF_PDU_MAX_SEGMENTS);
        assert!(CF_CODEC_IS_OK!(&state));

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeAllSegments(&mut state, out.as_mut_ptr() as *mut CF_Logical_SegmentList_t, 1);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), 8);
        assert_eq!(CF_CODEC_GET_REMAIN!(&state), 8);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeAllSegments(&mut state, out.as_mut_ptr() as *mut CF_Logical_SegmentList_t, CF_PDU_MAX_SEGMENTS);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());

        ut_cf_setup_decode_state(&mut state, &long_input, long_input.len());
        CF_CFDP_DecodeAllSegments(&mut state, out.as_mut_ptr() as *mut CF_Logical_SegmentList_t, 1 + CF_PDU_MAX_SEGMENTS);
        assert!(!CF_CODEC_IS_OK!(&state));
    }

    #[test]
    fn test_cf_cfdp_decode_md() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 64];
        let bytes = [0x00, 0x00, 0x00, 0x12, 0x34, 0x03, b's', b'r', b'c', 0x04, b'd', b'e', b's', b't'];
        let bad_input = [0x00, 0x00, 0x00, 0x12, 0x34, 0x56, b's', b'r', b'c', 0x04, b'd', b'e', b's', b't'];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeMd(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduMd_t);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 64]);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeMd(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduMd_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());

        ut_cf_setup_decode_state(&mut state, &bad_input, bad_input.len());
        CF_CFDP_DecodeMd(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduMd_t);
        assert!(!CF_CODEC_IS_OK!(&state));
    }

    #[test]
    fn test_cf_cfdp_decode_file_data_header() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 128];
        let bytes_basic = [0x00, 0x00, 0x00, 0x13, 0xdd];
        let bytes_meta = [0x41, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x00, 0x00, 0x13, 0xcc];
        let bad_input_1 = [0xff, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x00, 0x00, 0x13, 0xcc];
        let bad_input_2 = [0x41, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];

        ut_cf_setup_decode_state(&mut state, &bytes_basic, 0);
        CF_CFDP_DecodeFileDataHeader(&mut state, false, out.as_mut_ptr() as *mut CF_Logical_PduFileDataHeader_t);
        assert!(!CF_CODEC_IS_OK!(&state));

        ut_cf_setup_decode_state(&mut state, &bytes_basic, bytes_basic.len());
        CF_CFDP_DecodeFileDataHeader(&mut state, false, out.as_mut_ptr() as *mut CF_Logical_PduFileDataHeader_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes_basic.len());

        ut_cf_setup_decode_state(&mut state, &bytes_meta, bytes_meta.len());
        CF_CFDP_DecodeFileDataHeader(&mut state, true, out.as_mut_ptr() as *mut CF_Logical_PduFileDataHeader_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes_meta.len());

        ut_cf_setup_decode_state(&mut state, &bad_input_1, bad_input_1.len());
        CF_CFDP_DecodeFileDataHeader(&mut state, true, out.as_mut_ptr() as *mut CF_Logical_PduFileDataHeader_t);
        assert!(!CF_CODEC_IS_OK!(&state));

        ut_cf_setup_decode_state(&mut state, &bad_input_2, bad_input_2.len());
        CF_CFDP_DecodeFileDataHeader(&mut state, true, out.as_mut_ptr() as *mut CF_Logical_PduFileDataHeader_t);
        assert!(!CF_CODEC_IS_OK!(&state));
    }

    #[test]
    fn test_cf_cfdp_decode_eof() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 128];
        let bytes = [0x10, 0x12, 0x34, 0x56, 0x78, 0x00, 0x00, 0x45, 0x67, 0x06, 0x01, 0xaa];
        let bad_input = [0x10, 0x12, 0x34, 0x56, 0x78, 0x00, 0x00, 0x45, 0x67, 0x06, 0x06, 0xaa, 0xbb];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeEof(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduEof_t);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 128]);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeEof(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduEof_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());

        ut_cf_setup_decode_state(&mut state, &bad_input, bad_input.len());
        CF_CFDP_DecodeEof(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduEof_t);
        assert!(!CF_CODEC_IS_OK!(&state));
    }

    #[test]
    fn test_cf_cfdp_decode_fin() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 16];
        let bytes = [0x16];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeFin(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduFin_t);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 16]);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeFin(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduFin_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());
    }

    #[test]
    fn test_cf_cfdp_decode_ack() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 16];
        let bytes = [0x51, 0x23];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeAck(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduAck_t);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 16]);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeAck(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduAck_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());
    }

    #[test]
    fn test_cf_cfdp_decode_nak() {
        let mut state = CF_DecoderState_t::default();
        let mut out = [0xEE; 128];
        let bytes = [0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x03, 0x04, 0x00, 0x00, 0x00, 0x05,
                    0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08];

        ut_cf_setup_decode_state(&mut state, &bytes, 0);
        CF_CFDP_DecodeNak(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduNak_t);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, [0xEE; 128]);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeNak(&mut state, out.as_mut_ptr() as *mut CF_Logical_PduNak_t);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());
    }

    #[test]
    fn test_cf_cfdp_decode_crc() {
        let mut state = CF_DecoderState_t::default();
        let mut out = 0xEEEEEEEE;
        let bytes = [0xde, 0xad, 0xbe, 0xef];

        ut_cf_setup_decode_state(&mut state, b"", 0);
        CF_CFDP_DecodeCrc(&mut state, &mut out);
        assert!(!CF_CODEC_IS_OK!(&state));
        assert_eq!(out, 0xEEEEEEEE);

        ut_cf_setup_decode_state(&mut state, &bytes, bytes.len());
        CF_CFDP_DecodeCrc(&mut state, &mut out);
        assert!(CF_CODEC_IS_OK!(&state));
        assert_eq!(CF_CODEC_GET_POSITION!(&state), bytes.len());
        assert_eq!(out, 0xdeadbeef);
    }
}