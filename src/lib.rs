#![allow(clippy::missing_safety_doc)]

mod drive;
mod instant;
mod output;
mod ptr;
mod range;
mod result;
mod rms;

use ptr::*;
use result::*;

use autd3::core::link::Link;
use autd3_emulator::{Emulator, Record, Recorder};
use autd3capi_driver::{autd3::prelude::*, *};

#[unsafe(no_mangle)]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDEmulator(
    pos: *const Point3,
    rot: *const Quaternion,
    len: u16,
) -> EmulatorPtr {
    let pos = vec_from_raw!(pos, Point3, len);
    let rot = vec_from_raw!(rot, Quaternion, len);
    Emulator::new(pos.into_iter().zip(rot).map(|(pos, rot)| AUTD3 {
        pos,
        rot: UnitQuaternion::from_quaternion(rot),
    }))
    .into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEmulatorFree(emulator: EmulatorPtr) {
    let _ = take!(emulator, Emulator);
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorGeometry(emulator: EmulatorPtr) -> GeometryPtr {
    GeometryPtr(emulator.geometry() as *const _ as _)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordFrom(
    emulator: EmulatorPtr,
    start_time: DcSysTime,
    f: ConstPtr,
) -> ResultRecord {
    unsafe {
        emulator
            .record_from_take(start_time, move |cnt| {
                let f = std::mem::transmute::<ConstPtr, unsafe extern "C" fn(ControllerPtr)>(f);
                let cnt = cnt.into_boxed_link();
                let cnt_ptr = ControllerPtr(Box::into_raw(Box::new(cnt)) as _);
                f(cnt_ptr);
                let cnt: Controller<Recorder> = Controller::from_boxed_link(*Box::from_raw(
                    cnt_ptr.0 as *mut Controller<Box<dyn Link>>,
                ));
                Ok(cnt)
            })
            .into()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEmulatorRecordFree(record: RecordPtr) {
    let _ = take!(record, Record);
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorTickNs(mut record: LinkPtr, tick: Duration) -> ResultStatus {
    record.cast_mut::<Recorder>().tick(tick.into()).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorTransducerTableRows(emulator: EmulatorPtr) -> u64 {
    emulator.transducer_table_rows() as _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEmulatorTransducerTable(
    emulator: EmulatorPtr,
    dev_indices: *mut u16,
    tr_indices: *mut u8,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
    nx: *mut f32,
    ny: *mut f32,
    nz: *mut f32,
) {
    unsafe {
        let n = emulator.transducer_table_rows();
        emulator.dev_indices_inplace(std::slice::from_raw_parts_mut(dev_indices, n));
        emulator.tr_indices_inplace(std::slice::from_raw_parts_mut(tr_indices, n));
        emulator.tr_positions_inplace(
            std::slice::from_raw_parts_mut(x, n),
            std::slice::from_raw_parts_mut(y, n),
            std::slice::from_raw_parts_mut(z, n),
        );
        emulator.tr_dir_inplace(
            std::slice::from_raw_parts_mut(nx, n),
            std::slice::from_raw_parts_mut(ny, n),
            std::slice::from_raw_parts_mut(nz, n),
        );
    }
}
