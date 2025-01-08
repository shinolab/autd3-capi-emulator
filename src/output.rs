use crate::RecordPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputCols(record: RecordPtr) -> u64 {
    record.output_cols() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputVoltage(record: RecordPtr, v: *const *mut f32) {
    record.output_voltage_inplace((0..record.output_cols()).map(move |i| v.add(i as _).read()));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputUltrasound(record: RecordPtr, v: *const *mut f32) {
    record.output_ultrasound_inplace((0..record.output_cols()).map(move |i| v.add(i as _).read()));
}
