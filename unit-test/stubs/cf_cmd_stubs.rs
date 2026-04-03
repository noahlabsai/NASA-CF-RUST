#[cfg(test)]
mod cf_cmd_stubs {
    use crate::cf_cmd::*;

    pub fn cf_abandon_cmd(msg: Option<&CF_AbandonCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_abandon_txn_cmd(txn: Option<&mut CF_Transaction_t>, ignored: Option<&mut std::ffi::c_void>) {
        // Stub implementation - no return value
    }

    pub fn cf_cancel_cmd(msg: Option<&CF_CancelCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cancel_txn_cmd(txn: Option<&mut CF_Transaction_t>, ignored: Option<&mut std::ffi::c_void>) {
        // Stub implementation - no return value
    }

    pub fn cf_disable_dequeue_cmd(msg: Option<&CF_DisableDequeueCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_disable_dir_polling_cmd(msg: Option<&CF_DisableDirPollingCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_disable_engine_cmd(msg: Option<&CF_DisableEngineCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_do_chan_action(
        data: Option<&CF_UnionArgs_Payload_t>,
        errstr: Option<&str>,
        fn_ptr: Option<CF_ChanActionFn_t>,
        context: Option<&mut std::ffi::c_void>
    ) -> CF_ChanAction_Status_t {
        CF_ChanAction_Status_t::default()
    }

    pub fn cf_do_enable_disable_dequeue(chan_num: u8, arg: Option<&mut std::ffi::c_void>) -> CF_ChanAction_Status_t {
        CF_ChanAction_Status_t::default()
    }

    pub fn cf_do_enable_disable_polldir(chan_num: u8, arg: Option<&mut std::ffi::c_void>) -> CF_ChanAction_Status_t {
        CF_ChanAction_Status_t::default()
    }

    pub fn cf_do_freeze_thaw(chan_num: u8, arg: Option<&mut std::ffi::c_void>) -> CF_ChanAction_Status_t {
        CF_ChanAction_Status_t::default()
    }

    pub fn cf_do_purge_queue(chan_num: u8, arg: Option<&mut std::ffi::c_void>) -> CF_ChanAction_Status_t {
        CF_ChanAction_Status_t::default()
    }

    pub fn cf_do_susp_res(payload: Option<&CF_Transaction_Payload_t>, action: u8) {
        // Stub implementation - no return value
    }

    pub fn cf_do_susp_res_txn(txn: Option<&mut CF_Transaction_t>, context: Option<&mut CF_ChanAction_SuspResArg_t>) {
        // Stub implementation - no return value
    }

    pub fn cf_enable_dequeue_cmd(msg: Option<&CF_EnableDequeueCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_enable_dir_polling_cmd(msg: Option<&CF_EnableDirPollingCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_enable_engine_cmd(msg: Option<&CF_EnableEngineCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_find_transaction_by_sequence_number_all_channels(
        ts: CF_TransactionSeq_t,
        eid: CF_EntityId_t
    ) -> Option<Box<CF_Transaction_t>> {
        None
    }

    pub fn cf_freeze_cmd(msg: Option<&CF_FreezeCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_get_param_cmd(msg: Option<&CF_GetParamCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_get_set_param_cmd(is_set: bool, param_id: CF_GetSet_ValueID_t, value: u32, chan_num: u8) {
        // Stub implementation - no return value
    }

    pub fn cf_noop_cmd(msg: Option<&CF_NoopCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_playback_dir_cmd(msg: Option<&CF_PlaybackDirCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_purge_history(node: Option<&mut CF_CListNode_t>, arg: Option<&mut std::ffi::c_void>) -> CF_CListTraverse_Status_t {
        CF_CListTraverse_Status_t::default()
    }

    pub fn cf_purge_queue_cmd(msg: Option<&CF_PurgeQueueCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_purge_transaction(node: Option<&mut CF_CListNode_t>, ignored: Option<&mut std::ffi::c_void>) -> CF_CListTraverse_Status_t {
        CF_CListTraverse_Status_t::default()
    }

    pub fn cf_reset_counters_cmd(msg: Option<&CF_ResetCountersCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_resume_cmd(msg: Option<&CF_ResumeCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_send_hk_cmd(msg: Option<&CF_SendHkCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_set_param_cmd(msg: Option<&CF_SetParamCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_suspend_cmd(msg: Option<&CF_SuspendCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_thaw_cmd(msg: Option<&CF_ThawCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_tsn_chan_action(
        data: Option<&CF_Transaction_Payload_t>,
        cmdstr: Option<&str>,
        fn_ptr: Option<CF_TsnChanAction_fn_t>,
        context: Option<&mut std::ffi::c_void>
    ) -> i32 {
        0
    }

    pub fn cf_tx_file_cmd(msg: Option<&CF_TxFileCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_validate_chunk_size_cmd(val: CF_ChunkSize_t, chan_num: u8) -> CF_ChanAction_Status_t {
        CF_ChanAction_Status_t::default()
    }

    pub fn cf_validate_max_outgoing_cmd(val: u32, chan_num: u8) -> CF_ChanAction_Status_t {
        CF_ChanAction_Status_t::default()
    }

    pub fn cf_wakeup_cmd(msg: Option<&CF_WakeupCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_write_queue_cmd(msg: Option<&CF_WriteQueueCmd_t>) -> CFE_Status_t {
        CFE_Status_t::default()
    }
}