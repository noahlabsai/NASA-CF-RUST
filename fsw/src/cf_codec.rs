use std::mem;

// Type aliases for NASA/CFDP types
type CFE_Status_t = i32;
const CFE_SUCCESS: CFE_Status_t = 0;
const CF_ERROR: CFE_Status_t = -1;

type uint8 = u8;
type uint16 = u16;
type uint32 = u32;
type uint64 = u64;

// Entity/Transaction types (matching C's uint32 CF_EntityId_t and CF_TransactionSeq_t)
type CfEntityId = u32;
type CfTransactionSeq = u32;

// Constants
const CF_PDU_MAX_TLV: u8 = 16;
const CF_PDU_MAX_SEGMENTS: u8 = 32;
const CF_CFDP_TLV_TYPE_ENTITY_ID: u8 = 6;

// Bitfield structure
#[derive(Debug, Clone, Copy)]
struct CfCodecBitField {
    shift: u32,
    mask: u32,
}

impl CfCodecBitField {
    const fn new(nbits: u32, shift: u32) -> Self {
        Self {
            shift,
            mask: (1 << nbits) - 1,
        }
    }
}

// Bitfield constants
const CF_CFDP_PDU_HEADER_FLAGS_VERSION: CfCodecBitField = CfCodecBitField::new(3, 5);
const CF_CFDP_PDU_HEADER_FLAGS_TYPE: CfCodecBitField = CfCodecBitField::new(1, 4);
const CF_CFDP_PDU_HEADER_FLAGS_DIR: CfCodecBitField = CfCodecBitField::new(1, 3);
const CF_CFDP_PDU_HEADER_FLAGS_MODE: CfCodecBitField = CfCodecBitField::new(1, 2);
const CF_CFDP_PDU_HEADER_FLAGS_CRC: CfCodecBitField = CfCodecBitField::new(1, 1);
const CF_CFDP_PDU_HEADER_FLAGS_LARGEFILE: CfCodecBitField = CfCodecBitField::new(1, 0);

const CF_CFDP_PDU_HEADER_SEGMENTATION_CONTROL: CfCodecBitField = CfCodecBitField::new(1, 7);
const CF_CFDP_PDU_HEADER_LENGTHS_ENTITY: CfCodecBitField = CfCodecBitField::new(3, 4);
const CF_CFDP_PDU_HEADER_SEGMENT_METADATA: CfCodecBitField = CfCodecBitField::new(1, 3);
const CF_CFDP_PDU_HEADER_LENGTHS_TRANSACTION_SEQUENCE: CfCodecBitField = CfCodecBitField::new(3, 0);

const CF_CFDP_PDU_EOF_FLAGS_CC: CfCodecBitField = CfCodecBitField::new(4, 4);

const CF_CFDP_PDU_FIN_FLAGS_CC: CfCodecBitField = CfCodecBitField::new(4, 4);
const CF_CFDP_PDU_FIN_FLAGS_DELIVERY_CODE: CfCodecBitField = CfCodecBitField::new(1, 2);
const CF_CFDP_PDU_FIN_FLAGS_FILE_STATUS: CfCodecBitField = CfCodecBitField::new(2, 0);

const CF_CFDP_PDU_ACK_DIR_CODE: CfCodecBitField = CfCodecBitField::new(4, 4);
const CF_CFDP_PDU_ACK_DIR_SUBTYPE_CODE: CfCodecBitField = CfCodecBitField::new(4, 0);
const CF_CFDP_PDU_ACK_CC: CfCodecBitField = CfCodecBitField::new(4, 4);
const CF_CFDP_PDU_ACK_TRANSACTION_STATUS: CfCodecBitField = CfCodecBitField::new(2, 0);

const CF_CFDP_PDU_MD_CLOSURE_REQUESTED: CfCodecBitField = CfCodecBitField::new(1, 7);
const CF_CFDP_PDU_MD_CHECKSUM_TYPE: CfCodecBitField = CfCodecBitField::new(4, 0);

const CF_CFDP_PDU_FILE_DATA_RECORD_CONTINUATION_STATE: CfCodecBitField = CfCodecBitField::new(2, 6);
const CF_CFDP_PDU_FILE_DATA_SEGMENT_METADATA_LENGTH: CfCodecBitField = CfCodecBitField::new(6, 0);

// CFDP wire format types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpUint8 {
    octets: [u8; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpUint16 {
    octets: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpUint32 {
    octets: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpUint64 {
    octets: [u8; 8],
}

// PDU structures
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpPduHeader {
    flags: CfCfdpUint8,
    length: CfCfdpUint16,
    eid_tsn_lengths: CfCfdpUint8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpPduFileDirectiveHeader {
    directive_code: CfCfdpUint8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpLv {
    length: CfCfdpUint8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpTlv {
    type_field: CfCfdpUint8,
    length: CfCfdpUint8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpSegmentRequest {
    offset_start: CfCfdpUint32,
    offset_end: CfCfdpUint32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpPduMd {
    segmentation_control: CfCfdpUint8,
    size: CfCfdpUint32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpPduFileDataHeader {
    offset: CfCfdpUint32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpPduEof {
    cc: CfCfdpUint8,
    crc: CfCfdpUint32,
    size: CfCfdpUint32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpPduFin {
    flags: CfCfdpUint8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpPduAck {
    directive_and_subtype_code: CfCfdpUint8,
    cc_and_transaction_status: CfCfdpUint8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CfCfdpPduNak {
    scope_start: CfCfdpUint32,
    scope_end: CfCfdpUint32,
}

// Logical structures
#[derive(Debug, Clone)]
struct CfLogicalPduHeader {
    version: u8,
    direction: u8,
    pdu_type: u8,
    txm_mode: u8,
    crc_flag: u8,
    large_flag: u8,
    segmentation_control: u8,
    eid_length: u8,
    segment_meta_flag: u8,
    txn_seq_length: u8,
    data_encoded_length: u16,
    source_eid: u64,
    sequence_num: u64,
    destination_eid: u64,
    header_encoded_length: usize,
}

#[derive(Debug, Clone)]
struct CfLogicalPduFileDirectiveHeader {
    directive_code: u8,
}

#[derive(Debug, Clone)]
struct CfLogicalLv {
    length: u8,
    data_ptr: Option<*const u8>,
}

#[derive(Clone, Copy)]
union CfLogicalTlvData {
    eid: u64,
    data_ptr: *const u8,
}

impl core::fmt::Debug for CfLogicalTlvData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CfLogicalTlvData {{ ... }}")
    }
}

#[derive(Debug, Clone, Copy)]
struct CfLogicalTlv {
    type_field: u8,
    length: u8,
    data: CfLogicalTlvData,
}

#[derive(Debug, Clone, Copy)]
struct CfLogicalTlvList {
    num_tlv: u8,
    tlv: [CfLogicalTlv; CF_PDU_MAX_TLV as usize],
}

#[derive(Debug, Clone, Copy)]
struct CfLogicalSegmentRequest {
    offset_start: u32,
    offset_end: u32,
}

#[derive(Debug, Clone)]
struct CfLogicalSegmentList {
    num_segments: u8,
    segments: [CfLogicalSegmentRequest; CF_PDU_MAX_SEGMENTS as usize],
}

#[derive(Debug, Clone)]
struct CfLogicalPduMd {
    close_req: u8,
    checksum_type: u8,
    size: u32,
    source_filename: CfLogicalLv,
    dest_filename: CfLogicalLv,
}

#[derive(Debug, Clone)]
struct CfLogicalPduFileDataHeader {
    continuation_state: u8,
    segment_list: CfLogicalSegmentList,
    offset: u32,
    data_len: usize,
    data_ptr: Option<*const u8>,
}

#[derive(Debug, Clone)]
struct CfLogicalPduEof {
    cc: u8,
    crc: u32,
    size: u32,
    tlv_list: CfLogicalTlvList,
}

#[derive(Debug, Clone)]
struct CfLogicalPduFin {
    cc: u8,
    delivery_code: u8,
    file_status: u8,
    tlv_list: CfLogicalTlvList,
}

#[derive(Debug, Clone)]
struct CfLogicalPduAck {
    ack_directive_code: u8,
    ack_subtype_code: u8,
    cc: u8,
    txn_status: u8,
}

#[derive(Debug, Clone)]
struct CfLogicalPduNak {
    scope_start: u32,
    scope_end: u32,
    segment_list: CfLogicalSegmentList,
}

// Codec state structures
#[derive(Debug)]
struct CfCodecState {
    is_valid: bool,
    next_offset: usize,
    max_size: usize,
}

#[derive(Debug)]
struct CfEncoderState {
    codec_state: CfCodecState,
    base: *mut u8,
}

#[derive(Debug)]
struct CfDecoderState {
    codec_state: CfCodecState,
    base: *const u8,
}

// Bitfield operations
fn cf_field_get_val(src: &CfCfdpUint8, shift: u8, mask: u8) -> u8 {
    (src.octets[0] >> shift) & mask
}

fn cf_field_set_val(dest: &mut CfCfdpUint8, shift: u8, mask: u8, val: u8) {
    dest.octets[0] &= !(mask << shift);
    dest.octets[0] |= (val & mask) << shift;
}

fn fgv(src: &CfCfdpUint8, field: &CfCodecBitField) -> u8 {
    cf_field_get_val(src, field.shift as u8, field.mask as u8)
}

fn fsv(dest: &mut CfCfdpUint8, field: &CfCodecBitField, val: u8) {
    cf_field_set_val(dest, field.shift as u8, field.mask as u8, val);
}

// Codec helper functions
fn cf_codec_store_uint8(pdst: &mut CfCfdpUint8, val: u8) {
    pdst.octets[0] = val;
}

fn cf_codec_store_uint16(pdst: &mut CfCfdpUint16, val: u16) {
    pdst.octets[1] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[0] = (val & 0xFF) as u8;
}

fn cf_codec_store_uint32(pdst: &mut CfCfdpUint32, val: u32) {
    pdst.octets[3] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[2] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[1] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[0] = (val & 0xFF) as u8;
}

fn cf_codec_store_uint64(pdst: &mut CfCfdpUint64, val: u64) {
    pdst.octets[7] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[6] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[5] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[4] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[3] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[2] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[1] = (val & 0xFF) as u8;
    let val = val >> 8;
    pdst.octets[0] = (val & 0xFF) as u8;
}

fn cf_codec_load_uint8(pdst: &mut u8, psrc: &CfCfdpUint8) {
    *pdst = psrc.octets[0];
}

fn cf_codec_load_uint16(pdst: &mut u16, psrc: &CfCfdpUint16) {
    let mut val = 0u16;
    val |= psrc.octets[0] as u16;
    val <<= 8;
    val |= psrc.octets[1] as u16;
    *pdst = val;
}

fn cf_codec_load_uint32(pdst: &mut u32, psrc: &CfCfdpUint32) {
    let mut val = 0u32;
    val |= psrc.octets[0] as u32;
    val <<= 8;
    val |= psrc.octets[1] as u32;
    val <<= 8;
    val |= psrc.octets[2] as u32;
    val <<= 8;
    val |= psrc.octets[3] as u32;
    *pdst = val;
}

fn cf_codec_load_uint64(pdst: &mut u64, psrc: &CfCfdpUint64) {
    let mut val = 0u64;
    val |= psrc.octets[0] as u64;
    val <<= 8;
    val |= psrc.octets[1] as u64;
    val <<= 8;
    val |= psrc.octets[2] as u64;
    val <<= 8;
    val |= psrc.octets[3] as u64;
    val <<= 8;
    val |= psrc.octets[4] as u64;
    val <<= 8;
    val |= psrc.octets[5] as u64;
    val <<= 8;
    val |= psrc.octets[6] as u64;
    val <<= 8;
    val |= psrc.octets[7] as u64;
    *pdst = val;
}

// Codec state management
impl CfCodecState {
    fn is_ok(&self) -> bool {
        self.next_offset <= self.max_size
    }

    fn set_done(&mut self) {
        self.next_offset = self.max_size + 1;
    }

    fn get_position(&self) -> usize {
        self.next_offset
    }

    fn get_remain(&self) -> usize {
        if self.next_offset <= self.max_size {
            self.max_size - self.next_offset
        } else {
            0
        }
    }
}

impl CfEncoderState {
    fn is_ok(&self) -> bool {
        self.codec_state.is_ok()
    }

    fn set_done(&mut self) {
        self.codec_state.set_done();
    }

    fn get_position(&self) -> usize {
        self.codec_state.get_position()
    }
}

impl CfDecoderState {
    fn is_ok(&self) -> bool {
        self.codec_state.is_ok()
    }

    fn set_done(&mut self) {
        self.codec_state.set_done();
    }

    fn get_position(&self) -> usize {
        self.codec_state.get_position()
    }

    fn get_remain(&self) -> usize {
        self.codec_state.get_remain()
    }
}

fn cf_cfdp_codec_check_size(state: &mut CfCodecState, chunksize: usize) -> bool {
    let next_offset = state.next_offset + chunksize;
    if next_offset > state.max_size {
        state.set_done();
    } else {
        state.next_offset = next_offset;
    }
    state.is_ok()
}

fn cf_cfdp_do_encode_chunk(state: &mut CfEncoderState, chunksize: usize) -> Option<*mut u8> {
    if cf_cfdp_codec_check_size(&mut state.codec_state, chunksize) {
        // SAFETY: base pointer is valid and position is within bounds
        unsafe { Some(state.base.add(state.codec_state.get_position() - chunksize)) }
    } else {
        None
    }
}

fn cf_cfdp_do_decode_chunk(state: &mut CfDecoderState, chunksize: usize) -> Option<*const u8> {
    if cf_cfdp_codec_check_size(&mut state.codec_state, chunksize) {
        // SAFETY: base pointer is valid and position is within bounds
        unsafe { Some(state.base.add(state.codec_state.get_position() - chunksize)) }
    } else {
        None
    }
}

fn cf_cfdp_get_value_encoded_size(value: u64) -> u8 {
    let mut min_size = 1u8;
    let mut limit = 0x100u64;
    
    while min_size < 8 && value >= limit {
        min_size += 1;
        limit <<= 8;
    }
    
    min_size
}

fn cf_encode_integer_in_size(state: &mut CfEncoderState, value: u64, encode_size: u8) {
    if let Some(dptr) = cf_cfdp_do_encode_chunk(state, encode_size as usize) {
        let mut val = value;
        let mut size = encode_size;
        
        // SAFETY: dptr is valid for encode_size bytes
        unsafe {
            let mut ptr = dptr.add(encode_size as usize);
            while size > 0 {
                size -= 1;
                ptr = ptr.sub(1);
                *ptr = (val & 0xFF) as u8;
                val >>= 8;
            }
        }
    }
}

fn cf_cfdp_encode_header_without_size(state: &mut CfEncoderState, plh: &mut CfLogicalPduHeader) {
    if let Some(peh_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpPduHeader>()) {
        // SAFETY: peh_ptr is valid for CfCfdpPduHeader size
        let peh = unsafe { &mut *(peh_ptr as *mut CfCfdpPduHeader) };
        
        cf_codec_store_uint8(&mut peh.flags, 0);
        fsv(&mut peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_VERSION, plh.version);
        fsv(&mut peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_DIR, plh.direction);
        fsv(&mut peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_TYPE, plh.pdu_type);
        fsv(&mut peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_MODE, plh.txm_mode);

        cf_codec_store_uint8(&mut peh.eid_tsn_lengths, 0);
        fsv(&mut peh.eid_tsn_lengths, &CF_CFDP_PDU_HEADER_SEGMENTATION_CONTROL, plh.segmentation_control);
        fsv(&mut peh.eid_tsn_lengths, &CF_CFDP_PDU_HEADER_LENGTHS_ENTITY, plh.eid_length - 1);
        fsv(&mut peh.eid_tsn_lengths, &CF_CFDP_PDU_HEADER_SEGMENT_METADATA, plh.segment_meta_flag);
        fsv(&mut peh.eid_tsn_lengths, &CF_CFDP_PDU_HEADER_LENGTHS_TRANSACTION_SEQUENCE, plh.txn_seq_length - 1);

        cf_encode_integer_in_size(state, plh.source_eid, plh.eid_length);
        cf_encode_integer_in_size(state, plh.sequence_num, plh.txn_seq_length);
        cf_encode_integer_in_size(state, plh.destination_eid, plh.eid_length);

        plh.header_encoded_length = state.get_position();
    }
}

fn cf_cfdp_encode_header_final_size(state: &mut CfEncoderState, plh: &CfLogicalPduHeader) {
    if state.is_ok() && state.get_position() >= mem::size_of::<CfCfdpPduHeader>() {
        // SAFETY: base pointer is valid and points to a CfCfdpPduHeader
        let peh = unsafe { &mut *(state.base as *mut CfCfdpPduHeader) };
        cf_codec_store_uint16(&mut peh.length, plh.data_encoded_length);
    }
    state.set_done();
}

fn cf_cfdp_encode_file_directive_header(state: &mut CfEncoderState, pfdir: &CfLogicalPduFileDirectiveHeader) {
    if let Some(peh_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpPduFileDirectiveHeader>()) {
        // SAFETY: peh_ptr is valid for CfCfdpPduFileDirectiveHeader size
        let peh = unsafe { &mut *(peh_ptr as *mut CfCfdpPduFileDirectiveHeader) };
        cf_codec_store_uint8(&mut peh.directive_code, pfdir.directive_code);
    }
}

fn cf_cfdp_encode_lv(state: &mut CfEncoderState, pllv: &CfLogicalLv) {
    if let Some(lv_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpLv>()) {
        // SAFETY: lv_ptr is valid for CfCfdpLv size
        let lv = unsafe { &mut *(lv_ptr as *mut CfCfdpLv) };
        cf_codec_store_uint8(&mut lv.length, pllv.length);
        
        if pllv.length > 0 {
            if let Some(data_ptr) = cf_cfdp_do_encode_chunk(state, pllv.length as usize) {
                if let Some(src_ptr) = pllv.data_ptr {
                    // SAFETY: Both pointers are valid for pllv.length bytes
                    unsafe {
                        std::ptr::copy_nonoverlapping(src_ptr, data_ptr, pllv.length as usize);
                    }
                } else {
                    state.set_done();
                }
            }
        }
    }
}

fn cf_cfdp_encode_tlv(state: &mut CfEncoderState, pltlv: &CfLogicalTlv) {
    if let Some(tlv_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpTlv>()) {
        // SAFETY: tlv_ptr is valid for CfCfdpTlv size
        let tlv = unsafe { &mut *(tlv_ptr as *mut CfCfdpTlv) };
        cf_codec_store_uint8(&mut tlv.type_field, pltlv.type_field);
        cf_codec_store_uint8(&mut tlv.length, pltlv.length);

        if pltlv.type_field == CF_CFDP_TLV_TYPE_ENTITY_ID {
            // SAFETY: accessing eid field of union
            let eid = unsafe { pltlv.data.eid };
            cf_encode_integer_in_size(state, eid, pltlv.length);
        } else if pltlv.length > 0 {
            if let Some(data_ptr) = cf_cfdp_do_encode_chunk(state, pltlv.length as usize) {
                // SAFETY: accessing data_ptr field of union
                let src_ptr = unsafe { pltlv.data.data_ptr };
                if !src_ptr.is_null() {
                    // SAFETY: Both pointers are valid for pltlv.length bytes
                    unsafe {
                        std::ptr::copy_nonoverlapping(src_ptr, data_ptr, pltlv.length as usize);
                    }
                } else {
                    state.set_done();
                }
            }
        }
    }
}

fn cf_cfdp_encode_segment_request(state: &mut CfEncoderState, plseg: &CfLogicalSegmentRequest) {
    if let Some(sr_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpSegmentRequest>()) {
        // SAFETY: sr_ptr is valid for CfCfdpSegmentRequest size
        let sr = unsafe { &mut *(sr_ptr as *mut CfCfdpSegmentRequest) };
        cf_codec_store_uint32(&mut sr.offset_start, plseg.offset_start);
        cf_codec_store_uint32(&mut sr.offset_end, plseg.offset_end);
    }
}

fn cf_cfdp_encode_all_tlv(state: &mut CfEncoderState, pltlv: &CfLogicalTlvList) {
    for i in 0..pltlv.num_tlv {
        if !state.is_ok() {
            break;
        }
        cf_cfdp_encode_tlv(state, &pltlv.tlv[i as usize]);
    }
}

fn cf_cfdp_encode_all_segments(state: &mut CfEncoderState, plseg: &CfLogicalSegmentList) {
    for i in 0..plseg.num_segments {
        if !state.is_ok() {
            break;
        }
        cf_cfdp_encode_segment_request(state, &plseg.segments[i as usize]);
    }
}

fn cf_cfdp_encode_md(state: &mut CfEncoderState, plmd: &CfLogicalPduMd) {
    if let Some(md_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpPduMd>()) {
        // SAFETY: md_ptr is valid for CfCfdpPduMd size
        let md = unsafe { &mut *(md_ptr as *mut CfCfdpPduMd) };
        cf_codec_store_uint8(&mut md.segmentation_control, 0);
        fsv(&mut md.segmentation_control, &CF_CFDP_PDU_MD_CLOSURE_REQUESTED, plmd.close_req);
        fsv(&mut md.segmentation_control, &CF_CFDP_PDU_MD_CHECKSUM_TYPE, plmd.checksum_type);
        cf_codec_store_uint32(&mut md.size, plmd.size);

        cf_cfdp_encode_lv(state, &plmd.source_filename);
        cf_cfdp_encode_lv(state, &plmd.dest_filename);
    }
}

fn cf_cfdp_encode_file_data_header(state: &mut CfEncoderState, with_meta: bool, plfd: &CfLogicalPduFileDataHeader) {
    let optional_fields = if with_meta {
        cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpUint8>())
    } else {
        None
    };

    if let Some(opt_ptr) = optional_fields {
        // SAFETY: opt_ptr is valid for CfCfdpUint8 size
        let optional_fields = unsafe { &mut *(opt_ptr as *mut CfCfdpUint8) };
        cf_codec_store_uint8(optional_fields, 0);
        fsv(optional_fields, &CF_CFDP_PDU_FILE_DATA_RECORD_CONTINUATION_STATE, plfd.continuation_state);
        fsv(optional_fields, &CF_CFDP_PDU_FILE_DATA_SEGMENT_METADATA_LENGTH, plfd.segment_list.num_segments);

        cf_cfdp_encode_all_segments(state, &plfd.segment_list);
    }

    if let Some(fd_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpPduFileDataHeader>()) {
        // SAFETY: fd_ptr is valid for CfCfdpPduFileDataHeader size
        let fd = unsafe { &mut *(fd_ptr as *mut CfCfdpPduFileDataHeader) };
        cf_codec_store_uint32(&mut fd.offset, plfd.offset);
    }
}

fn cf_cfdp_encode_eof(state: &mut CfEncoderState, pleof: &CfLogicalPduEof) {
    if let Some(eof_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpPduEof>()) {
        // SAFETY: eof_ptr is valid for CfCfdpPduEof size
        let eof = unsafe { &mut *(eof_ptr as *mut CfCfdpPduEof) };
        cf_codec_store_uint8(&mut eof.cc, 0);
        fsv(&mut eof.cc, &CF_CFDP_PDU_EOF_FLAGS_CC, pleof.cc);
        cf_codec_store_uint32(&mut eof.crc, pleof.crc);
        cf_codec_store_uint32(&mut eof.size, pleof.size);

        cf_cfdp_encode_all_tlv(state, &pleof.tlv_list);
    }
}

fn cf_cfdp_encode_fin(state: &mut CfEncoderState, plfin: &CfLogicalPduFin) {
    if let Some(fin_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpPduFin>()) {
        // SAFETY: fin_ptr is valid for CfCfdpPduFin size
        let fin = unsafe { &mut *(fin_ptr as *mut CfCfdpPduFin) };
        cf_codec_store_uint8(&mut fin.flags, 0);
        fsv(&mut fin.flags, &CF_CFDP_PDU_FIN_FLAGS_CC, plfin.cc);
        fsv(&mut fin.flags, &CF_CFDP_PDU_FIN_FLAGS_DELIVERY_CODE, plfin.delivery_code);
        fsv(&mut fin.flags, &CF_CFDP_PDU_FIN_FLAGS_FILE_STATUS, plfin.file_status);

        cf_cfdp_encode_all_tlv(state, &plfin.tlv_list);
    }
}

fn cf_cfdp_encode_ack(state: &mut CfEncoderState, plack: &CfLogicalPduAck) {
    if let Some(ack_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpPduAck>()) {
        // SAFETY: ack_ptr is valid for CfCfdpPduAck size
        let ack = unsafe { &mut *(ack_ptr as *mut CfCfdpPduAck) };
        cf_codec_store_uint8(&mut ack.directive_and_subtype_code, 0);
        fsv(&mut ack.directive_and_subtype_code, &CF_CFDP_PDU_ACK_DIR_CODE, plack.ack_directive_code);
        fsv(&mut ack.directive_and_subtype_code, &CF_CFDP_PDU_ACK_DIR_SUBTYPE_CODE, plack.ack_subtype_code);

        cf_codec_store_uint8(&mut ack.cc_and_transaction_status, 0);
        fsv(&mut ack.cc_and_transaction_status, &CF_CFDP_PDU_ACK_CC, plack.cc);
        fsv(&mut ack.cc_and_transaction_status, &CF_CFDP_PDU_ACK_TRANSACTION_STATUS, plack.txn_status);
    }
}

fn cf_cfdp_encode_nak(state: &mut CfEncoderState, plnak: &CfLogicalPduNak) {
    if let Some(nak_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpPduNak>()) {
        // SAFETY: nak_ptr is valid for CfCfdpPduNak size
        let nak = unsafe { &mut *(nak_ptr as *mut CfCfdpPduNak) };
        cf_codec_store_uint32(&mut nak.scope_start, plnak.scope_start);
        cf_codec_store_uint32(&mut nak.scope_end, plnak.scope_end);

        cf_cfdp_encode_all_segments(state, &plnak.segment_list);
    }
}

fn cf_cfdp_encode_crc(state: &mut CfEncoderState, plcrc: &u32) {
    if let Some(pecrc_ptr) = cf_cfdp_do_encode_chunk(state, mem::size_of::<CfCfdpUint32>()) {
        // SAFETY: pecrc_ptr is valid for CfCfdpUint32 size
        let pecrc = unsafe { &mut *(pecrc_ptr as *mut CfCfdpUint32) };
        cf_codec_store_uint32(pecrc, *plcrc);
    }
}

fn cf_decode_integer_in_size(state: &mut CfDecoderState, decode_size: u8) -> u64 {
    let mut temp_val = 0u64;
    if let Some(sptr) = cf_cfdp_do_decode_chunk(state, decode_size as usize) {
        let mut size = decode_size;
        // SAFETY: sptr is valid for decode_size bytes
        unsafe {
            let mut ptr = sptr;
            while size > 0 {
                temp_val <<= 8;
                temp_val |= (*ptr & 0xFF) as u64;
                ptr = ptr.add(1);
                size -= 1;
            }
        }
    }
    temp_val
}

fn cf_cfdp_decode_header(state: &mut CfDecoderState, plh: &mut CfLogicalPduHeader) -> CFE_Status_t {
    if let Some(peh_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpPduHeader>()) {
        // SAFETY: peh_ptr is valid for CfCfdpPduHeader size
        let peh = unsafe { &*(peh_ptr as *const CfCfdpPduHeader) };
        
        plh.version = fgv(&peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_VERSION);
        plh.direction = fgv(&peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_DIR);
        plh.pdu_type = fgv(&peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_TYPE);
        plh.txm_mode = fgv(&peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_MODE);
        plh.crc_flag = fgv(&peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_CRC);
        plh.large_flag = fgv(&peh.flags, &CF_CFDP_PDU_HEADER_FLAGS_LARGEFILE);

        plh.segmentation_control = fgv(&peh.eid_tsn_lengths, &CF_CFDP_PDU_HEADER_SEGMENTATION_CONTROL);
        plh.eid_length = fgv(&peh.eid_tsn_lengths, &CF_CFDP_PDU_HEADER_LENGTHS_ENTITY) + 1;
        plh.segment_meta_flag = fgv(&peh.eid_tsn_lengths, &CF_CFDP_PDU_HEADER_SEGMENT_METADATA);
        plh.txn_seq_length = fgv(&peh.eid_tsn_lengths, &CF_CFDP_PDU_HEADER_LENGTHS_TRANSACTION_SEQUENCE) + 1;

        cf_codec_load_uint16(&mut plh.data_encoded_length, &peh.length);
        
        // C original: sizeof(plh->source_eid) and sizeof(plh->sequence_num) which are u32
        if (plh.eid_length as usize > mem::size_of::<CfEntityId>()) || (plh.txn_seq_length as usize > mem::size_of::<CfTransactionSeq>()) {
            CF_ERROR
        } else {
            plh.source_eid = cf_decode_integer_in_size(state, plh.eid_length);
            plh.sequence_num = cf_decode_integer_in_size(state, plh.txn_seq_length);
            plh.destination_eid = cf_decode_integer_in_size(state, plh.eid_length);
            plh.header_encoded_length = state.get_position();
            CFE_SUCCESS
        }
    } else {
        // C original returns CFE_SUCCESS (initial value of ret) when decode chunk returns NULL
        // The codec state will be marked invalid, which callers check via CF_CODEC_IS_OK
        CFE_SUCCESS
    }
}

fn cf_cfdp_decode_file_directive_header(state: &mut CfDecoderState, pfdir: &mut CfLogicalPduFileDirectiveHeader) {
    if let Some(peh_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpPduFileDirectiveHeader>()) {
        // SAFETY: peh_ptr is valid for CfCfdpPduFileDirectiveHeader size
        let peh = unsafe { &*(peh_ptr as *const CfCfdpPduFileDirectiveHeader) };
        cf_codec_load_uint8(&mut pfdir.directive_code, &peh.directive_code);
    }
}

fn cf_cfdp_decode_lv(state: &mut CfDecoderState, pllv: &mut CfLogicalLv) {
    if let Some(lv_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpLv>()) {
        // SAFETY: lv_ptr is valid for CfCfdpLv size
        let lv = unsafe { &*(lv_ptr as *const CfCfdpLv) };
        cf_codec_load_uint8(&mut pllv.length, &lv.length);
        pllv.data_ptr = cf_cfdp_do_decode_chunk(state, pllv.length as usize);
    }
}

fn cf_cfdp_decode_tlv(state: &mut CfDecoderState, pltlv: &mut CfLogicalTlv) {
    if let Some(tlv_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpTlv>()) {
        // SAFETY: tlv_ptr is valid for CfCfdpTlv size
        let tlv = unsafe { &*(tlv_ptr as *const CfCfdpTlv) };
        cf_codec_load_uint8(&mut pltlv.type_field, &tlv.type_field);
        cf_codec_load_uint8(&mut pltlv.length, &tlv.length);

        if pltlv.type_field == CF_CFDP_TLV_TYPE_ENTITY_ID {
            // SAFETY: setting eid field of union
            pltlv.data.eid = cf_decode_integer_in_size(state, pltlv.length);
        } else {
            // SAFETY: setting data_ptr field of union
            pltlv.data.data_ptr = cf_cfdp_do_decode_chunk(state, pltlv.length as usize).unwrap_or(std::ptr::null());
        }
    }
}

fn cf_cfdp_decode_segment_request(state: &mut CfDecoderState, plseg: &mut CfLogicalSegmentRequest) {
    if let Some(sr_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpSegmentRequest>()) {
        // SAFETY: sr_ptr is valid for CfCfdpSegmentRequest size
        let sr = unsafe { &*(sr_ptr as *const CfCfdpSegmentRequest) };
        cf_codec_load_uint32(&mut plseg.offset_start, &sr.offset_start);
        cf_codec_load_uint32(&mut plseg.offset_end, &sr.offset_end);
    }
}

fn cf_cfdp_decode_md(state: &mut CfDecoderState, plmd: &mut CfLogicalPduMd) {
    if let Some(md_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpPduMd>()) {
        // SAFETY: md_ptr is valid for CfCfdpPduMd size
        let md = unsafe { &*(md_ptr as *const CfCfdpPduMd) };
        plmd.close_req = fgv(&md.segmentation_control, &CF_CFDP_PDU_MD_CLOSURE_REQUESTED);
        plmd.checksum_type = fgv(&md.segmentation_control, &CF_CFDP_PDU_MD_CHECKSUM_TYPE);
        cf_codec_load_uint32(&mut plmd.size, &md.size);

        cf_cfdp_decode_lv(state, &mut plmd.source_filename);
        cf_cfdp_decode_lv(state, &mut plmd.dest_filename);
    }
}

fn cf_cfdp_decode_file_data_header(state: &mut CfDecoderState, with_meta: bool, plfd: &mut CfLogicalPduFileDataHeader) {
    plfd.continuation_state = 0;
    plfd.segment_list.num_segments = 0;

    if with_meta {
        if let Some(opt_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpUint8>()) {
            // SAFETY: opt_ptr is valid for CfCfdpUint8 size
            let optional_fields = unsafe { &*(opt_ptr as *const CfCfdpUint8) };
            plfd.continuation_state = fgv(optional_fields, &CF_CFDP_PDU_FILE_DATA_RECORD_CONTINUATION_STATE);
            let mut field_count = fgv(optional_fields, &CF_CFDP_PDU_FILE_DATA_SEGMENT_METADATA_LENGTH);
            
            if field_count > CF_PDU_MAX_SEGMENTS {
                state.set_done();
                field_count = 0;
            }

            while field_count > 0 && state.is_ok() {
                field_count -= 1;
                cf_cfdp_decode_segment_request(state, &mut plfd.segment_list.segments[plfd.segment_list.num_segments as usize]);
                if state.is_ok() {
                    plfd.segment_list.num_segments += 1;
                }
            }
        }
    }

    if let Some(fd_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpPduFileDataHeader>()) {
        // SAFETY: fd_ptr is valid for CfCfdpPduFileDataHeader size
        let fd = unsafe { &*(fd_ptr as *const CfCfdpPduFileDataHeader) };
        cf_codec_load_uint32(&mut plfd.offset, &fd.offset);
        plfd.data_len = state.get_remain();
        plfd.data_ptr = cf_cfdp_do_decode_chunk(state, plfd.data_len);
    }
}

fn cf_cfdp_decode_crc(state: &mut CfDecoderState, plcrc: &mut u32) {
    if let Some(pecrc_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpUint32>()) {
        // SAFETY: pecrc_ptr is valid for CfCfdpUint32 size
        let pecrc = unsafe { &*(pecrc_ptr as *const CfCfdpUint32) };
        cf_codec_load_uint32(plcrc, pecrc);
    }
}

fn cf_cfdp_decode_eof(state: &mut CfDecoderState, pleof: &mut CfLogicalPduEof) {
    if let Some(eof_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpPduEof>()) {
        // SAFETY: eof_ptr is valid for CfCfdpPduEof size
        let eof = unsafe { &*(eof_ptr as *const CfCfdpPduEof) };
        pleof.cc = fgv(&eof.cc, &CF_CFDP_PDU_EOF_FLAGS_CC);
        cf_codec_load_uint32(&mut pleof.crc, &eof.crc);
        cf_codec_load_uint32(&mut pleof.size, &eof.size);

        cf_cfdp_decode_all_tlv(state, &mut pleof.tlv_list, CF_PDU_MAX_TLV);
    }
}

fn cf_cfdp_decode_fin(state: &mut CfDecoderState, plfin: &mut CfLogicalPduFin) {
    if let Some(fin_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpPduFin>()) {
        // SAFETY: fin_ptr is valid for CfCfdpPduFin size
        let fin = unsafe { &*(fin_ptr as *const CfCfdpPduFin) };
        plfin.cc = fgv(&fin.flags, &CF_CFDP_PDU_FIN_FLAGS_CC);
        plfin.delivery_code = fgv(&fin.flags, &CF_CFDP_PDU_FIN_FLAGS_DELIVERY_CODE);
        plfin.file_status = fgv(&fin.flags, &CF_CFDP_PDU_FIN_FLAGS_FILE_STATUS);

        cf_cfdp_decode_all_tlv(state, &mut plfin.tlv_list, CF_PDU_MAX_TLV);
    }
}

fn cf_cfdp_decode_ack(state: &mut CfDecoderState, plack: &mut CfLogicalPduAck) {
    if let Some(ack_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpPduAck>()) {
        // SAFETY: ack_ptr is valid for CfCfdpPduAck size
        let ack = unsafe { &*(ack_ptr as *const CfCfdpPduAck) };
        plack.ack_directive_code = fgv(&ack.directive_and_subtype_code, &CF_CFDP_PDU_ACK_DIR_CODE);
        plack.ack_subtype_code = fgv(&ack.directive_and_subtype_code, &CF_CFDP_PDU_ACK_DIR_SUBTYPE_CODE);
        plack.cc = fgv(&ack.cc_and_transaction_status, &CF_CFDP_PDU_ACK_CC);
        plack.txn_status = fgv(&ack.cc_and_transaction_status, &CF_CFDP_PDU_ACK_TRANSACTION_STATUS);
    }
}

fn cf_cfdp_decode_nak(state: &mut CfDecoderState, plnak: &mut CfLogicalPduNak) {
    if let Some(nak_ptr) = cf_cfdp_do_decode_chunk(state, mem::size_of::<CfCfdpPduNak>()) {
        // SAFETY: nak_ptr is valid for CfCfdpPduNak size
        let nak = unsafe { &*(nak_ptr as *const CfCfdpPduNak) };
        cf_codec_load_uint32(&mut plnak.scope_start, &nak.scope_start);
        cf_codec_load_uint32(&mut plnak.scope_end, &nak.scope_end);

        cf_cfdp_decode_all_segments(state, &mut plnak.segment_list, CF_PDU_MAX_SEGMENTS);
    }
}

fn cf_cfdp_decode_all_tlv(state: &mut CfDecoderState, pltlv: &mut CfLogicalTlvList, limit: u8) {
    pltlv.num_tlv = 0;
    let mut remaining_limit = limit;

    while remaining_limit > 0 && state.get_remain() != 0 {
        remaining_limit -= 1;

        if pltlv.num_tlv >= CF_PDU_MAX_TLV {
            state.set_done();
        } else {
            cf_cfdp_decode_tlv(state, &mut pltlv.tlv[pltlv.num_tlv as usize]);
        }

        if !state.is_ok() {
            break;
        }

        pltlv.num_tlv += 1;
    }
}

fn cf_cfdp_decode_all_segments(state: &mut CfDecoderState, plseg: &mut CfLogicalSegmentList, limit: u8) {
    plseg.num_segments = 0;
    let mut remaining_limit = limit;

    while remaining_limit > 0 && state.get_remain() != 0 {
        remaining_limit -= 1;

        if plseg.num_segments >= CF_PDU_MAX_SEGMENTS {
            state.set_done();
        } else {
            cf_cfdp_decode_segment_request(state, &mut plseg.segments[plseg.num_segments as usize]);
        }

        if !state.is_ok() {
            break;
        }

        plseg.num_segments += 1;
    }
}