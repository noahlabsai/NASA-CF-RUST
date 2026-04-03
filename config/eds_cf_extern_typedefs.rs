use std::os::raw::{c_char, c_uint};

// Type mappings for CF-specific types
pub type CF_QueueIdx_t = CF_QueueIdx_Enum_t;

// Constants
pub const CF_QueueIdx_NUM: usize = 1 + EdsDataType_EdsEnum_CF_QueueIdx_t_MAX as usize;
pub const CF_GetSet_ValueID_MAX: usize = 1 + EdsDataType_EdsEnum_CF_GetSet_ValueID_t_MAX as usize;

// Additional type mappings
pub type CF_EntityId_t = CF_EntityId_Atom_t;
pub type CF_TransactionSeq_t = CF_TransactionSeq_Atom_t;
pub type CF_CFDP_Class_t = CF_CFDP_Enum_t;
pub type CF_GetSet_ValueID_t = CF_GetSet_ValueID_Enum_t;
pub type CF_PathName_t = EdsDataType_BASE_TYPES_PathName_t;
pub type CF_FileName_t = EdsDataType_BASE_TYPES_FileName_t;

// Placeholder types for EDS-sourced definitions that would be imported
// These would normally come from cf_eds_typedefs module
pub type CF_QueueIdx_Enum_t = u32;
pub type CF_EntityId_Atom_t = u32;
pub type CF_TransactionSeq_Atom_t = u32;
pub type CF_CFDP_Enum_t = u32;
pub type CF_GetSet_ValueID_Enum_t = u32;
pub type EdsDataType_BASE_TYPES_PathName_t = [c_char; 256];
pub type EdsDataType_BASE_TYPES_FileName_t = [c_char; 256];

// Constants for enum maximums (would be defined in EDS modules)
pub const EdsDataType_EdsEnum_CF_QueueIdx_t_MAX: u32 = 15;
pub const EdsDataType_EdsEnum_CF_GetSet_ValueID_t_MAX: u32 = 31;