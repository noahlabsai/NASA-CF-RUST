//! CF CFDP Receive-File (R) transaction handlers.
//!
//! Translated from: cf_cfdp_r.c / cf_cfdp_r.h

use std::os::raw::{c_char, c_void};
use std::ptr;

use crate::common_types::*;
use crate::cf_platform_cfg::*;
use crate::cf_extern_typedefs::*;
use crate::cf_cfdp_pdu::*;
use crate::cf_logical_pdu::*;
use crate::cf_cfdp_types::*;
use crate::cf_clist_types::*;
use crate::cf_chunk_types::*;
use crate::cf_timer_types::*;
use crate::cf_crc_types::*;
use crate::cf_codec_types::*;
use crate::cf_msg::*;
use crate::cf_eventids::*;
use crate::cf_app::CF_AppData;
use crate::cf_app_types::*;
use crate::cf_cfdp::*;
use crate::cf_utils::*;
use crate::cf_clist::*;
use crate::cf_chunk::*;
use crate::cf_timer::*;
use crate::cf_crc::*;
use crate::cf_cfdp_dispatch::*;
use crate::cf_cfdp_r_types::*;
use crate::cf_cfdp_dispatch_types::*;

/// C: `CFE_Status_t CF_CFDP_R_CheckCrc(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CheckCrc(txn: *mut CF_Transaction_t) -> CFE_Status_t {
    if (*txn).crc.result != (*txn).state_data.eof_crc {
        CFE_EVS_SendEvent!(
            CF_CFDP_R_CRC_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF R%d(%lu:%lu): CRC mismatch for R trans. got 0x%08lx expected 0x%08lx\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
            (*txn).crc.result as u64,
            (*txn).state_data.eof_crc as u64,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.crc_mismatch += 1;
        CF_ERROR
    } else {
        CFE_SUCCESS
    }
}

/// C: `bool CF_CFDP_R_CheckComplete(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CheckComplete(txn: *mut CF_Transaction_t) -> bool {
    /* Active transaction with no errors: The MD and EOF flags must be set with no pending NAKs */
    if (*txn).flags.rx.send_nak || (*txn).flags.rx.eof_count == 0 || !(*txn).flags.rx.md_recv {
        return false; /* not complete yet due to missing PDU of some type */
    }

    /* Finally if all other state seems OK check for gaps in the file data */
    let ret = CF_ChunkList_ComputeGaps(
        &(*(*txn).chunks).chunks,
        1,
        (*txn).fsize,
        0,
        None,
        ptr::null_mut(),
    );

    /* The file is complete if there are no gaps */
    ret == 0
}

/// C: `CFE_Status_t CF_CFDP_R_ProcessFd(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_R_ProcessFd(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) -> CFE_Status_t {
    let fd = &(*ph).int_header.fd;
    let mut ret: CFE_Status_t = CFE_SUCCESS;

    if (*txn).state_data.cached_pos != fd.offset {
        let fret = CF_WrappedLseek((*txn).fd, fd.offset as i32, OS_SEEK_SET as i32);
        if fret != fd.offset as i32 {
            CFE_EVS_SendEvent!(
                CF_CFDP_R_SEEK_FD_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF R%d(%lu:%lu): failed to seek offset %ld, got %ld\0".as_ptr() as *const c_char,
                CF_CFDP_GetPrintClass(txn),
                (*(*txn).history).src_eid as u64,
                (*(*txn).history).seq_num as u64,
                fd.offset as i64,
                fret as i64,
            );
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILE_SIZE_ERROR);
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_seek += 1;
            ret = CF_ERROR;
        } else {
            (*txn).state_data.cached_pos = fd.offset;
        }
    }

    if ret != CF_ERROR {
        let fret = CF_WrappedWrite((*txn).fd, fd.data_ptr, fd.data_len as usize);
        if fret != fd.data_len as i32 {
            CFE_EVS_SendEvent!(
                CF_CFDP_R_WRITE_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF R%d(%lu:%lu): OS_write expected %ld, got %ld\0".as_ptr() as *const c_char,
                CF_CFDP_GetPrintClass(txn),
                (*(*txn).history).src_eid as u64,
                (*(*txn).history).seq_num as u64,
                fd.data_len as i64,
                fret as i64,
            );
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILESTORE_REJECTION);
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_write += 1;
            ret = CF_ERROR;
        } else {
            (*txn).state_data.cached_pos = (fd.data_len as u32) + fd.offset;
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.file_data_bytes += fd.data_len as u64;
            CF_ChunkListAdd(&mut (*(*txn).chunks).chunks, fd.offset, fd.data_len as u32);
        }
    }

    ret
}

/// C: `void CF_CFDP_R_SubstateRecvEof(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_R_SubstateRecvEof(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    let ret = CF_CFDP_RecvEof(txn, ph);
    if ret == CFE_SUCCESS {
        let eof = &(*ph).int_header.eof;

        /* only accept the first EOF, ignore dupes */
        if (*txn).flags.rx.eof_count == 0 {
            (*txn).state_data.eof_crc = eof.crc;
            (*txn).state_data.eof_size = eof.size;
            (*txn).state_data.peer_cc = eof.cc as u8;
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_From_ConditionCode(eof.cc));
        }

        if !CF_CFDP_CheckAckNakCount(txn, &mut (*txn).flags.rx.eof_count) {
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_POS_ACK_LIMIT_REACHED);
        }
    } else {
        CFE_EVS_SendEvent!(
            CF_CFDP_R_PDU_EOF_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF R%d(%lu:%lu): invalid EOF packet\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error += 1;
    }
}

/// C: `void CF_CFDP_R_SubstateRecvFileData(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_R_SubstateRecvFileData(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    let mut ret = CF_CFDP_RecvFd(txn, ph);
    if ret == CFE_SUCCESS {
        ret = CF_CFDP_R_ProcessFd(txn, ph);
    }

    if ret == CFE_SUCCESS {
        /* All new file data will reset the NAK counter */
        (*txn).state_data.acknak_count = 0;
    }
}

/// C: `void CF_CFDP_R2_GapCompute(const CF_ChunkList_t *chunks, const CF_Chunk_t *chunk, void *opaque)`
pub unsafe fn CF_CFDP_R2_GapCompute(
    _chunks: &CF_ChunkList_t,
    chunk: &CF_Chunk_t,
    opaque: *mut u8,
) {
    let args = opaque as *mut CF_GapComputeArgs_t;
    let nak: *mut CF_Logical_PduNak_t = (*args).nak;
    let pseglist: *mut CF_Logical_SegmentList_t = &mut (*nak).segment_list;

    CF_Assert!((*chunk).size > 0);

    if ((*pseglist).num_segments as u32) < CF_PDU_MAX_SEGMENTS as u32 {
        let pseg = &mut (*pseglist).segments[(*pseglist).num_segments as usize];
        pseg.offset_start = (*chunk).offset - (*nak).scope_start;
        pseg.offset_end = pseg.offset_start + (*chunk).size;
        (*pseglist).num_segments += 1;
    }
}

/// C: `CFE_Status_t CF_CFDP_R_SendNak(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_SendNak(txn: *mut CF_Transaction_t) -> CFE_Status_t {
    let ph = CF_CFDP_ConstructPduHeader(
        txn,
        CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_NAK,
        (*(*txn).history).peer_eid,
        (*CF_AppData.config_table).local_eid,
        true,
        (*(*txn).history).seq_num,
        true,
    );

    if ph.is_null() {
        return CF_SEND_PDU_NO_BUF_AVAIL_ERROR;
    }

    let ret: CFE_Status_t = CFE_SUCCESS;
    let nak: *mut CF_Logical_PduNak_t = &mut (*ph).int_header.nak;

    (*nak).scope_start = 0;
    (*nak).scope_end = 0;

    if (*txn).flags.rx.md_recv {
        let mut args = CF_GapComputeArgs_t { txn, nak };
        let max_c = (*(*txn).chunks).chunks.max_chunks;
        let limit = if (*(*txn).chunks).chunks.count < max_c {
            max_c
        } else {
            max_c - 1
        };

        let cret = CF_ChunkList_ComputeGaps(
            &(*(*txn).chunks).chunks,
            limit,
            (*txn).fsize,
            0,
            Some(CF_CFDP_R2_GapCompute),
            &mut args as *mut _ as *mut u8,
        );

        if cret != 0 { 
            CF_CFDP_SendNak(txn, ph);
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.sent.nak_segment_requests += cret as u32;
        }
    } else {
        CFE_EVS_SendEvent!(
            CF_CFDP_R_REQUEST_MD_INF_EID,
            CFE_EVS_EventType_INFORMATION,
            b"CF R%d(%lu:%lu): requesting MD\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
        );

        (*nak).scope_start = 0;
        (*nak).scope_end = 0;
        (*nak).segment_list.segments[0].offset_start = 0;
        (*nak).segment_list.segments[0].offset_end = 0;
        (*nak).segment_list.num_segments = 1;

        CF_CFDP_SendNak(txn, ph);
    }

    ret
}

/// C: `void CF_CFDP_R_Init(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_Init(txn: *mut CF_Transaction_t) {
    let mut temp_name = [0u8; CF_FILENAME_MAX_LEN];

    /* set default FIN status */
    (*txn).state_data.fin_dc = CF_CFDP_FinDeliveryCode_t::CF_CFDP_FinDeliveryCode_INVALID as u8;
    (*txn).state_data.fin_fs = CF_CFDP_FinFileStatus_t::CF_CFDP_FinFileStatus_INVALID as u8;

    /* make a temp file to hold the data */
    CF_CFDP_GetTempName((*txn).history, temp_name.as_mut_ptr() as *mut c_char, temp_name.len());

    let ret = CF_WrappedOpenCreate(
        &mut (*txn).fd,
        temp_name.as_ptr(),
        OS_FILE_FLAG_CREATE | OS_FILE_FLAG_TRUNCATE,
        OS_READ_WRITE,
    );
    if ret < 0 {
        CFE_EVS_SendEvent!(
            CF_CFDP_R_CREAT_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF R%d(%lu:%lu): failed to create file %s for writing, error=%ld\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
            temp_name.as_ptr(),
            ret as i64,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_open += 1;
        (*txn).fd = OS_OBJECT_ID_UNDEFINED;
        CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILESTORE_REJECTION);
    } else {
        CFE_EVS_SendEvent!(
            CF_CFDP_R_TEMP_FILE_INF_EID,
            CFE_EVS_EventType_INFORMATION,
            b"CF R%d(%lu:%lu): starting transaction using temp file '%s'\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
            temp_name.as_ptr(),
        );
        (*txn).flags.rx.tempfile_created = true;
    }
}

/// C: `void CF_CFDP_R_CalcCrcStart(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CalcCrcStart(txn: *mut CF_Transaction_t) {
    if (*txn).fsize != (*txn).state_data.eof_size {
        CFE_EVS_SendEvent!(
            CF_CFDP_R_SIZE_MISMATCH_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF R%d(%lu:%lu): file size mismatch, md=%lu eof=%lu\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
            (*txn).fsize as u64,
            (*txn).state_data.eof_size as u64,
        );
        CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILE_SIZE_ERROR);
        (*txn).flags.com.crc_complete = true;
    } else {
        let os_status = CF_WrappedLseek((*txn).fd, 0, OS_SEEK_SET as i32);
        if os_status != 0 { 
            CFE_EVS_SendEvent!(
                CF_CFDP_R_SEEK_CRC_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF R%d(%lu:%lu): failed to seek offset 0, got %ld\0".as_ptr() as *const c_char,
                CF_CFDP_GetPrintClass(txn),
                (*(*txn).history).src_eid as u64,
                (*(*txn).history).seq_num as u64,
                os_status as i64,
            );
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_seek += 1;
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILE_SIZE_ERROR);
            (*txn).flags.com.crc_complete = true;
        } else {
            CF_CrcStart(&mut (*txn).crc);
            (*txn).state_data.cached_pos = 0;
        }
    }
}

/// C: `void CF_CFDP_R_CalcCrcChunk(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CalcCrcChunk(txn: *mut CF_Transaction_t) {
    let mut buf = [0u8; CF_R2_CRC_CHUNK_SIZE];
    let mut count_bytes: usize = 0;
    let mut success = true;

    while count_bytes < (*CF_AppData.config_table).rx_crc_calc_bytes_per_wakeup as usize
        && (*txn).state_data.cached_pos < (*txn).fsize
    {
        let want_offs_size = (*txn).state_data.cached_pos as usize + buf.len();
        let read_size = if want_offs_size > (*txn).fsize as usize {
            (*txn).fsize as usize - (*txn).state_data.cached_pos as usize
        } else {
            buf.len()
        };

        let fret = CF_WrappedRead((*txn).fd, buf.as_mut_ptr(), read_size);
        if fret != read_size as i32 {
            CFE_EVS_SendEvent!(
                CF_CFDP_R_READ_ERR_EID,
                CFE_EVS_EventType_ERROR,
                b"CF R%d(%lu:%lu): failed to read file expected %lu, got %ld\0".as_ptr() as *const c_char,
                CF_CFDP_GetPrintClass(txn),
                (*(*txn).history).src_eid as u64,
                (*(*txn).history).seq_num as u64,
                read_size as u64,
                fret as i64,
            );
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILE_SIZE_ERROR);
            CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.fault.file_read += 1;
            success = false;
            break;
        }

        CF_CrcDigest(&mut (*txn).crc, buf.as_ptr(), read_size as u32);
        (*txn).state_data.cached_pos += read_size as u32;
        count_bytes += read_size;
    }

    if !success {
        (*txn).flags.com.crc_complete = true;
    } else if (*txn).state_data.cached_pos == (*txn).fsize {
        CF_CrcFinalize(&mut (*txn).crc);
        (*txn).flags.com.crc_complete = true;
    }
}

/// C: `void CF_CFDP_R2_SubstateRecvFinAck(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_R2_SubstateRecvFinAck(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    let status = CF_CFDP_RecvAck(txn, ph);
    if status == CFE_SUCCESS {
        let ack = &(*ph).int_header.ack;
        if ack.ack_directive_code == CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_FIN as u8 {
            (*txn).flags.rx.finack_recv = true;
        }
    }

    if status != CFE_SUCCESS || !(*txn).flags.rx.finack_recv {
        CFE_EVS_SendEvent!(
            CF_CFDP_R_PDU_FINACK_ERR_EID,
            CFE_EVS_EventType_ERROR,
            b"CF R%d(%lu:%lu): received invalid FIN-ACK PDU\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
        );
        CF_AppData.hk.Payload.channel_hk[(*txn).chan_num as usize].counters.recv.error += 1;
    }
}

/// C: `void CF_CFDP_R_SubstateRecvMd(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_R_SubstateRecvMd(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    if !(*txn).flags.rx.md_recv {
        let status = CF_CFDP_RecvMd(txn, ph);
        if status == CFE_SUCCESS {
            (*txn).flags.rx.md_recv = true;
        }
    }
}

/// C: `void CF_CFDP_R1_Recv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_R1_Recv(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    use crate::cf_cfdp_dispatch::CF_CFDP_RxStateDispatch;

    static R1_FDIR: CF_CFDP_FileDirectiveDispatchTable_t = CF_CFDP_FileDirectiveDispatchTable_t {
        fdirective: {
            let mut t: [CF_CFDP_RxFileDirectiveHandler_t; CF_CFDP_FileDirective_INVALID_MAX as usize] =
                [None; CF_CFDP_FileDirective_INVALID_MAX as usize];
            t[CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_EOF as usize] = Some(CF_CFDP_R_SubstateRecvEof);
            t[CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_METADATA as usize] = Some(CF_CFDP_R_SubstateRecvMd);
            t
        },
    };

    /* R1 uses same table for both DATA_NORMAL and DATA_EOF substates */
    let substate = (*txn).state_data.sub_state as usize;
    if (*ph).fdirective.directive_code != CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_INVALID_MIN {
        /* file directive */
        let dc = (*ph).fdirective.directive_code as usize;
        if dc < CF_CFDP_FileDirective_INVALID_MAX as usize {
            if let Some(handler) = R1_FDIR.fdirective[dc] {
                handler(txn, ph);
            } else {
                CF_CFDP_RecvDrop(txn, ph);
            }
        } else {
            CF_CFDP_RecvDrop(txn, ph);
        }
    } else {
        /* file data */
        CF_CFDP_R_SubstateRecvFileData(txn, ph);
    }
}

/// C: `void CF_CFDP_R2_Recv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph)`
pub unsafe fn CF_CFDP_R2_Recv(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    let substate = (*txn).state_data.sub_state;

    if (*ph).pdu_header.pdu_type != 0 { 
        /* file data PDU */
        CF_CFDP_R_SubstateRecvFileData(txn, ph);
    } else {
        /* file directive PDU */
        let dc = (*ph).fdirective.directive_code;
        match dc {
            CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_EOF => {
                CF_CFDP_R_SubstateRecvEof(txn, ph);
            }
            CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_METADATA => {
                CF_CFDP_R_SubstateRecvMd(txn, ph);
            }
            CF_CFDP_FileDirective_t::CF_CFDP_FileDirective_ACK => {
                /* ACK (FIN-ACK) only valid in FINACK substate */
                CF_CFDP_R2_SubstateRecvFinAck(txn, ph);
            }
            _ => {
                CF_CFDP_RecvDrop(txn, ph);
            }
        }
    }
}

/// C: `void CF_CFDP_R_HandleFileRetention(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_HandleFileRetention(txn: *mut CF_Transaction_t) {
    let mut temp_name = [0u8; CF_FILENAME_MAX_LEN];
    let mut subject_file: *const c_char;
    let move_dest: *const c_char;
    let mut pending_fs: u8 = CF_CFDP_FinFileStatus_t::CF_CFDP_FinFileStatus_INVALID as u8;
    let pending_dc: u8;

    /* close file if still open */
    if OS_ObjectIdDefined((*txn).fd) {
        CF_WrappedClose((*txn).fd);
        (*txn).fd = OS_OBJECT_ID_UNDEFINED;
    }

    if (*txn).flags.rx.tempfile_created {
        move_dest = (*(*txn).history).fnames.dst_filename.as_ptr() as *const c_char;
        CF_CFDP_GetTempName((*txn).history, temp_name.as_mut_ptr() as *mut c_char, temp_name.len());
        subject_file = temp_name.as_ptr() as *const c_char;
    } else {
        move_dest = ptr::null();
        subject_file = ptr::null();
    }

    if subject_file.is_null() {
        pending_dc = CF_CFDP_FinDeliveryCode_t::CF_CFDP_FinDeliveryCode_INVALID as u8;
        CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILESTORE_REJECTION);
    } else if !(*txn).flags.com.is_complete {
        pending_dc = CF_CFDP_FinDeliveryCode_t::CF_CFDP_FinDeliveryCode_INCOMPLETE as u8;
        CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_INVALID_FILE_STRUCTURE);
    } else {
        pending_dc = CF_CFDP_FinDeliveryCode_t::CF_CFDP_FinDeliveryCode_COMPLETE as u8;

        if !CF_CFDP_TxnIsOK(txn) || CF_CFDP_R_CheckCrc(txn) != CFE_SUCCESS {
            CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILE_CHECKSUM_FAILURE);
        } else {
            let os_status = OS_mv(subject_file, move_dest);
            if os_status == OS_SUCCESS {
                CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_NO_ERROR);
                pending_fs = CF_CFDP_FinFileStatus_t::CF_CFDP_FinFileStatus_RETAINED as u8;
                CFE_EVS_SendEvent!(
                    CF_CFDP_R_FILE_RETAINED_EID,
                    CFE_EVS_EventType_INFORMATION,
                    b"CF R%d(%lu:%lu): successfully retained file as %s\0".as_ptr() as *const c_char,
                    CF_CFDP_GetPrintClass(txn),
                    (*(*txn).history).src_eid as u64,
                    (*(*txn).history).seq_num as u64,
                    move_dest,
                );
                subject_file = ptr::null(); /* already moved */
            } else {
                CF_CFDP_SetTxnStatus(txn, CF_TxnStatus_t::CF_TxnStatus_FILESTORE_REJECTION);
                CFE_EVS_SendEvent!(
                    CF_CFDP_R_RENAME_ERR_EID,
                    CFE_EVS_EventType_ERROR,
                    b"CF R%d(%lu:%lu): cannot move file to %s, error=%d\0".as_ptr() as *const c_char,
                    CF_CFDP_GetPrintClass(txn),
                    (*(*txn).history).src_eid as u64,
                    (*(*txn).history).seq_num as u64,
                    move_dest,
                    os_status as i32,
                );
            }
        }
    }

    /* If we still have a temp file then remove it */
    if !subject_file.is_null() {
        let os_status = OS_remove(subject_file);
        CFE_EVS_SendEvent!(
            CF_CFDP_R_NOT_RETAINED_EID,
            CFE_EVS_EventType_INFORMATION,
            b"CF R%d(%lu:%lu): removed temp file %s, status=%d, txn_stat=%d\0".as_ptr() as *const c_char,
            CF_CFDP_GetPrintClass(txn),
            (*(*txn).history).src_eid as u64,
            (*(*txn).history).seq_num as u64,
            subject_file,
            os_status as i32,
            (*(*txn).history).txn_stat as i32,
        );

        if (*(*txn).history).txn_stat == CF_TxnStatus_t::CF_TxnStatus_FILESTORE_REJECTION {
            pending_fs = CF_CFDP_FinFileStatus_t::CF_CFDP_FinFileStatus_DISCARDED_FILESTORE as u8;
        } else {
            pending_fs = CF_CFDP_FinFileStatus_t::CF_CFDP_FinFileStatus_DISCARDED as u8;
        }
    }

    (*txn).state_data.fin_dc = pending_dc;
    (*txn).state_data.fin_fs = pending_fs;
}

/// C: `void CF_CFDP_R_AckTimerTick(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_AckTimerTick(txn: *mut CF_Transaction_t) {
    /* note: the ack timer is only ever armed on class 2 */
    if !(*txn).reliable_mode || !(*txn).flags.com.ack_timer_armed {
        return;
    }

    if !CF_Timer_Expired(&(*txn).ack_timer) {
        CF_Timer_Tick(&mut (*txn).ack_timer);
    } else {
        (*txn).flags.com.ack_timer_armed = false;
    }
}

/// C: `CF_RxSubState_t CF_CFDP_R_CheckState_DATA_NORMAL(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CheckState_DATA_NORMAL(txn: *mut CF_Transaction_t) -> CF_RxSubState_t {
    let mut next_state: CF_RxSubState_t = core::mem::transmute((*txn).state_data.sub_state);

    if (*txn).flags.rx.eof_count != 0 { 
        /* we got EOF, do gap check tasks */
        next_state = CF_CFDP_R_CheckState_DATA_EOF(txn);
    }

    next_state
}

/// C: `CF_RxSubState_t CF_CFDP_R_CheckState_DATA_EOF(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CheckState_DATA_EOF(txn: *mut CF_Transaction_t) -> CF_RxSubState_t {
    let mut next_state: CF_RxSubState_t = core::mem::transmute((*txn).state_data.sub_state);

    if CF_CFDP_R_CheckComplete(txn) {
        (*txn).flags.com.is_complete = true;
        CF_CFDP_R_CalcCrcStart(txn);
        next_state = CF_RxSubState_t::CF_RxSubState_VALIDATE;
    } else if (*txn).flags.rx.send_nak {
        (*txn).flags.rx.send_nak = false;
        let ret = CF_CFDP_R_SendNak(txn);
        if ret == CFE_SUCCESS {
            CF_CFDP_ArmAckTimer(txn);
        }
    }

    next_state
}

/// C: `CF_RxSubState_t CF_CFDP_R_CheckState_VALIDATE(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CheckState_VALIDATE(txn: *mut CF_Transaction_t) -> CF_RxSubState_t {
    let mut next_state: CF_RxSubState_t = core::mem::transmute((*txn).state_data.sub_state);

    if (*txn).flags.com.crc_complete {
        CF_CFDP_R_HandleFileRetention(txn);
        next_state = CF_RxSubState_t::CF_RxSubState_FILESTORE;
    } else {
        CF_CFDP_R_CalcCrcChunk(txn);
    }

    next_state
}

/// C: `CF_RxSubState_t CF_CFDP_R_CheckState_FILESTORE(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CheckState_FILESTORE(txn: *mut CF_Transaction_t) -> CF_RxSubState_t {
    let mut next_state: CF_RxSubState_t = core::mem::transmute((*txn).state_data.sub_state);

    /* For class 1, we are done. For class 2, send FIN */
    if (*txn).reliable_mode {
        let ret = CF_CFDP_SendFin(txn);
        if ret == CFE_SUCCESS {
            CF_CFDP_ArmAckTimer(txn);
            next_state = CF_RxSubState_t::CF_RxSubState_FINACK;
        }
    } else {
        CF_CFDP_FinishTransaction(txn, true);
    }

    next_state
}

/// C: `CF_RxSubState_t CF_CFDP_R_CheckState_FINACK(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CheckState_FINACK(txn: *mut CF_Transaction_t) -> CF_RxSubState_t {
    let next_state: CF_RxSubState_t = core::mem::transmute((*txn).state_data.sub_state);

    if (*txn).flags.rx.finack_recv {
        CF_CFDP_FinishTransaction(txn, true);
    } else if (*txn).flags.rx.send_fin {
        (*txn).flags.rx.send_fin = false;
        let ret = CF_CFDP_SendFin(txn);
        if ret == CFE_SUCCESS {
            CF_CFDP_ArmAckTimer(txn);
        }
    }

    next_state
}

/// C: `void CF_CFDP_R_CheckState(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_CheckState(txn: *mut CF_Transaction_t) {
    type StateFn = unsafe fn(*mut CF_Transaction_t) -> CF_RxSubState_t;
    /// Identity function for COMPLETE state — no state change needed.
    unsafe fn CF_CFDP_R_CheckState_COMPLETE(txn: *mut CF_Transaction_t) -> CF_RxSubState_t {
        core::mem::transmute((*txn).state_data.sub_state)
    }
    static STATE_FNS: [StateFn; CF_RxSubState_NUM_STATES as usize] = [
        CF_CFDP_R_CheckState_DATA_NORMAL,
        CF_CFDP_R_CheckState_DATA_EOF,
        CF_CFDP_R_CheckState_VALIDATE,
        CF_CFDP_R_CheckState_FILESTORE,
        CF_CFDP_R_CheckState_FINACK,
        CF_CFDP_R_CheckState_COMPLETE,
    ];

    let sub = (*txn).state_data.sub_state as usize;
    if sub < CF_RxSubState_NUM_STATES as usize {
        (*txn).state_data.sub_state = STATE_FNS[sub](txn) as u8;
    }
}

/// C: `void CF_CFDP_R_Tick_Maintenance(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_Tick_Maintenance(txn: *mut CF_Transaction_t) {
    /* Handle canceled transactions */
    if (*txn).flags.com.canceled {
        CF_CFDP_R_HandleFileRetention(txn);
        CF_CFDP_FinishTransaction(txn, true);
        return;
    }

    CF_CFDP_R_AckTimerTick(txn);
    CF_CFDP_R_CheckState(txn);
}

/// C: `void CF_CFDP_R_Tick(CF_Transaction_t *txn)`
pub unsafe fn CF_CFDP_R_Tick(txn: *mut CF_Transaction_t) {
    if !(*txn).flags.com.suspended {
        CF_CFDP_R_Tick_Maintenance(txn);
    }
}

/// C: `void CF_CFDP_R_DispatchRecv(CF_Transaction_t *txn, CF_Logical_PduBuffer_t *ph, ...)`
pub unsafe fn CF_CFDP_R_DispatchRecv(
    txn: *mut CF_Transaction_t,
    ph: *mut CF_Logical_PduBuffer_t,
) {
    /* Arm the inactivity timer on any received PDU */
    CF_CFDP_ArmInactTimer(txn);

    /* Dispatch based on state */
    if (*txn).state == CF_TxnState_t::CF_TxnState_R1 {
        CF_CFDP_R1_Recv(txn, ph);
    } else {
        CF_CFDP_R2_Recv(txn, ph);
    }
}
