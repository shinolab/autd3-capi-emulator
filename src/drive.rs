use crate::RecordPtr;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordDriveCols(record: RecordPtr) -> u64 {
    record.drive_cols() as _
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordDriveRows(record: RecordPtr) -> u64 {
    record.drive_rows() as _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEmulatorRecordPhase(
    record: RecordPtr,
    time: *mut u64,
    v: *const *mut u8,
) {
    unsafe {
        let n = record.drive_cols();
        record.phase_inplace(
            std::slice::from_raw_parts_mut(time, n),
            (0..n).map(move |i| v.add(i as _).read()),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEmulatorRecordPulseWidth(
    record: RecordPtr,
    time: *mut u64,
    v: *const *mut u16,
) {
    unsafe {
        let n = record.drive_cols();
        record.pulse_width_inplace(
            std::slice::from_raw_parts_mut(time, n),
            (0..n).map(move |i| v.add(i as _).read()),
        );
    }
}
