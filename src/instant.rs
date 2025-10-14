use crate::ptr::*;
use crate::range::*;
use crate::result::*;

use autd3_emulator::Instant;
use autd3capi_driver::{Duration, *};

#[repr(C)]
pub struct InstantRecordOption {
    pub sound_speed: f32,
    pub time_step: Duration,
    pub memory_limits_hint_mb: u64,
}

impl From<InstantRecordOption> for autd3_emulator::InstantRecordOption {
    fn from(value: InstantRecordOption) -> Self {
        autd3_emulator::InstantRecordOption {
            sound_speed: value.sound_speed,
            time_step: value.time_step.into(),
            memory_limits_hint_mb: value.memory_limits_hint_mb as _,
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstant(
    record: RecordPtr,
    range: RangeXYZ,
    option: InstantRecordOption,
) -> ResultInstant {
    record
        .static_deref()
        .sound_field(
            autd3_emulator::RangeXYZ::from(range),
            autd3_emulator::InstantRecordOption::from(option),
        )
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantTimeLen(
    sound_field: InstantPtr,
    duration: Duration,
) -> u64 {
    sound_field.next_time_len(duration.into()) as _
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantPointsLen(sound_field: InstantPtr) -> u64 {
    sound_field.next_points_len() as _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantGetX(sound_field: InstantPtr, x: *mut f32) {
    unsafe {
        sound_field.x_inplace(std::slice::from_raw_parts_mut(
            x,
            sound_field.next_points_len(),
        ));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantGetY(sound_field: InstantPtr, y: *mut f32) {
    unsafe {
        sound_field.y_inplace(std::slice::from_raw_parts_mut(
            y,
            sound_field.next_points_len(),
        ));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantGetZ(sound_field: InstantPtr, z: *mut f32) {
    unsafe {
        sound_field.z_inplace(std::slice::from_raw_parts_mut(
            z,
            sound_field.next_points_len(),
        ));
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantSkip(
    mut sound_field: InstantPtr,
    duration: Duration,
) -> ResultStatus {
    sound_field
        .next_inplace(duration.into(), true, &mut [], std::iter::empty())
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantNext(
    mut sound_field: InstantPtr,
    duration: Duration,
    time: *mut u64,
    v: *const *mut f32,
) -> ResultStatus {
    unsafe {
        let n = sound_field.next_time_len(duration.into());
        let time = std::slice::from_raw_parts_mut(time, n as _);
        let iter = (0..n).map(move |i| v.add(i as _).read());
        sound_field
            .next_inplace(duration.into(), false, time, iter)
            .into()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantFree(sound_field: InstantPtr) {
    let _ = take!(sound_field, Instant<'static>);
}
