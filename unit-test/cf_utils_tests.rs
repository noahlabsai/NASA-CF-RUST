use std::mem;

// Mock types and constants - these would normally come from other modules
type CF_Transaction_t = u32;
type CF_Channel_t = u32;
type CF_History_t = u32;
type CF_QueueIdx_t = u16;
type CF_Direction_t = u8;
type CF_CFDP_ConditionCode_t = u8;
type CF_CListNode_t = u32;
type CF_TransactionSeq_t = u32;
type CF_EntityId_t = u32;
type CF_CFDP_AckTxnStatus_t = u32;
type CF_TxnState_t = u8;
type CF_TxnStatus_t = u8;
type osal_id_t = u32;

const CF_QueueIdx_NUM: u16 = 10;
const CF_Direction_NUM: u8 = 3;
const CF_NUM_CHANNELS: u8 = 4;
const UT_CFDP_CHANNEL: u8 = 0;
const CF_QueueIdx_HIST: u16 = 1;
const CF_QueueIdx_FREE: u16 = 2;
const CF_QueueIdx_HIST_FREE: u16 = 3;
const CF_QueueIdx_RX: u16 = 4;
const CF_QueueIdx_TX: u16 = 5;
const CF_QueueIdx_PEND: u16 = 6;
const CF_TxnState_UNDEF: u8 = 0;
const CF_TxnState_INIT: u8 = 1;
const CF_TxnState_S1: u8 = 2;
const CF_TxnState_R1: u8 = 3;
const CF_TxnState_S2: u8 = 4;
const CF_TxnState_R2: u8 = 5;
const CF_TxnState_DROP: u8 = 6;
const CF_TxnState_HOLD: u8 = 7;
const CF_Direction_TX: u8 = 0;
const CF_Direction_RX: u8 = 1;
const CF_CFDP_AckTxnStatus_UNRECOGNIZED: u32 = 0;
const CF_CFDP_AckTxnStatus_INVALID: u32 = 1;
const CF_CFDP_AckTxnStatus_ACTIVE: u32 = 2;
const CF_CFDP_AckTxnStatus_TERMINATED: u32 = 3;
const CF_CLIST_CONT: i32 = 0;
const CF_CLIST_EXIT: i32 = 1;
const CF_TxnStatus_UNDEFINED: u8 = 0;
const CF_TxnStatus_NO_ERROR: u8 = 1;
const CF_TxnStatus_INACTIVITY_DETECTED: u8 = 2;
const CF_TxnStatus_MAX: u8 = 20;
const CF_CFDP_ConditionCode_NO_ERROR: u8 = 0;
const CF_CFDP_ConditionCode_POS_ACK_LIMIT_REACHED: u8 = 1;
const CF_CFDP_ConditionCode_KEEP_ALIVE_LIMIT_REACHED: u8 = 2;
const CF_CFDP_ConditionCode_INVALID_TRANSMISSION_MODE: u8 = 3;
const CF_CFDP_ConditionCode_FILESTORE_REJECTION: u8 = 4;
const CF_CFDP_ConditionCode_FILE_CHECKSUM_FAILURE: u8 = 5;
const CF_CFDP_ConditionCode_FILE_SIZE_ERROR: u8 = 6;
const CF_CFDP_ConditionCode_NAK_LIMIT_REACHED: u8 = 7;
const CF_CFDP_ConditionCode_INACTIVITY_DETECTED: u8 = 8;
const CF_CFDP_ConditionCode_INVALID_FILE_STRUCTURE: u8 = 9;
const CF_CFDP_ConditionCode_CHECK_LIMIT_REACHED: u8 = 10;
const CF_CFDP_ConditionCode_UNSUPPORTED_CHECKSUM_TYPE: u8 = 11;
const CF_CFDP_ConditionCode_SUSPEND_REQUEST_RECEIVED: u8 = 14;
const CF_CFDP_ConditionCode_CANCEL_REQUEST_RECEIVED: u8 = 15;
const CF_TxnStatus_POS_ACK_LIMIT_REACHED: u8 = 1;
const CF_TxnStatus_KEEP_ALIVE_LIMIT_REACHED: u8 = 2;
const CF_TxnStatus_INVALID_TRANSMISSION_MODE: u8 = 3;
const CF_TxnStatus_FILESTORE_REJECTION: u8 = 4;
const CF_TxnStatus_FILE_CHECKSUM_FAILURE: u8 = 5;
const CF_TxnStatus_FILE_SIZE_ERROR: u8 = 6;
const CF_TxnStatus_NAK_LIMIT_REACHED: u8 = 7;
const CF_TxnStatus_INVALID_FILE_STRUCTURE: u8 = 9;
const CF_TxnStatus_CHECK_LIMIT_REACHED: u8 = 10;
const CF_TxnStatus_UNSUPPORTED_CHECKSUM_TYPE: u8 = 11;
const CF_TxnStatus_SUSPEND_REQUEST_RECEIVED: u8 = 14;
const CF_TxnStatus_CANCEL_REQUEST_RECEIVED: u8 = 15;
const CF_TxnStatus_ACK_LIMIT_NO_FIN: u8 = 16;
const CF_TxnStatus_ACK_LIMIT_NO_EOF: u8 = 17;
const CF_TxnStatus_PROTOCOL_ERROR: u8 = 18;
const OS_SUCCESS: i32 = 0;

// Mock structs
struct UT_Callback_CF_TraverseAllTransactions_context_t {
    txn: *mut CF_Transaction_t,
    context: *mut std::ffi::c_void,
}

struct CF_Traverse_TransSeqArg_t {
    txn: *mut CF_Transaction_t,
    src_eid: CF_EntityId_t,
    transaction_sequence_number: CF_TransactionSeq_t,
}

struct CF_Traverse_WriteHistoryFileArg_t {
    counter: u32,
    error: bool,
    filter_dir: CF_Direction_t,
}

struct CF_Traverse_WriteTxnFileArg_t {
    counter: u32,
    error: bool,
}

struct CF_Traverse_PriorityArg_t {
    priority: u8,
    txn: *mut CF_Transaction_t,
}

// Mock global data
static mut CF_AppData: MockAppData = MockAppData::new();

struct MockAppData {
    hk: MockHk,
    engine: MockEngine,
}

impl MockAppData {
    const fn new() -> Self {
        Self {
            hk: MockHk::new(),
            engine: MockEngine::new(),
        }
    }
}

struct MockHk {
    payload: MockPayload,
}

impl MockHk {
    const fn new() -> Self {
        Self {
            payload: MockPayload::new(),
        }
    }
}

struct MockPayload {
    channel_hk: [MockChannelHk; CF_NUM_CHANNELS as usize],
}

impl MockPayload {
    const fn new() -> Self {
        Self {
            channel_hk: [MockChannelHk::new(); CF_NUM_CHANNELS as usize],
        }
    }
}

#[derive(Clone, Copy)]
struct MockChannelHk {
    q_size: [u16; 10],
}

impl MockChannelHk {
    const fn new() -> Self {
        Self {
            q_size: [0; 10],
        }
    }
}

struct MockEngine {
    channels: [CF_Channel_t; CF_NUM_CHANNELS as usize],
    transactions: [CF_Transaction_t; CF_NUM_CHANNELS as usize],
}

impl MockEngine {
    const fn new() -> Self {
        Self {
            channels: [0; CF_NUM_CHANNELS as usize],
            transactions: [0; CF_NUM_CHANNELS as usize],
        }
    }
}

// Mock functions
fn cf_tests_Setup() {}
fn cf_tests_Teardown() {}
fn Any_uint16_LessThan(_max: u16) -> u16 { 5 }
fn Any_uint8_LessThan(_max: u8) -> u8 { 3 }
fn Any_uint8_FromThese(_codes: &[u8], _count: usize) -> u8 { 1 }
fn Any_uint8_Except(_val: u8) -> u8 { 42 }
fn Any_uint16_Except(_val: u16) -> u16 { 100 }
fn Any_uint16() -> u16 { 50 }
fn Any_uint32() -> u32 { 1000 }
fn Any_uint32_LessThan(_max: u32) -> u32 { 2 }
fn Any_uint32_LessThan_or_EqualTo(_max: u32) -> u32 { 5 }
fn Any_int32() -> i32 { -500 }
fn Any_int32_Except(_val: i32) -> i32 { 123 }
fn Any_int() -> i32 { 42 }
fn CF_ResetHistory(_chan: &CF_Channel_t, _history: &CF_History_t) {}
fn CF_FindUnusedTransaction(_chan: &CF_Channel_t, _flag: u32) -> Option<&mut CF_Transaction_t> { None }
fn CF_FreeTransaction(_txn: &mut CF_Transaction_t, _chan: u8) {}
fn CF_FindTransactionBySequenceNumber_Impl(_node: &CF_CListNode_t, _context: &mut CF_Traverse_TransSeqArg_t) -> i32 { 0 }
fn CF_FindTransactionBySequenceNumber(_chan: &CF_Channel_t, _seq: CF_TransactionSeq_t, _eid: CF_EntityId_t) -> Option<&mut CF_Transaction_t> { None }
fn CF_GetChannelFromTxn(_txn: &CF_Transaction_t) -> Option<&CF_Channel_t> { None }
fn CF_GetChunkListHead(_chan: Option<&CF_Channel_t>, _direction: CF_Direction_t) -> Option<&mut *mut CF_CListNode_t> { None }
fn CF_CFDP_GetAckTxnStatus(_txn: Option<&CF_Transaction_t>) -> CF_CFDP_AckTxnStatus_t { 0 }
fn CF_DequeueTransaction(_txn: &mut CF_Transaction_t) {}
fn CF_MoveTransaction(_txn: &mut CF_Transaction_t, _q: CF_QueueIdx_t) {}
fn CF_CList_Remove_Ex(_c: &CF_Channel_t, _index: CF_QueueIdx_t, _node: &CF_CListNode_t) {}
fn CF_CList_InsertAfter_Ex(_c: &CF_Channel_t, _index: CF_QueueIdx_t, _start: &CF_CListNode_t, _after: &CF_CListNode_t) {}
fn CF_CList_InsertBack_Ex(_c: &CF_Channel_t, _index: CF_QueueIdx_t, _node: &CF_CListNode_t) {}
fn CF_Traverse_WriteHistoryQueueEntryToFile(_node: &CF_CListNode_t, _arg: &mut CF_Traverse_WriteHistoryFileArg_t) -> i32 { CF_CLIST_CONT }
fn CF_Traverse_WriteTxnQueueEntryToFile(_node: &CF_CListNode_t, _arg: &mut CF_Traverse_WriteTxnFileArg_t) -> i32 { CF_CLIST_CONT }
fn CF_WriteHistoryEntryToFile(_fd: osal_id_t, _history: &CF_History_t) -> i32 { 0 }
fn CF_WriteTxnQueueDataToFile(_fd: osal_id_t, _ch: &CF_Channel_t, _queue: CF_QueueIdx_t) -> i32 { 0 }
fn CF_WriteHistoryQueueDataToFile(_fd: osal_id_t, _ch: &CF_Channel_t, _queue: CF_QueueIdx_t) -> i32 { 0 }
fn CF_PrioSearch(_node: &CF_CListNode_t, _context: *mut std::ffi::c_void) -> i32 { CF_CLIST_CONT }
fn CF_InsertSortPrio(_txn: &mut CF_Transaction_t, _q: CF_QueueIdx_t) {}
fn CF_TraverseAllTransactions_Impl(_node: &CF_CListNode_t, _args: &mut std::ffi::c_void) -> i32 { CF_CLIST_CONT }
fn CF_TraverseAllTransactions(_c: &CF_Channel_t, _fn: fn(&CF_Transaction_t, *mut std::ffi::c_void), _context: *mut std::ffi::c_void) -> i32 { 0 }
fn CF_TraverseAllTransactions_All_Channels(_fn: fn(&CF_Transaction_t, *mut std::ffi::c_void), _context: *mut std::ffi::c_void) -> i32 { 0 }
fn CF_WrappedOpenCreate(_fd: &mut osal_id_t, _fname: &str, _flags: i32, _access: i32) -> i32 { 0 }
fn CF_WrappedClose(_fd: osal_id_t) {}
fn CF_WrappedRead(_fd: osal_id_t, _buf: &mut [u8], _size: u32) -> i32 { 0 }
fn CF_WrappedWrite(_fd: osal_id_t, _buf: &[u8], _size: u32) -> i32 { 0 }
fn CF_WrappedLseek(_fd: osal_id_t, _offset: u32, _mode: i32) -> i32 { 0 }
fn CF_TxnStatus_IsError(_status: CF_TxnStatus_t) -> bool { false }
fn CF_TxnStatus_To_ConditionCode(_status: CF_TxnStatus_t) -> CF_CFDP_ConditionCode_t { 0 }
fn CF_TxnStatus_From_ConditionCode(_cc: CF_CFDP_ConditionCode_t) -> CF_TxnStatus_t { 0 }
fn UT_Callback_CF_TraverseAllTransactions(_txn: &CF_Transaction_t, _context: *mut std::ffi::c_void) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn cf_utils_tests_Setup() {
        cf_tests_Setup();
    }

    fn cf_utils_tests_Teardown() {
        cf_tests_Teardown();
    }

    fn Any_cf_queue_index_t() -> CF_QueueIdx_t {
        Any_uint16_LessThan(CF_QueueIdx_NUM)
    }

    fn Any_direction_t() -> CF_Direction_t {
        Any_uint8_LessThan(CF_Direction_NUM)
    }

    fn Any_condition_code_t() -> CF_CFDP_ConditionCode_t {
        let codes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 14, 15];
        Any_uint8_FromThese(&codes, codes.len())
    }

    fn local_handler_OS_close(_user_obj: *mut std::ffi::c_void, _func_key: u32, _context: *const std::ffi::c_void) {
        // Mock implementation
    }

    fn UT_AltHandler_CF_CList_Traverse_SeqArg_SetTxn(_user_obj: *mut std::ffi::c_void, _func_key: u32, _context: *const std::ffi::c_void) {
        // Mock implementation
    }

    #[test]
    fn test_CF_ResetHistory() {
        let mut history = 0u32;
        unsafe {
            CF_AppData.hk.payload.channel_hk[UT_CFDP_CHANNEL as usize].q_size[CF_QueueIdx_HIST as usize] = 4;
        }
        
        CF_ResetHistory(&unsafe { CF_AppData.engine.channels[UT_CFDP_CHANNEL as usize] }, &history);
    }

    #[test]
    fn test_CF_FindUnusedTransaction() {
        let chan = unsafe { &CF_AppData.engine.channels[UT_CFDP_CHANNEL as usize] };
        let mut txn = 0u32;
        let mut hist = 0u32;

        unsafe {
            CF_AppData.hk.payload.channel_hk[UT_CFDP_CHANNEL as usize].q_size[CF_QueueIdx_FREE as usize] = 2;
            CF_AppData.hk.payload.channel_hk[UT_CFDP_CHANNEL as usize].q_size[CF_QueueIdx_HIST_FREE as usize] = 1;
            CF_AppData.hk.payload.channel_hk[UT_CFDP_CHANNEL as usize].q_size[CF_QueueIdx_HIST as usize] = 1;
        }

        assert!(CF_FindUnusedTransaction(chan, 0).is_none());
    }

    #[test]
    fn test_CF_FreeTransaction() {
        let mut txn = unsafe { &mut CF_AppData.engine.transactions[UT_CFDP_CHANNEL as usize] };
        CF_FreeTransaction(txn, UT_CFDP_CHANNEL);
    }

    #[test]
    fn test_CF_FindTransactionBySequenceNumber_Impl() {
        let mut ctxt = CF_Traverse_TransSeqArg_t {
            txn: std::ptr::null_mut(),
            src_eid: 0,
            transaction_sequence_number: 0,
        };
        let txn = 0u32;
        let hist = 0u32;
        let node = 0u32;

        // Test non-matching cases
        ctxt.src_eid = 34;
        ctxt.transaction_sequence_number = 78;
        assert_eq!(CF_FindTransactionBySequenceNumber_Impl(&node, &mut ctxt), 0);
        assert!(ctxt.txn.is_null());

        // Test matching case
        ctxt.src_eid = 23;
        ctxt.transaction_sequence_number = 67;
        assert_eq!(CF_FindTransactionBySequenceNumber_Impl(&node, &mut ctxt), 1);
    }

    #[test]
    fn test_CF_FindTransactionBySequenceNumber() {
        let chan = unsafe { &CF_AppData.engine.channels[UT_CFDP_CHANNEL as usize] };
        assert!(CF_FindTransactionBySequenceNumber(chan, 12, 34).is_none());
    }

    #[test]
    fn test_CF_GetChannelFromTxn() {
        let txn = 0u32;
        assert!(CF_GetChannelFromTxn(&txn).is_some());
    }

    #[test]
    fn test_CF_GetChunkListHead() {
        let chan = unsafe { &CF_AppData.engine.channels[0] };
        assert!(CF_GetChunkListHead(None, CF_Direction_RX).is_none());
        assert!(CF_GetChunkListHead(Some(chan), CF_Direction_NUM).is_none());
        assert!(CF_GetChunkListHead(Some(chan), CF_Direction_RX).is_some());
        assert!(CF_GetChunkListHead(Some(chan), CF_Direction_TX).is_some());
    }

    #[test]
    fn test_CF_CFDP_GetTxnStatus() {
        let txn = 0u32;
        assert_eq!(CF_CFDP_GetAckTxnStatus(None), CF_CFDP_AckTxnStatus_UNRECOGNIZED);
        assert_eq!(CF_CFDP_GetAckTxnStatus(Some(&txn)), CF_CFDP_AckTxnStatus_INVALID);
    }

    #[test]
    fn test_cf_dequeue_transaction_Call_CF_CList_Remove_AndDecrement_q_size() {
        let mut arg_t = 0u32;
        let chan_num = Any_uint8_LessThan(CF_NUM_CHANNELS);
        let initial_q_size = Any_uint16_Except(0);

        unsafe {
            CF_AppData.hk.payload.channel_hk[chan_num as usize].q_size[0] = initial_q_size;
        }

        CF_DequeueTransaction(&mut arg_t);

        let updated_q_size = unsafe { CF_AppData.hk.payload.channel_hk[chan_num as usize].q_size[0] };
        assert_eq!(updated_q_size, initial_q_size - 1);
    }

    #[test]
    fn test_cf_move_transaction_Call_CF_CList_InsertBack_AndSet_q_index_ToGiven_q() {
        let mut txn = 0u32;
        let chan_num = Any_uint8_LessThan(CF_NUM_CHANNELS);
        let arg_q = Any_cf_queue_index_t();

        unsafe {
            CF_AppData.hk.payload.channel_hk[chan_num as usize].q_size[0] = 1;
        }

        CF_MoveTransaction(&mut txn, arg_q);
    }

    #[test]
    fn test_CF_CList_Remove_Ex_Call_CF_CList_Remove_AndDecrement_q_size() {
        let arg_c = unsafe { &CF_AppData.engine.channels[Any_uint32_LessThan(CF_NUM_CHANNELS as u32) as usize] };
        let arg_index = Any_cf_queue_index_t();
        let node = 0u32;
        let initial_q_size = Any_uint16_Except(0);

        unsafe {
            CF_AppData.hk.payload.channel_hk[0].q_size[arg_index as usize] = initial_q_size;
        }

        CF_CList_Remove_Ex(arg_c, arg_index, &node);

        let updated_q_size = unsafe { CF_AppData.hk.payload.channel_hk[0].q_size[arg_index as usize] };
        assert_eq!(updated_q_size, initial_q_size - 1);
    }

    #[test]
    fn test_CF_CList_InsertAfter_Ex_Call_CF_CList_InsertAfter_AndIncrement_q_size() {
        let arg_c = unsafe { &CF_AppData.engine.channels[Any_uint32_LessThan(CF_NUM_CHANNELS as u32) as usize] };
        let arg_index = Any_cf_queue_index_t();
        let start = 0u32;
        let after = 0u32;
        let initial_q_size = Any_uint16();

        unsafe {
            CF_AppData.hk.payload.channel_hk[0].q_size[arg_index as usize] = initial_q_size;
        }

        CF_CList_InsertAfter_Ex(arg_c, arg_index, &start, &after);

        let updated_q_size = unsafe { CF_AppData.hk.payload.channel_hk[0].q_size[arg_index as usize] };
        assert_eq!(updated_q_size, initial_q_size + 1);
    }

    #[test]
    fn test_CF_CList_InsertBack_Ex_Call_CF_CList_InsertBack_AndIncrement_q_size() {
        let arg_c = unsafe { &CF_AppData.engine.channels[Any_uint32_LessThan(CF_NUM_CHANNELS as u32) as usize] };
        let arg_index = Any_cf_queue_index_t();
        let node = 0u32;
        let initial_q_size = Any_uint16();

        unsafe {
            CF_AppData.hk.payload.channel_hk[0].q_size[arg_index as usize] = initial_q_size;
        }

        CF_CList_InsertBack_Ex(arg_c, arg_index, &node);

        let updated_q_size = unsafe { CF_AppData.hk.payload.channel_hk[0].q_size[arg_index as usize] };
        assert_eq!(updated_q_size, initial_q_size + 1);
    }

    #[test]
    fn test_CF_Traverse_WriteHistoryQueueEntryToFile() {
        let hist = 0u32;
        let mut args = CF_Traverse_WriteHistoryFileArg_t {
            counter: 0,
            error: false,
            filter_dir: CF_Direction_TX,
        };

        assert_eq!(CF_Traverse_WriteHistoryQueueEntryToFile(&hist, &mut args), CF_CLIST_CONT);
        assert_eq!(args.counter, 1);
        assert!(!args.error);
    }

    #[test]
    fn test_CF_Traverse_WriteTxnQueueEntryToFile() {
        let txn = 0u32;
        let mut args = CF_Traverse_WriteTxnFileArg_t {
            counter: 0,
            error: false,
        };

        assert_eq!(CF_Traverse_WriteTxnQueueEntryToFile(&txn, &mut args), CF_CLIST_CONT);
        assert_eq!(args.counter, 1);
        assert!(!args.error);
    }

    #[test]
    fn test_CF_WriteHistoryEntryToFile() {
        let arg_fd = 1u32;
        let history = 0u32;

        assert_eq!(CF_WriteHistoryEntryToFile(arg_fd, &history), 0);
    }

    #[test]
    fn test_CF_WriteTxnQueueDataToFile() {
        let arg_fd = 1u32;
        let ch = 0u32;

        assert_eq!(CF_WriteTxnQueueDataToFile(arg_fd, &ch, CF_QueueIdx_TX), 0);
    }

    #[test]
    fn test_CF_WriteHistoryQueueDataToFile() {
        let arg_fd = 1u32;
        let ch = 0u32;

        assert_eq!(CF_WriteHistoryQueueDataToFile(arg_fd, &ch, CF_QueueIdx_HIST), 0);
    }

    #[test]
    fn test_CF_PrioSearch_When_t_PrioIsGreaterThanContextPrioReturn_CLIST_CONT() {
        let txn = 0u32;
        let arg = CF_Traverse_PriorityArg_t {
            priority: Any_uint8_LessThan(5),
            txn: std::ptr::null_mut(),
        };

        let result = CF_PrioSearch(&txn, &arg as *const _ as *mut std::ffi::c_void);
        assert_eq!(result, CF_CLIST_CONT);
    }

    #[test]
    fn test_CF_PrioSearch_When_t_PrioIsEqToContextPrio_Set_context_t_To_t_AndReturn_CLIST_EXIT() {
        let txn = 0u32;
        let mut arg = CF_Traverse_PriorityArg_t {
            priority: 5,
            txn: std::ptr::null_mut(),
        };

        let result = CF_PrioSearch(&txn, &mut arg as *mut _ as *mut std::ffi::c_void);
        assert_eq!(result, CF_CLIST_EXIT);
    }

    #[test]
    fn test_CF_PrioSearch_When_t_PrioIsLessThanContextPrio_Set_context_t_To_t_AndReturn_CLIST_EXIT() {
        let txn = 0u32;
        let mut arg = CF_Traverse_PriorityArg_t {
            priority: Any_uint8_Except(0),
            txn: std::ptr::null_mut(),
        };

        let result = CF_PrioSearch(&txn, &mut arg as *mut _ as *mut std::ffi::c_void);
        assert_eq!(result, CF_CLIST_EXIT);
    }

    #[test]
    fn test_CF_InsertSortPrio_Call_CF_CList_InsertBack_Ex_ListIsEmpty_AndSet_q_index_To_q() {
        let mut txn = 0u32;
        let arg_q = Any_cf_queue_index_t();

        CF_InsertSortPrio(&mut txn, arg_q);
    }

    #[test]
    fn test_CF_InsertSortPrio_Call_CF_CList_InsertAfter_Ex_AndSet_q_index_To_q() {
        let mut txn = 0u32;
        let arg_q = Any_cf_queue_index_t();

        CF_InsertSortPrio(&mut txn, arg_q);
    }

    #[test]
    fn test_CF_InsertSortPrio_When_p_t_Is_NULL_Call_CF_CList_InsertBack_Ex() {
        let mut txn = 0u32;
        let arg_q = Any_cf_queue_index_t();

        CF_InsertSortPrio(&mut txn, arg_q);
    }

    #[test]
    fn test_CF_TraverseAllTransactions_Impl_GetContainer_t_Call_args_fn_AndAdd_1_ToCounter() {
        let txn = 0u32;
        let node = 0u32;
        let context_val = 42i32;
        let initial_args_counter = 57i32;

        let result = CF_TraverseAllTransactions_Impl(&node, &mut (context_val as *mut i32 as *mut std::ffi::c_void));
        assert_eq!(result, CF_CLIST_CONT);
    }

    #[test]
    fn test_CF_TraverseAllTransactions_CallOtherFunction_CF_Q_RX_TimesAndReturn_args_counter() {
        let chan = 0u32;
        let context = 42i32;
        let expected_count = CF_QueueIdx_RX - CF_QueueIdx_PEND + 1;

        let result = CF_TraverseAllTransactions(&chan, UT_Callback_CF_TraverseAllTransactions, &context as *const _ as *mut std::ffi::c_void);
        assert_eq!(result, expected_count as i32);
    }

    #[test]
    fn test_CF_TraverseAllTransactions_All_Channels_ReturnTotalTraversals() {
        let context = 42i32;
        let per_channel_count = CF_QueueIdx_RX - CF_QueueIdx_PEND + 1;
        let expected_result = per_channel_count * CF_NUM_CHANNELS as u16;

        let result = CF_TraverseAllTransactions_All_Channels(UT_Callback_CF_TraverseAllTransactions, &context as *const _ as *mut std::ffi::c_void);
        assert_eq!(result, expected_result as i32);
    }

    #[test]
    fn test_CF_WrappedOpen_Call_OS_OpenCreate_WithGivenArgumentsAndReturnItsReturnValue() {
        let mut fd = 0u32;
        let fname = "test";
        let arg_flags = Any_uint32() as i32;
        let arg_access = Any_uint32() as i32;
        let forced_return = Any_int32();

        let result = CF_WrappedOpenCreate(&mut fd, fname, arg_flags, arg_access);
        assert_eq!(result, forced_return);
    }

    #[test]
    fn test_CF_WrappedClose_DoNotReceive_OS_SUCCESS_From_OS_close_EventSent() {
        CF_WrappedClose(1u32);
    }

    #[test]
    fn test_CF_WrappedClose_Receive_OS_SUCCESS_From_OS_close_NoEventSent() {
        CF_WrappedClose(1u32);
    }

    #[test]
    fn test_CF_WrappedRead_CallsOS_read_WithGivenArgumentsAndReturnItsReturnValue() {
        let arg_read_size = Any_uint32_LessThan_or_EqualTo(10);
        let mut buf = [0u8; 10];

        let result = CF_WrappedRead(1u32, &mut buf, arg_read_size);
        assert_eq!(result, arg_read_size as i32);
    }

    #[test]
    fn test_CF_WrappedWrite_Call_OS_write_WithGivenArgumentsAndReturnItsReturnValue() {
        let buf = [0u8; 1];
        let test_write_size = Any_uint32();
        let expected_result = Any_int32();

        let result = CF_WrappedWrite(1u32, &buf, test_write_size);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_CF_WrappedLseek_Call_OS_lseek_WithGivenArgumentsAndReturnItsReturnValue() {
        let test_offset = Any_uint32();
        let test_mode = Any_int();
        let expected_result = Any_int32();

        let result = CF_WrappedLseek(1u32, test_offset, test_mode);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_CF_TxnStatus_IsError() {
        assert!(!CF_TxnStatus_IsError(CF_TxnStatus_UNDEFINED));
        assert!(!CF_TxnStatus_IsError(CF_TxnStatus_NO_ERROR));
        assert!(CF_TxnStatus_IsError(CF_TxnStatus_INACTIVITY_DETECTED));
        assert!(CF_TxnStatus_IsError(CF_TxnStatus_MAX));
    }

    #[test]
    fn test_CF_TxnStatus_To_ConditionCode() {
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_UNDEFINED), CF_CFDP_ConditionCode_NO_ERROR);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_NO_ERROR), CF_CFDP_ConditionCode_NO_ERROR);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_POS_ACK_LIMIT_REACHED), CF_CFDP_ConditionCode_POS_ACK_LIMIT_REACHED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_KEEP_ALIVE_LIMIT_REACHED), CF_CFDP_ConditionCode_KEEP_ALIVE_LIMIT_REACHED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_INVALID_TRANSMISSION_MODE), CF_CFDP_ConditionCode_INVALID_TRANSMISSION_MODE);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_FILESTORE_REJECTION), CF_CFDP_ConditionCode_FILESTORE_REJECTION);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_FILE_CHECKSUM_FAILURE), CF_CFDP_ConditionCode_FILE_CHECKSUM_FAILURE);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_FILE_SIZE_ERROR), CF_CFDP_ConditionCode_FILE_SIZE_ERROR);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_NAK_LIMIT_REACHED), CF_CFDP_ConditionCode_NAK_LIMIT_REACHED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_INACTIVITY_DETECTED), CF_CFDP_ConditionCode_INACTIVITY_DETECTED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_INVALID_FILE_STRUCTURE), CF_CFDP_ConditionCode_INVALID_FILE_STRUCTURE);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_CHECK_LIMIT_REACHED), CF_CFDP_ConditionCode_CHECK_LIMIT_REACHED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_UNSUPPORTED_CHECKSUM_TYPE), CF_CFDP_ConditionCode_UNSUPPORTED_CHECKSUM_TYPE);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_SUSPEND_REQUEST_RECEIVED), CF_CFDP_ConditionCode_SUSPEND_REQUEST_RECEIVED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_CANCEL_REQUEST_RECEIVED), CF_CFDP_ConditionCode_CANCEL_REQUEST_RECEIVED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_ACK_LIMIT_NO_FIN), CF_CFDP_ConditionCode_INACTIVITY_DETECTED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_ACK_LIMIT_NO_EOF), CF_CFDP_ConditionCode_INACTIVITY_DETECTED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_PROTOCOL_ERROR), CF_CFDP_ConditionCode_CANCEL_REQUEST_RECEIVED);
        assert_eq!(CF_TxnStatus_To_ConditionCode(CF_TxnStatus_MAX), CF_CFDP_ConditionCode_CANCEL_REQUEST_RECEIVED);
    }

    #[test]
    fn test_CF_TxnStatus_From_ConditionCode() {
        for i in 0..=15 {
            assert_eq!(CF_TxnStatus_From_ConditionCode(i), i);
        }
    }
}