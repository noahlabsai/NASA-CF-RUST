//! CF Application event ID definitions.
//!
//! Translated from: cf_events.h
//!
//! All event IDs used by the CF application.

// Init events
pub const CF_INIT_INF_EID: u16 = 1;
pub const CF_INIT_TBL_REG_ERR_EID: u16 = 2;
pub const CF_INIT_TBL_LOAD_ERR_EID: u16 = 3;
pub const CF_INIT_TBL_MANAGE_ERR_EID: u16 = 4;
pub const CF_INIT_TBL_GETADDR_ERR_EID: u16 = 5;
pub const CF_INIT_TBL_CHECK_REL_ERR_EID: u16 = 6;
pub const CF_INIT_TBL_CHECK_MAN_ERR_EID: u16 = 7;
pub const CF_INIT_TBL_CHECK_GA_ERR_EID: u16 = 8;
pub const CF_INIT_MSG_RECV_ERR_EID: u16 = 9;
pub const CF_INIT_SEM_ERR_EID: u16 = 10;
pub const CF_INIT_TPS_ERR_EID: u16 = 11;
pub const CF_INIT_CRC_ALIGN_ERR_EID: u16 = 12;
pub const CF_INIT_OUTGOING_SIZE_ERR_EID: u16 = 13;
pub const CF_INIT_SUB_ERR_EID: u16 = 14;

// Pipe events
pub const CF_CR_PIPE_ERR_EID: u16 = 20;
pub const CF_CR_CHANNEL_PIPE_ERR_EID: u16 = 21;

// Command events
pub const CF_NOOP_INF_EID: u16 = 30;
pub const CF_RESET_INF_EID: u16 = 31;
pub const CF_RESET_FREED_XACT_DBG_EID: u16 = 32;
pub const CF_CMD_LEN_ERR_EID: u16 = 33;
pub const CF_CC_ERR_EID: u16 = 34;
pub const CF_MID_ERR_EID: u16 = 35;
pub const CF_CMD_TX_FILE_INF_EID: u16 = 36;
pub const CF_CMD_TX_FILE_ERR_EID: u16 = 37;
pub const CF_CMD_PLAYBACK_DIR_INF_EID: u16 = 38;
pub const CF_CMD_PLAYBACK_DIR_ERR_EID: u16 = 39;
pub const CF_CMD_FREEZE_INF_EID: u16 = 40;
pub const CF_CMD_FREEZE_ERR_EID: u16 = 41;
pub const CF_CMD_THAW_INF_EID: u16 = 42;
pub const CF_CMD_THAW_ERR_EID: u16 = 43;
pub const CF_CMD_SUSPRES_INF_EID: u16 = 44;
pub const CF_CMD_SUSPRES_SAME_INF_EID: u16 = 45;
pub const CF_CMD_SUSPRES_CHAN_ERR_EID: u16 = 46;
pub const CF_CMD_CANCEL_INF_EID: u16 = 47;
pub const CF_CMD_CANCEL_CHAN_ERR_EID: u16 = 48;
pub const CF_CMD_ABANDON_INF_EID: u16 = 49;
pub const CF_CMD_ABANDON_CHAN_ERR_EID: u16 = 50;
pub const CF_CMD_ENABLE_DEQUEUE_INF_EID: u16 = 51;
pub const CF_CMD_ENABLE_DEQUEUE_ERR_EID: u16 = 52;
pub const CF_CMD_DISABLE_DEQUEUE_INF_EID: u16 = 53;
pub const CF_CMD_DISABLE_DEQUEUE_ERR_EID: u16 = 54;
pub const CF_CMD_ENABLE_POLLDIR_INF_EID: u16 = 55;
pub const CF_CMD_ENABLE_POLLDIR_ERR_EID: u16 = 56;
pub const CF_CMD_DISABLE_POLLDIR_INF_EID: u16 = 57;
pub const CF_CMD_DISABLE_POLLDIR_ERR_EID: u16 = 58;
pub const CF_CMD_POLLDIR_INVALID_ERR_EID: u16 = 59;
pub const CF_CMD_PURGE_QUEUE_INF_EID: u16 = 60;
pub const CF_CMD_PURGE_QUEUE_ERR_EID: u16 = 61;
pub const CF_CMD_PURGE_ARG_ERR_EID: u16 = 62;
pub const CF_CMD_WQ_INF_EID: u16 = 63;
pub const CF_CMD_WQ_ARGS_ERR_EID: u16 = 64;
pub const CF_CMD_WQ_CHAN_ERR_EID: u16 = 65;
pub const CF_CMD_WQ_OPEN_ERR_EID: u16 = 66;
pub const CF_CMD_WQ_WRITEQ_TX_ERR_EID: u16 = 67;
pub const CF_CMD_WQ_WRITEQ_RX_ERR_EID: u16 = 68;
pub const CF_CMD_WQ_WRITEQ_PEND_ERR_EID: u16 = 69;
pub const CF_CMD_WQ_WRITEHIST_TX_ERR_EID: u16 = 70;
pub const CF_CMD_WQ_WRITEHIST_RX_ERR_EID: u16 = 71;
pub const CF_CMD_WHIST_WRITE_ERR_EID: u16 = 72;
pub const CF_CMD_GETSET_PARAM_ERR_EID: u16 = 73;
pub const CF_CMD_GETSET_CHAN_ERR_EID: u16 = 74;
pub const CF_CMD_GETSET_VALIDATE_ERR_EID: u16 = 75;
pub const CF_CMD_GETSET1_INF_EID: u16 = 76;
pub const CF_CMD_GETSET2_INF_EID: u16 = 77;
pub const CF_CMD_ENABLE_ENGINE_INF_EID: u16 = 78;
pub const CF_CMD_ENABLE_ENGINE_ERR_EID: u16 = 79;
pub const CF_CMD_DISABLE_ENGINE_INF_EID: u16 = 80;
pub const CF_CMD_ENG_ALREADY_ENA_INF_EID: u16 = 81;
pub const CF_CMD_ENG_ALREADY_DIS_INF_EID: u16 = 82;
pub const CF_CMD_CHAN_PARAM_ERR_EID: u16 = 83;
pub const CF_CMD_BAD_PARAM_ERR_EID: u16 = 84;
pub const CF_CMD_TRANS_NOT_FOUND_ERR_EID: u16 = 85;
pub const CF_CMD_TSN_CHAN_INVALID_ERR_EID: u16 = 86;
pub const CF_CMD_RESET_INVALID_ERR_EID: u16 = 87;
pub const CF_CMD_SUSPRES_ERR_EID: u16 = 88;

// CFDP engine events
pub const CF_CFDP_S_START_SEND_INF_EID: u16 = 100;
pub const CF_CFDP_S_OPEN_ERR_EID: u16 = 101;
pub const CF_CFDP_S_ALREADY_OPEN_ERR_EID: u16 = 102;
pub const CF_CFDP_S_SEEK_END_ERR_EID: u16 = 103;
pub const CF_CFDP_S_SEEK_BEG_ERR_EID: u16 = 104;
pub const CF_CFDP_S_SEEK_FD_ERR_EID: u16 = 105;
pub const CF_CFDP_S_READ_ERR_EID: u16 = 106;
pub const CF_CFDP_S_PDU_EOF_ERR_EID: u16 = 107;
pub const CF_CFDP_S_PDU_NAK_ERR_EID: u16 = 108;
pub const CF_CFDP_S_DC_INV_ERR_EID: u16 = 109;
pub const CF_CFDP_S_NON_FD_PDU_ERR_EID: u16 = 110;
pub const CF_CFDP_S_EARLY_FIN_ERR_EID: u16 = 111;
pub const CF_CFDP_S_INVALID_SR_ERR_EID: u16 = 112;
pub const CF_CFDP_S_ACK_LIMIT_ERR_EID: u16 = 113;
pub const CF_CFDP_S_INACT_TIMER_ERR_EID: u16 = 114;
pub const CF_CFDP_S_FILE_MOVED_EID: u16 = 115;
pub const CF_CFDP_S_FILE_REMOVED_EID: u16 = 116;

pub const CF_CFDP_R_CREAT_ERR_EID: u16 = 120;
pub const CF_CFDP_R_SEEK_FD_ERR_EID: u16 = 121;
pub const CF_CFDP_R_WRITE_ERR_EID: u16 = 122;
pub const CF_CFDP_R_SIZE_MISMATCH_ERR_EID: u16 = 123;
pub const CF_CFDP_R_CRC_ERR_EID: u16 = 124;
pub const CF_CFDP_R_PDU_EOF_ERR_EID: u16 = 125;
pub const CF_CFDP_R_PDU_FINACK_ERR_EID: u16 = 126;
pub const CF_CFDP_R_DC_INV_ERR_EID: u16 = 127;
pub const CF_CFDP_R_RENAME_ERR_EID: u16 = 128;
pub const CF_CFDP_R_SEEK_CRC_ERR_EID: u16 = 129;
pub const CF_CFDP_R_READ_ERR_EID: u16 = 130;
pub const CF_CFDP_R_INACT_TIMER_ERR_EID: u16 = 131;
pub const CF_CFDP_R_ACK_LIMIT_ERR_EID: u16 = 132;
pub const CF_CFDP_R_REQUEST_MD_INF_EID: u16 = 133;
pub const CF_CFDP_R_TEMP_FILE_INF_EID: u16 = 134;
pub const CF_CFDP_R_FILE_RETAINED_EID: u16 = 135;
pub const CF_CFDP_R_NOT_RETAINED_EID: u16 = 136;

pub const CF_CFDP_CLOSE_ERR_EID: u16 = 140;
pub const CF_CFDP_DIR_SLOT_ERR_EID: u16 = 141;
pub const CF_CFDP_INVALID_DST_ERR_EID: u16 = 142;
pub const CF_CFDP_MAX_CMD_TX_ERR_EID: u16 = 143;
pub const CF_CFDP_NO_CHUNKLIST_AVAIL_EID: u16 = 144;
pub const CF_CFDP_NO_MSG_ERR_EID: u16 = 145;
pub const CF_CFDP_OPENDIR_ERR_EID: u16 = 146;
pub const CF_CFDP_RX_DROPPED_ERR_EID: u16 = 147;

// PDU events
pub const CF_PDU_SHORT_HEADER_ERR_EID: u16 = 150;
pub const CF_PDU_TRUNCATION_ERR_EID: u16 = 151;
pub const CF_PDU_MD_SHORT_ERR_EID: u16 = 152;
pub const CF_PDU_MD_RECVD_INF_EID: u16 = 153;
pub const CF_PDU_FD_SHORT_ERR_EID: u16 = 154;
pub const CF_PDU_FD_UNSUPPORTED_ERR_EID: u16 = 155;
pub const CF_PDU_EOF_SHORT_ERR_EID: u16 = 156;
pub const CF_PDU_ACK_SHORT_ERR_EID: u16 = 157;
pub const CF_PDU_FIN_SHORT_ERR_EID: u16 = 158;
pub const CF_PDU_NAK_SHORT_ERR_EID: u16 = 159;
pub const CF_PDU_INVALID_SRC_LEN_ERR_EID: u16 = 160;
pub const CF_PDU_INVALID_DST_LEN_ERR_EID: u16 = 161;
pub const CF_PDU_LARGE_FILE_ERR_EID: u16 = 162;
