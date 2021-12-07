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

        unsafe {
            let mut err = EVRInitError::VRInitError_None;
            let system_version = CString::new(IVRSYSTEM_VERSION).unwrap();

            let p_system = VR_GetGenericInterface(
                system_version.as_ptr(),
                &mut err
            ) as *const IVRSystem;
            assert_eq!(err, EVRInitError::VRInitError_None);
            assert!(!p_system.is_null());

            ((*p_system).IsDisplayOnDesktop)();
        }
    }
}