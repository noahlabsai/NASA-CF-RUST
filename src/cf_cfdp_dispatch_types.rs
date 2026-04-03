use crate::cf_cfdp_types::CF_Transaction_t;
use crate::cf_logical_pdu::CF_Logical_PduBuffer_t;
use crate::cf_cfdp_types::{CF_TxnState_INVALID, CF_CFDP_FileDirective_INVALID_MAX, CF_RxSubState_NUM_STATES, CF_TxSubState_NUM_STATES};

/// A function for dispatching actions to a handler, without existing PDU data
pub type CF_CFDP_StateSendFunc_t = Option<fn(txn: *mut CF_Transaction_t)>;

/// A function for dispatching actions to a handler, with existing PDU data
pub type CF_CFDP_StateRecvFunc_t = Option<fn(txn: *mut CF_Transaction_t, ph: *mut CF_Logical_PduBuffer_t)>;

/// A table of transmit handler functions based on transaction state
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_CFDP_TxnSendDispatchTable_t {
    pub tx: [CF_CFDP_StateSendFunc_t; CF_TxnState_INVALID],
}

impl Default for CF_CFDP_TxnSendDispatchTable_t {
    fn default() -> Self {
        Self {
            tx: [None; CF_TxnState_INVALID],
        }
    }
}

/// A table of receive handler functions based on transaction state
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_CFDP_TxnRecvDispatchTable_t {
    pub rx: [CF_CFDP_StateRecvFunc_t; CF_TxnState_INVALID],
}

impl Default for CF_CFDP_TxnRecvDispatchTable_t {
    fn default() -> Self {
        Self {
            rx: [None; CF_TxnState_INVALID],
        }
    }
}

/// A table of receive handler functions based on file directive code
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_CFDP_FileDirectiveDispatchTable_t {
    pub fdirective: [CF_CFDP_StateRecvFunc_t; CF_CFDP_FileDirective_INVALID_MAX],
}

impl Default for CF_CFDP_FileDirectiveDispatchTable_t {
    fn default() -> Self {
        Self {
            fdirective: [None; CF_CFDP_FileDirective_INVALID_MAX],
        }
    }
}

/// A dispatch table for receive file transactions, receive side
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_CFDP_R_SubstateDispatchTable_t {
    pub state: [*const CF_CFDP_FileDirectiveDispatchTable_t; CF_RxSubState_NUM_STATES],
}

impl Default for CF_CFDP_R_SubstateDispatchTable_t {
    fn default() -> Self {
        Self {
            state: [core::ptr::null(); CF_RxSubState_NUM_STATES],
        }
    }
}

/// A dispatch table for send file transactions, receive side
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_CFDP_S_SubstateRecvDispatchTable_t {
    pub substate: [*const CF_CFDP_FileDirectiveDispatchTable_t; CF_TxSubState_NUM_STATES],
}

impl Default for CF_CFDP_S_SubstateRecvDispatchTable_t {
    fn default() -> Self {
        Self {
            substate: [core::ptr::null(); CF_TxSubState_NUM_STATES],
        }
    }
}

/// A dispatch table for send file transactions, transmit side
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_CFDP_S_SubstateSendDispatchTable_t {
    pub substate: [CF_CFDP_StateSendFunc_t; CF_TxSubState_NUM_STATES],
}

impl Default for CF_CFDP_S_SubstateSendDispatchTable_t {
    fn default() -> Self {
        Self {
            substate: [None; CF_TxSubState_NUM_STATES],
        }
    }
}
