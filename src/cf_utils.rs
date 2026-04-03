//! CF Application general utility functions.
//!
//! Translated from: cf_utils.c / cf_utils.h
//!
//! Various odds and ends are put here.

use std::ptr;

use crate::common_types::*;
use crate::cf_platform_cfg::*;
use crate::cf_extern_typedefs::*;
use crate::cf_cfdp_pdu::*;
use crate::cf_logical_pdu::*;
use crate::cf_cfdp_types::*;
use crate::cf_clist_types::*;
use crate::cf_chunk_types::*;
use crate::cf_msg::*;
use crate::cf_eventids::*;
use crate::cf_app_types::CF_AppData_t;
use crate::cf_perfids::*;
use crate::CF_Assert;

// =====================================================================
// cFE/OSAL API stubs
// =====================================================================

unsafe fn CFE_ES_PerfLogEntry(_id: u32) {}
unsafe fn CFE_ES_PerfLogExit(_id: u32) {}
unsafe fn OS_OpenCreate(_fd: *mut osal_id_t, _fname: *const u8, _flags: i32, _access: i32) -> i32 { 0 }
unsafe fn OS_close(_fd: osal_id_t) -> i32 { 0 }
unsafe fn OS_read(_fd: osal_id_t, _buf: *mut u8, _size: usize) -> i32 { 0 }
unsafe fn OS_write(_fd: osal_id_t, _buf: *const u8, _size: usize) -> i32 { 0 }
unsafe fn OS_lseek(_fd: osal_id_t, _offset: i32, _mode: i32) -> i32 { 0 }

const CFE_EVS_EventType_ERROR: u16 = 1;
const CFE_EVS_EventType_INFORMATION: u16 = 4;

// =====================================================================
// Traverse argument types (from cf_utils.h)
// =====================================================================

/// Argument for CF_FindTransactionBySequenceNumber traversal.
///
/// C original: `CF_Traverse_TransSeqArg_t`
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CF_Traverse_TransSeqArg_t {
    pub transaction_sequence_number: CF_TransactionSeq_t,
    pub src_eid: CF_EntityId_t,
    pub txn: *mut CF_Transaction_t,
}

/// Argument for CF_WriteHistoryQueueEntryToFile traversal.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_Traverse_WriteFileArg_t {
    pub fd: osal_id_t,
    pub result: CFE_Status_t,
}

/// Argument for CF_TraverseAllTransactions.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CF_Traverse_AllTransactions_Arg_t {
    pub fn_ptr: Option<unsafe fn(*mut CF_Transaction_t, *mut core::ffi::c_void)>,
    pub context: *mut core::ffi::c_void,
    pub counter: i32,
}

/// Channel action function type.
pub type CF_ChanActionFn_t = unsafe fn(chan_num: u8, context: *mut core::ffi::c_void) -> CFE_Status_t;

/// Channel action status.
pub type CF_ChanAction_Status_t = i32;

// =====================================================================
// CF_GetChannelFromTxn
// =====================================================================

/// Get the channel pointer from a transaction.
///
/// C original: `CF_Channel_t *CF_GetChannelFromTxn(CF_Transaction_t *txn)`
pub unsafe fn CF_GetChannelFromTxn(txn: *mut CF_Transaction_t) -> *mut CF_Channel_t {
    let app = CF_AppData_ptr();
    if (*txn).chan_num < CF_NUM_CHANNELS as u8 {
        &mut (*app).engine.channels[(*txn).chan_num as usize]
    } else {
        ptr::null_mut()
    }
}

// =====================================================================
// CF_GetChunkListHead
// =====================================================================

/// Get the head of the chunk list for a channel/direction.
///
/// C original: `CF_CListNode_t **CF_GetChunkListHead(CF_Channel_t *chan, uint8 direction)`
pub unsafe fn CF_GetChunkListHead(
    chan: *mut CF_Channel_t,
    direction: u8,
) -> *mut *mut CF_CListNode_t {
    let app = CF_AppData_ptr();
    let chan_index = (chan as usize - &(*app).engine.channels[0] as *const _ as usize)
        / core::mem::size_of::<CF_Channel_t>();
    if direction == CF_Direction_RX as u8 {
        &mut (*app).engine.channels[chan_index].cs[CF_Direction_RX as usize]
    } else {
        &mut (*app).engine.channels[chan_index].cs[CF_Direction_TX as usize]
    }
}

// =====================================================================
// CF_FindUnusedTransaction
// =====================================================================

/// Find an unused transaction on a channel.
///
/// C original: `CF_Transaction_t *CF_FindUnusedTransaction(CF_Channel_t *chan, CF_Direction_t direction)`
pub unsafe fn CF_FindUnusedTransaction(
    chan: *mut CF_Channel_t,
    direction: CF_Direction_t,
) -> *mut CF_Transaction_t {
    CF_Assert(!chan.is_null());

    if (*chan).qs[CF_QueueIdx_FREE as usize].is_null() {
        return ptr::null_mut();
    }

    let node = (*chan).qs[CF_QueueIdx_FREE as usize];
    let txn = container_of!(node, CF_Transaction_t, cl_node);

    CF_CList_Remove_Ex(chan, CF_QueueIdx_FREE as CF_QueueIdx_t, &mut (*txn).cl_node);

    /* acquire a history slot */
    let q_index: usize;
    if !(*chan).qs[CF_QueueIdx_HIST_FREE as usize].is_null() {
        q_index = CF_QueueIdx_HIST_FREE as usize;
    } else {
        CF_Assert(!(*chan).qs[CF_QueueIdx_HIST as usize].is_null());
        q_index = CF_QueueIdx_HIST as usize;
    }

    (*txn).history = container_of!((*chan).qs[q_index], CF_History_t, cl_node);
    CF_CList_Remove_Ex(chan, q_index as CF_QueueIdx_t, &mut (*(*txn).history).cl_node);

    (*txn).state = CF_TxnState_INIT as u8;
    (*(*txn).history).dir = direction;

    txn
}

// =====================================================================
// CF_ResetHistory
// =====================================================================

/// Reset a history entry and move it back to the free queue.
///
/// C original: `void CF_ResetHistory(CF_Channel_t *chan, CF_History_t *history)`
pub unsafe fn CF_ResetHistory(chan: *mut CF_Channel_t, history: *mut CF_History_t) {
    crate::cf_clist::CF_CList_Remove(
        &mut (*chan).qs[CF_QueueIdx_HIST as usize],
        &mut (*history).cl_node,
    );
    crate::cf_clist::CF_CList_InsertBack(
        &mut (*chan).qs[CF_QueueIdx_HIST_FREE as usize],
        &mut (*history).cl_node,
    );
}

// =====================================================================
// CF_FreeTransaction
// =====================================================================

/// Free a transaction and return it to the free queue.
///
/// C original: `void CF_FreeTransaction(CF_Transaction_t *txn, uint8 chan)`
pub unsafe fn CF_FreeTransaction(txn: *mut CF_Transaction_t, chan: u8) {
    let app = CF_AppData_ptr();
    ptr::write_bytes(txn as *mut u8, 0, core::mem::size_of::<CF_Transaction_t>());
    (*txn).chan_num = chan;
    crate::cf_clist::CF_CList_InitNode(&mut (*txn).cl_node);
    crate::cf_clist::CF_CList_InsertBack(
        &mut (*app).engine.channels[chan as usize].qs[CF_QueueIdx_FREE as usize],
        &mut (*txn).cl_node,
    );
    (*txn).flags.com.q_index = CF_QueueIdx_FREE;
}

// =====================================================================
// CF_FindTransactionBySequenceNumber
// =====================================================================

/// Find a transaction by sequence number on a channel.
///
/// C original: `CF_Transaction_t *CF_FindTransactionBySequenceNumber(CF_Channel_t *chan, ...)`
pub unsafe fn CF_FindTransactionBySequenceNumber(
    chan: *mut CF_Channel_t,
    tsn: CF_TransactionSeq_t,
    eid: CF_EntityId_t,
) -> *mut CF_Transaction_t {
    let mut ctx = CF_Traverse_TransSeqArg_t {
        transaction_sequence_number: tsn,
        src_eid: eid,
        txn: ptr::null_mut(),
    };
    /* Search RX, PEND, TX queues in that order */
    let queues = [CF_QueueIdx_RX, CF_QueueIdx_PEND, CF_QueueIdx_TX];
    for &q in queues.iter() {
        crate::cf_clist::CF_CList_Traverse(
            (*chan).qs[q as usize],
            Some(CF_FindTransactionBySequenceNumber_Impl),
            &mut ctx as *mut _ as *mut core::ffi::c_void,
        );
        if !ctx.txn.is_null() {
            return ctx.txn;
        }
    }
    ptr::null_mut()
}

/// Traversal callback for CF_FindTransactionBySequenceNumber.
unsafe fn CF_FindTransactionBySequenceNumber_Impl(
    node: *mut CF_CListNode_t,
    arg: *mut core::ffi::c_void,
) -> i32 {
    let ctx = &mut *(arg as *mut CF_Traverse_TransSeqArg_t);
    let txn = container_of!(node, CF_Transaction_t, cl_node);
    if !(*txn).history.is_null()
        && (*(*txn).history).src_eid == ctx.src_eid
        && (*(*txn).history).seq_num == ctx.transaction_sequence_number
    {
        ctx.txn = txn;
        return 1; /* exit early */
    }
    0 /* continue */
}

// =====================================================================
// CF_WriteHistoryEntryToFile
// =====================================================================

/// Write a single history entry to a file.
///
/// C original: `CFE_Status_t CF_WriteHistoryEntryToFile(osal_id_t fd, const CF_History_t *history)`
pub unsafe fn CF_WriteHistoryEntryToFile(
    fd: osal_id_t,
    history: *const CF_History_t,
) -> CFE_Status_t {
    let mut linebuf = [0u8; CF_FILENAME_MAX_LEN * 2 + 128];
    let dir_str: &[u8] = if (*history).dir == CF_Direction_RX { b"RX" } else { b"TX" };

    /* Write 3 lines: header, src filename, dst filename */
    for i in 0..3 {
        let len = match i {
            0 => libc_snprintf!(
                linebuf.as_mut_ptr() as *mut std::os::raw::c_char,
                linebuf.len(),
                b"SEQ (%lu, %lu)\tDIR: %s\tPEER %lu\tSTAT: %d\t\0".as_ptr() as *const std::os::raw::c_char,
                (*history).src_eid as u64,
                (*history).seq_num as u64,
                dir_str.as_ptr(),
                (*history).peer_eid as u64,
                (*history).txn_stat as i32,
            ),
            1 => libc_snprintf!(
                linebuf.as_mut_ptr() as *mut std::os::raw::c_char,
                linebuf.len(),
                b"SRC: %s\t\0".as_ptr() as *const std::os::raw::c_char,
                (*history).fnames.src_filename.as_ptr(),
            ),
            _ => libc_snprintf!(
                linebuf.as_mut_ptr() as *mut std::os::raw::c_char,
                linebuf.len(),
                b"DST: %s\n\0".as_ptr() as *const std::os::raw::c_char,
                (*history).fnames.dst_filename.as_ptr(),
            ),
        };
        if len > 0 {
            let wret = CF_WrappedWrite(fd, linebuf.as_ptr(), len as usize);
            if wret != len {
                return CF_ERROR;
            }
        }
    }
    CFE_SUCCESS
}

// =====================================================================
// CF_WriteTxnQueueDataToFile / CF_WriteHistoryQueueDataToFile
// =====================================================================

pub unsafe fn CF_WriteTxnQueueDataToFile(
    fd: osal_id_t,
    chan: *mut CF_Channel_t,
    queue: CF_QueueIdx_t,
) -> CFE_Status_t {
    /* Traverse the queue and write each transaction's history to the file */
    let mut node = (*chan).qs[queue as usize];
    if node.is_null() {
        return CFE_SUCCESS;
    }
    let head = node;
    loop {
        let txn = container_of!(node, CF_Transaction_t, cl_node);
        if !(*txn).history.is_null() {
            let ret = CF_WriteHistoryEntryToFile(fd, (*txn).history);
            if ret != CFE_SUCCESS {
                return ret;
            }
        }
        node = (*node).next;
        if node == head {
            break;
        }
    }
    CFE_SUCCESS
}

pub unsafe fn CF_WriteHistoryQueueDataToFile(
    fd: osal_id_t,
    chan: *mut CF_Channel_t,
    dir: CF_Direction_t,
) -> CFE_Status_t {
    /* History queue entries are CF_History_t nodes, not transactions */
    let q = CF_QueueIdx_HIST as usize;
    let mut node = (*chan).qs[q];
    if node.is_null() {
        return CFE_SUCCESS;
    }
    let head = node;
    loop {
        let hist = container_of!(node, CF_History_t, cl_node);
        if (*hist).dir == dir {
            let ret = CF_WriteHistoryEntryToFile(fd, hist);
            if ret != CFE_SUCCESS {
                return ret;
            }
        }
        node = (*node).next;
        if node == head {
            break;
        }
    }
    CFE_SUCCESS
}

// =====================================================================
// CF_InsertSortPrio
// =====================================================================

/// Insert a transaction into a queue sorted by priority.
///
/// C original: `void CF_InsertSortPrio(CF_Transaction_t *txn, CF_QueueIdx_t queue)`
pub unsafe fn CF_InsertSortPrio(txn: *mut CF_Transaction_t, queue: CF_QueueIdx_t) {
    let chan = CF_GetChannelFromTxn(txn);
    CF_Assert(!chan.is_null());

    /* Walk the queue looking for the first node with lower priority (higher number).
     * Insert before that node. If none found, insert at back. */
    let q_head = &mut (*chan).qs[queue as usize];
    if (*q_head).is_null() {
        /* empty queue, just insert */
        crate::cf_clist::CF_CList_InsertBack(q_head, &mut (*txn).cl_node);
    } else {
        let mut node = *q_head;
        let mut inserted = false;
        loop {
            let t = container_of!(node, CF_Transaction_t, cl_node);
            if (*txn).priority < (*t).priority {
                /* insert before this node */
                crate::cf_clist::CF_CList_InsertAfter(q_head, (*node).prev, &mut (*txn).cl_node);
                inserted = true;
                break;
            }
            node = (*node).next;
            if node == *q_head {
                break;
            }
        }
        if !inserted {
            crate::cf_clist::CF_CList_InsertBack(q_head, &mut (*txn).cl_node);
        }
    }
    (*txn).flags.com.q_index = queue as u8;
}

// =====================================================================
// CF_TraverseAllTransactions
// =====================================================================

/// Traverse all transactions on a channel.
///
/// C original: `CFE_Status_t CF_TraverseAllTransactions(...)`
pub unsafe fn CF_TraverseAllTransactions(
    chan: *mut CF_Channel_t,
    fn_ptr: Option<unsafe fn(*mut CF_Transaction_t, *mut core::ffi::c_void)>,
    context: *mut core::ffi::c_void,
) -> i32 {
    let mut counter: i32 = 0;
    let queues = [CF_QueueIdx_RX, CF_QueueIdx_PEND, CF_QueueIdx_TX];
    for &q in queues.iter() {
        let mut node = (*chan).qs[q as usize];
        if node.is_null() {
            continue;
        }
        let head = node;
        loop {
            let next = (*node).next;
            let txn = container_of!(node, CF_Transaction_t, cl_node);
            if let Some(f) = fn_ptr {
                f(txn, context);
            }
            counter += 1;
            node = next;
            if node == head {
                break;
            }
        }
    }
    counter
}

pub unsafe fn CF_TraverseAllTransactions_All_Channels(
    fn_ptr: Option<unsafe fn(*mut CF_Transaction_t, *mut core::ffi::c_void)>,
    context: *mut core::ffi::c_void,
) -> i32 {
    let app = CF_AppData_ptr();
    let mut counter: i32 = 0;
    for i in 0..CF_NUM_CHANNELS {
        counter += CF_TraverseAllTransactions(
            &mut (*app).engine.channels[i],
            fn_ptr,
            context,
        );
    }
    counter
}

// =====================================================================
// OS Wrapper functions
// =====================================================================

/// Wrapped file open/create.
///
/// C original: `CFE_Status_t CF_WrappedOpenCreate(osal_id_t *fd, const char *fname, int32 flags, int32 access)`
pub unsafe fn CF_WrappedOpenCreate(
    fd: *mut osal_id_t,
    fname: *const u8,
    flags: i32,
    access: i32,
) -> CFE_Status_t {
    CFE_ES_PerfLogEntry(CF_PERF_ID_FOPEN);
    let ret = OS_OpenCreate(fd, fname, flags, access);
    CFE_ES_PerfLogExit(CF_PERF_ID_FOPEN);
    ret
}

/// Wrapped file close.
pub unsafe fn CF_WrappedClose(fd: osal_id_t) {
    CFE_ES_PerfLogEntry(CF_PERF_ID_FCLOSE);
    OS_close(fd);
    CFE_ES_PerfLogExit(CF_PERF_ID_FCLOSE);
}

/// Wrapped file read.
pub unsafe fn CF_WrappedRead(fd: osal_id_t, buf: *mut u8, read_size: usize) -> CFE_Status_t {
    CFE_ES_PerfLogEntry(CF_PERF_ID_FREAD);
    let ret = OS_read(fd, buf, read_size);
    CFE_ES_PerfLogExit(CF_PERF_ID_FREAD);
    ret
}

/// Wrapped file write.
pub unsafe fn CF_WrappedWrite(fd: osal_id_t, buf: *const u8, write_size: usize) -> CFE_Status_t {
    CFE_ES_PerfLogEntry(CF_PERF_ID_FWRITE);
    let ret = OS_write(fd, buf, write_size);
    CFE_ES_PerfLogExit(CF_PERF_ID_FWRITE);
    ret
}

/// Wrapped file seek.
pub unsafe fn CF_WrappedLseek(fd: osal_id_t, offset: i32, mode: i32) -> CFE_Status_t {
    CFE_ES_PerfLogEntry(CF_PERF_ID_FSEEK);
    let ret = OS_lseek(fd, offset, mode);
    CFE_ES_PerfLogExit(CF_PERF_ID_FSEEK);
    ret
}

// =====================================================================
// Condition code / TxnStatus conversion
// =====================================================================

/// Convert a TxnStatus to a CFDP condition code.
pub fn CF_TxnStatus_To_ConditionCode(txn_stat: CF_TxnStatus_t) -> CF_CFDP_ConditionCode_t {
    // The C code does a direct cast
    unsafe { core::mem::transmute(txn_stat as u8) }
}

/// Convert a CFDP condition code to a TxnStatus.
pub fn CF_TxnStatus_From_ConditionCode(cc: CF_CFDP_ConditionCode_t) -> CF_TxnStatus_t {
    // In C this is a direct cast between enums with matching values 0-15
    unsafe { core::mem::transmute(cc as i32) }
}

// =====================================================================
// Inline helpers from cf_utils.h
// =====================================================================

/// Move a transaction to a different queue.
///
/// C original: `static inline void CF_MoveTransaction(CF_Transaction_t *txn, CF_QueueIdx_t queue)`
pub unsafe fn CF_MoveTransaction(txn: *mut CF_Transaction_t, queue: CF_QueueIdx_t) {
    let chan = CF_GetChannelFromTxn(txn);
    if !chan.is_null() {
        let old_q = (*txn).flags.com.q_index;
        crate::cf_clist::CF_CList_Remove(
            &mut (*chan).qs[old_q as usize],
            &mut (*txn).cl_node,
        );
        crate::cf_clist::CF_CList_InsertBack(
            &mut (*chan).qs[queue as usize],
            &mut (*txn).cl_node,
        );
        (*txn).flags.com.q_index = queue as u8;
    }
}

/// Remove a node from a channel queue.
pub unsafe fn CF_CList_Remove_Ex(
    chan: *mut CF_Channel_t,
    queueidx: CF_QueueIdx_t,
    node: *mut CF_CListNode_t,
) {
    crate::cf_clist::CF_CList_Remove(&mut (*chan).qs[queueidx as usize], node);
}

/// Insert a node after another in a channel queue.
pub unsafe fn CF_CList_InsertAfter_Ex(
    chan: *mut CF_Channel_t,
    queueidx: CF_QueueIdx_t,
    start: *mut CF_CListNode_t,
    after: *mut CF_CListNode_t,
) {
    crate::cf_clist::CF_CList_InsertAfter(
        &mut (*chan).qs[queueidx as usize],
        start,
        after,
    );
}

/// Returns a mutable pointer to the global CF_AppData singleton.
unsafe fn CF_AppData_ptr() -> *mut CF_AppData_t {
    crate::cf_dispatch::CF_AppData_ptr()
}
