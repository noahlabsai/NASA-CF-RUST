use std::os::raw::{c_char, c_uint};

// Include mission configuration constants
// Note: CF_FILENAME_MAX_LEN would be defined in cf_mission_cfg module

/**
 * @brief Values for CFDP file transfer class
 *
 * The CFDP specification prescribes two classes/modes of file
 * transfer protocol operation - unacknowledged/simple or
 * acknowledged/reliable.
 *
 * Defined per section 7.1 of CCSDS 727.0-B-5
 */
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum CF_CFDP_Class_t {
    CF_CFDP_CLASS_1 = 0, /**< \brief CFDP class 1 - Unreliable transfer */
    CF_CFDP_CLASS_2 = 1, /**< \brief CFDP class 2 - Reliable transfer */
}

/**
 * @brief CF queue identifiers
 */
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum CF_QueueIdx_t {
    CF_QueueIdx_PEND      = 0, /**< \brief tx transactions that have not started */
    CF_QueueIdx_TX        = 1, /**< \brief tx transactions in progress */
    CF_QueueIdx_RX        = 2, /**< \brief rx transactions in progress */
    CF_QueueIdx_HIST      = 3, /**< \brief transaction history (completed) */
    CF_QueueIdx_HIST_FREE = 4, /**< \brief unused transaction history structs */
    CF_QueueIdx_FREE      = 5, /**< \brief unused transaction structs */
    CF_QueueIdx_NUM       = 6,
}

/**
 * @brief Cache of source and destination filename
 *
 * This pairs a source and destination file name together
 * to be retained for future reference in the transaction/history
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_TxnFilenames {
    pub src_filename: [c_char; CF_FILENAME_MAX_LEN],
    pub dst_filename: [c_char; CF_FILENAME_MAX_LEN],
}

pub type CF_TxnFilenames_t = CF_TxnFilenames;

/**
 * @brief Entity id size
 *
 * @par Description:
 *      The maximum size of the entity id as expected for all CFDP packets.
 *      CF supports the spec's variable size of EID, where the actual size is
 *      selected at runtime, and therefore the size in CFDP PDUs may be smaller
 *      than the size specified here.  This type only establishes the maximum
 *      size (and therefore maximum value) that an EID may be.
 *
 * @note This type is used in several CF commands, and so changing the size
 *       of this type will affect the following structs:
 *        CF_ConfigTable_t, configuration table - will change size of file
 *        CF_ConfigPacket_t, set config params command
 *        CF_TxFileCmd_t, transmit file command
 *        CF_PlaybackDirCmd_t, equivalent to above
 *        CF_Transaction_Payload_t, any command that selects a transaction based on EID
 *
 * @par Limits
 *         Must be one of uint8, uint16, uint32, uint64.
 */
pub type CF_EntityId_t = u32;

/**
 * @brief transaction sequence number size
 *
 * @par Description:
 *      The max size of the transaction sequence number as expected for all CFDP packets.
 *      CF supports the spec's variable size of TSN, where the actual size is
 *      selected at runtime, and therefore the size in CFDP PDUs may be smaller
 *      than the size specified here.  This type only establishes the maximum
 *      size (and therefore maximum value) that a TSN may be.
 *
 * @note This type is used in several CF commands, and so changing the size
 *       of this type will affect the following structure:
 *        CF_Transaction_Payload_t, any command that selects a transaction based on TSN
 *
 * @par Limits
 *         Must be one of uint8, uint16, uint32, uint64.
 */
pub type CF_TransactionSeq_t = u32;

// Note: CF_FILENAME_MAX_LEN constant would need to be imported from cf_mission_cfg module
// For compilation, assuming it's defined as:
pub const CF_FILENAME_MAX_LEN: usize = 256;