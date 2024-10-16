use crate::dispatch::debug_assert_running_serially;
use coreaudio_sys::*;
use std::convert::TryFrom;
use std::os::raw::c_void;
use std::ptr;

// See https://github.com/mozilla/cubeb-coreaudio-rs/issues/237 for this and
// all other such warning suppressions in this file.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_get_property_info(
    unit: AudioUnit,
    property: AudioUnitPropertyID,
    scope: AudioUnitScope,
    element: AudioUnitElement,
    size: &mut usize,
    writable: Option<&mut bool>, // Use `Option` since `writable` is nullable.
) -> OSStatus {
    assert!(!unit.is_null());
    assert!(UInt32::try_from(*size).is_ok()); // Check if `size` can be converted to a UInt32.
    debug_assert_running_serially();
    unsafe {
        AudioUnitGetPropertyInfo(
            unit,
            property,
            scope,
            element,
            size as *mut usize as *mut UInt32,
            writable.map_or(ptr::null_mut(), |v| v as *mut bool as *mut Boolean),
        )
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_get_property<T>(
    unit: AudioUnit,
    property: AudioUnitPropertyID,
    scope: AudioUnitScope,
    element: AudioUnitElement,
    data: &mut T,
    size: &mut usize,
) -> OSStatus {
    assert!(!unit.is_null());
    assert!(UInt32::try_from(*size).is_ok()); // Check if `size` can be converted to a UInt32.
    debug_assert_running_serially();
    unsafe {
        AudioUnitGetProperty(
            unit,
            property,
            scope,
            element,
            data as *mut T as *mut c_void,
            size as *mut usize as *mut UInt32,
        )
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_set_property<T>(
    unit: AudioUnit,
    property: AudioUnitPropertyID,
    scope: AudioUnitScope,
    element: AudioUnitElement,
    data: &T,
    size: usize,
) -> OSStatus {
    assert!(!unit.is_null());
    debug_assert_running_serially();
    unsafe {
        AudioUnitSetProperty(
            unit,
            property,
            scope,
            element,
            data as *const T as *const c_void,
            size as UInt32,
        )
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_get_parameter(
    unit: AudioUnit,
    id: AudioUnitParameterID,
    scope: AudioUnitScope,
    element: AudioUnitElement,
    value: &mut AudioUnitParameterValue,
) -> OSStatus {
    assert!(!unit.is_null());
    debug_assert_running_serially();
    unsafe {
        AudioUnitGetParameter(
            unit,
            id,
            scope,
            element,
            value as *mut AudioUnitParameterValue,
        )
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_set_parameter(
    unit: AudioUnit,
    id: AudioUnitParameterID,
    scope: AudioUnitScope,
    element: AudioUnitElement,
    value: AudioUnitParameterValue,
    buffer_offset_in_frames: UInt32,
) -> OSStatus {
    assert!(!unit.is_null());
    debug_assert_running_serially();
    unsafe { AudioUnitSetParameter(unit, id, scope, element, value, buffer_offset_in_frames) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_initialize(unit: AudioUnit) -> OSStatus {
    assert!(!unit.is_null());
    debug_assert_running_serially();
    unsafe { AudioUnitInitialize(unit) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_uninitialize(unit: AudioUnit) -> OSStatus {
    assert!(!unit.is_null());
    debug_assert_running_serially();
    unsafe { AudioUnitUninitialize(unit) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn dispose_audio_unit(unit: AudioUnit) -> OSStatus {
    debug_assert_running_serially();
    unsafe { AudioComponentInstanceDispose(unit) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_output_unit_start(unit: AudioUnit) -> OSStatus {
    assert!(!unit.is_null());
    debug_assert_running_serially();
    unsafe { AudioOutputUnitStart(unit) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_output_unit_stop(unit: AudioUnit) -> OSStatus {
    assert!(!unit.is_null());
    debug_assert_running_serially();
    unsafe { AudioOutputUnitStop(unit) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_render(
    in_unit: AudioUnit,
    io_action_flags: &mut AudioUnitRenderActionFlags,
    in_time_stamp: &AudioTimeStamp,
    in_output_bus_number: u32,
    in_number_frames: u32,
    io_data: &mut AudioBufferList,
) -> OSStatus {
    assert!(!in_unit.is_null());
    unsafe {
        AudioUnitRender(
            in_unit,
            io_action_flags,
            in_time_stamp,
            in_output_bus_number,
            in_number_frames,
            io_data,
        )
    }
}

#[allow(non_camel_case_types)]
pub type audio_unit_property_listener_proc =
    extern "C" fn(*mut c_void, AudioUnit, AudioUnitPropertyID, AudioUnitScope, AudioUnitElement);

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_add_property_listener<T>(
    unit: AudioUnit,
    id: AudioUnitPropertyID,
    listener: audio_unit_property_listener_proc,
    data: *mut T,
) -> OSStatus {
    assert!(!unit.is_null());
    debug_assert_running_serially();
    unsafe { AudioUnitAddPropertyListener(unit, id, Some(listener), data as *mut c_void) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn audio_unit_remove_property_listener_with_user_data<T>(
    unit: AudioUnit,
    id: AudioUnitPropertyID,
    listener: audio_unit_property_listener_proc,
    data: *mut T,
) -> OSStatus {
    assert!(!unit.is_null());
    debug_assert_running_serially();
    unsafe {
        AudioUnitRemovePropertyListenerWithUserData(unit, id, Some(listener), data as *mut c_void)
    }
}
