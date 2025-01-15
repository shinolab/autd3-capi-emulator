#![allow(clippy::missing_safety_doc)]

mod drive;
mod instant;
mod output;
mod ptr;
mod range;
mod result;
mod rms;

use std::ffi::c_char;

use ptr::*;
use result::*;

use autd3::controller::ControllerBuilder;
use autd3::core::link::Link;
use autd3_emulator::{ControllerBuilderIntoEmulatorExt, Emulator, Record, Recorder};
use autd3capi_driver::{autd3::prelude::*, *};

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorTracingInitWithFile(path: *const c_char) -> ResultStatus {
    let path = validate_cstr!(path, AUTDStatus, ResultStatus);
    std::fs::File::options()
        .append(true)
        .create(true)
        .open(path)
        .map(|f| {
            tracing_subscriber::fmt()
                .with_writer(f)
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_ansi(false)
                .init();
            AUTDStatus::AUTDTrue
        })
        .into()
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDEmulator(builder: ControllerBuilderPtr) -> EmulatorPtr {
    take!(builder, ControllerBuilder).into_emulator().into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorFree(emulator: EmulatorPtr) {
    let _ = take!(emulator, Emulator);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorGeometry(emulator: EmulatorPtr) -> GeometryPtr {
    GeometryPtr(emulator.geometry() as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordFrom(
    emulator: EmulatorPtr,
    start_time: DcSysTime,
    f: ConstPtr,
) -> ResultRecord {
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

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordFree(record: RecordPtr) {
    let _ = take!(record, Record);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorTickNs(mut record: LinkPtr, tick: Duration) -> ResultStatus {
    record.cast_mut::<Recorder>().tick(tick.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorTransducerTableRows(emulator: EmulatorPtr) -> u64 {
    emulator.transducer_table_rows() as _
}

#[no_mangle]
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
