use crate::cf_cfdp_s::*;
use crate::types::{CF_Transaction_t, CF_Logical_PduBuffer_t, CFE_Status_t};

#[cfg(test)]
mod stubs {
    use super::*;

    pub fn cf_cfdp_s1_recv(txn: &mut CF_Transaction_t, ph: &mut CF_Logical_PduBuffer_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s2_substate_eof_ack(txn: &mut CF_Transaction_t, ph: &mut CF_Logical_PduBuffer_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s2_substate_nak(txn: &mut CF_Transaction_t, ph: &mut CF_Logical_PduBuffer_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s2_recv(txn: &mut CF_Transaction_t, ph: &mut CF_Logical_PduBuffer_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_ack_timer_tick(txn: &mut CF_Transaction_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_check_state(txn: &mut CF_Transaction_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_substate_early_fin(txn: &mut CF_Transaction_t, ph: &mut CF_Logical_PduBuffer_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_substate_recv_fin(txn: &mut CF_Transaction_t, ph: &mut CF_Logical_PduBuffer_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_handle_file_retention(txn: &mut CF_Transaction_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_init(txn: &mut CF_Transaction_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_send_file_data(
        txn: &mut CF_Transaction_t,
        foffs: u32,
        bytes_to_read: u32,
        calc_crc: u8,
    ) -> CFE_Status_t {
        CFE_Status_t::default()
    }

    pub fn cf_cfdp_s_substate_send_file_data(txn: &mut CF_Transaction_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_tick(txn: &mut CF_Transaction_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_tick_maintenance(txn: &mut CF_Transaction_t) {
        // Stub implementation - no operation
    }

    pub fn cf_cfdp_s_tick_nak(txn: &mut CF_Transaction_t) {
        // Stub implementation - no operation
    }
}