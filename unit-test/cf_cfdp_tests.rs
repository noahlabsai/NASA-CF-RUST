use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    /*******************************************************************************
    **
    **  cf_cfdp_tests local utility functions
    **
    *******************************************************************************/

    fn ut_cfdp_setup_basic_rx_state(pdu_buffer: &mut CF_Logical_PduBuffer_t) {
        static mut UT_DECODER: CF_DecoderState_t = CF_DecoderState_t {
            base: std::ptr::null_mut(),
            codec_state: CF_CodecState_t {
                is_valid: false,
                max_size: 0,
                next_offset: 0,
            },
        };
        static mut BYTES: [u8; CF_CFDP_MAX_HEADER_SIZE] = [0; CF_CFDP_MAX_HEADER_SIZE];

        unsafe {
            BYTES.fill(0);

            UT_DECODER.base = BYTES.as_mut_ptr();
            UT_DECODER.codec_state.is_valid = true;
            UT_DECODER.codec_state.max_size = BYTES.len();
            UT_DECODER.codec_state.next_offset = 0;

            pdu_buffer.pdec = &mut UT_DECODER;
        }
    }

    fn ut_cfdp_setup_basic_tx_state(pdu_buffer: &mut CF_Logical_PduBuffer_t) {
        static mut UT_ENCODER: CF_EncoderState_t = CF_EncoderState_t {
            base: std::ptr::null_mut(),
            codec_state: CF_CodecState_t {
                is_valid: false,
                max_size: 0,
                next_offset: 0,
            },
        };
        static mut BYTES: [u8; CF_CFDP_MAX_HEADER_SIZE] = [0; CF_CFDP_MAX_HEADER_SIZE];

        unsafe {
            BYTES.fill(0);

            UT_ENCODER.base = BYTES.as_mut_ptr();
            UT_ENCODER.codec_state.is_valid = true;
            UT_ENCODER.codec_state.max_size = BYTES.len();
            UT_ENCODER.codec_state.next_offset = 0;

            pdu_buffer.penc = &mut UT_ENCODER;
        }
    }

    fn ut_cfdp_setup_basic_test_state(
        setup: UT_CF_Setup_t,
        pdu_buffer_p: Option<&mut Option<&mut CF_Logical_PduBuffer_t>>,
        channel_p: Option<&mut Option<&mut CF_Channel_t>>,
        history_p: Option<&mut Option<&mut CF_History_t>>,
        txn_p: Option<&mut Option<&mut CF_Transaction_t>>,
        config_table_p: Option<&mut Option<&mut CF_ConfigTable_t>>,
    ) {
        static mut UT_PDU_BUFFER: CF_Logical_PduBuffer_t = CF_Logical_PduBuffer_t::default();
        static mut UT_HISTORY: CF_History_t = CF_History_t::default();
        static mut UT_TRANSACTION: CF_Transaction_t = CF_Transaction_t::default();
        static mut UT_CONFIG_TABLE: CF_ConfigTable_t = CF_ConfigTable_t::default();

        unsafe {
            UT_PDU_BUFFER = CF_Logical_PduBuffer_t::default();
            UT_HISTORY = CF_History_t::default();
            UT_TRANSACTION = CF_Transaction_t::default();
            UT_CONFIG_TABLE = CF_ConfigTable_t::default();

            UT_TRANSACTION.history = &mut UT_HISTORY;
            CF_AppData.config_table = &mut UT_CONFIG_TABLE;

            if let Some(pdu_buffer_ref) = pdu_buffer_p {
                if setup == UT_CF_Setup_TX || setup == UT_CF_Setup_RX {
                    *pdu_buffer_ref = Some(&mut UT_PDU_BUFFER);
                } else {
                    *pdu_buffer_ref = None;
                }
            }

            if let Some(channel_ref) = channel_p {
                *channel_ref = Some(&mut CF_AppData.engine.channels[UT_CFDP_CHANNEL]);
            }

            if let Some(history_ref) = history_p {
                *history_ref = Some(&mut UT_HISTORY);
            }

            if let Some(txn_ref) = txn_p {
                *txn_ref = Some(&mut UT_TRANSACTION);
            }

            if let Some(config_ref) = config_table_p {
                *config_ref = Some(&mut UT_CONFIG_TABLE);
            }

            if setup == UT_CF_Setup_TX {
                UT_HISTORY.dir = CF_Direction_TX;
                ut_cfdp_setup_basic_tx_state(&mut UT_PDU_BUFFER);
                UT_SetHandlerFunction(UT_KEY(CF_CFDP_MsgOutGet), UT_AltHandler_GenericPointerReturn, &UT_PDU_BUFFER);
            } else if setup == UT_CF_Setup_RX {
                UT_HISTORY.dir = CF_Direction_RX;
                ut_cfdp_setup_basic_rx_state(&mut UT_PDU_BUFFER);
            }

            UT_SetDefaultReturnValue(
                UT_KEY(CF_GetChannelFromTxn),
                &CF_AppData.engine.channels[UT_CFDP_CHANNEL] as *const _ as UT_IntReturn_t,
            );
            UT_SetDefaultReturnValue(
                UT_KEY(CF_GetChunkListHead),
                &CF_AppData.engine.channels[UT_CFDP_CHANNEL].cs[UT_HISTORY.dir] as *const _ as UT_IntReturn_t,
            );

            UT_CF_ResetEventCapture();
        }
    }

    /*******************************************************************************
    **
    **  cf_cfdp_tests Setup and Teardown
    **
    *******************************************************************************/

    fn cf_cfdp_tests_setup() {
        cf_tests_Setup();
        unsafe {
            CF_AppData = CF_AppData_t::default();
        }
    }

    fn cf_cfdp_tests_teardown() {
        cf_tests_Teardown();
    }

    /*******************************************************************************
    **
    **  cf_cfdp_tests Implementation-specific tests
    **
    *******************************************************************************/

    #[test]
    fn test_cf_cfdp_cf_cfdp_encode_start() {
        let mut enc = CF_EncoderState_t::default();
        let mut msg = [0u32; 2];
        let mut pdubuf = CF_Logical_PduBuffer_t::default();

        msg.fill(0xEE);

        CF_CFDP_EncodeStart(&mut enc, &mut msg, &mut pdubuf, std::mem::size_of::<u32>(), std::mem::size_of_val(&msg));
        assert!(CF_CODEC_IS_OK(&enc));
        assert!(CF_CODEC_GET_REMAIN(&enc) < std::mem::size_of_val(&msg) as u32);
        assert_eq!(CF_CODEC_GET_POSITION(&enc), 0);
        assert_eq!(CF_CODEC_GET_SIZE(&enc), CF_CODEC_GET_REMAIN(&enc));

        CF_CFDP_EncodeStart(&mut enc, &mut msg, &mut pdubuf, std::mem::size_of::<u32>(), std::mem::size_of::<u32>() - 1);
        assert!(!CF_CODEC_IS_OK(&enc));
    }

    #[test]
    fn test_cf_cfdp_cf_cfdp_decode_start() {
        let mut dec = CF_DecoderState_t::default();
        let mut msg = [0u32; 2];
        let mut pdubuf = CF_Logical_PduBuffer_t::default();

        msg.fill(0xEE);

        CF_CFDP_DecodeStart(&mut dec, &mut msg, &mut pdubuf, std::mem::size_of::<u32>(), std::mem::size_of_val(&msg));
        assert!(CF_CODEC_IS_OK(&dec));
        assert!(CF_CODEC_GET_REMAIN(&dec) < std::mem::size_of_val(&msg) as u32);
        assert_eq!(CF_CODEC_GET_POSITION(&dec), 0);
        assert_eq!(CF_CODEC_GET_SIZE(&dec), CF_CODEC_GET_REMAIN(&dec));

        CF_CFDP_DecodeStart(&mut dec, &mut msg, &mut pdubuf, std::mem::size_of::<u32>(), std::mem::size_of::<u32>() - 1);
        assert!(!CF_CODEC_IS_OK(&dec));
    }

    #[test]
    fn test_cf_cfdp_arm_ack_timer() {
        let mut txn = None;
        let mut config = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), Some(&mut config));

        CF_CFDP_ArmAckTimer(txn.unwrap());
    }

    #[test]
    fn test_cf_cfdp_recv_ph() {
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, None, None);
        assert_eq!(CF_CFDP_RecvPh(UT_CFDP_CHANNEL, ph.unwrap()), 0);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, None, None);
        ph.unwrap().pdu_header.pdu_type = 1;
        assert_eq!(CF_CFDP_RecvPh(UT_CFDP_CHANNEL, ph.unwrap()), 0);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, None, None);
        CF_CODEC_SET_DONE(ph.unwrap().pdec);
        assert_eq!(CF_CFDP_RecvPh(UT_CFDP_CHANNEL, ph.unwrap()), CF_SHORT_PDU_ERROR);
        UT_CF_AssertEventID(CF_PDU_SHORT_HEADER_ERR_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, None, None);
        ph.unwrap().pdu_header.large_flag = true;
        assert_eq!(CF_CFDP_RecvPh(UT_CFDP_CHANNEL, ph.unwrap()), CF_ERROR);
        UT_CF_AssertEventID(CF_PDU_LARGE_FILE_ERR_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, None, None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_DecodeHeader), 1, CF_ERROR);
        assert_eq!(CF_CFDP_RecvPh(UT_CFDP_CHANNEL, ph.unwrap()), CF_ERROR);
        UT_CF_AssertEventID(CF_PDU_TRUNCATION_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_recv_md() {
        let mut txn = None;
        let mut history = None;
        let mut ph = None;
        let src = b"mds";
        let dest = b"mdd";

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, Some(&mut history), Some(&mut txn), None);
        let md = &mut ph.unwrap().int_header.md;
        md.size = 10;
        md.dest_filename.length = dest.len() - 1;
        md.dest_filename.data_ptr = dest.as_ptr() as *const i8;
        md.source_filename.length = src.len() - 1;
        md.source_filename.data_ptr = src.as_ptr() as *const i8;
        assert_eq!(CF_CFDP_RecvMd(txn.unwrap(), ph.unwrap()), 0);
        assert_eq!(txn.unwrap().fsize, md.size);
        UtAssert_STRINGBUF_EQ(
            md.dest_filename.data_ptr,
            md.dest_filename.length,
            history.unwrap().fnames.dst_filename.as_ptr(),
            history.unwrap().fnames.dst_filename.len(),
        );
        UtAssert_STRINGBUF_EQ(
            md.source_filename.data_ptr,
            md.source_filename.length,
            history.unwrap().fnames.src_filename.as_ptr(),
            history.unwrap().fnames.src_filename.len(),
        );

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CODEC_SET_DONE(ph.unwrap().pdec);
        assert_eq!(CF_CFDP_RecvMd(txn.unwrap(), ph.unwrap()), CF_PDU_METADATA_ERROR);
        UT_CF_AssertEventID(CF_PDU_MD_SHORT_ERR_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        let md = &mut ph.unwrap().int_header.md;
        md.dest_filename.length = CF_FILENAME_MAX_LEN + 1;
        assert_eq!(CF_CFDP_RecvMd(txn.unwrap(), ph.unwrap()), CF_PDU_METADATA_ERROR);
        UT_CF_AssertEventID(CF_PDU_INVALID_DST_LEN_ERR_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        let md = &mut ph.unwrap().int_header.md;
        md.source_filename.length = CF_FILENAME_MAX_LEN + 1;
        assert_eq!(CF_CFDP_RecvMd(txn.unwrap(), ph.unwrap()), CF_PDU_METADATA_ERROR);
        UT_CF_AssertEventID(CF_PDU_INVALID_SRC_LEN_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_recv_fd() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_RecvFd(txn.unwrap(), ph.unwrap()), 0);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        ph.unwrap().pdu_header.crc_flag = 1;
        ph.unwrap().int_header.fd.data_len = 10 + std::mem::size_of::<CF_CFDP_uint32_t>() as u32;
        assert_eq!(CF_CFDP_RecvFd(txn.unwrap(), ph.unwrap()), 0);
        assert_eq!(ph.unwrap().int_header.fd.data_len, 10);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CODEC_SET_DONE(ph.unwrap().pdec);
        assert_eq!(CF_CFDP_RecvFd(txn.unwrap(), ph.unwrap()), CF_SHORT_PDU_ERROR);
        UT_CF_AssertEventID(CF_PDU_FD_SHORT_ERR_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        ph.unwrap().pdu_header.crc_flag = 1;
        ph.unwrap().int_header.fd.data_len = std::mem::size_of::<CF_CFDP_uint32_t>() as u32 - 1;
        assert_eq!(CF_CFDP_RecvFd(txn.unwrap(), ph.unwrap()), CF_SHORT_PDU_ERROR);
        assert!(!CF_CODEC_IS_OK(ph.unwrap().pdec));

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        ph.unwrap().pdu_header.segment_meta_flag = 1;
        assert_eq!(CF_CFDP_RecvFd(txn.unwrap(), ph.unwrap()), CF_ERROR);
        UT_CF_AssertEventID(CF_PDU_FD_UNSUPPORTED_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_recv_eof() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_RecvEof(txn.unwrap(), ph.unwrap()), 0);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CODEC_SET_DONE(ph.unwrap().pdec);
        assert_eq!(CF_CFDP_RecvEof(txn.unwrap(), ph.unwrap()), CF_SHORT_PDU_ERROR);
        UT_CF_AssertEventID(CF_PDU_EOF_SHORT_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_recv_ack() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_RecvAck(txn.unwrap(), ph.unwrap()), 0);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CODEC_SET_DONE(ph.unwrap().pdec);
        assert_eq!(CF_CFDP_RecvAck(txn.unwrap(), ph.unwrap()), CF_SHORT_PDU_ERROR);
        UT_CF_AssertEventID(CF_PDU_ACK_SHORT_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_recv_fin() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_RecvFin(txn.unwrap(), ph.unwrap()), 0);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CODEC_SET_DONE(ph.unwrap().pdec);
        assert_eq!(CF_CFDP_RecvFin(txn.unwrap(), ph.unwrap()), CF_SHORT_PDU_ERROR);
        UT_CF_AssertEventID(CF_PDU_FIN_SHORT_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_recv_nak() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_RecvNak(txn.unwrap(), ph.unwrap()), 0);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CODEC_SET_DONE(ph.unwrap().pdec);
        assert_eq!(CF_CFDP_RecvNak(txn.unwrap(), ph.unwrap()), CF_SHORT_PDU_ERROR);
        UT_CF_AssertEventID(CF_PDU_NAK_SHORT_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_recv_drop() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        CF_CFDP_RecvDrop(txn.unwrap(), ph.unwrap());
    }

    #[test]
    fn test_cf_cfdp_recv_hold() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        txn.unwrap().history.dir = CF_Direction_TX;
        ph.unwrap().fdirective.directive_code = CF_CFDP_FileDirective_FIN;
        CF_CFDP_RecvHold(txn.unwrap(), ph.unwrap());
        UtAssert_STUB_COUNT(CF_CFDP_S_SubstateRecvFin, 1);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        txn.unwrap().history.dir = CF_Direction_RX;
        ph.unwrap().fdirective.directive_code = CF_CFDP_FileDirective_EOF;
        CF_CFDP_RecvHold(txn.unwrap(), ph.unwrap());
        UtAssert_STUB_COUNT(CF_CFDP_R_SubstateRecvEof, 1);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        txn.unwrap().history.dir = CF_Direction_TX;
        ph.unwrap().fdirective.directive_code = CF_CFDP_FileDirective_INVALID_MAX;
        CF_CFDP_RecvHold(txn.unwrap(), ph.unwrap());
        UtAssert_STUB_COUNT(CF_CFDP_S_SubstateRecvFin, 1);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, None, Some(&mut txn), None);
        txn.unwrap().history.dir = CF_Direction_RX;
        ph.unwrap().fdirective.directive_code = CF_CFDP_FileDirective_INVALID_MAX;
        CF_CFDP_RecvHold(txn.unwrap(), ph.unwrap());
        UtAssert_STUB_COUNT(CF_CFDP_R_SubstateRecvEof, 1);
    }

    #[test]
    fn test_cf_cfdp_recv_init() {
        let mut txn = None;
        let mut history = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), None, Some(&mut history), Some(&mut txn), None);
        txn.unwrap().state = CF_TxnState_INIT;
        CF_CFDP_RecvInit(txn.unwrap(), ph.unwrap());
        assert_eq!(txn.unwrap().state, CF_TxnState_HOLD);
    }

    #[test]
    fn test_cf_cfdp_copy_string_from_lv() {
        let mut buf = [0i8; 20];
        let refstr = b"refstr";
        let mut input = CF_Logical_Lv_t {
            data_ptr: refstr.as_ptr() as *const i8,
            length: refstr.len() - 1,
        };

        assert_eq!(CF_CFDP_CopyStringFromLV(buf.as_mut_ptr(), buf.len(), &input), input.length as i32);
    }

    #[test]
    fn test_cf_cfdp_construct_pdu_header() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        assert!(CF_CFDP_ConstructPduHeader(txn.unwrap(), CF_CFDP_FileDirective_ACK, 3, 2, true, 42, false).is_null());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        assert!(CF_CFDP_ConstructPduHeader(txn.unwrap(), CF_CFDP_FileDirective_ACK, 3, 2, true, 42, true).is_null());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        txn.unwrap().state = CF_TxnState_S1;
        assert!(!CF_CFDP_ConstructPduHeader(txn.unwrap(), CF_CFDP_FileDirective_ACK, 3, 2, true, 42, false).is_null());
        let hdr = &ph.unwrap().pdu_header;
        assert_eq!(hdr.version, 1);
        assert_eq!(hdr.pdu_type, 0);
        assert_eq!(hdr.direction, 1);
        assert_eq!(hdr.txm_mode, 1);
        assert_eq!(hdr.eid_length, 1);
        assert_eq!(hdr.txn_seq_length, 1);
        assert_eq!(hdr.source_eid, 3);
        assert_eq!(hdr.destination_eid, 2);
        assert_eq!(hdr.sequence_num, 42);
        assert_eq!(ph.unwrap().fdirective.directive_code, CF_CFDP_FileDirective_ACK);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_GetValueEncodedSize), 5);
        txn.unwrap().state = CF_TxnState_S2;
        assert!(!CF_CFDP_ConstructPduHeader(txn.unwrap(), 0, 7, 6, false, 44, false).is_null());
        let hdr = &ph.unwrap().pdu_header;
        assert_eq!(hdr.version, 1);
        assert_eq!(hdr.pdu_type, 1);
        assert_eq!(hdr.direction, 0);
        assert_eq!(hdr.txm_mode, 0);
        assert_eq!(hdr.eid_length, 5);
        assert_eq!(hdr.txn_seq_length, 5);
        assert_eq!(hdr.source_eid, 7);
        assert_eq!(hdr.destination_eid, 6);
        assert_eq!(hdr.sequence_num, 44);
    }

    #[test]
    fn test_cf_cfdp_send_md() {
        let mut txn = None;
        let mut ph = None;
        let mut history = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_SendMd(txn.unwrap()), CF_SEND_PDU_NO_BUF_AVAIL_ERROR);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, Some(&mut history), Some(&mut txn), None);
        let md = &mut ph.unwrap().int_header.md;
        history.unwrap().fnames.dst_filename[..4].copy_from_slice(b"dst1");
        history.unwrap().fnames.src_filename[..4].copy_from_slice(b"src1");
        txn.unwrap().state = CF_TxnState_S1;
        txn.unwrap().fsize = 1234;
        assert_eq!(CF_CFDP_SendMd(txn.unwrap()), CFE_SUCCESS);
        assert_eq!(md.size, txn.unwrap().fsize);
        UtAssert_STRINGBUF_EQ(
            md.dest_filename.data_ptr,
            md.dest_filename.length,
            history.unwrap().fnames.dst_filename.as_ptr(),
            history.unwrap().fnames.dst_filename.len(),
        );
        UtAssert_STRINGBUF_EQ(
            md.source_filename.data_ptr,
            md.source_filename.length,
            history.unwrap().fnames.src_filename.as_ptr(),
            history.unwrap().fnames.src_filename.len(),
        );

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, Some(&mut history), Some(&mut txn), None);
        let md = &mut ph.unwrap().int_header.md;
        history.unwrap().fnames.dst_filename.fill(0xFF);
        history.unwrap().fnames.src_filename[..4].copy_from_slice(b"src2");
        txn.unwrap().state = CF_TxnState_S2;
        txn.unwrap().fsize = 5678;
        assert_eq!(CF_CFDP_SendMd(txn.unwrap()), CFE_SUCCESS);
        assert_eq!(md.size, txn.unwrap().fsize);
        assert_eq!(md.dest_filename.length, history.unwrap().fnames.dst_filename.len());
        UtAssert_STRINGBUF_EQ(
            md.source_filename.data_ptr,
            md.source_filename.length,
            history.unwrap().fnames.src_filename.as_ptr(),
            history.unwrap().fnames.src_filename.len(),
        );
    }

    #[test]
    fn test_cf_cfdp_send_fd() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_SendFd(txn.unwrap(), ph.unwrap()), CFE_SUCCESS);

        ph.unwrap().pdu_header.header_encoded_length = CF_CODEC_GET_POSITION(ph.unwrap().penc) + 1;
        ph.unwrap().pdu_header.data_encoded_length = 0;

        assert_eq!(CF_CFDP_SendFd(txn.unwrap(), ph.unwrap()), CFE_SUCCESS);
        assert_eq!(ph.unwrap().pdu_header.data_encoded_length, 0);
    }

    #[test]
    fn test_cf_cfdp_send_eof() {
        let mut txn = None;
        let mut history = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_SendEof(txn.unwrap()), CF_SEND_PDU_NO_BUF_AVAIL_ERROR);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        let eof = &ph.unwrap().int_header.eof;
        assert_eq!(CF_CFDP_SendEof(txn.unwrap()), CFE_SUCCESS);
        assert_eq!(eof.tlv_list.num_tlv, 0);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, Some(&mut history), Some(&mut txn), None);
        let eof = &ph.unwrap().int_header.eof;
        UT_SetDefaultReturnValue(UT_KEY(CF_TxnStatus_To_ConditionCode), CF_CFDP_ConditionCode_FILESTORE_REJECTION);
        assert_eq!(CF_CFDP_SendEof(txn.unwrap()), CFE_SUCCESS);
        assert_eq!(eof.tlv_list.num_tlv, 1);
        UtAssert_STUB_COUNT(CF_CFDP_Send, 2);
    }

    #[test]
    fn test_cf_cfdp_send_ack() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_SendAck(txn.unwrap(), CF_CFDP_FileDirective_EOF), CF_SEND_PDU_NO_BUF_AVAIL_ERROR);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_GetAckTxnStatus), 1, CF_CFDP_AckTxnStatus_ACTIVE);
        txn.unwrap().state = CF_TxnState_R2;
        assert_eq!(CF_CFDP_SendAck(txn.unwrap(), CF_CFDP_FileDirective_EOF), CFE_SUCCESS);
        let ack = &ph.unwrap().int_header.ack;
        assert_eq!(ack.ack_directive_code, CF_CFDP_FileDirective_EOF);
        assert_eq!(ack.ack_subtype_code, 0);
        assert_eq!(ack.txn_status, CF_CFDP_AckTxnStatus_ACTIVE);
        assert_eq!(ack.cc, CF_CFDP_ConditionCode_NO_ERROR);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_GetAckTxnStatus), 1, CF_CFDP_AckTxnStatus_ACTIVE);
        txn.unwrap().state = CF_TxnState_S2;
        assert_eq!(CF_CFDP_SendAck(txn.unwrap(), CF_CFDP_FileDirective_EOF), CFE_SUCCESS);
        let ack = &ph.unwrap().int_header.ack;
        assert_eq!(ack.ack_directive_code, CF_CFDP_FileDirective_EOF);
        assert_eq!(ack.ack_subtype_code, 0);
        assert_eq!(ack.txn_status, CF_CFDP_AckTxnStatus_ACTIVE);
        assert_eq!(ack.cc, CF_CFDP_ConditionCode_NO_ERROR);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_GetAckTxnStatus), 1, CF_CFDP_AckTxnStatus_TERMINATED);
        txn.unwrap().state = CF_TxnState_R2;
        txn.unwrap().state_data.peer_cc = CF_CFDP_ConditionCode_FILESTORE_REJECTION;
        assert_eq!(CF_CFDP_SendAck(txn.unwrap(), CF_CFDP_FileDirective_FIN), CFE_SUCCESS);
        let ack = &ph.unwrap().int_header.ack;
        assert_eq!(ack.ack_directive_code, CF_CFDP_FileDirective_FIN);
        assert_eq!(ack.ack_subtype_code, 1);
        assert_eq!(ack.txn_status, CF_CFDP_AckTxnStatus_TERMINATED);
        assert_eq!(ack.cc, CF_CFDP_ConditionCode_FILESTORE_REJECTION);
    }

    #[test]
    fn test_cf_cfdp_send_fin() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_SendFin(txn.unwrap()), CF_SEND_PDU_NO_BUF_AVAIL_ERROR);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        txn.unwrap().state_data.fin_dc = CF_CFDP_FinDeliveryCode_COMPLETE;
        txn.unwrap().state_data.fin_fs = CF_CFDP_FinFileStatus_RETAINED;
        assert_eq!(CF_CFDP_SendFin(txn.unwrap()), CFE_SUCCESS);
        let fin = &ph.unwrap().int_header.fin;
        assert_eq!(fin.tlv_list.num_tlv, 0);
        assert_eq!(fin.delivery_code, CF_CFDP_FinDeliveryCode_COMPLETE);
        assert_eq!(fin.file_status, CF_CFDP_FinFileStatus_RETAINED);
        assert_eq!(fin.cc, CF_CFDP_ConditionCode_NO_ERROR);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        txn.unwrap().state_data.fin_dc = CF_CFDP_FinDeliveryCode_INCOMPLETE;
        txn.unwrap().state_data.fin_fs = CF_CFDP_FinFileStatus_DISCARDED;
        UT_SetDeferredRetcode(UT_KEY(CF_TxnStatus_To_ConditionCode), 1, CF_TxnStatus_FILESTORE_REJECTION);
        assert_eq!(CF_CFDP_SendFin(txn.unwrap()), CFE_SUCCESS);
        let fin = &ph.unwrap().int_header.fin;
        assert_eq!(fin.delivery_code, CF_CFDP_FinDeliveryCode_INCOMPLETE);
        assert_eq!(fin.file_status, CF_CFDP_FinFileStatus_DISCARDED);
        assert_eq!(fin.cc, CF_CFDP_ConditionCode_FILESTORE_REJECTION);
        assert_eq!(fin.tlv_list.num_tlv, 1);
        UtAssert_STUB_COUNT(CF_CFDP_Send, 2);
    }

    #[test]
    fn test_cf_cfdp_send_nak() {
        let mut txn = None;
        let mut ph = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, Some(&mut txn), None);
        txn.unwrap().state = CF_TxnState_S2;
        CF_CFDP_SendNak(txn.unwrap(), ph.unwrap());
    }

    #[test]
    fn test_cf_cfdp_append_tlv() {
        let mut ph = None;
        let mut config = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, Some(&mut ph), None, None, None, Some(&mut config));
        config.unwrap().local_eid = 123;
        let tlv_list = &mut ph.unwrap().int_header.eof.tlv_list;

        CF_CFDP_AppendTlv(tlv_list, 1);
        assert!(tlv_list.tlv[0].data.data_ptr.is_null());
        assert_eq!(tlv_list.tlv[0].length, 0);
        assert_eq!(tlv_list.num_tlv, 1);

        CF_CFDP_AppendTlv(tlv_list, CF_CFDP_TLV_TYPE_ENTITY_ID);
        assert_eq!(tlv_list.tlv[1].data.eid, config.unwrap().local_eid);
        assert_eq!(tlv_list.num_tlv, 2);

        tlv_list.num_tlv = CF_PDU_MAX_TLV;
        CF_CFDP_AppendTlv(tlv_list, 1);
        assert_eq!(tlv_list.num_tlv, CF_PDU_MAX_TLV);
    }

    #[test]
    fn test_cf_cfdp_init_engine() {
        let mut config = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, None, Some(&mut config));
        assert_eq!(CF_CFDP_InitEngine(), 0);
        assert!(CF_AppData.engine.enabled);
        UtAssert_STUB_COUNT(CF_FreeTransaction, CF_NUM_TRANSACTIONS_PER_CHANNEL * CF_NUM_CHANNELS);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, None, Some(&mut config));
        config.unwrap().chan[0].sem_name[0] = b'u' as i8;
        assert_eq!(CF_CFDP_InitEngine(), 0);
        assert!(CF_AppData.engine.enabled);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, None, Some(&mut config));
        config.unwrap().chan[0].sem_name[0] = b'u' as i8;
        UT_SetDefaultReturnValue(UT_KEY(OS_CountSemGetIdByName), OS_ERROR);
        assert_eq!(CF_CFDP_InitEngine(), OS_ERROR);
        assert!(!CF_AppData.engine.enabled);
        UT_CF_AssertEventID(CF_INIT_SEM_ERR_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, None, Some(&mut config));
        config.unwrap().chan[0].sem_name[0] = b'u' as i8;
        UT_SetDefaultReturnValue(UT_KEY(OS_CountSemGetIdByName), OS_ERR_NAME_NOT_FOUND);
        assert_eq!(CF_CFDP_InitEngine(), OS_ERR_NAME_NOT_FOUND);
        assert!(!CF_AppData.engine.enabled);
        UT_CF_AssertEventID(CF_INIT_SEM_ERR_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, None, Some(&mut config));
        config.unwrap().chan[0].sem_name[0] = b'u' as i8;
        UT_SetDefaultReturnValue(UT_KEY(OS_CountSemGetIdByName), OS_SUCCESS);
        UT_SetDeferredRetcode(UT_KEY(OS_CountSemGetIdByName), 1, OS_ERR_NAME_NOT_FOUND);
        assert_eq!(CF_CFDP_InitEngine(), 0);
        assert!(CF_AppData.engine.enabled);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, None, Some(&mut config));
        UT_SetDeferredRetcode(UT_KEY(CFE_SB_CreatePipe), 1, CFE_STATUS_EXTERNAL_RESOURCE_FAIL);
        assert_eq!(CF_CFDP_InitEngine(), CFE_STATUS_EXTERNAL_RESOURCE_FAIL);
        assert!(!CF_AppData.engine.enabled);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, None, Some(&mut config));
        UT_SetDeferredRetcode(UT_KEY(CFE_SB_SubscribeLocal), 1, CFE_STATUS_EXTERNAL_RESOURCE_FAIL);
        assert_eq!(CF_CFDP_InitEngine(), CFE_STATUS_EXTERNAL_RESOURCE_FAIL);
        assert!(!CF_AppData.engine.enabled);
    }

    #[test]
    fn test_cf_cfdp_tx_file() {
        let src = "tsrc";
        let dest = "tdest";
        let mut history = None;
        let mut txn = None;
        let mut chan = None;
        let mut chunk_wrap = CF_ChunkWrapper_t::default();

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut chan), Some(&mut history), Some(&mut txn), None);
        UT_SetHandlerFunction(UT_KEY(CF_FindUnusedTransaction), UT_AltHandler_GenericPointerReturn, txn.unwrap());
        UT_SetHandlerFunction(UT_KEY(CF_CList_Pop), UT_AltHandler_GenericPointerReturn, &mut chunk_wrap.cl_node);
        chan.unwrap().cs[CF_Direction_TX] = &mut chunk_wrap.cl_node;
        assert_eq!(CF_CFDP_TxFile(src.as_ptr() as *const i8, dest.as_ptr() as *const i8, CF_CFDP_CLASS_1, 1, UT_CFDP_CHANNEL, 0, 1), 0);
        UtAssert_STRINGBUF_EQ(dest.as_ptr() as *const i8, -1, history.unwrap().fnames.dst_filename.as_ptr(), history.unwrap().fnames.dst_filename.len());
        UtAssert_STRINGBUF_EQ(src.as_ptr() as *const i8, -1, history.unwrap().fnames.src_filename.as_ptr(), history.unwrap().fnames.src_filename.len());
        assert_eq!(chan.unwrap().num_cmd_tx, 1);
        UT_CF_AssertEventID(CF_CFDP_S_START_SEND_INF_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut chan), Some(&mut history), Some(&mut txn), None);
        UT_SetHandlerFunction(UT_KEY(CF_FindUnusedTransaction), UT_AltHandler_GenericPointerReturn, txn.unwrap());
        UT_SetHandlerFunction(UT_KEY(CF_CList_Pop), UT_AltHandler_GenericPointerReturn, &mut chunk_wrap.cl_node);
        chan.unwrap().cs[CF_Direction_TX] = &mut chunk_wrap.cl_node;
        assert_eq!(CF_CFDP_TxFile(src.as_ptr() as *const i8, dest.as_ptr() as *const i8, CF_CFDP_CLASS_2, 1, UT_CFDP_CHANNEL, 0, 1), 0);
        UtAssert_STRINGBUF_EQ(dest.as_ptr() as *const i8, -1, history.unwrap().fnames.dst_filename.as_ptr(), history.unwrap().fnames.dst_filename.len());
        UtAssert_STRINGBUF_EQ(src.as_ptr() as *const i8, -1, history.unwrap().fnames.src_filename.as_ptr(), history.unwrap().fnames.src_filename.len());
        assert_eq!(chan.unwrap().num_cmd_tx, 2);
        UT_CF_AssertEventID(CF_CFDP_S_START_SEND_INF_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut chan), Some(&mut history), Some(&mut txn), None);
        chan.unwrap().num_cmd_tx = CF_MAX_COMMANDED_PLAYBACK_FILES_PER_CHAN;
        assert_eq!(CF_CFDP_TxFile(src.as_ptr() as *const i8, dest.as_ptr() as *const i8, CF_CFDP_CLASS_1, 1, UT_CFDP_CHANNEL, 0, 1), -1);
        UT_CF_AssertEventID(CF_CFDP_MAX_CMD_TX_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_start_rx_transaction() {
        let mut txn = None;

        unsafe {
            CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].q_size[CF_QueueIdx_RX] = CF_MAX_SIMULTANEOUS_RX;
        }
        assert!(CF_CFDP_StartRxTransaction(UT_CFDP_CHANNEL).is_null());

        unsafe {
            CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].q_size[CF_QueueIdx_RX] = CF_MAX_SIMULTANEOUS_RX - 1;
        }
        assert!(CF_CFDP_StartRxTransaction(UT_CFDP_CHANNEL).is_null());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        UT_SetHandlerFunction(UT_KEY(CF_FindUnusedTransaction), UT_AltHandler_GenericPointerReturn, txn.unwrap());
        assert_eq!(CF_CFDP_StartRxTransaction(UT_CFDP_CHANNEL), txn.unwrap());
        assert_eq!(txn.unwrap().flags.com.q_index, CF_QueueIdx_RX);
    }

    #[test]
    fn test_cf_cfdp_playback_dir() {
        let src = "psrc";
        let dest = "pdest";
        let mut chan = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), None, None, None);
        let pb = &mut chan.unwrap().playbook[0];
        *pb = CF_Playbook_t::default();
        assert_eq!(CF_CFDP_PlaybackDir(src.as_ptr() as *const i8, dest.as_ptr() as *const i8, CF_CFDP_CLASS_1, 1, UT_CFDP_CHANNEL, 0, 1), 0);
        UtAssert_STRINGBUF_EQ(dest.as_ptr() as *const i8, -1, pb.fnames.dst_filename.as_ptr(), pb.fnames.dst_filename.len());
        UtAssert_STRINGBUF_EQ(src.as_ptr() as *const i8, -1, pb.fnames.src_filename.as_ptr(), pb.fnames.src_filename.len());
        assert!(pb.diropen);
        assert!(pb.busy);

        *pb = CF_Playbook_t::default();
        UT_SetDeferredRetcode(UT_KEY(OS_DirectoryOpen), 1, OS_ERROR);
        assert_eq!(CF_CFDP_PlaybackDir(src.as_ptr() as *const i8, dest.as_ptr() as *const i8, CF_CFDP_CLASS_1, 1, UT_CFDP_CHANNEL, 0, 1), -1);
        UT_CF_AssertEventID(CF_CFDP_OPENDIR_ERR_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), None, None, None);
        for i in 0..CF_MAX_COMMANDED_PLAYBACK_DIRECTORIES_PER_CHAN {
            let pb = &mut chan.unwrap().playbook[i];
            pb.busy = true;
        }
        assert_eq!(CF_CFDP_PlaybackDir(src.as_ptr() as *const i8, dest.as_ptr() as *const i8, CF_CFDP_CLASS_1, 1, UT_CFDP_CHANNEL, 0, 1), -1);
        UT_CF_AssertEventID(CF_CFDP_DIR_SLOT_ERR_EID);
    }

    fn ut_hook_state_handler_set_count(_user_obj: *mut std::ffi::c_void, stub_retcode: i32, call_count: u32, _context: *const UT_StubContext_t) -> i32 {
        if call_count < 3 {
            unsafe {
                CF_AppData.engine.channels[UT_CFDP_CHANNEL].outgoing_counter += 1;
            }
        }
        stub_retcode
    }

    #[test]
    fn test_cf_cfdp_s_tick_new_data() {
        let mut txn = None;
        let mut chan = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        txn.unwrap().flags.com.suspended = true;
        CF_CFDP_S_Tick_NewData(txn.unwrap());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        txn.unwrap().flags.com.suspended = false;
        txn.unwrap().state_data.sub_state = CF_TxSubState_DATA_EOF;
        CF_CFDP_S_Tick_NewData(txn.unwrap());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut chan), None, Some(&mut txn), None);
        UT_SetHandlerFunction(UT_KEY(CF_GetChannelFromTxn), UT_AltHandler_GenericPointerReturn, chan.unwrap());
        txn.unwrap().flags.com.suspended = false;
        txn.unwrap().state_data.sub_state = CF_TxSubState_DATA_NORMAL;
        UT_SetHookFunction(UT_KEY(CF_CFDP_S_SubstateSendFileData), ut_hook_state_handler_set_count, std::ptr::null_mut());
        CF_CFDP_S_Tick_NewData(txn.unwrap());
    }

    fn do_tick_fn_set_blocked(txn: &mut CF_Transaction_t) {
        unsafe {
            CF_AppData.engine.channels[txn.chan_num].tx_blocked = true;
        }
    }

    fn do_tick_noop(_txn: &mut CF_Transaction_t) {
        UT_DEFAULT_IMPL(do_tick_noop);
    }

    #[test]
    fn test_cf_cfdp_do_tick() {
        let mut txn = None;
        let mut txn2 = CF_Transaction_t::default();
        let mut args = CF_CFDP_Tick_args_t::default();

        args.fn_ptr = do_tick_noop;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut args.chan), None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_DoTick(&mut txn.unwrap().cl_node, &mut args), CF_CLIST_CONT);
        UtAssert_STUB_COUNT(do_tick_noop, 1);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut args.chan), None, Some(&mut txn), None);
        args.resume_point = Some(&mut txn2);
        assert_eq!(CF_CFDP_DoTick(&mut txn.unwrap().cl_node, &mut args), CF_CLIST_CONT);
        UtAssert_STUB_COUNT(do_tick_noop, 1);
        assert_eq!(args.resume_point.unwrap(), &mut txn2);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut args.chan), None, Some(&mut txn), None);
        args.resume_point = Some(txn.unwrap());
        assert_eq!(CF_CFDP_DoTick(&mut txn.unwrap().cl_node, &mut args), CF_CLIST_CONT);
        UtAssert_STUB_COUNT(do_tick_noop, 2);
        assert!(args.resume_point.is_none());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut args.chan), None, Some(&mut txn), None);
        txn.unwrap().flags.com.suspended = true;
        assert_eq!(CF_CFDP_DoTick(&mut txn.unwrap().cl_node, &mut args), CF_CLIST_CONT);
        UtAssert_STUB_COUNT(do_tick_noop, 2);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut args.chan), None, Some(&mut txn), None);
        args.fn_ptr = do_tick_fn_set_blocked;
        assert_eq!(CF_CFDP_DoTick(&mut txn.unwrap().cl_node, &mut args), CF_CLIST_EXIT);
    }

    #[test]
    fn test_cf_cfdp_process_polling_directories() {
        let mut chan = None;
        let mut config = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut chan), None, None, Some(&mut config));
        let pdcfg = &mut config.unwrap().chan[UT_CFDP_CHANNEL].polldir[0];
        let poll = &mut chan.unwrap().poll[0];

        CF_CFDP_ProcessPollingDirectories(chan.unwrap());
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].poll_counter, 0);
        }

        pdcfg.enabled = 1;
        CF_CFDP_ProcessPollingDirectories(chan.unwrap());
        assert!(!poll.timer_set);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].poll_counter, 1);
        }
        UtAssert_STUB_COUNT(CF_Timer_Tick, 1);

        pdcfg.interval_sec = 1;
        CF_CFDP_ProcessPollingDirectories(chan.unwrap());
        assert!(poll.timer_set);
        UtAssert_STUB_COUNT(CF_Timer_Tick, 1);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].poll_counter, 1);
        }

        CF_CFDP_ProcessPollingDirectories(chan.unwrap());
        assert!(poll.timer_set);
        UtAssert_STUB_COUNT(CF_Timer_Tick, 2);

        UT_SetDeferredRetcode(UT_KEY(CF_Timer_Expired), 1, true);
        CF_CFDP_ProcessPollingDirectories(chan.unwrap());
        assert!(!poll.timer_set);
        assert!(poll.pb.busy);
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].poll_counter, 1);
        }

        poll.pb.busy = false;
        poll.timer_set = true;
        UT_SetDeferredRetcode(UT_KEY(CF_Timer_Expired), 1, true);
        UT_SetDeferredRetcode(UT_KEY(OS_DirectoryOpen), 1, OS_ERROR);
        CF_CFDP_ProcessPollingDirectories(chan.unwrap());
        assert!(poll.timer_set);
        UT_CF_AssertEventID(CF_CFDP_OPENDIR_ERR_EID);

        poll.pb.busy = false;
        poll.pb.diropen = false;
        poll.pb.num_ts = 1;
        CF_CFDP_ProcessPollingDirectories(chan.unwrap());
        assert!(!poll.pb.busy);

        poll.pb.busy = true;
        poll.pb.num_ts = 0;
        CF_CFDP_ProcessPollingDirectories(chan.unwrap());
        assert!(!poll.pb.busy);

        pdcfg.enabled = 0;
        CF_CFDP_ProcessPollingDirectories(chan.unwrap());
        unsafe {
            assert_eq!(CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].poll_counter, 0);
        }
    }

    #[test]
    fn test_cf_cfdp_process_playback_directory() {
        let mut txn = None;
        let mut history = None;
        let mut chan = None;
        let mut config = None;
        let mut pb = CF_Playbook_t::default();
        let mut dirent = [os_dirent_t::default(); 3];
        let mut chunk_wrap = CF_ChunkWrapper_t::default();

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), Some(&mut history), Some(&mut txn), Some(&mut config));
        unsafe {
            CF_AppData.engine.enabled = true;
        }

        pb.busy = true;
        pb.num_ts = CF_NUM_TRANSACTIONS_PER_PLAYBACK + 1;
        pb.diropen = true;
        CF_CFDP_ProcessPlaybackDirectory(chan.unwrap(), &mut pb);
        assert!(pb.busy);
        assert!(pb.diropen);

        pb.busy = true;
        pb.diropen = true;
        pb.num_ts = 0;
        OS_DirectoryOpen(&mut pb.dir_id, "ut".as_ptr() as *const i8);
        UT_SetDeferredRetcode(UT_KEY(OS_DirectoryRead), 1, OS_ERROR);
        CF_CFDP_ProcessPlaybackDirectory(chan.unwrap(), &mut pb);
        UtAssert_STUB_COUNT(OS_DirectoryClose, 1);
        assert!(!pb.busy);
        assert!(!pb.diropen);

        pb.busy = true;
        pb.diropen = true;
        pb.num_ts = 0;
        dirent[0].FileName[..1].copy_from_slice(b".");
        dirent[1].FileName[..2].copy_from_slice(b"..");
        dirent[2].FileName[..2].copy_from_slice(b"ut");
        OS_DirectoryOpen(&mut pb.dir_id, "ut".as_ptr() as *const i8);
        UT_SetDataBuffer(UT_KEY(OS_DirectoryRead), dirent.as_ptr() as *const std::ffi::c_void, std::mem::size_of_val(&dirent), false);
        UT_SetDeferredRetcode(UT_KEY(OS_DirectoryRead), 4, OS_ERROR);
        CF_CFDP_ProcessPlaybackDirectory(chan.unwrap(), &mut pb);
        assert!(pb.busy);
        assert!(pb.diropen);
        assert_eq!(pb.num_ts, 0);
        UtAssert_STRINGBUF_EQ(pb.pending_file.as_ptr(), pb.pending_file.len(), "ut".as_ptr() as *const i8, -1);

        UT_SetHandlerFunction(UT_KEY(CF_FindUnusedTransaction), UT_AltHandler_GenericPointerReturn, txn.unwrap());
        CF_CFDP_ProcessPlaybackDirectory(chan.unwrap(), &mut pb);
        assert!(pb.busy);
        assert!(!pb.diropen);
        assert_eq!(pb.num_ts, 1);
        UtAssert_STRINGBUF_EQ(history.unwrap().fnames.src_filename.as_ptr(), history.unwrap().fnames.src_filename.len(), "/ut".as_ptr() as *const i8, -1);
        UtAssert_STRINGBUF_EQ(history.unwrap().fnames.dst_filename.as_ptr(), history.unwrap().fnames.dst_filename.len(), "/ut".as_ptr() as *const i8, -1);
        UtAssert_STRINGBUF_EQ(pb.pending_file.as_ptr(), pb.pending_file.len(), "".as_ptr() as *const i8, -1);
        UT_CF_AssertEventID(CF_CFDP_S_START_SEND_INF_EID);
    }

    fn ut_hook_tick_transactions_set_blocked(_user_obj: *mut std::ffi::c_void, stub_retcode: i32, call_count: u32, context: *const UT_StubContext_t) -> i32 {
        if call_count != 0 {
            let args = UT_Hook_GetArgValueByName(context, "context".as_ptr() as *const i8) as *mut CF_CFDP_Tick_args_t;
            unsafe {
                (*args).chan.tx_blocked = true;
            }
        }
        stub_retcode
    }

    fn ut_hook_tick_transactions_update_count(_user_obj: *mut std::ffi::c_void, stub_retcode: i32, call_count: u32, _context: *const UT_StubContext_t) -> i32 {
        if call_count < 5 {
            unsafe {
                CF_AppData.engine.channels[UT_CFDP_CHANNEL].outgoing_counter += 1;
            }
        }
        stub_retcode
    }

    #[test]
    fn test_cf_cfdp_tick_transactions() {
        let mut chan = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut chan), None, None, None);
        CF_CFDP_TickTransactions(chan.unwrap());
        UtAssert_STUB_COUNT(CF_CList_Traverse, 4);

        UT_ResetState(UT_KEY(CF_CList_Traverse));
        UT_SetHookFunction(UT_KEY(CF_CList_Traverse), ut_hook_tick_transactions_set_blocked, std::ptr::null_mut());
        CF_CFDP_TickTransactions(chan.unwrap());
        UtAssert_STUB_COUNT(CF_CList_Traverse, 2);

        UT_ResetState(UT_KEY(CF_CList_Traverse));
        UT_SetHookFunction(UT_KEY(CF_CList_Traverse), ut_hook_tick_transactions_update_count, std::ptr::null_mut());
        chan.unwrap().tx_blocked = false;
        CF_CFDP_TickTransactions(chan.unwrap());
        UtAssert_STUB_COUNT(CF_CList_Traverse, 7);
    }

    #[test]
    fn test_cf_cfdp_cycle_engine() {
        let mut chan = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), None, None, None);
        CF_CFDP_CycleEngine();

        unsafe {
            CF_AppData.engine.enabled = true;
            CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].frozen = 1;
        }
        CF_CFDP_CycleEngine();

        unsafe {
            CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].frozen = 0;
        }
        CF_CFDP_CycleEngine();
    }

    #[test]
    fn test_cf_cfdp_finish_transaction() {
        let mut txn = None;
        let mut history = None;
        let mut chan = None;
        let mut pb = CF_Playbook_t::default();

        unsafe {
            CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].q_size[CF_QueueIdx_TX] = 10;
        }

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        txn.unwrap().flags.com.q_index = CF_QueueIdx_FREE;
        CF_CFDP_FinishTransaction(txn.unwrap(), false);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        UT_SetDefaultReturnValue(UT_KEY(CF_CFDP_GetAckTxnStatus), CF_CFDP_AckTxnStatus_ACTIVE);
        txn.unwrap().flags.com.q_index = CF_QueueIdx_TX;
        CF_CFDP_FinishTransaction(txn.unwrap(), true);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, Some(&mut history), Some(&mut txn), None);
        txn.unwrap().flags.com.q_index = CF_QueueIdx_TX;
        txn.unwrap().fd = OS_ObjectIdFromInteger(1);
        history.unwrap().dir = CF_Direction_TX;
        txn.unwrap().state = CF_TxnState_S1;
        txn.unwrap().flags.com.keep_history = true;
        CF_CFDP_FinishTransaction(txn.unwrap(), false);
        assert!(!txn.unwrap().flags.com.keep_history);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, None, None, Some(&mut history), Some(&mut txn), None);
        txn.unwrap().flags.com.q_index = CF_QueueIdx_RX;
        txn.unwrap().fd = OS_ObjectIdFromInteger(1);
        history.unwrap().dir = CF_Direction_RX;
        txn.unwrap().state = CF_TxnState_R1;
        txn.unwrap().flags.com.keep_history = false;
        CF_CFDP_FinishTransaction(txn.unwrap(), true);
        assert!(txn.unwrap().flags.com.keep_history);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        txn.unwrap().fd = OS_ObjectIdFromInteger(1);
        history.unwrap().dir = CF_Direction_TX;
        txn.unwrap().keep = 1;
        txn.unwrap().state = CF_TxnState_S1;
        txn.unwrap().flags.com.keep_history = false;
        CF_CFDP_FinishTransaction(txn.unwrap(), true);
        assert!(txn.unwrap().flags.com.keep_history);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, Some(&mut chan), Some(&mut history), Some(&mut txn), None);
        pb.num_ts = 10;
        txn.unwrap().pb = &mut pb;
        txn.unwrap().flags.tx.cmd_tx = 5;
        chan.unwrap().num_cmd_tx = 8;
        history.unwrap().dir = CF_Direction_TX;
        txn.unwrap().state = CF_TxnState_S1;
        CF_CFDP_FinishTransaction(txn.unwrap(), true);
        assert_eq!(pb.num_ts, 9);
        assert_eq!(chan.unwrap().num_cmd_tx, 7);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), Some(&mut history), Some(&mut txn), None);
        txn.unwrap().history = std::ptr::null_mut();
        txn.unwrap().state = CF_TxnState_S1;
        CF_CFDP_FinishTransaction(txn.unwrap(), true);
    }

    #[test]
    fn test_cf_cfdp_recycle_transaction() {
        let mut txn = None;
        let mut cl = CF_ChunkWrapper_t::default();

        unsafe {
            CF_AppData.hk.Payload.channel_hk[UT_CFDP_CHANNEL].q_size[CF_QueueIdx_TX] = 10;
        }

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        txn.unwrap().flags.com.q_index = CF_QueueIdx_TX;
        txn.unwrap().flags.com.keep_history = false;
        CF_CFDP_RecycleTransaction(txn.unwrap());
        UtAssert_STUB_COUNT(CF_WrappedClose, 0);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        txn.unwrap().flags.com.q_index = CF_QueueIdx_TX;
        txn.unwrap().flags.com.keep_history = true;
        OS_OpenCreate(&mut txn.unwrap().fd, "ut".as_ptr() as *const i8, 0, 0);
        CF_CFDP_RecycleTransaction(txn.unwrap());
        UtAssert_STUB_COUNT(CF_WrappedClose, 1);
        assert!(!OS_ObjectIdDefined(txn.unwrap().fd));

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        txn.unwrap().flags.com.q_index = CF_QueueIdx_TX;
        txn.unwrap().chunks = &mut cl;
        CF_CFDP_RecycleTransaction(txn.unwrap());
        assert!(txn.unwrap().chunks.is_null());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        UT_ResetState(UT_KEY(CF_GetChunkListHead));
        txn.unwrap().flags.com.q_index = CF_QueueIdx_TX;
        txn.unwrap().chunks = &mut cl;
        CF_CFDP_RecycleTransaction(txn.unwrap());
        assert_eq!(txn.unwrap().chunks, &mut cl);

        CF_CFDP_RecycleTransaction(txn.unwrap());

        UT_ResetState(UT_KEY(CF_GetChannelFromTxn));
        CF_CFDP_RecycleTransaction(txn.unwrap());
    }

    #[test]
    fn test_cf_cfdp_set_txn_status() {
        let mut txn = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        CF_CFDP_SetTxnStatus(txn.unwrap(), CF_TxnStatus_NO_ERROR);
        assert_eq!(txn.unwrap().history.txn_stat, CF_TxnStatus_NO_ERROR);

        CF_CFDP_SetTxnStatus(txn.unwrap(), CF_TxnStatus_FILESTORE_REJECTION);
        assert_eq!(txn.unwrap().history.txn_stat, CF_TxnStatus_FILESTORE_REJECTION);

        CF_CFDP_SetTxnStatus(txn.unwrap(), CF_TxnStatus_NO_ERROR);
        assert_eq!(txn.unwrap().history.txn_stat, CF_TxnStatus_FILESTORE_REJECTION);
    }

    #[test]
    fn test_cf_cfdp_send_eot_pkt() {
        let mut pkt_buf = CF_EotPacket_t::default();
        let mut txn = None;
        let mut pb = CF_Playbook_t::default();

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);

        CF_CFDP_SendEotPkt(txn.unwrap());

        UtAssert_STUB_COUNT(CFE_MSG_Init, 0);
        UtAssert_STUB_COUNT(CFE_SB_TimeStampMsg, 0);
        UtAssert_STUB_COUNT(CFE_SB_TransmitBuffer, 0);

        let pkt_buf_ptr = &mut pkt_buf;
        *pkt_buf_ptr = CF_EotPacket_t::default();
        UT_SetDataBuffer(UT_KEY(CFE_SB_AllocateMessageBuffer), &pkt_buf_ptr as *const _ as *const std::ffi::c_void, std::mem::size_of::<*const CF_EotPacket_t>(), true);

        CF_CFDP_SendEotPkt(txn.unwrap());

        UtAssert_STUB_COUNT(CFE_MSG_Init, 1);
        UtAssert_STUB_COUNT(CFE_SB_TimeStampMsg, 1);
        UtAssert_STUB_COUNT(CFE_SB_TransmitBuffer, 1);
    }

    #[test]
    fn test_cf_cfdp_disable_engine() {
        unsafe {
            CF_AppData.engine.enabled = true;
        }
        CF_CFDP_DisableEngine();
        UtAssert_STUB_COUNT(CFE_SB_DeletePipe, CF_NUM_CHANNELS);
        unsafe {
            assert!(!CF_AppData.engine.enabled);
        }

        unsafe {
            CF_AppData.engine.channels[UT_CFDP_CHANNEL].playbook[0].busy = true;
        }
        OS_DirectoryOpen(&mut unsafe { CF_AppData.engine.channels[UT_CFDP_CHANNEL].playbook[0].dir_id }, "ut".as_ptr() as *const i8);
        unsafe {
            CF_AppData.engine.channels[UT_CFDP_CHANNEL].poll[0].pb.busy = true;
        }
        OS_DirectoryOpen(&mut unsafe { CF_AppData.engine.channels[UT_CFDP_CHANNEL].poll[0].pb.dir_id }, "ut".as_ptr() as *const i8);
        CF_CFDP_DisableEngine();
        UtAssert_STUB_COUNT(OS_DirectoryClose, 2);
    }

    #[test]
    fn test_cf_cfdp_close_files() {
        let mut txn = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        assert_eq!(CF_CFDP_CloseFiles(&mut txn.unwrap().cl_node, std::ptr::null_mut()), CF_CLIST_CONT);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        txn.unwrap().fd = OS_ObjectIdFromInteger(1);
        assert_eq!(CF_CFDP_CloseFiles(&mut txn.unwrap().cl_node, std::ptr::null_mut()), CF_CLIST_CONT);
    }

    #[test]
    fn test_cf_cfdp_cancel_transaction() {
        let mut txn = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        txn.unwrap().history.dir = CF_Direction_TX;
        txn.unwrap().flags.com.canceled = true;
        CF_CFDP_CancelTransaction(txn.unwrap());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_TX, None, None, None, Some(&mut txn), None);
        txn.unwrap().history.dir = CF_Direction_TX;
        txn.unwrap().flags.com.canceled = false;
        CF_CFDP_CancelTransaction(txn.unwrap());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, None, None, None, Some(&mut txn), None);
        txn.unwrap().history.dir = CF_Direction_RX;
        txn.unwrap().flags.com.canceled = false;
        CF_CFDP_CancelTransaction(txn.unwrap());

        txn.unwrap().history.dir = CF_Direction_NUM;
        txn.unwrap().flags.com.canceled = false;
        CF_CFDP_CancelTransaction(txn.unwrap());
    }

    #[test]
    fn test_cf_cfdp_arm_inact_timer() {
        let mut txn = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), None);
        CF_CFDP_ArmAckTimer(txn.unwrap());
        assert!(txn.unwrap().flags.com.ack_timer_armed);
    }

    #[test]
    fn test_cf_cfdp_check_ack_nak_count() {
        let mut txn = None;
        let mut config = None;
        let mut counter: u8;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), Some(&mut config));
        config.unwrap().chan[txn.unwrap().chan_num].ack_limit = 10;

        counter = 9;
        assert!(CF_CFDP_CheckAckNakCount(txn.unwrap(), &mut counter));
        assert_eq!(counter, 10);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), Some(&mut config));
        counter = config.unwrap().chan[txn.unwrap().chan_num].ack_limit;
        txn.unwrap().history.dir = CF_Direction_RX;
        assert!(!CF_CFDP_CheckAckNakCount(txn.unwrap(), &mut counter));
        assert_eq!(counter, config.unwrap().chan[txn.unwrap().chan_num].ack_limit);
        UT_CF_AssertEventID(CF_CFDP_R_ACK_LIMIT_ERR_EID);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, None, None, Some(&mut txn), Some(&mut config));
        counter = config.unwrap().chan[txn.unwrap().chan_num].ack_limit;
        txn.unwrap().history.dir = CF_Direction_TX;
        assert!(!CF_CFDP_CheckAckNakCount(txn.unwrap(), &mut counter));
        assert_eq!(counter, config.unwrap().chan[txn.unwrap().chan_num].ack_limit);
        UT_CF_AssertEventID(CF_CFDP_S_ACK_LIMIT_ERR_EID);
    }

    #[test]
    fn test_cf_cfdp_dispatch_recv() {
        // Test case placeholder
    }

    #[test]
    fn test_cf_cfdp_alloc_chunk_list() {
        let mut txn = CF_Transaction_t::default();
        let mut hist = CF_History_t::default();
        let mut list_node = CF_ChunkWrapper_t::default();
        let mut list_ptr: Option<&mut CF_CListNode_t> = None;

        UT_SetHandlerFunction(UT_KEY(CF_GetChunkListHead), UT_AltHandler_GenericPointerReturn, &list_ptr as *const _ as *mut std::ffi::c_void);
        UT_SetHandlerFunction(UT_KEY(CF_CList_Pop), UT_AltHandler_GenericPointerReturn, &mut list_node.cl_node);

        txn.history = &mut hist;
        hist.dir = CF_Direction_TX;

        CF_CFDP_AllocChunkList(&mut txn);

        assert!(txn.chunks.is_null());
        UT_CF_AssertEventID(CF_CFDP_NO_CHUNKLIST_AVAIL_EID);
        assert_eq!(hist.txn_stat, CF_TxnStatus_NO_RESOURCE);

        list_ptr = Some(&mut list_node.cl_node);
        CF_CFDP_AllocChunkList(&mut txn);
        assert_eq!(txn.chunks, &mut list_node);
    }

    #[test]
    fn test_cf_cfdp_setup_tx_transaction() {
        let mut txn = CF_Transaction_t::default();
        let mut hist = CF_History_t::default();
        let mut config = CF_ConfigTable_t::default();
        let mut list_ptr: Option<&mut CF_CListNode_t> = None;
        let mut chunks = CF_ChunkWrapper_t::default();

        unsafe {
            CF_AppData.config_table = &mut config;
        }
        UT_SetHandlerFunction(UT_KEY(CF_GetChunkListHead), UT_AltHandler_GenericPointerReturn, &list_ptr as *const _ as *mut std::ffi::c_void);

        txn.history = &mut hist;

        CF_CFDP_SetTxnStatus(&mut txn, CF_TxnStatus_PROTOCOL_ERROR);
        unsafe {
            CF_AppData.hk.Payload.channel_hk[txn.chan_num].q_size[txn.flags.com.q_index] = 1;
        }
        CF_CFDP_SetupTxTransaction(&mut txn);
        UtAssert_STUB_COUNT(CF_CFDP_S_Init, 0);

        txn = CF_Transaction_t::default();
        hist = CF_History_t::default();
        txn.chunks = &mut chunks;
        txn.history = &mut hist;

        unsafe {
            CF_AppData.hk.Payload.channel_hk[txn.chan_num].q_size[txn.flags.com.q_index] = 1;
        }
        CF_CFDP_SetupTxTransaction(&mut txn);
        UtAssert_STUB_COUNT(CF_CFDP_S_Init, 1);
    }

    #[test]
    fn test_cf_cfdp_txn_status() {
        let mut txn = CF_Transaction_t::default();

        CF_CFDP_GetTxnStatus(&txn);
        CF_CFDP_SetTxnStatus(&mut txn, CF_TxnStatus_ACK_LIMIT_NO_EOF);
    }

    #[test]
    fn test_cf_cfdp_setup_rx_transaction() {
        let mut txn = CF_Transaction_t::default();
        let mut ph = CF_Logical_PduBuffer_t::default();
        let mut hist = CF_History_t::default();
        let mut chunks = CF_ChunkWrapper_t::default();
        let mut config = CF_ConfigTable_t::default();
        let mut list_ptr: Option<&mut CF_CListNode_t> = None;

        unsafe {
            CF_AppData.config_table = &mut config;
        }
        UT_SetHandlerFunction(UT_KEY(CF_GetChunkListHead), UT_AltHandler_GenericPointerReturn, &list_ptr as *const _ as *mut std::ffi::c_void);

        txn.history = &mut hist;

        CF_CFDP_SetTxnStatus(&mut txn, CF_TxnStatus_PROTOCOL_ERROR);
        unsafe {
            CF_AppData.hk.Payload.channel_hk[txn.chan_num].q_size[txn.flags.com.q_index] = 1;
        }
        CF_CFDP_SetupRxTransaction(&mut txn, &mut ph);
        assert!(txn.chunks.is_null());
        assert_eq!(txn.state, CF_TxnState_HOLD);

        txn = CF_Transaction_t::default();
        hist = CF_History_t::default();
        txn.history = &mut hist;
        txn.chunks = &mut chunks;

        unsafe {
            CF_AppData.hk.Payload.channel_hk[txn.chan_num].q_size[txn.flags.com.q_index] = 1;
        }
        ph.pdu_header.txm_mode = 1;
        CF_CFDP_SetupRxTransaction(&mut txn, &mut ph);
        assert_eq!(txn.state, CF_TxnState_R1);
        UtAssert_STUB_COUNT(CF_CFDP_R_Init, 1);

        txn = CF_Transaction_t::default();
        hist = CF_History_t::default();
        txn.history = &mut hist;
        txn.chunks = &mut chunks;

        unsafe {
            CF_AppData.hk.Payload.channel_hk[txn.chan_num].q_size[txn.flags.com.q_index] = 1;
        }
        ph.pdu_header.txm_mode = 0;
        CF_CFDP_SetupRxTransaction(&mut txn, &mut ph);
        assert_eq!(txn.state, CF_TxnState_R2);
        UtAssert_STUB_COUNT(CF_CFDP_R_Init, 2);
    }

    #[test]
    fn test_cf_cfdp_receive_pdu() {
        let mut chan = None;
        let mut ph = None;
        let mut txn = None;
        let mut config = None;
        let mut chunk_wrap = CF_ChunkWrapper_t::default();

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), Some(&mut chan), None, Some(&mut txn), Some(&mut config));
        txn.unwrap().state = CF_TxnState_R2;
        UT_SetHandlerFunction(UT_KEY(CF_FindTransactionBySequenceNumber), UT_AltHandler_GenericPointerReturn, txn.unwrap());
        CF_CFDP_ReceivePdu(chan.unwrap(), ph.unwrap());
        UtAssert_STUB_COUNT(CF_CFDP_RxStateDispatch, 1);
        UT_ResetState(UT_KEY(CF_FindTransactionBySequenceNumber));

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), Some(&mut chan), None, Some(&mut txn), Some(&mut config));
        txn.unwrap().state = CF_TxnState_R2;
        config.unwrap().local_eid = 123;
        ph.unwrap().pdu_header.destination_eid = !config.unwrap().local_eid;
        CF_CFDP_ReceivePdu(chan.unwrap(), ph.unwrap());
        UT_CF_AssertEventID(CF_CFDP_INVALID_DST_ERR_EID);

        UT_ResetState(UT_KEY(CF_CFDP_R_Init));
        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), Some(&mut chan), None, Some(&mut txn), Some(&mut config));
        UT_SetHandlerFunction(UT_KEY(CF_FindUnusedTransaction), UT_AltHandler_GenericPointerReturn, txn.unwrap());
        txn.unwrap().chunks = &mut chunk_wrap;
        txn.unwrap().state = CF_TxnState_R1;
        config.unwrap().local_eid = 123;
        ph.unwrap().pdu_header.destination_eid = config.unwrap().local_eid;
        CF_CFDP_ReceivePdu(chan.unwrap(), ph.unwrap());
        UtAssert_STUB_COUNT(CF_CFDP_R_Init, 1);

        UT_ResetState(UT_KEY(CF_FindUnusedTransaction));
        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), Some(&mut chan), None, Some(&mut txn), Some(&mut config));
        txn.unwrap().state = CF_TxnState_R1;
        config.unwrap().local_eid = 123;
        ph.unwrap().pdu_header.destination_eid = config.unwrap().local_eid;
        CF_CFDP_ReceivePdu(chan.unwrap(), ph.unwrap());
        UT_CF_AssertEventID(CF_CFDP_RX_DROPPED_ERR_EID);

        UT_ResetState(UT_KEY(CF_FindTransactionBySequenceNumber));
        ut_cfdp_setup_basic_test_state(UT_CF_Setup_RX, Some(&mut ph), Some(&mut chan), None, Some(&mut txn), Some(&mut config));
        UT_SetDeferredRetcode(UT_KEY(CF_CFDP_DecodeHeader), 1, -1);
        CF_CFDP_ReceivePdu(chan.unwrap(), ph.unwrap());
        UtAssert_STUB_COUNT(CF_FindTransactionBySequenceNumber, 0);
    }

    #[test]
    fn test_cf_cfdp_start_first_pending() {
        let mut chan = None;
        let mut txn = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), None, Some(&mut txn), None);

        assert!(!CF_CFDP_StartFirstPending(chan.unwrap()));

        txn.unwrap().flags.com.q_index = CF_QueueIdx_PEND;
        chan.unwrap().qs[CF_QueueIdx_PEND] = &mut txn.unwrap().cl_node;

        unsafe {
            CF_AppData.hk.Payload.channel_hk[txn.unwrap().chan_num].q_size[txn.unwrap().flags.com.q_index] = 1;
        }

        assert!(CF_CFDP_StartFirstPending(chan.unwrap()));
    }

    #[test]
    fn test_cf_cfdp_complete_tick() {
        let mut txn = None;
        let mut txn2 = CF_Transaction_t::default();
        let mut chan = None;

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), None, Some(&mut txn), None);
        UT_SetHandlerFunction(UT_KEY(CF_GetChannelFromTxn), UT_AltHandler_GenericPointerReturn, chan.unwrap());
        chan.unwrap().tx_blocked = true;
        chan.unwrap().tick_resume = std::ptr::null_mut();
        CF_CFDP_CompleteTick(txn.unwrap());
        assert_eq!(chan.unwrap().tick_resume, txn.unwrap());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), None, Some(&mut txn), None);
        UT_SetHandlerFunction(UT_KEY(CF_GetChannelFromTxn), UT_AltHandler_GenericPointerReturn, chan.unwrap());
        chan.unwrap().tx_blocked = true;
        chan.unwrap().tick_resume = &mut txn2;
        CF_CFDP_CompleteTick(txn.unwrap());
        assert_eq!(chan.unwrap().tick_resume, &mut txn2);

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), None, Some(&mut txn), None);
        UT_SetHandlerFunction(UT_KEY(CF_GetChannelFromTxn), UT_AltHandler_GenericPointerReturn, chan.unwrap());
        chan.unwrap().tx_blocked = false;
        chan.unwrap().tick_resume = std::ptr::null_mut();
        CF_CFDP_CompleteTick(txn.unwrap());
        assert!(chan.unwrap().tick_resume.is_null());

        ut_cfdp_setup_basic_test_state(UT_CF_Setup_NONE, None, Some(&mut chan), None, Some(&mut txn), None);
        UT_ResetState(UT_KEY(CF_GetChannelFromTxn));
        chan.unwrap().tx_blocked = false;
        chan.unwrap().tick_resume = std::ptr::null_mut();
        CF_CFDP_CompleteTick(txn.unwrap());
        assert!(chan.unwrap().tick_resume.is_null());
    }

    #[test]
    fn test_cf_cfdp_get_temp_name() {
        let mut hist = CF_History_t::default();
        let mut config = CF_ConfigTable_t::default();
        let mut file_name_buf = [0i8; 12];

        unsafe {
            CF_AppData.config_table = &mut config;
        }
        config.tmp_dir[..2].copy_from_slice(b"ut");
        hist.src_eid = 3;
        hist.seq_num = 4;
        CF_CFDP_GetTempName(&hist, file_name_buf.as_mut_ptr(), file_name_buf.len());
        UtAssert_STRINGBUF_EQ(file_name_buf.as_ptr(), file_name_buf.len(), "ut/3_4.tmp".as_ptr() as *const i8, -1);
    }

    #[test]
    fn test_cf_cfdp_get_move_target() {
        let mut file_name_buf = [0i8; 6];

        assert!(CF_CFDP_GetMoveTarget(std::ptr::null(), "ut".as_ptr() as *const i8, file_name_buf.as_mut_ptr(), file_name_buf.len()).is_null());
        assert!(CF_CFDP_GetMoveTarget("".as_ptr() as *const i8, "ut".as_ptr() as *const i8, file_name_buf.as_mut_ptr(), file_name_buf.len()).is_null());

        assert!(!CF_CFDP_GetMoveTarget("d".as_ptr() as *const i8, "ut".as_ptr() as *const i8, file_name_buf.as_mut_ptr(), file_name_buf.len()).is_null());
        UtAssert_STRINGBUF_EQ(file_name_buf.as_ptr(), file_name_buf.len(), "d/ut".as_ptr() as *const i8, -1);

        assert!(!CF_CFDP_GetMoveTarget("d".as_ptr() as *const i8, "b/ut".as_ptr() as *const i8, file_name_buf.as_mut_ptr(), file_name_buf.len()).is_null());
        UtAssert_STRINGBUF_EQ(file_name_buf.as_ptr(), file_name_buf.len(), "d/ut".as_ptr() as *const i8, -1);

        assert!(!CF_CFDP_GetMoveTarget("d".as_ptr() as *const i8, "longname".as_ptr() as *const i8, file_name_buf.as_mut_ptr(), file_name_buf.len()).is_null());
        UtAssert_STRINGBUF_EQ(file_name_buf.as_ptr(), file_name_buf.len(), "d/lo$".as_ptr() as *const i8, -1);

        assert!(!CF_CFDP_GetMoveTarget("d".as_ptr() as *const i8, "longname".as_ptr() as *const i8, file_name_buf.as_mut_ptr(), 2).is_null());
        UtAssert_STRINGBUF_EQ(file_name_buf.as_ptr(), file_name_buf.len(), "d".as_ptr() as *const i8, -1);
    }
}