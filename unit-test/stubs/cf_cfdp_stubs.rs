#[cfg(test)]
mod cf_cfdp_stubs {
    use crate::*;

    pub fn ut_default_handler_cf_cfdp_cancel_transaction(_: *mut std::ffi::c_void, _: u32, _: *const std::ffi::c_void) {}
    pub fn ut_default_handler_cf_cfdp_construct_pdu_header(_: *mut std::ffi::c_void, _: u32, _: *const std::ffi::c_void) {}
    pub fn ut_default_handler_cf_cfdp_playback_dir(_: *mut std::ffi::c_void, _: u32, _: *const std::ffi::c_void) {}
    pub fn ut_default_handler_cf_cfdp_tx_file(_: *mut std::ffi::c_void, _: u32, _: *const std::ffi::c_void) {}

    pub fn cf_cfdp_alloc_chunk_list(_txn: *mut CF_Transaction_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_append_tlv(_ptlv_list: *mut CF_Logical_TlvList_t, _tlv_type: CF_CFDP_TlvType_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_arm_ack_timer(_txn: *mut CF_Transaction_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_arm_inact_timer(_txn: *mut CF_Transaction_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_cancel_transaction(_txn: *mut CF_Transaction_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_check_ack_nak_count(_txn: *mut CF_Transaction_t, _counter: *mut u8) -> bool {
        bool::default()
    }

    pub fn cf_cfdp_close_files(_node: *mut CF_CListNode_t, _context: *mut std::ffi::c_void) -> CF_CListTraverse_Status_t {
        CF_CListTraverse_Status_t::default()
    }

    pub fn cf_cfdp_complete_tick(_txn: *mut CF_Transaction_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_construct_pdu_header(
        _txn: *const CF_Transaction_t,
        _directive_code: CF_CFDP_FileDirective_t,
        _src_eid: CF_EntityId_t,
        _dst_eid: CF_EntityId_t,
        _towards_sender: bool,
        _tsn: CF_TransactionSeq_t,
        _silent: bool,
    ) -> *mut CF_Logical_PduBuffer_t {
        std::ptr::null_mut()
    }

    pub fn cf_cfdp_copy_string_from_lv(
        _buf: *mut std::os::raw::c_char,
        _buf_maxsz: usize,
        _src_lv: *const CF_Logical_Lv_t,
    ) -> std::os::raw::c_int {
        std::os::raw::c_int::default()
    }

    pub fn cf_cfdp_cycle_engine() {
        // Stub implementation
    }

    pub fn cf_cfdp_decode_start(
        _pdec: *mut CF_DecoderState_t,
        _msgbuf: *const std::ffi::c_void,
        _ph: *mut CF_Logical_PduBuffer_t,
        _encap_hdr_size: usize,
        _total_size: usize,
    ) {
        // Stub implementation
    }

    pub fn cf_cfdp_disable_engine() {
        // Stub implementation
    }

    pub fn cf_cfdp_dispatch_recv(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_do_tick(_node: *mut CF_CListNode_t, _context: *mut std::ffi::c_void) -> CF_CListTraverse_Status_t {
        CF_CListTraverse_Status_t::default()
    }

    pub fn cf_cfdp_encode_start(
        _penc: *mut CF_EncoderState_t,
        _msgbuf: *mut std::ffi::c_void,
        _ph: *mut CF_Logical_PduBuffer_t,
        _encap_hdr_size: usize,
        _total_size: usize,
    ) {
        // Stub implementation
    }

    pub fn cf_cfdp_finish_transaction(_txn: *mut CF_Transaction_t, _keep_history: bool) {
        // Stub implementation
    }

    pub fn cf_cfdp_get_move_target(
        _dest_dir: *const std::os::raw::c_char,
        _subject_file: *const std::os::raw::c_char,
        _dest_buf: *mut std::os::raw::c_char,
        _dest_size: usize,
    ) -> *const std::os::raw::c_char {
        std::ptr::null()
    }

    pub fn cf_cfdp_get_temp_name(
        _hist: *const CF_History_t,
        _file_name_buf: *mut std::os::raw::c_char,
        _file_name_size: usize,
    ) {
        // Stub implementation
    }

    pub fn cf_cfdp_get_txn_status(_txn: *const CF_Transaction_t) -> CF_TxnStatus_t {
        CF_TxnStatus_t::default()
    }

    pub fn cf_cfdp_init_engine() -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_init_txn_tx_file(
        _txn: *mut CF_Transaction_t,
        _cfdp_class: CF_CFDP_Class_t,
        _keep: u8,
        _chan: u8,
        _priority: u8,
    ) {
        // Stub implementation
    }

    pub fn cf_cfdp_playback_dir(
        _src_filename: *const std::os::raw::c_char,
        _dst_filename: *const std::os::raw::c_char,
        _cfdp_class: CF_CFDP_Class_t,
        _keep: u8,
        _chan: u8,
        _priority: u8,
        _dest_id: u16,
    ) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_process_playbook_directory(_chan: *mut CF_Channel_t, _pb: *mut CF_Playback_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_process_polling_directories(_chan: *mut CF_Channel_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_receive_pdu(_chan: *mut CF_Channel_t, _ph: *mut CF_Logical_PduBuffer_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_recv_ack(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_recv_drop(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_recv_eof(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_recv_fd(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_recv_fin(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_recv_hold(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_recv_init(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_recv_md(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_recv_nak(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_recv_ph(_chan_num: u8, _ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_recycle_transaction(_txn: *mut CF_Transaction_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_s_tick_new_data(_txn: *mut CF_Transaction_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_send_ack(_txn: *mut CF_Transaction_t, _dir_code: CF_CFDP_FileDirective_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_send_eof(_txn: *mut CF_Transaction_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_send_eot_pkt(_txn: *mut CF_Transaction_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_send_fd(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_send_fin(_txn: *mut CF_Transaction_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_send_md(_txn: *mut CF_Transaction_t) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_send_nak(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_set_txn_status(_txn: *mut CF_Transaction_t, _txn_stat: CF_TxnStatus_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_setup_rx_transaction(_txn: *mut CF_Transaction_t, _ph: *mut CF_Logical_PduBuffer_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_setup_tx_transaction(_txn: *mut CF_Transaction_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_start_first_pending(_chan: *mut CF_Channel_t) -> bool {
        bool::default()
    }

    pub fn cf_cfdp_start_rx_transaction(_chan_num: u8) -> *mut CF_Transaction_t {
        std::ptr::null_mut()
    }

    pub fn cf_cfdp_tick_transactions(_chan: *mut CF_Channel_t) {
        // Stub implementation
    }

    pub fn cf_cfdp_tx_file(
        _src_filename: *const std::os::raw::c_char,
        _dst_filename: *const std::os::raw::c_char,
        _cfdp_class: CF_CFDP_Class_t,
        _keep: u8,
        _chan: u8,
        _priority: u8,
        _dest_id: CF_EntityId_t,
    ) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_txn_is_ok(_txn: *const CF_Transaction_t) -> CF_TxnStatus_t {
        CF_TxnStatus_t::default()
    }
}