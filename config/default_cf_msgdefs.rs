use std::os::raw::{c_char, c_uint};

// Assuming these types are defined elsewhere or in dependencies
pub type CF_QueueIdx_NUM = usize;
pub type CF_NUM_CHANNELS = usize;
pub type CF_TransactionSeq_t = u32;
pub type CF_EntityId_t = u32;
pub type CF_TxnFilenames_t = [c_char; 256]; // Placeholder size
pub const CF_FILENAME_MAX_LEN: usize = 256; // Placeholder value

/**
 * \brief Housekeeping command counters
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkCmdCounters {
    pub cmd: u16, /**< \brief Command success counter */
    pub err: u16, /**< \brief Command error counter */
}

pub type CF_HkCmdCounters_t = CF_HkCmdCounters;

/**
 * \brief Housekeeping sent counters
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkSent {
    pub file_data_bytes: u64,      /**< \brief Sent File data bytes */
    pub pdu: u32,                  /**< \brief Sent PDUs counter */
    pub nak_segment_requests: u32, /**< \brief Sent NAK segment requests counter */
}

pub type CF_HkSent_t = CF_HkSent;

/**
 * \brief Housekeeping received counters
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkRecv {
    pub file_data_bytes: u64,      /**< \brief Received File data bytes */
    pub pdu: u32,                  /**< \brief Received PDUs with valid header counter */
    pub error: u32,                /**< \brief Received PDUs with error counter, see related event for cause */
    pub spurious: u16,             /**< \brief Received PDUs with invalid directive code for current context or
                                    *          file directive FIN without matching active transaction counter,
                                    *          see related event for cause
                                    */
    pub dropped: u16,              /**< \brief Received PDUs dropped due to a transaction error */
    pub nak_segment_requests: u32, /**< \brief Received NAK segment requests counter */
}

pub type CF_HkRecv_t = CF_HkRecv;

/**
 * \brief Housekeeping fault counters
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkFault {
    pub file_open: u16,          /**< \brief File open fault counter */
    pub file_read: u16,          /**< \brief File read fault counter */
    pub file_seek: u16,          /**< \brief File seek fault counter */
    pub file_write: u16,         /**< \brief File write fault counter */
    pub file_rename: u16,        /**< \brief File rename fault counter */
    pub directory_read: u16,     /**< \brief Directory read fault counter */
    pub crc_mismatch: u16,       /**< \brief CRC mismatch fault counter */
    pub file_size_mismatch: u16, /**< \brief File size mismatch fault counter */
    pub nak_limit: u16,          /**< \brief NAK limit exceeded fault counter */
    pub ack_limit: u16,          /**< \brief ACK limit exceeded fault counter */
    pub inactivity_timer: u16,   /**< \brief Inactivity timer exceeded counter */
    pub spare: u16,              /**< \brief Alignment spare to avoid implicit padding */
}

pub type CF_HkFault_t = CF_HkFault;

/**
 * \brief Housekeeping counters
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkCounters {
    pub sent: CF_HkSent_t,  /**< \brief Sent counters */
    pub recv: CF_HkRecv_t,  /**< \brief Received counters */
    pub fault: CF_HkFault_t, /**< \brief Fault counters */
}

pub type CF_HkCounters_t = CF_HkCounters;

/**
 * \brief Housekeeping channel data
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkChannel_Data {
    pub counters: CF_HkCounters_t,                /**< \brief Counters */
    pub q_size: [u16; CF_QueueIdx_NUM],           /**< \brief Queue sizes */
    pub poll_counter: u8,                         /**< \brief Number of active polling directories */
    pub playback_counter: u8,                     /**< \brief Number of active playback directories */
    pub frozen: u8,                               /**< \brief Frozen state: 0 == not frozen, else frozen */
    pub spare: [u8; 7],                           /**< \brief Alignment spare (uint64 values in the counters) */
}

pub type CF_HkChannel_Data_t = CF_HkChannel_Data;

/**
 * \brief Housekeeping packet
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkPacket_Payload {
    pub counters: CF_HkCmdCounters_t, /**< \brief Command counters */
    pub spare: [u8; 4],               /**< \brief Alignment spare (CF_HkCmdCounters_t is 4 bytes) */
    pub channel_hk: [CF_HkChannel_Data_t; CF_NUM_CHANNELS], /**< \brief Per channel housekeeping data */
}

pub type CF_HkPacket_Payload_t = CF_HkPacket_Payload;

/**
 * \brief End of transaction packet
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_EotPacket_Payload {
    pub seq_num: CF_TransactionSeq_t,    /**< \brief transaction identifier, stays constant for entire transfer */
    pub channel: u32,                    /**< \brief Channel number */
    pub direction: u32,                  /**< \brief direction of this transaction */
    pub state: u32,                      /**< \brief Transaction state */
    pub txn_stat: u32,                   /**< \brief final status code of transaction (extended CFDP CC) */
    pub src_eid: CF_EntityId_t,          /**< \brief the source eid of the transaction */
    pub peer_eid: CF_EntityId_t,         /**< \brief peer_eid is always the "other guy", same src_eid for RX */
    pub fsize: u32,                      /**< \brief File size */
    pub crc_result: u32,                 /**< \brief CRC result */
    pub fnames: CF_TxnFilenames_t,       /**< \brief file names associated with this transaction */
}

pub type CF_EotPacket_Payload_t = CF_EotPacket_Payload;

/**
 * \brief Command payload argument union to support 4 uint8's, 2 uint16's or 1 uint32
 */
#[derive(Clone, Copy)]
#[repr(C)]
pub union CF_UnionArgs_Payload {
    pub dword: u32,       /**< \brief Generic uint32 argument */
    pub hword: [u16; 2],  /**< \brief Generic uint16 array of arguments */
    pub byte: [u8; 4],    /**< \brief Generic uint8 array of arguments */
}

pub type CF_UnionArgs_Payload_t = CF_UnionArgs_Payload;

/**
 * \brief IDs for use for Reset cmd
 */
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum CF_Reset {
    CF_Reset_all = 0,     /**< \brief Reset all */
    CF_Reset_command = 1, /**< \brief Reset command */
    CF_Reset_fault = 2,   /**< \brief Reset fault */
    CF_Reset_up = 3,      /**< \brief Reset up */
    CF_Reset_down = 4,    /**< \brief Reset down */
}

pub type CF_Reset_t = CF_Reset;

/**
 * \brief Type IDs for use for Write Queue cmd
 */
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum CF_Type {
    CF_Type_all = 0,  /**< \brief Type all */
    CF_Type_up = 1,   /**< \brief Type up */
    CF_Type_down = 2, /**< \brief Type down */
}

pub type CF_Type_t = CF_Type;

/**
 * \brief Queue IDs for use for Write Queue cmd
 */
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum CF_Queue {
    CF_Queue_pend = 0,    /**< \brief Queue pending */
    CF_Queue_active = 1,  /**< \brief Queue active */
    CF_Queue_history = 2, /**< \brief Queue history */
    CF_Queue_all = 3,     /**< \brief Queue all */
}

pub type CF_Queue_t = CF_Queue;

/**
 * \brief Parameter IDs for use with Get/Set parameter messages
 *
 * Specifically these are used for the "key" field within CF_GetParamCmd_t and
 * CF_SetParamCmd_t message structures.
 */
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum CF_GetSet_ValueID {
    CF_GetSet_ValueID_ticks_per_second = 0,                      /**< \brief Ticks per second key */
    CF_GetSet_ValueID_rx_crc_calc_bytes_per_wakeup = 1,          /**< \brief Receive CRC calculated bytes per wake-up key */
    CF_GetSet_ValueID_ack_timer_s = 2,                           /**< \brief ACK timer in seconds key */
    CF_GetSet_ValueID_nak_timer_s = 3,                           /**< \brief NAK timer in seconds key */
    CF_GetSet_ValueID_inactivity_timer_s = 4,                    /**< \brief Inactivity timer in seconds key */
    CF_GetSet_ValueID_outgoing_file_chunk_size = 5,              /**< \brief Outgoing file chunk size key */
    CF_GetSet_ValueID_ack_limit = 6,                             /**< \brief ACK retry limit key */
    CF_GetSet_ValueID_nak_limit = 7,                             /**< \brief NAK retry limit key */
    CF_GetSet_ValueID_local_eid = 8,                             /**< \brief Local entity id key */
    CF_GetSet_ValueID_chan_max_outgoing_messages_per_wakeup = 9, /**< \brief Max outgoing messages per wake-up key */
    CF_GetSet_ValueID_MAX = 10,                                  /**< \brief Key limit used for validity check */
}

pub type CF_GetSet_ValueID_t = CF_GetSet_ValueID;

/**
 * \brief Get parameter command structure
 *
 * For command details see #CF_GET_PARAM_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_GetParam_Payload {
    pub key: u8,      /**< \brief Parameter key, see #CF_GetSet_ValueID_t */
    pub chan_num: u8, /**< \brief Channel number */
}

pub type CF_GetParam_Payload_t = CF_GetParam_Payload;

/**
 * \brief Set parameter command structure
 *
 * For command details see #CF_SET_PARAM_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_SetParam_Payload {
    pub value: u32,       /**< \brief Parameter value to set */
    pub key: u8,          /**< \brief Parameter key, see #CF_GetSet_ValueID_t */
    pub chan_num: u8,     /**< \brief Channel number */
    pub spare: [u8; 2],   /**< \brief Alignment spare, uint32 multiple */
}

pub type CF_SetParam_Payload_t = CF_SetParam_Payload;

/**
 * \brief Transmit file command structure
 *
 * For command details see #CF_TX_FILE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_TxFile_Payload {
    pub cfdp_class: u8,                                  /**< \brief CFDP class: 0=class 1, 1=class 2 */
    pub keep: u8,                                        /**< \brief Keep file flag: 1=keep, else delete */
    pub chan_num: u8,                                    /**< \brief Channel number */
    pub priority: u8,                                    /**< \brief Priority: 0=highest priority */
    pub dest_id: CF_EntityId_t,                          /**< \brief Destination entity id */
    pub src_filename: [c_char; CF_FILENAME_MAX_LEN],     /**< \brief Source file/directory name */
    pub dst_filename: [c_char; CF_FILENAME_MAX_LEN],     /**< \brief Destination file/directory name */
}

pub type CF_TxFile_Payload_t = CF_TxFile_Payload;

/**
 * \brief Write Queue command structure
 *
 * For command details see #CF_WRITE_QUEUE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_WriteQueue_Payload {
    pub r#type: u8,  /**< \brief Transaction direction: all=0, up=1, down=2 */
    pub chan: u8,    /**< \brief Channel number */
    pub queue: u8,   /**< \brief Queue type: 0=pending, 1=active, 2=history, 3=all */
    pub spare: u8,   /**< \brief Alignment spare, puts filename on 32-bit boundary */
    pub filename: [c_char; CF_FILENAME_MAX_LEN], /**< \brief Filename written to */
}

pub type CF_WriteQueue_Payload_t = CF_WriteQueue_Payload;

/**
 * \brief Transaction command structure
 *
 * For command details see #CF_SUSPEND_CC, #CF_RESUME_CC, #CF_CANCEL_CC, #CF_ABANDON_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Transaction_Payload {
    pub ts: CF_TransactionSeq_t,  /**< \brief Transaction sequence number */
    pub eid: CF_EntityId_t,       /**< \brief Entity id */
    pub chan: u8,                 /**< \brief Channel number: 254=use ts, 255=all channels, else channel */
    pub spare: [u8; 3],           /**< \brief Alignment spare for 32-bit multiple */
}

pub type CF_Transaction_Payload_t = CF_Transaction_Payload;