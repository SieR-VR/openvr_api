#[allow(non_camel_case_types)]
pub enum EVRDebugError {
    VRDebugError_Success = 0,
    VRDebugError_BadParameter,
}

pub type VrProfilerEventHandle = u64;

#[allow(non_camel_case_types, non_snake_case)]
pub struct VR_IVRDebug_FnTable {
    /** Create a vr profiler discrete event (point)
    * The event will be associated with the message provided in pchMessage, and the current
    * time will be used as the event timestamp. */
    pub EmitVrProfilerEvent: unsafe extern "stdcall" fn(pchMessage: *const i8) -> EVRDebugError,

    /** Create an vr profiler duration event (line)
    * The current time will be used as the timestamp for the start of the line.
    * On success, pHandleOut will contain a handle valid for terminating this event. */
    pub BeginVrProfilerEvent: unsafe extern "stdcall" fn(pHandleOut: *mut VrProfilerEventHandle) -> EVRDebugError,

    /** Terminate a vr profiler event
    * The event associated with hHandle will be considered completed when this method is called.
    * The current time will be used assocaited to the termination time of the event, and
    * pchMessage will be used as the event title. */
    pub FinishVrProfilerEvent: unsafe extern "stdcall" fn( hHandle: VrProfilerEventHandle, pchMessage: *const i8) -> EVRDebugError,
}

pub const IVRDEBUG_VERSION: &'static str = "IVRDebug_001";