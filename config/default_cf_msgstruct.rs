use cfe_msg_hdr::{CFE_MSG_TelemetryHeader_t, CFE_MSG_CommandHeader_t};
use cf_msgdefs::{
    CF_HkPacket_Payload_t, CF_EotPacket_Payload_t, CF_UnionArgs_Payload_t,
    CF_GetParam_Payload_t, CF_SetParam_Payload_t, CF_TxFile_Payload_t,
    CF_WriteQueue_Payload_t, CF_Transaction_Payload_t
};

/**
 * \brief Housekeeping packet
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_HkPacket {
    /// \brief Telemetry header
    pub TelemetryHeader: CFE_MSG_TelemetryHeader_t,
    pub Payload: CF_HkPacket_Payload_t,
}

pub type CF_HkPacket_t = CF_HkPacket;

/**
 * \brief End of transaction packet
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_EotPacket {
    /// \brief Telemetry header
    pub TelemetryHeader: CFE_MSG_TelemetryHeader_t,
    pub Payload: CF_EotPacket_Payload_t,
}

pub type CF_EotPacket_t = CF_EotPacket;

/**
 * \brief Noop command structure
 *
 * For command details see #CF_NOOP_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_NoopCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
}

pub type CF_NoopCmd_t = CF_NoopCmd;

/**
 * \brief EnableEngine command structure
 *
 * For command details see #CF_ENABLE_ENGINE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_EnableEngineCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
}

pub type CF_EnableEngineCmd_t = CF_EnableEngineCmd;

/**
 * \brief DisableEngine command structure
 *
 * For command details see #CF_DISABLE_ENGINE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_DisableEngineCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
}

pub type CF_DisableEngineCmd_t = CF_DisableEngineCmd;

/**
 * \brief Reset command structure
 *
 * For command details see #CF_RESET_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ResetCountersCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    /// \brief Generic command arguments
    pub Payload: CF_UnionArgs_Payload_t,
}

pub type CF_ResetCountersCmd_t = CF_ResetCountersCmd;

/**
 * \brief Freeze command structure
 *
 * For command details see #CF_FREEZE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_FreezeCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    /// \brief Generic command arguments
    pub Payload: CF_UnionArgs_Payload_t,
}

pub type CF_FreezeCmd_t = CF_FreezeCmd;

/**
 * \brief Thaw command structure
 *
 * For command details see #CF_THAW_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ThawCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    /// \brief Generic command arguments
    pub Payload: CF_UnionArgs_Payload_t,
}

pub type CF_ThawCmd_t = CF_ThawCmd;

/**
 * \brief EnableDequeue command structure
 *
 * For command details see #CF_ENABLE_DEQUEUE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_EnableDequeueCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    /// \brief Generic command arguments
    pub Payload: CF_UnionArgs_Payload_t,
}

pub type CF_EnableDequeueCmd_t = CF_EnableDequeueCmd;

/**
 * \brief DisableDequeue command structure
 *
 * For command details see #CF_DISABLE_DEQUEUE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_DisableDequeueCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    /// \brief Generic command arguments
    pub Payload: CF_UnionArgs_Payload_t,
}

pub type CF_DisableDequeueCmd_t = CF_DisableDequeueCmd;

/**
 * \brief EnableDirPolling command structure
 *
 * For command details see #CF_ENABLE_DIR_POLLING_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_EnableDirPollingCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    /// \brief Generic command arguments
    pub Payload: CF_UnionArgs_Payload_t,
}

pub type CF_EnableDirPollingCmd_t = CF_EnableDirPollingCmd;

/**
 * \brief DisableDirPolling command structure
 *
 * For command details see #CF_DISABLE_DIR_POLLING_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_DisableDirPollingCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    /// \brief Generic command arguments
    pub Payload: CF_UnionArgs_Payload_t,
}

pub type CF_DisableDirPollingCmd_t = CF_DisableDirPollingCmd;

/**
 * \brief PurgeQueue command structure
 *
 * For command details see #CF_PURGE_QUEUE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_PurgeQueueCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    /// \brief Generic command arguments
    pub Payload: CF_UnionArgs_Payload_t,
}

pub type CF_PurgeQueueCmd_t = CF_PurgeQueueCmd;

/**
 * \brief Get parameter command structure
 *
 * For command details see #CF_GET_PARAM_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_GetParamCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_GetParam_Payload_t,
}

pub type CF_GetParamCmd_t = CF_GetParamCmd;

/**
 * \brief Set parameter command structure
 *
 * For command details see #CF_SET_PARAM_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_SetParamCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_SetParam_Payload_t,
}

pub type CF_SetParamCmd_t = CF_SetParamCmd;

/**
 * \brief Transmit file command structure
 *
 * For command details see #CF_TX_FILE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_TxFileCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_TxFile_Payload_t,
}

pub type CF_TxFileCmd_t = CF_TxFileCmd;

/**
 * \brief Write Queue command structure
 *
 * For command details see #CF_WRITE_QUEUE_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_WriteQueueCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_WriteQueue_Payload_t,
}

pub type CF_WriteQueueCmd_t = CF_WriteQueueCmd;

/**
 * \brief Playback directory command structure
 *
 * For command details see #CF_PLAYBACK_DIR_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_PlaybackDirCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_TxFile_Payload_t,
}

pub type CF_PlaybackDirCmd_t = CF_PlaybackDirCmd;

/**
 * \brief Suspend command structure
 *
 * For command details see #CF_SUSPEND_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_SuspendCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_Transaction_Payload_t,
}

pub type CF_SuspendCmd_t = CF_SuspendCmd;

/**
 * \brief Resume command structure
 *
 * For command details see #CF_RESUME_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_ResumeCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_Transaction_Payload_t,
}

pub type CF_ResumeCmd_t = CF_ResumeCmd;

/**
 * \brief Cancel command structure
 *
 * For command details see #CF_CANCEL_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_CancelCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_Transaction_Payload_t,
}

pub type CF_CancelCmd_t = CF_CancelCmd;

/**
 * \brief Abandon command structure
 *
 * For command details see #CF_ABANDON_CC
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_AbandonCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
    pub Payload: CF_Transaction_Payload_t,
}

pub type CF_AbandonCmd_t = CF_AbandonCmd;

/**
 * \brief Send Housekeeping Command
 *
 * Internal notification from SCH with no payload
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_SendHkCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
}

pub type CF_SendHkCmd_t = CF_SendHkCmd;

/**
 * \brief Wake Up Command
 *
 * Internal notification from SCH with no payload
 */
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_WakeupCmd {
    /// \brief Command header
    pub CommandHeader: CFE_MSG_CommandHeader_t,
}

pub type CF_WakeupCmd_t = CF_WakeupCmd;