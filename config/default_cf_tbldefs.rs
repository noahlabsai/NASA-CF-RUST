use crate::cf_mission_cfg::*;
use crate::cf_extern_typedefs::*;

/**
 * \brief Configuration entry for directory polling
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_PollDir {
    /// \brief number of seconds to wait before trying a new directory
    pub interval_sec: u32,

    /// \brief priority to use when placing transactions on the pending queue
    pub priority: u8,
    /// \brief the CFDP class to send
    pub cfdp_class: CF_CFDP_Class_t,
    /// \brief destination entity id
    pub dest_eid: CF_EntityId_t,

    /// \brief path to source dir
    pub src_dir: [u8; CF_FILENAME_MAX_PATH],
    /// \brief path to destination dir
    pub dst_dir: [u8; CF_FILENAME_MAX_PATH],

    /// \brief Enabled flag
    pub enabled: u8,
}

pub type CF_PollDir_t = CF_PollDir;

/**
 * \brief Configuration entry for CFDP channel
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ChannelConfig {
    /// \brief max number of messages to send per wakeup (0 - unlimited)
    pub max_outgoing_messages_per_wakeup: u32,
    /// \brief max number of rx messages to process per wakeup
    pub rx_max_messages_per_wakeup: u32,

    /// \brief Acknowledge timer in seconds
    pub ack_timer_s: u32,
    /// \brief Non-acknowledge timer in seconds
    pub nak_timer_s: u32,
    /// \brief Inactivity timer in seconds
    pub inactivity_timer_s: u32,

    /// number of times to retry ACK (for ex, send FIN and wait for fin-ack)
    pub ack_limit: u8,
    /// number of times to retry NAK before giving up (resets on a single response
    pub nak_limit: u8,

    /// \brief msgid integer value for incoming messages
    pub mid_input: CFE_SB_MsgId_Atom_t,
    /// \brief msgid integer value for outgoing messages
    pub mid_output: CFE_SB_MsgId_Atom_t,

    /// \brief depth of pipe to receive incoming PDU
    pub pipe_depth_input: u16,

    /// \brief Configuration for polled directories
    pub polldir: [CF_PollDir_t; CF_MAX_POLLING_DIR_PER_CHAN],

    /// \brief name of throttling semaphore in TO
    pub sem_name: [u8; OS_MAX_API_NAME],
    /// \brief if 1, then the channel will make pending transactions active
    pub dequeue_enabled: u8,
    /// \brief Move directory if not empty
    pub move_dir: [u8; OS_MAX_PATH_LEN],
}

pub type CF_ChannelConfig_t = CF_ChannelConfig;