/************************************************************************
 * NASA Docket No. GSC-18,447-1, and identified as "CFS CFDP (CF)
 * Application version 3.0.0"
 *
 * Copyright (c) 2019 United States Government as represented by the
 * Administrator of the National Aeronautics and Space Administration.
 * All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License. You may obtain
 * a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 ************************************************************************/

/**
 * @file
 *
 *  The CF Application event id definition header file
 */

/**************************************************************************
 * CF_INIT event IDs - Initialization
 */

/**
 * \brief CF Initialization Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Successful completion of application initialization
 */
pub const CF_INIT_INF_EID: i32 = 20;

/**
 * \brief CF Check Table Release Address Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from release table address call during periodic table check
 */
pub const CF_INIT_TBL_CHECK_REL_ERR_EID: i32 = 21;

/**
 * \brief CF Check Table Manage Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from manage table call during periodic table check
 */
pub const CF_INIT_TBL_CHECK_MAN_ERR_EID: i32 = 22;

/**
 * \brief CF Check Table Get Address Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from get table call during periodic table check
 */
pub const CF_INIT_TBL_CHECK_GA_ERR_EID: i32 = 23;

/**
 * \brief CF Table Registration At Initialization Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from table register call during application initialization
 */
pub const CF_INIT_TBL_REG_ERR_EID: i32 = 24;

/**
 * \brief CF Table Load At Initialization Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from table load call during application initialization
 */
pub const CF_INIT_TBL_LOAD_ERR_EID: i32 = 25;

/**
 * \brief CF Table Manage At Initialization Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from table manage call during application initialization
 */
pub const CF_INIT_TBL_MANAGE_ERR_EID: i32 = 26;

/**
 * \brief CF Table Get Address At Initialization Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from table get address call during application initialization
 */
pub const CF_INIT_TBL_GETADDR_ERR_EID: i32 = 27;

/**
 * \brief CF Message ID Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Invalid message ID received on the software bus pipe
 */
pub const CF_MID_ERR_EID: i32 = 28;

/**
 * \brief CF SB Receive Buffer Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from SB Receive Buffer call in application run loop
 */
pub const CF_INIT_MSG_RECV_ERR_EID: i32 = 29;

/**
 * \brief CF Channel Semaphore Initialization Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from get semaphore by name call during engine channel initialization,
 *  semaphore needs to exist before engine is initialized.
 */
pub const CF_INIT_SEM_ERR_EID: i32 = 30;

/**
 * \brief CF Channel Create Pipe Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from create pipe call during engine channel initialization
 */
pub const CF_CR_CHANNEL_PIPE_ERR_EID: i32 = 31;

/**
 * \brief CF Channel Message Subscription Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from message subscription call during engine channel initialization
 */
pub const CF_INIT_SUB_ERR_EID: i32 = 32;

/**
 * \brief CF Ticks Per Second Config Table Validation Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Configuration table ticks per second set to zero
 */
pub const CF_INIT_TPS_ERR_EID: i32 = 33;

/**
 * \brief CF CRC Bytes Per Wakeup Config Table Validation Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Configuration table CRC bytes per wakeup not aligned or zero
 */
pub const CF_INIT_CRC_ALIGN_ERR_EID: i32 = 34;

/**
 * \brief CF Outgoing Chunk Size Config Table Validation Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Configuration table outgoing chunk size larger than PDU data size
 */
pub const CF_INIT_OUTGOING_SIZE_ERR_EID: i32 = 35;

/**
 * \brief CF Create SB Command Pipe at Initialization Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from create command pipe call during application initialization
 */
pub const CF_CR_PIPE_ERR_EID: i32 = 36;

/**************************************************************************
 * CF_PDU event IDs - Protocol data unit
 */

/**
 * \brief CF Metadata PDU Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Successful processing of metadata PDU
 */
pub const CF_PDU_MD_RECVD_INF_EID: i32 = 40;

/**
 * \brief CF PDU Header Too Short Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure processing PDU header
 */
pub const CF_PDU_SHORT_HEADER_ERR_EID: i32 = 41;

/**
 * \brief CF Metadata PDU Too Short Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure processing metadata PDU
 */
pub const CF_PDU_MD_SHORT_ERR_EID: i32 = 43;

/**
 * \brief CF Metadata PDU Source Filename Length Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Metadata PDU source filename length exceeds buffer size
 */
pub const CF_PDU_INVALID_SRC_LEN_ERR_EID: i32 = 44;

/**
 * \brief CF Metadata PDU Destination Filename Length Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Metadata PDU destination filename length exceeds buffer size
 */
pub const CF_PDU_INVALID_DST_LEN_ERR_EID: i32 = 45;

/**
 * \brief CF File Data PDU Too Short Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure processing file data PDU
 */
pub const CF_PDU_FD_SHORT_ERR_EID: i32 = 46;

/**
 * \brief CF End-Of-File PDU Too Short Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure processing end-of-file PDU
 */
pub const CF_PDU_EOF_SHORT_ERR_EID: i32 = 47;

/**
 * \brief CF Acknowledgment PDU Too Short Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure processing acknowledgment PDU
 */
pub const CF_PDU_ACK_SHORT_ERR_EID: i32 = 48;

/**
 * \brief CF Finished PDU Too Short Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure processing finished PDU
 */
pub const CF_PDU_FIN_SHORT_ERR_EID: i32 = 49;

/**
 * \brief CF Negative Acknowledgment PDU Too Short Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure processing negative acknowledgment PDU
 */
pub const CF_PDU_NAK_SHORT_ERR_EID: i32 = 50;

/**
 * \brief CF File Data PDU Unsupported Option Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  File Data PDU received with the segment metadata flag set
 */
pub const CF_PDU_FD_UNSUPPORTED_ERR_EID: i32 = 54;

/**
 * \brief CF PDU Header Large File Flag Set Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  PDU Header received with the unsupported large file flag set
 */
pub const CF_PDU_LARGE_FILE_ERR_EID: i32 = 55;

/**
 * \brief CF PDU Header Field Truncation
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  PDU Header received with fields that would be truncated with the cf configuration
 */
pub const CF_PDU_TRUNCATION_ERR_EID: i32 = 56;

/**************************************************************************
 * CF_CFDP event IDs - Engine
 */

/**
 * \brief Attempt to reset a transaction that has already been freed
 *
 *  \par Type: DEBUG
 *
 *  \par Cause:
 *
 *  Can be induced via various off-nominal conditions - such as sending a META-data PDU
 *  with an invalid file destination.
 *
 */
pub const CF_RESET_FREED_XACT_DBG_EID: i32 = 59;

/**
 * \brief CF PDU Received Without Existing Transaction, Dropped Due To Max RX Reached Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  PDU without a matching/existing transaction received when channel receive queue is already
 *  handling the maximum number of concurrent receive transactions
 */
pub const CF_CFDP_RX_DROPPED_ERR_EID: i32 = 60;

/**
 * \brief CF PDU Received With Invalid Destination Entity ID Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  PDU without a matching/existing transaction received with an entity ID that doesn't
 *  match the receiving channel's entity ID
 */
pub const CF_CFDP_INVALID_DST_ERR_EID: i32 = 61;

/**
 * \brief CF Invalid Metadata PDU Received On Idle Transaction Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Metadata PDU received for an idle transaction failed decoding
 */
pub const CF_CFDP_IDLE_MD_ERR_EID: i32 = 62;

/**
 * \brief CF Non-metadata File Directive PDU Received On Idle Transaction Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  File Directive PDU received without the metadata directive code on an idle transaction
 */
pub const CF_CFDP_FD_UNHANDLED_ERR_EID: i32 = 63;

/**
 * \brief CF Transmission Request Rejected Due To Max Commanded TX Reached Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Command request to transmit a file received when channel is already
 *  handling the maximum number of concurrent command transmit transactions
 */
pub const CF_CFDP_MAX_CMD_TX_ERR_EID: i32 = 64;

/**
 * \brief CF Playback/Polling Directory Open Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure opening directory during playback or polling initialization
 */
pub const CF_CFDP_OPENDIR_ERR_EID: i32 = 65;

/**
 * \brief CF Playback Request Rejected Due to Max Playback Directories Reached Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Command request to playback a directory received when channel is already
 *  handling the maximum number of concurrent playback directories
 */
pub const CF_CFDP_DIR_SLOT_ERR_EID: i32 = 66;

/**
 * \brief CF No Message Buffer Available Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from SB allocate message buffer call when constructing PDU
 */
pub const CF_CFDP_NO_MSG_ERR_EID: i32 = 67;

/**
 * \brief CF Close File Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from file close call
 */
pub const CF_CFDP_CLOSE_ERR_EID: i32 = 68;

/**
 * \brief CF No chunklist available
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Engine has aborted a transaction due to lack of an
 *  available resource to track the chunks associated
 *  with the file.
 */
pub const CF_CFDP_NO_CHUNKLIST_AVAIL_EID: i32 = 69;

/**************************************************************************
 * CF_CFDP_R event IDs - Engine receive
 */

/**
 * \brief CF Requesting RX Metadata Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  RX transaction missing metadata which results in a NAK being sent to
 *  request a metadata PDU for the transaction
 */
pub const CF_CFDP_R_REQUEST_MD_INF_EID: i32 = 70;

/**
 * \brief CF Creating Temp File For RX Transaction
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  RX transaction creation of a temporary
 *  filename to store the data
 */
pub const CF_CFDP_R_TEMP_FILE_INF_EID: i32 = 71;

/**
 * \brief CF RX Transaction NAK Limit Reached Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Condition that triggers a NAK occurred that would meet or exceed the NAK limit
 */
pub const CF_CFDP_R_NAK_LIMIT_ERR_EID: i32 = 72;

/**
 * \brief CF RX Transaction ACK Limit Reached Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Condition that triggers an ACK occurred that would meet or exceed the ACK limit
 */
pub const CF_CFDP_R_ACK_LIMIT_ERR_EID: i32 = 73;

/**
 * \brief CF RX Transaction CRC Mismatch Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  RX Transaction final CRC mismatch
 */
pub const CF_CFDP_R_CRC_ERR_EID: i32 = 74;

/**
 * \brief CF RX File Data PDU Seek Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of lseek call when processing out of order file data PDUs
 */
pub const CF_CFDP_R_SEEK_FD_ERR_EID: i32 = 75;

/**
 * \brief CF RX Class 2 CRC Seek Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of lseek call when calculating CRC from the file at
 *  the end of a Class 2 RX transaction
 */
pub const CF_CFDP_R_SEEK_CRC_ERR_EID: i32 = 76;

/**
 * \brief CF RX File Data PDU Write Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 * Failure of write to file call when processing file data PDUs
 */
pub const CF_CFDP_R_WRITE_ERR_EID: i32 = 77;

/**
 * \brief CF RX End-Of-File PDU File Size Mismatch Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  End-of-file PDU file size does not match transaction expected file size
 */
pub const CF_CFDP_R_SIZE_MISMATCH_ERR_EID: i32 = 78;

/**
 * \brief CF Invalid End-Of-File PDU Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  End-of-file PDU failed decoding
 */
pub const CF_CFDP_R_PDU_EOF_ERR_EID: i32 = 79;

/**
 * \brief CF RX Transaction File Create Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure in opencreate file call for an RX transaction
 */
pub const CF_CFDP_R_CREAT_ERR_EID: i32 = 80;

/**
 * \brief CF Class 2 RX Transaction Invalid FIN-ACK PDU Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  ACK PDU failed decoding during Class 2 RX Transaction
 */
pub const CF_CFDP_R_PDU_FINACK_ERR_EID: i32 = 81;

/**
 * \brief CF RX Class 2 Metadata PDU Size Mismatch Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Out-of-order RX Class 2 Metadata PDU received with file size that doesn't
 *  match already received EOF PDU file size
 */
pub const CF_CFDP_R_EOF_MD_SIZE_ERR_EID: i32 = 82;

/**
 * \brief CF RX Class 2 Metadata PDU File Rename Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from file rename call after end of transaction
 */
pub const CF_CFDP_R_RENAME_ERR_EID: i32 = 83;

/**
 * \brief CF File retained
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 * Engine has fully retained the file after a successful transaction
 */
pub const CF_CFDP_R_FILE_RETAINED_EID: i32 = 84;

/**
 * \brief CF RX File not retained
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 * Temporary file associated with a receive transaction was discarded
 * without being retained.  This may be due to an error in the
 * transaction, failure to validate, or cancellation.
 */
pub const CF_CFDP_R_NOT_RETAINED_EID: i32 = 85;

/**
 * \brief CF Class 2 CRC Read From File Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure from file read call during RX Class 2 CRC calculation
 */
pub const CF_CFDP_R_READ_ERR_EID: i32 = 86;

/**
 * \brief CF RX Invalid File Directive PDU Code Received Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Unrecognized file directive PDU directive code received for
 *  a current transaction
 */
pub const CF_CFDP_R_DC_INV_ERR_EID: i32 = 87;

/**
 * \brief CF RX Inactivity Timer Expired Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Expiration of the RX inactivity timer
 */
pub const CF_CFDP_R_INACT_TIMER_ERR_EID: i32 = 88;

/**************************************************************************
 * CF_CFDP_S event IDs - Engine send
 */

/**
 * \brief CF TX Initiated Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  File TX transaction initiated
 */
pub const CF_CFDP_S_START_SEND_INF_EID: i32 = 90;

/**
 * \brief CF TX File Data PDU Seek Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of lseek call when preparing to send file data PDU
 */
pub const CF_CFDP_S_SEEK_FD_ERR_EID: i32 = 91;

/**
 * \brief CF TX File Data PDU Read Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of read file call when preparing to send file data PDU
 */
pub const CF_CFDP_S_READ_ERR_EID: i32 = 92;

/**
 * \brief CF TX File Data PDU Send Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure to send the file data PDU
 */
pub const CF_CFDP_S_SEND_FD_ERR_EID: i32 = 93;

/**
 * \brief CF TX Metadata PDU File Already Open Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure to send metadata PDU due to file already being open
 */
pub const CF_CFDP_S_ALREADY_OPEN_ERR_EID: i32 = 94;

/**
 * \brief CF TX Metadata PDU File Open Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure in file open call when preparing to send metadata PDU
 */
pub const CF_CFDP_S_OPEN_ERR_EID: i32 = 95;

/**
 * \brief CF TX Metadata PDU File Seek End Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure in file lseek to end of file call when preparing
 *  to send metadata PDU
 */
pub const CF_CFDP_S_SEEK_END_ERR_EID: i32 = 96;

/**
 * \brief CF TX Metadata PDU File Seek Beginning Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure in file lseek to beginning of file call when
 *  preparing to send metadata PDU
 */
pub const CF_CFDP_S_SEEK_BEG_ERR_EID: i32 = 97;

/**
 * \brief CF TX Metadata PDU Send Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure to send the metadata PDU
 */
pub const CF_CFDP_S_SEND_MD_ERR_EID: i32 = 98;

/**
 * \brief CF TX Received NAK PDU Bad Segment Request Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Bad segment request values in received NAK PDU relating
 *  to a current transaction
 */
pub const CF_CFDP_S_INVALID_SR_ERR_EID: i32 = 100;

/**
 * \brief CF TX Received NAK PDU Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure processing received NAK PDU relating
 *  to a current transaction
 */
pub const CF_CFDP_S_PDU_NAK_ERR_EID: i32 = 101;

/**
 * \brief CF TX Received EOF ACK PDU Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure processing received ACK PDU relating
 *  to a current transaction
 */
pub const CF_CFDP_S_PDU_EOF_ERR_EID: i32 = 102;

/**
 * \brief CF TX Received Early FIN PDU Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Early FIN PDU received prior to completion of a current transaction
 */
pub const CF_CFDP_S_EARLY_FIN_ERR_EID: i32 = 103;

/**
 * \brief CF Invalid TX File Directive PDU Code Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Unrecognized file directive PDU directive code received for
 *  a current transaction
 */
pub const CF_CFDP_S_DC_INV_ERR_EID: i32 = 104;

/**
 * \brief CF Received TX Non-File Directive PDU Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Received a non-file directive PDU on a send transaction
 */
pub const CF_CFDP_S_NON_FD_PDU_ERR_EID: i32 = 105;

/**
 * \brief CF TX EOF PDU Send Limit Reached Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Timed out the limit number of times waiting for an ACK PDU for the EOF PDU on a
 *  current transaction
 */
pub const CF_CFDP_S_ACK_LIMIT_ERR_EID: i32 = 106;

/**
 * \brief CF TX Inactivity Timer Expired Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Send transaction activity timeout expired
 */
pub const CF_CFDP_S_INACT_TIMER_ERR_EID: i32 = 107;

/**
 * \brief CF TX File Moved
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Source File has been moved after a TX transaction
 *  This occurs when the move directory is configured.
 */
pub const CF_CFDP_S_FILE_MOVED_EID: i32 = 108;

/**
 * \brief CF TX File Removed
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Source File has been removed after a successful TX transaction
 *  where the "keep" flag was false and there is no move directory
 *  configured.
 */
pub const CF_CFDP_S_FILE_REMOVED_EID: i32 = 109;

/**************************************************************************
 * CF_CMD event IDs - Command processing
 */

/**
 * \brief CF NOOP Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt of NOOP command
 */
pub const CF_NOOP_INF_EID: i32 = 110;

/**
 * \brief CF Reset Counters Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of reset counters command
 */
pub const CF_RESET_INF_EID: i32 = 111;

/**
 * \brief CF Set Parameter Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of set parameter command
 */
pub const CF_CMD_GETSET1_INF_EID: i32 = 112;

/**
 * \brief CF Get Parameter Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of get parameter command
 */
pub const CF_CMD_GETSET2_INF_EID: i32 = 113;

/**
 * \brief CF Suspend/Resume Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of suspend/resume command
 */
pub const CF_CMD_SUSPRES_INF_EID: i32 = 114;

/**
 * \brief CF Write Queue Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of write queue command
 */
pub const CF_CMD_WQ_INF_EID: i32 = 115;

/**
 * \brief CF Enable Engine Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of enable engine command
 */
pub const CF_CMD_ENABLE_ENGINE_INF_EID: i32 = 116;

/**
 * \brief CF Disable Engine Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of disable engine command
 */
pub const CF_CMD_DISABLE_ENGINE_INF_EID: i32 = 117;

/**
 * \brief CF Transfer File Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of transfer file command
 */
pub const CF_CMD_TX_FILE_INF_EID: i32 = 118;

/**
 * \brief CF Playback Directory Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of playback directory command
 */
pub const CF_CMD_PLAYBACK_DIR_INF_EID: i32 = 119;

/**
 * \brief CF Freeze Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of freeze command
 */
pub const CF_CMD_FREEZE_INF_EID: i32 = 120;

/**
 * \brief CF Thaw Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of thaw command
 */
pub const CF_CMD_THAW_INF_EID: i32 = 121;

/**
 * \brief CF Cancel Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of cancel command
 */
pub const CF_CMD_CANCEL_INF_EID: i32 = 122;

/**
 * \brief CF Abandon Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of abandon command
 */
pub const CF_CMD_ABANDON_INF_EID: i32 = 123;

/**
 * \brief CF Enable Dequeue Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of enable dequeue command
 */
pub const CF_CMD_ENABLE_DEQUEUE_INF_EID: i32 = 124;

/**
 * \brief CF Disable Dequeue Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of disable dequeue command
 */
pub const CF_CMD_DISABLE_DEQUEUE_INF_EID: i32 = 125;

/**
 * \brief CF Enable Polldir Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of enable polldir command
 */
pub const CF_CMD_ENABLE_POLLDIR_INF_EID: i32 = 126;

/**
 * \brief CF Disable Polldir Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of disable polldir command
 */
pub const CF_CMD_DISABLE_POLLDIR_INF_EID: i32 = 127;

/**
 * \brief CF Purge Queue Command Received Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Receipt and successful processing of purge queue command
 */
pub const CF_CMD_PURGE_QUEUE_INF_EID: i32 = 128;

/**
 * \brief CF Reset Counters Command Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Reset counters command received with invalid parameter
 */
pub const CF_CMD_RESET_INVALID_ERR_EID: i32 = 129;

/**
 * \brief CF Command Channel Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Command received with channel parameter out of range
 */
pub const CF_CMD_CHAN_PARAM_ERR_EID: i32 = 130;

/**
 * \brief CF Command Transaction Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Command received without a matching transaction
 */
pub const CF_CMD_TRANS_NOT_FOUND_ERR_EID: i32 = 131;

/**
 * \brief CF Command All Transaction Channel Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Command received to act on all transactions with invalid channel
 */
pub const CF_CMD_TSN_CHAN_INVALID_ERR_EID: i32 = 132;

/**
 * \brief CF Suspend/Resume Command For Single Transaction State Unchanged Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Suspend/resume command received affecting single transaction already set to that state
 */
pub const CF_CMD_SUSPRES_SAME_INF_EID: i32 = 133;

/**
 * \brief CF Suspend/Resume Command No Matching Transaction Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Suspend/resume command received without a matching transaction
 */
pub const CF_CMD_SUSPRES_CHAN_ERR_EID: i32 = 134;

/**
 * \brief CF Enable/Disable Polling Directory Command Invalid Polling Directory Index Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Enable/disable polling directory command received with invalid poling directory index
 */
pub const CF_CMD_POLLDIR_INVALID_ERR_EID: i32 = 135;

/**
 * \brief CF Purge Queue Command Invalid Argument Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Purge Queue command received with invalid queue argument
 */
pub const CF_CMD_PURGE_ARG_ERR_EID: i32 = 136;

/**
 * \brief CF Write Queue Command Invalid Channel Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Write Queue command received with invalid channel argument
 */
pub const CF_CMD_WQ_CHAN_ERR_EID: i32 = 137;

/**
 * \brief CF Write Queue Command Invalid Queue Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Write Queue command received with invalid queue selection arguments
 */
pub const CF_CMD_WQ_ARGS_ERR_EID: i32 = 138;

/**
 * \brief CF Write Queue Command File Open Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of open file call during processing of write queue command
 */
pub const CF_CMD_WQ_OPEN_ERR_EID: i32 = 139;

/**
 * \brief CF Write Queue Command RX Active File Write Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of file write call for RX active transactions during processing of write queue command
 */
pub const CF_CMD_WQ_WRITEQ_RX_ERR_EID: i32 = 140;

/**
 * \brief CF Write Queue Command RX History File Write Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of file write call for RX history during processing of write queue command
 */
pub const CF_CMD_WQ_WRITEHIST_RX_ERR_EID: i32 = 141;

/**
 * \brief CF Write Queue Command TX Active File Write Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of file write call for TX active transactions during processing of write queue command
 */
pub const CF_CMD_WQ_WRITEQ_TX_ERR_EID: i32 = 142;

/**
 * \brief CF Write Queue Command TX Pending File Write Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of file write call for TX pending transactions during processing of write queue command
 */
pub const CF_CMD_WQ_WRITEQ_PEND_ERR_EID: i32 = 143;

/**
 * \brief CF Write Queue Command TX History File Write Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failure of file write call for TX history during processing of write queue command
 */
pub const CF_CMD_WQ_WRITEHIST_TX_ERR_EID: i32 = 144;

/**
 * \brief CF Set Parameter Command Parameter Validation Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Parameter validation failed during processing of set parameter command
 */
pub const CF_CMD_GETSET_VALIDATE_ERR_EID: i32 = 145;

/**
 * \brief CF Set/Get Parameter Command Invalid Parameter ID Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Invalid parameter id value received in set or get parameter command
 */
pub const CF_CMD_GETSET_PARAM_ERR_EID: i32 = 146;

/**
 * \brief CF Set/Get Parameter Command Invalid Channel Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Invalid channel value received in set or get parameter command
 */
pub const CF_CMD_GETSET_CHAN_ERR_EID: i32 = 147;

/**
 * \brief CF Enable Engine Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Failed to initialize engine when processing engine enable command
 */
pub const CF_CMD_ENABLE_ENGINE_ERR_EID: i32 = 148;

/**
 * \brief CF Enable Engine Command Engine Already Enabled Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Enable engine command received while engine is already enabled
 */
pub const CF_CMD_ENG_ALREADY_ENA_INF_EID: i32 = 149;

/**
 * \brief CF Disable Engine Command Engine Already Disabled Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Disable engine command received while engine is already disabled
 */
pub const CF_CMD_ENG_ALREADY_DIS_INF_EID: i32 = 150;

/**
 * \brief CF Command Length Verification Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Received command length verification failure
 */
pub const CF_CMD_LEN_ERR_EID: i32 = 151;

/**
 * \brief CF Command Code Invalid Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Received command code unrecognized
 */
pub const CF_CC_ERR_EID: i32 = 152;

/**
 * \brief CF Write Entry To File Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Write entry to file did not match expected length
 */
pub const CF_CMD_WHIST_WRITE_ERR_EID: i32 = 153;

/**
 * \brief CF Playback Dir Or TX File Command Bad Parameter Event ID
 *
 *  \par Type:  ERROR
 *
 *  \par Cause:
 *
 *  Bad parameter received in playback directory or transfer file command
 */
pub const CF_CMD_BAD_PARAM_ERR_EID: i32 = 154;

/**
 * \brief CF Cancel Command No Matching Transaction Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Cancel command received without a matching transaction
 */
pub const CF_CMD_CANCEL_CHAN_ERR_EID: i32 = 155;

/**
 * \brief CF Abandon Command No Matching Transaction Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Abandon command received without a matching transaction
 */
pub const CF_CMD_ABANDON_CHAN_ERR_EID: i32 = 156;

/**
 * \brief CF Transfer File Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Transfer file command was unsuccessful
 */
pub const CF_CMD_TX_FILE_ERR_EID: i32 = 157;

/**
 * \brief CF Playback Directory Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Playback directory command was unsuccessful
 */
pub const CF_CMD_PLAYBACK_DIR_ERR_EID: i32 = 158;

/**
 * \brief CF Freeze Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Freeze command was unsuccessful
 */
pub const CF_CMD_FREEZE_ERR_EID: i32 = 159;

/**
 * \brief CF Thaw Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Thaw command was unsuccessful
 */
pub const CF_CMD_THAW_ERR_EID: i32 = 160;

/**
 * \brief CF Enable Dequeue Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Enable Dequeue command was unsuccessful
 */
pub const CF_CMD_ENABLE_DEQUEUE_ERR_EID: i32 = 161;

/**
 * \brief CF Disable Dequeue Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Disable dequeue command was unsuccessful
 */
pub const CF_CMD_DISABLE_DEQUEUE_ERR_EID: i32 = 162;

/**
 * \brief CF Enable Polldir Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Enable polldir command was unsuccessful
 */
pub const CF_CMD_ENABLE_POLLDIR_ERR_EID: i32 = 163;

/**
 * \brief CF Disable Polldir Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Disable polldir command was unsuccessful
 */
pub const CF_CMD_DISABLE_POLLDIR_ERR_EID: i32 = 164;

/**
 * \brief CF Purge Queue Command Failed Event ID
 *
 *  \par Type: ERROR
 *
 *  \par Cause:
 *
 *  Purge queue command was unsuccessful
 */
pub const CF_CMD_PURGE_QUEUE_ERR_EID: i32 = 165;

/**
 * \brief CF Move Path Length Verification Too Long Event ID
 *
 *  \par Type: INFORMATION
 *
 *  \par Cause:
 *
 *  Combined move filename length exceeds buffer size
 */
pub const CF_EID_INF_CFDP_BUF_EXCEED: i32 = 166;