use crate::ptr::*;
use crate::range::*;
use crate::result::*;

use autd3_emulator::Rms;
use autd3capi_driver::{async_ffi::*, Duration, *};

#[repr(C)]
pub struct RmsRecordOption {
    pub sound_speed: f32,
    pub print_progress: bool,
    pub gpu: bool,
}

impl From<RmsRecordOption> for autd3_emulator::RmsRecordOption {
    fn from(value: RmsRecordOption) -> Self {
        autd3_emulator::RmsRecordOption {
            sound_speed: value.sound_speed,
            print_progress: value.print_progress,
            gpu: value.gpu,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRms(
    record: RecordPtr,
    range: RangeXYZ,
    option: RmsRecordOption,
) -> LocalFfiFuture<ResultRms> {
    async move {
        let r = record
            .sound_field(
                autd3_emulator::RangeXYZ::from(range),
                autd3_emulator::RmsRecordOption::from(option),
            )
            .await;
        r.into()
    }
    .into_local_ffi()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsWait(
    handle: HandlePtr,
    future: LocalFfiFuture<ResultRms>,
) -> ResultRms {
    handle.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsTimeLen(
    sound_field: RmsPtr,
    duration: Duration,
) -> u64 {
    sound_field.next_time_len(duration.into()) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsPointsLen(sound_field: RmsPtr) -> u64 {
    sound_field.next_points_len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsGetX(sound_field: RmsPtr, x: *mut f32) {
    sound_field.x_inplace(std::slice::from_raw_parts_mut(
        x,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsGetY(sound_field: RmsPtr, y: *mut f32) {
    sound_field.y_inplace(std::slice::from_raw_parts_mut(
        y,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsGetZ(sound_field: RmsPtr, z: *mut f32) {
    sound_field.z_inplace(std::slice::from_raw_parts_mut(
        z,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsSkip(
    mut sound_field: RmsPtr,
    duration: Duration,
) -> LocalFfiFuture<ResultStatus> {
    async move {
        sound_field
            .next_inplace(duration.into(), true, &mut [], std::iter::empty())
            .await
            .into()
    }
    .into_local_ffi()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsNext(
    mut sound_field: RmsPtr,
    duration: Duration,
    time: *mut u64,
    v: *const *mut f32,
) -> LocalFfiFuture<ResultStatus> {
    let n = sound_field.next_time_len(duration.into());
    let time = std::slice::from_raw_parts_mut(time, n as _);
    let iter = (0..n).map(move |i| v.add(i as _).read());
    async move {
        sound_field
            .next_inplace(duration.into(), false, time, iter)
            .await
            .into()
    }
    .into_local_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsFree(sound_field: RmsPtr) {
    let _ = take!(sound_field, Rms);
}
