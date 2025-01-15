use autd3capi_driver::{impl_result, ConstPtr};

use crate::{InstantPtr, RecordPtr, RmsPtr};

#[repr(C)]
pub struct ResultRecord {
    pub result: RecordPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_result!(ResultRecord, RecordPtr);

#[repr(C)]
pub struct ResultInstant {
    pub result: InstantPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_result!(ResultInstant, InstantPtr);

#[repr(C)]
pub struct ResultRms {
    pub result: RmsPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_result!(ResultRms, RmsPtr);
