pub mod vr {
    use std::ffi::c_void;

    pub mod vrtypes;
    use vrtypes::*;

    pub mod version;
    pub mod public_vrtypes;
    pub mod ivrsystem;
    pub mod ivrapplications;
    pub mod ivrsettings;

    #[link(name = "openvr_api")]
    extern "C" {
        pub fn VR_IsHmdPresent() -> bool;
        pub fn VR_IsRuntimeInstalled() -> bool;
        pub fn VR_GetRuntimePath(pchPathBuffer: *mut i8, unBufferSize: u32, punRequiredBufferSize: *mut u32) -> bool;
        pub fn VR_GetVRInitErrorAsSymbol(error: EVRInitError) -> *const i8;
        pub fn VR_GetVRInitErrorAsEnglishDescription(error: EVRInitError) -> *const i8;
        pub fn VR_GetGenericInterface(pchInterfaceVersion: *const i8, error: *mut EVRInitError) -> *const c_void;
        pub fn VR_IsInterfaceVersionValid(pchInterfaceVersion: *const i8) -> bool;
        pub fn VR_GetInitToken() -> u32;

        pub fn VR_InitInternal2(peError: *mut EVRInitError, eApplicationType: EVRApplicationType, pStartupInfo : *const i8) -> u32;
        pub fn VR_ShutdownInternal();
    }
}