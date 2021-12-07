extern crate openvr_api;

#[cfg(test)]
mod tests {
    use openvr_api::vr::{*, vrtypes::*, ivrsystem::*};
    use std::ptr::null;
    use std::ffi::CString;

    #[test]
    fn vr_init_() {
        unsafe {
            let mut e_error = EVRInitError::VRInitError_None;
            let application_type = EVRApplicationType::VRApplication_Scene;
            VR_InitInternal2(&mut e_error, application_type, null());
        }
    }

    #[test]
    fn get_ivrsystem() {
        vr_init_();

        let mut p_system: *const VR_IVRSystem_FnTable = core::ptr::null_mut();

        unsafe {
            let mut err = EVRInitError::VRInitError_None;
            let system_version = CString::new("FnTable:IVRSystem_022").unwrap();

            p_system = VR_GetGenericInterface(
                system_version.as_ptr(),
                &mut err
            ) as *const VR_IVRSystem_FnTable;
        }
    }
}