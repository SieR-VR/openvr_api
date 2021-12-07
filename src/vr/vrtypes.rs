use super::public_vrtypes::*;
use std::ffi::c_void;

pub struct VkDevice;
pub struct VkPhysicalDevice;
pub struct VkInstance;
pub struct VkQueue;

pub type SpatialAnchorHandle = u32;
pub type GLSharedTextureHandle = *mut c_void;
pub type GLInt = i32;
pub type GLUint = u32;

pub struct DistortionCoordinates {
    pub red: [f32; 2],
    pub green: [f32; 2],
    pub blue: [f32; 2],
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVREye {
    Eye_Left = 0,
    Eye_Right = 1,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum ETextureType {
    TextureType_Invalid = -1,         // Handle has been invalidated
    TextureType_DirectX = 0,          // Handle is an ID3D11Texture
    TextureType_OpenGL = 1, // Handle is an OpenGL texture name or an OpenGL render buffer name, depending on submit flags
    TextureType_Vulkan = 2, // Handle is a pointer to a VRVulkanTextureData_t structure
    TextureType_IOSurface = 3, // Handle is a macOS cross-process-sharable IOSurfaceRef, deprecated in favor of TextureType_Metal on supported platforms
    TextureType_DirectX12 = 4, // Handle is a pointer to a D3D12TextureData_t structure
    TextureType_DXGISharedHandle = 5, // Handle is a HANDLE DXGI share handle, only supported for Overlay render targets.
    // this texture is used directly by our renderer, so only perform atomic (copyresource or resolve) on it
    TextureType_Metal = 6, // Handle is a MTLTexture conforming to the MTLSharedTexture protocol. Textures submitted to IVRCompositor::Submit which
                           // are of type MTLTextureType2DArray assume layer 0 is the left eye texture (vr::EVREye::Eye_left), layer 1 is the right
                           // eye texture (vr::EVREye::Eye_Right)
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EColorSpace {
    ColorSpace_Auto = 0, // Assumes 'gamma' for 8-bit per component formats, otherwise 'linear'.  This mirrors the DXGI formats which have _SRGB variants.
    ColorSpace_Gamma = 1, // Texture data can be displayed directly on the display without any conversion (a.k.a. display native format).
    ColorSpace_Linear = 2, // Same as gamma but has been converted to a linear representation using DXGI's sRGB conversion algorithm.
}

pub struct Texture {
    pub handle: *mut c_void,
    pub texture_type: ETextureType,
    pub color_space: EColorSpace,
}

pub type SharedTextureHandle = u64;
pub const INVALID_SHARED_TEXTURE_HANDLE: SharedTextureHandle = 0;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum ETrackingResult {
    TrackingResult_Uninitialized = 1,

    TrackingResult_Calibrating_InProgress = 100,
    TrackingResult_Calibrating_OutOfRange = 101,

    TrackingResult_Running_OK = 200,
    TrackingResult_Running_OutOfRange = 201,

    TrackingResult_Fallback_RotationOnly = 300,
}

pub type DriverId = u32;
#[allow(non_upper_case_globals)]
pub const k_nDriverNone: u32 = 0xFFFFFFFF;

#[allow(non_upper_case_globals)]
pub const k_unMaxDriverDebugResponseSize: u32 = 32768;

pub type TrackedDeviceIndex = u32;
#[allow(non_upper_case_globals)]
pub const k_unTrackedDeviceIndex_Hmd: u32 = 0;
#[allow(non_upper_case_globals)]
pub const k_unMaxTrackedDeviceCount: u32 = 64;
#[allow(non_upper_case_globals)]
pub const k_unTrackedDeviceIndexOther: u32 = 0xFFFFFFFE;
#[allow(non_upper_case_globals)]
pub const k_unTrackedDeviceIndexInvalid: u32 = 0xFFFFFFFF;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum ETrackedDeviceClass {
    TrackedDeviceClass_Invalid = 0,           // the ID was not valid.
    TrackedDeviceClass_HMD = 1,               // Head-Mounted Displays
    TrackedDeviceClass_Controller = 2,        // Tracked controllers
    TrackedDeviceClass_GenericTracker = 3,    // Generic trackers, similar to controllers
    TrackedDeviceClass_TrackingReference = 4, // Camera and base stations that serve as tracking reference points
    TrackedDeviceClass_DisplayRedirect = 5, // Accessories that aren't necessarily tracked themselves, but may redirect video output from other tracked devices

    TrackedDeviceClass_Max,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum ETrackedControllerRole {
    TrackedControllerRole_Invalid = 0, // Invalid value for controller type
    TrackedControllerRole_LeftHand = 1, // Tracked device associated with the left hand
    TrackedControllerRole_RightHand = 2, // Tracked device associated with the right hand
    TrackedControllerRole_OptOut = 3,  // Tracked device is opting out of left/right hand selection
    TrackedControllerRole_Treadmill = 4, // Tracked device is a treadmill or other locomotion device
    TrackedControllerRole_Stylus = 5,  // Tracked device is a stylus
}

pub fn is_role_allowed_as_hand(role: ETrackedControllerRole) -> bool {
    match role {
        ETrackedControllerRole::TrackedControllerRole_Invalid
        | ETrackedControllerRole::TrackedControllerRole_LeftHand
        | ETrackedControllerRole::TrackedControllerRole_RightHand => return true,
        _ => return false,
    }
}

pub struct TrackedDevicePose {
    pub device_to_absolute_tracking: HmdMatrix34,
    pub velocity: HmdVector3,         // velocity in tracker space in m/s
    pub angular_velocity: HmdVector3, // angular velocity in radians/s (?)
    pub tracking_result: ETrackingResult,
    pub pose_is_valid: bool,

    // This indicates that there is a device connected for this spot in the pose array.
    // It could go from true to false if the user unplugs the device.
    pub device_is_connected: bool,
}

/** Identifies which style of tracking origin the application wants to use
* for the poses it is requesting */
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum ETrackingUniverseOrigin {
    TrackingUniverseSeated = 0, // Poses are provided relative to the seated zero pose
    TrackingUniverseStanding = 1, // Poses are provided relative to the safe bounds configured by the user
    TrackingUniverseRawAndUncalibrated = 2, // Poses are provided in the coordinate system defined by the driver.  It has Y up and is unified for devices of the same driver. You usually don't want this one.
}

pub type WebConsoleHandle = u64;
pub const INVALID_WEB_CONSOLE_HANDLE: WebConsoleHandle = 0;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EAdditionalRadioFeatures {
    AdditionalRadioFeatures_None = 0x00000000,
    AdditionalRadioFeatures_HTCLinkBox = 0x00000001,
    AdditionalRadioFeatures_InternalDongle = 0x00000002,
    AdditionalRadioFeatures_ExternalDongle = 0x00000004,
}

pub type PropertyContainerHandle = u64;
pub type PropertyTypeTag = u32;
pub type DriverHandle = PropertyContainerHandle;

#[allow(non_upper_case_globals)]
pub const k_ulInvalidPropertyContainer: PropertyContainerHandle = 0;
#[allow(non_upper_case_globals)]
pub const k_unInvalidPropertyTag: PropertyTypeTag = 0;
#[allow(non_upper_case_globals)]
pub const k_ulInvalidDriverHandle: DriverHandle = 0;

#[allow(non_upper_case_globals)]
pub const k_unFloatPropertyTag: PropertyTypeTag = 1;
#[allow(non_upper_case_globals)]
pub const k_unInt32PropertyTag: PropertyTypeTag = 2;
#[allow(non_upper_case_globals)]
pub const k_unUint64PropertyTag: PropertyTypeTag = 3;
#[allow(non_upper_case_globals)]
pub const k_unBoolPropertyTag: PropertyTypeTag = 4;
#[allow(non_upper_case_globals)]
pub const k_unStringPropertyTag: PropertyTypeTag = 5;
#[allow(non_upper_case_globals)]
pub const k_unErrorPropertyTag: PropertyTypeTag = 6;
#[allow(non_upper_case_globals)]
pub const k_unDoublePropertyTag: PropertyTypeTag = 7;

#[allow(non_upper_case_globals)]
pub const k_unHmdMatrix34PropertyTag: PropertyTypeTag = 20;
#[allow(non_upper_case_globals)]
pub const k_unHmdMatrix44PropertyTag: PropertyTypeTag = 21;
#[allow(non_upper_case_globals)]
pub const k_unHmdVector3PropertyTag: PropertyTypeTag = 22;
#[allow(non_upper_case_globals)]
pub const k_unHmdVector4PropertyTag: PropertyTypeTag = 23;
#[allow(non_upper_case_globals)]
pub const k_unHmdVector2PropertyTag: PropertyTypeTag = 24;
#[allow(non_upper_case_globals)]
pub const k_unHmdQuadPropertyTag: PropertyTypeTag = 25;

#[allow(non_upper_case_globals)]
pub const k_unHiddenAreaPropertyTag: PropertyTypeTag = 30;
#[allow(non_upper_case_globals)]
pub const k_unPathHandleInfoTag: PropertyTypeTag = 31;
#[allow(non_upper_case_globals)]
pub const k_unActionPropertyTag: PropertyTypeTag = 32;
#[allow(non_upper_case_globals)]
pub const k_unInputValuePropertyTag: PropertyTypeTag = 33;
#[allow(non_upper_case_globals)]
pub const k_unWildcardPropertyTag: PropertyTypeTag = 34;
#[allow(non_upper_case_globals)]
pub const k_unHapticVibrationPropertyTag: PropertyTypeTag = 35;
#[allow(non_upper_case_globals)]
pub const k_unSkeletonPropertyTag: PropertyTypeTag = 36;

#[allow(non_upper_case_globals)]
pub const k_unSpatialAnchorPosePropertyTag: PropertyTypeTag = 40;
#[allow(non_upper_case_globals)]
pub const k_unJsonPropertyTag: PropertyTypeTag = 41;
#[allow(non_upper_case_globals)]
pub const k_unActiveActionSetPropertyTag: PropertyTypeTag = 42;

#[allow(non_upper_case_globals)]
pub const k_unOpenVRInternalReserved_Start: PropertyTypeTag = 1000;
#[allow(non_upper_case_globals)]
pub const k_unOpenVRInternalReserved_End: PropertyTypeTag = 10000;

/** Each entry in this enum represents a property that can be retrieved about a
* tracked device. Many fields are only valid for one ETrackedDeviceClass. */
#[allow(non_camel_case_types)]
#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub enum ETrackedDeviceProperty {
    Prop_Invalid = 0,

    // general properties that apply to all device classes
    Prop_TrackingSystemName_String = 1000,
    Prop_ModelNumber_String = 1001,
    Prop_SerialNumber_String = 1002,
    Prop_RenderModelName_String = 1003,
    Prop_WillDriftInYaw_Bool = 1004,
    Prop_ManufacturerName_String = 1005,
    Prop_TrackingFirmwareVersion_String = 1006,
    Prop_HardwareRevision_String = 1007,
    Prop_AllWirelessDongleDescriptions_String = 1008,
    Prop_ConnectedWirelessDongle_String = 1009,
    Prop_DeviceIsWireless_Bool = 1010,
    Prop_DeviceIsCharging_Bool = 1011,
    Prop_DeviceBatteryPercentage_Float = 1012, // 0 is empty, 1 is full
    Prop_StatusDisplayTransform_Matrix34 = 1013,
    Prop_Firmware_UpdateAvailable_Bool = 1014,
    Prop_Firmware_ManualUpdate_Bool = 1015,
    Prop_Firmware_ManualUpdateURL_String = 1016,
    Prop_HardwareRevision_Uint64 = 1017,
    Prop_FirmwareVersion_Uint64 = 1018,
    Prop_FPGAVersion_Uint64 = 1019,
    Prop_VRCVersion_Uint64 = 1020,
    Prop_RadioVersion_Uint64 = 1021,
    Prop_DongleVersion_Uint64 = 1022,
    Prop_BlockServerShutdown_Bool = 1023,
    Prop_CanUnifyCoordinateSystemWithHmd_Bool = 1024,
    Prop_ContainsProximitySensor_Bool = 1025,
    Prop_DeviceProvidesBatteryStatus_Bool = 1026,
    Prop_DeviceCanPowerOff_Bool = 1027,
    Prop_Firmware_ProgrammingTarget_String = 1028,
    Prop_DeviceClass_Int32 = 1029,
    Prop_HasCamera_Bool = 1030,
    Prop_DriverVersion_String = 1031,
    Prop_Firmware_ForceUpdateRequired_Bool = 1032,
    Prop_ViveSystemButtonFixRequired_Bool = 1033,
    Prop_ParentDriver_Uint64 = 1034,
    Prop_ResourceRoot_String = 1035,
    Prop_RegisteredDeviceType_String = 1036,
    Prop_InputProfilePath_String = 1037, // input profile to use for this device in the input system. Will default to tracking system name if this isn't provided
    Prop_NeverTracked_Bool = 1038, // Used for devices that will never have a valid pose by design
    Prop_NumCameras_Int32 = 1039,
    Prop_CameraFrameLayout_Int32 = 1040, // EVRTrackedCameraFrameLayout value
    Prop_CameraStreamFormat_Int32 = 1041, // ECameraVideoStreamFormat value
    Prop_AdditionalDeviceSettingsPath_String = 1042, // driver-relative path to additional device and global configuration settings
    Prop_Identifiable_Bool = 1043, // Whether device supports being identified from vrmonitor (e.g. blink LED, vibrate haptics, etc)
    Prop_BootloaderVersion_Uint64 = 1044,
    Prop_AdditionalSystemReportData_String = 1045, // additional string to include in system reports about a tracked device
    Prop_CompositeFirmwareVersion_String = 1046, // additional FW components from a device that gets propagated into reports
    Prop_Firmware_RemindUpdate_Bool = 1047,
    Prop_PeripheralApplicationVersion_Uint64 = 1048,
    Prop_ManufacturerSerialNumber_String = 1049,
    Prop_ComputedSerialNumber_String = 1050,
    Prop_EstimatedDeviceFirstUseTime_Int32 = 1051,

    // Properties that are unique to TrackedDeviceClass_HMD
    Prop_ReportsTimeSinceVSync_Bool = 2000,
    Prop_SecondsFromVsyncToPhotons_Float = 2001,
    Prop_DisplayFrequency_Float = 2002,
    Prop_UserIpdMeters_Float = 2003,
    Prop_CurrentUniverseId_Uint64 = 2004,
    Prop_PreviousUniverseId_Uint64 = 2005,
    Prop_DisplayFirmwareVersion_Uint64 = 2006,
    Prop_IsOnDesktop_Bool = 2007,
    Prop_DisplayMCType_Int32 = 2008,
    Prop_DisplayMCOffset_Float = 2009,
    Prop_DisplayMCScale_Float = 2010,
    Prop_EdidVendorID_Int32 = 2011,
    Prop_DisplayMCImageLeft_String = 2012,
    Prop_DisplayMCImageRight_String = 2013,
    Prop_DisplayGCBlackClamp_Float = 2014,
    Prop_EdidProductID_Int32 = 2015,
    Prop_CameraToHeadTransform_Matrix34 = 2016,
    Prop_DisplayGCType_Int32 = 2017,
    Prop_DisplayGCOffset_Float = 2018,
    Prop_DisplayGCScale_Float = 2019,
    Prop_DisplayGCPrescale_Float = 2020,
    Prop_DisplayGCImage_String = 2021,
    Prop_LensCenterLeftU_Float = 2022,
    Prop_LensCenterLeftV_Float = 2023,
    Prop_LensCenterRightU_Float = 2024,
    Prop_LensCenterRightV_Float = 2025,
    Prop_UserHeadToEyeDepthMeters_Float = 2026,
    Prop_CameraFirmwareVersion_Uint64 = 2027,
    Prop_CameraFirmwareDescription_String = 2028,
    Prop_DisplayFPGAVersion_Uint64 = 2029,
    Prop_DisplayBootloaderVersion_Uint64 = 2030,
    Prop_DisplayHardwareVersion_Uint64 = 2031,
    Prop_AudioFirmwareVersion_Uint64 = 2032,
    Prop_CameraCompatibilityMode_Int32 = 2033,
    Prop_ScreenshotHorizontalFieldOfViewDegrees_Float = 2034,
    Prop_ScreenshotVerticalFieldOfViewDegrees_Float = 2035,
    Prop_DisplaySuppressed_Bool = 2036,
    Prop_DisplayAllowNightMode_Bool = 2037,
    Prop_DisplayMCImageWidth_Int32 = 2038,
    Prop_DisplayMCImageHeight_Int32 = 2039,
    Prop_DisplayMCImageNumChannels_Int32 = 2040,
    Prop_DisplayMCImageData_Binary = 2041,
    Prop_SecondsFromPhotonsToVblank_Float = 2042,
    Prop_DriverDirectModeSendsVsyncEvents_Bool = 2043,
    Prop_DisplayDebugMode_Bool = 2044,
    Prop_GraphicsAdapterLuid_Uint64 = 2045,
    Prop_DriverProvidedChaperonePath_String = 2048,
    Prop_ExpectedTrackingReferenceCount_Int32 = 2049, // expected number of sensors or basestations to reserve UI space for
    Prop_ExpectedControllerCount_Int32 = 2050, // expected number of tracked controllers to reserve UI space for
    Prop_NamedIconPathControllerLeftDeviceOff_String = 2051, // placeholder icon for "left" controller if not yet detected/loaded
    Prop_NamedIconPathControllerRightDeviceOff_String = 2052, // placeholder icon for "right" controller if not yet detected/loaded
    Prop_NamedIconPathTrackingReferenceDeviceOff_String = 2053, // placeholder icon for sensor/base if not yet detected/loaded
    Prop_DoNotApplyPrediction_Bool = 2054, // currently no effect. was used to disable HMD pose prediction on MR, which is now done by MR driver setting velocity=0
    Prop_CameraToHeadTransforms_Matrix34_Array = 2055,
    Prop_DistortionMeshResolution_Int32 = 2056, // custom resolution of compositor calls to IVRSystem::ComputeDistortion
    Prop_DriverIsDrawingControllers_Bool = 2057,
    Prop_DriverRequestsApplicationPause_Bool = 2058,
    Prop_DriverRequestsReducedRendering_Bool = 2059,
    Prop_MinimumIpdStepMeters_Float = 2060,
    Prop_AudioBridgeFirmwareVersion_Uint64 = 2061,
    Prop_ImageBridgeFirmwareVersion_Uint64 = 2062,
    Prop_ImuToHeadTransform_Matrix34 = 2063,
    Prop_ImuFactoryGyroBias_Vector3 = 2064,
    Prop_ImuFactoryGyroScale_Vector3 = 2065,
    Prop_ImuFactoryAccelerometerBias_Vector3 = 2066,
    Prop_ImuFactoryAccelerometerScale_Vector3 = 2067,
    // reserved 2068
    Prop_ConfigurationIncludesLighthouse20Features_Bool = 2069,
    Prop_AdditionalRadioFeatures_Uint64 = 2070,
    Prop_CameraWhiteBalance_Vector4_Array = 2071, // Prop_NumCameras_Int32-sized array of float[4] RGBG white balance calibration data (max size is vr::k_unMaxCameras)
    Prop_CameraDistortionFunction_Int32_Array = 2072, // Prop_NumCameras_Int32-sized array of vr::EVRDistortionFunctionType values (max size is vr::k_unMaxCameras)
    Prop_CameraDistortionCoefficients_Float_Array = 2073, // Prop_NumCameras_Int32-sized array of double[vr::k_unMaxDistortionFunctionParameters] (max size is vr::k_unMaxCameras)
    Prop_ExpectedControllerType_String = 2074,
    Prop_HmdTrackingStyle_Int32 = 2075, // one of EHmdTrackingStyle
    Prop_DriverProvidedChaperoneVisibility_Bool = 2076,
    Prop_HmdColumnCorrectionSettingPrefix_String = 2077,
    Prop_CameraSupportsCompatibilityModes_Bool = 2078,
    Prop_SupportsRoomViewDepthProjection_Bool = 2079,
    Prop_DisplayAvailableFrameRates_Float_Array = 2080, // populated by compositor from actual EDID list when available from GPU driver
    Prop_DisplaySupportsMultipleFramerates_Bool = 2081, // if this is true but Prop_DisplayAvailableFrameRates_Float_Array is empty, explain to user
    Prop_DisplayColorMultLeft_Vector3 = 2082,
    Prop_DisplayColorMultRight_Vector3 = 2083,
    Prop_DisplaySupportsRuntimeFramerateChange_Bool = 2084,
    Prop_DisplaySupportsAnalogGain_Bool = 2085,
    Prop_DisplayMinAnalogGain_Float = 2086,
    Prop_DisplayMaxAnalogGain_Float = 2087,
    Prop_CameraExposureTime_Float = 2088,
    Prop_CameraGlobalGain_Float = 2089,
    // Prop_DashboardLayoutPathName_String 		= 2090, // DELETED
    Prop_DashboardScale_Float = 2091,
    Prop_IpdUIRangeMinMeters_Float = 2100,
    Prop_IpdUIRangeMaxMeters_Float = 2101,
    Prop_Hmd_SupportsHDCP14LegacyCompat_Bool = 2102,
    Prop_Hmd_SupportsMicMonitoring_Bool = 2103,

    // Driver requested mura correction properties
    Prop_DriverRequestedMuraCorrectionMode_Int32 = 2200,
    Prop_DriverRequestedMuraFeather_InnerLeft_Int32 = 2201,
    Prop_DriverRequestedMuraFeather_InnerRight_Int32 = 2202,
    Prop_DriverRequestedMuraFeather_InnerTop_Int32 = 2203,
    Prop_DriverRequestedMuraFeather_InnerBottom_Int32 = 2204,
    Prop_DriverRequestedMuraFeather_OuterLeft_Int32 = 2205,
    Prop_DriverRequestedMuraFeather_OuterRight_Int32 = 2206,
    Prop_DriverRequestedMuraFeather_OuterTop_Int32 = 2207,
    Prop_DriverRequestedMuraFeather_OuterBottom_Int32 = 2208,

    Prop_Audio_DefaultPlaybackDeviceId_String = 2300,
    Prop_Audio_DefaultRecordingDeviceId_String = 2301,
    Prop_Audio_DefaultPlaybackDeviceVolume_Float = 2302,
    Prop_Audio_SupportsDualSpeakerAndJackOutput_Bool = 2303,

    // Properties that are unique to TrackedDeviceClass_Controller
    Prop_AttachedDeviceId_String = 3000,
    Prop_SupportedButtons_Uint64 = 3001,
    Prop_Axis0Type_Int32 = 3002, // Return value is of type EVRControllerAxisType
    Prop_Axis1Type_Int32 = 3003, // Return value is of type EVRControllerAxisType
    Prop_Axis2Type_Int32 = 3004, // Return value is of type EVRControllerAxisType
    Prop_Axis3Type_Int32 = 3005, // Return value is of type EVRControllerAxisType
    Prop_Axis4Type_Int32 = 3006, // Return value is of type EVRControllerAxisType
    Prop_ControllerRoleHint_Int32 = 3007, // Return value is of type ETrackedControllerRole

    // Properties that are unique to TrackedDeviceClass_TrackingReference
    Prop_FieldOfViewLeftDegrees_Float = 4000,
    Prop_FieldOfViewRightDegrees_Float = 4001,
    Prop_FieldOfViewTopDegrees_Float = 4002,
    Prop_FieldOfViewBottomDegrees_Float = 4003,
    Prop_TrackingRangeMinimumMeters_Float = 4004,
    Prop_TrackingRangeMaximumMeters_Float = 4005,
    Prop_ModeLabel_String = 4006,
    Prop_CanWirelessIdentify_Bool = 4007, // volatile, based on radio presence and fw discovery
    Prop_Nonce_Int32 = 4008,

    // Properties that are used for user interface like icons names
    Prop_IconPathName_String = 5000, // DEPRECATED. Value not referenced. Now expected to be part of icon path properties.
    Prop_NamedIconPathDeviceOff_String = 5001, // {driver}/icons/icon_filename - PNG for static icon, or GIF for animation, 50x32 for headsets and 32x32 for others
    Prop_NamedIconPathDeviceSearching_String = 5002, // {driver}/icons/icon_filename - PNG for static icon, or GIF for animation, 50x32 for headsets and 32x32 for others
    Prop_NamedIconPathDeviceSearchingAlert_String = 5003, // {driver}/icons/icon_filename - PNG for static icon, or GIF for animation, 50x32 for headsets and 32x32 for others
    Prop_NamedIconPathDeviceReady_String = 5004, // {driver}/icons/icon_filename - PNG for static icon, or GIF for animation, 50x32 for headsets and 32x32 for others
    Prop_NamedIconPathDeviceReadyAlert_String = 5005, // {driver}/icons/icon_filename - PNG for static icon, or GIF for animation, 50x32 for headsets and 32x32 for others
    Prop_NamedIconPathDeviceNotReady_String = 5006, // {driver}/icons/icon_filename - PNG for static icon, or GIF for animation, 50x32 for headsets and 32x32 for others
    Prop_NamedIconPathDeviceStandby_String = 5007, // {driver}/icons/icon_filename - PNG for static icon, or GIF for animation, 50x32 for headsets and 32x32 for others
    Prop_NamedIconPathDeviceAlertLow_String = 5008, // {driver}/icons/icon_filename - PNG for static icon, or GIF for animation, 50x32 for headsets and 32x32 for others
    Prop_NamedIconPathDeviceStandbyAlert_String = 5009, // {driver}/icons/icon_filename - PNG for static icon, or GIF for animation, 50x32 for headsets and 32x32 for others

    // Properties that are used by helpers, but are opaque to applications
    Prop_DisplayHiddenArea_Binary_Start = 5100,
    Prop_DisplayHiddenArea_Binary_End = 5150,
    Prop_ParentContainer = 5151,
    Prop_OverrideContainer_Uint64 = 5152,

    // Properties that are unique to drivers
    Prop_UserConfigPath_String = 6000,
    Prop_InstallPath_String = 6001,
    Prop_HasDisplayComponent_Bool = 6002,
    Prop_HasControllerComponent_Bool = 6003,
    Prop_HasCameraComponent_Bool = 6004,
    Prop_HasDriverDirectModeComponent_Bool = 6005,
    Prop_HasVirtualDisplayComponent_Bool = 6006,
    Prop_HasSpatialAnchorsSupport_Bool = 6007,

    // Properties that are set internally based on other information provided by drivers
    Prop_ControllerType_String = 7000,
    //Prop_LegacyInputProfile_String				= 7001, // This is no longer used. See "legacy_binding" in the input profile instead.
    Prop_ControllerHandSelectionPriority_Int32 = 7002, // Allows hand assignments to prefer some controllers over others. High numbers are selected over low numbers

    // Vendors are free to expose private debug data in this reserved region
    Prop_VendorSpecific_Reserved_Start = 10000,
    Prop_VendorSpecific_Reserved_End = 10999,

    Prop_TrackedDeviceProperty_Max = 1000000,
}

#[allow(non_upper_case_globals)]
pub const k_unMaxPropertyStringSize: u32 = 32 * 1024;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum ETrackedPropertyError {
    TrackedProp_Success = 0,
    TrackedProp_WrongDataType = 1,
    TrackedProp_WrongDeviceClass = 2,
    TrackedProp_BufferTooSmall = 3,
    TrackedProp_UnknownProperty = 4, // Driver has not set the property (and may not ever).
    TrackedProp_InvalidDevice = 5,
    TrackedProp_CouldNotContactServer = 6,
    TrackedProp_ValueNotProvidedByDevice = 7,
    TrackedProp_StringExceedsMaximumLength = 8,
    TrackedProp_NotYetAvailable = 9, // The property value isn't known yet, but is expected soon. Call again later.
    TrackedProp_PermissionDenied = 10,
    TrackedProp_InvalidOperation = 11,
    TrackedProp_CannotWriteToWildcards = 12,
    TrackedProp_IPCReadFailure = 13,
    TrackedProp_OutOfMemory = 14,
    TrackedProp_InvalidContainer = 15,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EHmdTrackingStyle {
    HmdTrackingStyle_Unknown = 0,

    HmdTrackingStyle_Lighthouse = 1,       // base stations and lasers
    HmdTrackingStyle_OutsideInCameras = 2, // Cameras and LED, Rift 1 style
    HmdTrackingStyle_InsideOutCameras = 3, // Cameras on HMD looking at the world
}

pub type VRActionHandle = u64;
pub type VRActionSetHandle = u64;
pub type VRInputValueHandle = u64;

#[allow(non_upper_case_globals)]
pub const k_ulInvalidActionHandle: VRActionHandle = 0;
#[allow(non_upper_case_globals)]
pub const k_ulInvalidActionSetHandle: VRActionSetHandle = 0;
#[allow(non_upper_case_globals)]
pub const k_ulInvalidInputValueHandle: VRInputValueHandle = 0;

pub struct VRTextureBounds {
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
}

pub struct VRTextureWithPose {
    pub handle: *mut c_void,
    pub texture_type: ETextureType,
    pub color_space: EColorSpace,
    pub device_to_absolute_tracking: HmdMatrix34,
}

pub struct VRTextureDepthInfo {
    pub handle: *mut c_void, // See ETextureType definition above
    pub projection: HmdMatrix44,
    pub range: HmdVector2, // 0..1
}

pub struct VRTextureWithPoseAndDepth {
    pub handle: *mut c_void,
    pub texture_type: ETextureType,
    pub color_space: EColorSpace,
    pub device_to_absolute_tracking: HmdMatrix34,
    pub depth: VRTextureDepthInfo,
}

/* TODO: some vulkan features */
/* TODO: some D3D12 features */

/** Status of the overall system or tracked objects */
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRState {
    VRState_Undefined = -1,
    VRState_Off = 0,
    VRState_Searching = 1,
    VRState_Searching_Alert = 2,
    VRState_Ready = 3,
    VRState_Ready_Alert = 4,
    VRState_NotReady = 5,
    VRState_Standby = 6,
    VRState_Ready_Alert_Low = 7,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRSubmitFlags {
    // Simple render path. App submits rendered left and right eye images with no lens distortion correction applied.
    Submit_Default = 0x00,

    // App submits final left and right eye images with lens distortion already applied (lens distortion makes the images appear
    // barrel distorted with chromatic aberration correction applied). The app would have used the data returned by
    // vr::IVRSystem::ComputeDistortion() to apply the correct distortion to the rendered images before calling Submit().
    Submit_LensDistortionAlreadyApplied = 0x01,

    // If the texture pointer passed in is actually a renderbuffer (e.g. for MSAA in OpenGL) then set this flag.
    Submit_GlRenderBuffer = 0x02,

    // Do not use
    Submit_Reserved = 0x04,

    // Set to indicate that pTexture is a pointer to a VRTextureWithPose_t.
    // This flag can be combined with Submit_TextureWithDepth to pass a VRTextureWithPoseAndDepth_t.
    Submit_TextureWithPose = 0x08,

    // Set to indicate that pTexture is a pointer to a VRTextureWithDepth_t.
    // This flag can be combined with Submit_TextureWithPose to pass a VRTextureWithPoseAndDepth_t.
    Submit_TextureWithDepth = 0x10,

    // Set to indicate a discontinuity between this and the last frame.
    // This will prevent motion smoothing from attempting to extrapolate using the pair.
    Submit_FrameDiscontinuty = 0x20,

    // Set to indicate that pTexture->handle is a contains VRVulkanTextureArrayData_t
    Submit_VulkanTextureWithArrayData = 0x40,

    // If the texture pointer passed in is an OpenGL Array texture, set this flag
    Submit_GlArrayTexture = 0x80,

    // Do not use
    Submit_Reserved2 = 0x8000,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EDeviceActivityLevel {
    k_EDeviceActivityLevel_Unknown = -1,
    k_EDeviceActivityLevel_Idle = 0, // No activity for the last 10 seconds
    k_EDeviceActivityLevel_UserInteraction = 1, // Activity (movement or prox sensor) is happening now
    k_EDeviceActivityLevel_UserInteraction_Timeout = 2, // No activity for the last 0.5 seconds
    k_EDeviceActivityLevel_Standby = 3, // Idle for at least 5 seconds (configurable in Settings -> Power Management)
    k_EDeviceActivityLevel_Idle_Timeout = 4,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRButtonId {
    k_EButton_System = 0,
    k_EButton_ApplicationMenu = 1,
    k_EButton_Grip = 2,
    k_EButton_DPad_Left = 3,
    k_EButton_DPad_Up = 4,
    k_EButton_DPad_Right = 5,
    k_EButton_DPad_Down = 6,
    k_EButton_A = 7,

    k_EButton_ProximitySensor = 31,

    k_EButton_Axis0 = 32,
    k_EButton_Axis1 = 33,
    k_EButton_Axis2 = 34,
    k_EButton_Axis3 = 35,
    k_EButton_Axis4 = 36,

    /* these aliases are not allowed in rust */
    // aliases for well known controllers
    // k_EButton_SteamVR_Touchpad	= k_EButton_Axis0,
    // k_EButton_SteamVR_Trigger	= k_EButton_Axis1,

    // k_EButton_Dashboard_Back	= k_EButton_Grip,

    // k_EButton_IndexController_A		= k_EButton_Grip,
    // k_EButton_IndexController_B		= k_EButton_ApplicationMenu,
    // k_EButton_IndexController_JoyStick	= k_EButton_Axis3,
    k_EButton_Max = 64,
}

/** used for controller button events */
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Controller {
    pub button: u32, // EVRButtonId enum
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
/** used for simulated mouse events in overlay space */
pub enum EVRMouseButton {
    VRMouseButton_Left = 0x0001,
    VRMouseButton_Right = 0x0002,
    VRMouseButton_Middle = 0x0004,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
/** used for simulated mouse events in overlay space */
pub struct VREvent_Mouse {
    pub x: f32,
    pub y: f32,      // co-ords are in GL space, bottom left of the texture is 0,0
    pub button: u32, // EVRMouseButton enum
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
/** used for simulated mouse wheel scroll */
pub struct VREvent_Scroll {
    pub x_delta: f32,
    pub y_delta: f32,
    pub unused: u32,
    pub viewport_scale: f32, // For scrolling on an overlay with laser mouse, this is the overlay's vertical size relative to the overlay height. Range: [0,1]
}

/** when in mouse input mode you can receive data from the touchpad, these events are only sent if the users finger
   is on the touchpad (or just released from it). These events are sent to overlays with the VROverlayFlags_SendVRTouchpadEvents
   flag set.
**/
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_TouchPadMove {
    // true if the users finger is detected on the touch pad
    pub finger_down: bool,

    // How long the finger has been down in seconds
    pub seconds_finger_down: f32,

    // These values indicate the starting finger position (so you can do some basic swipe stuff)
    pub value_x_first: f32,
    pub value_y_first: f32,

    // This is the raw sampled coordinate without deadzoning
    pub value_x_raw: f32,
    pub value_y_raw: f32,
}

/** notification related events. Details will still change at this point */
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Notification {
    pub user_value: u64,
    pub notification_id: u32,
}

/** Used for events about processes */
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Process {
    pub pid: u32,
    pub old_pid: u32,
    pub forced: bool,
    // If the associated event was triggered by a connection loss
    pub connection_lost: bool,
}

/** Used for a few events about overlays */
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Overlay {
    pub overlay_handle: u64,
    pub device_path: u64,
    pub memory_block_id: u64,
}

/** Used for a few events about overlays */
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Status {
    pub status_state: u32, // EVRState enum
}

/** Used for keyboard events **/
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Keyboard {
    pub new_input: [u8; 8], // Up to 11 bytes of new input
    pub user_value: u64,    // Possible flags about the new input
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Ipd {
    pub ipd_meters: f32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Chaperone {
    pub previous_universe: u64,
    pub current_universe: u64,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
/** Not actually used for any events */
pub struct VREvent_Reserved {
    pub reserved0: u64,
    pub reserved1: u64,
    pub reserved2: u64,
    pub reserved3: u64,
    pub reserved4: u64,
    pub reserved5: u64,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_PerformanceTest {
    pub fidelity_level: u32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_SeatedZeroPoseReset {
    pub reset_by_system_menu: bool,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Screenshot {
    pub handle: u32,
    pub type_: u32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_ScreenshotProgress {
    pub progress: f32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_ApplicationLaunch {
    pub pid: u32,
    pub args_handle: u32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_EditingCameraSurface {
    pub overlay_handle: u64,
    pub visual_mode: u32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_MessageOverlay {
    pub vr_message_overlay_response: u32, // vr::VRMessageOverlayResponse enum
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_Property {
    pub container: PropertyContainerHandle,
    pub prop: ETrackedDeviceProperty,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_HapticVibration {
    pub container_handle: u64, // property container handle of the device with the haptic component
    pub component_handle: u64, // Which haptic component needs to vibrate
    pub duration_seconds: f32,
    pub frequency: f32,
    pub amplitude: f32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_WebConsole {
    pub web_console_handle: WebConsoleHandle,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_InputBindingLoad {
    pub app_container: PropertyContainerHandle,
    pub path_message: u64,
    pub path_url: u64,
    pub path_controller_type: u64,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_InputActionManifestLoad {
    pub path_app_key: u64,
    pub path_message: u64,
    pub path_message_param: u64,
    pub path_manifest_path: u64,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_SpatialAnchor {
    pub handle: SpatialAnchorHandle,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_ProgressUpdate {
    pub application_property_container: u64,
    pub path_device: u64,
    pub path_input_source: u64,
    pub path_progress_action: u64,
    pub path_icon: u64,
    pub f_progress: u64,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub enum EShowUIType {
    ShowUI_ControllerBinding = 0,
    ShowUI_ManageTrackers = 1,
    // ShowUI_QuickStart = 2, // Deprecated
    ShowUI_Pairing = 3,
    ShowUI_Settings = 4,
    ShowUI_DebugCommands = 5,
    ShowUI_FullControllerBinding = 6,
    ShowUI_ManageDrivers = 7,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_ShowUI {
    pub type_: EShowUIType,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_ShowDevTools {
    pub browser_identifier: i32,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub enum EHDCPError {
    HDCPError_None = 0,
    HDCPError_LinkLost = 1,
    HDCPError_Tampered = 2,
    HDCPError_DeviceRevoked = 3,
    HDCPError_Unknown = 4,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct VREvent_HDCPError {
    pub code: EHDCPError,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVREventType {
    VREvent_None = 0,

    VREvent_TrackedDeviceActivated = 100,
    VREvent_TrackedDeviceDeactivated = 101,
    VREvent_TrackedDeviceUpdated = 102,
    VREvent_TrackedDeviceUserInteractionStarted = 103,
    VREvent_TrackedDeviceUserInteractionEnded = 104,
    VREvent_IpdChanged = 105,
    VREvent_EnterStandbyMode = 106,
    VREvent_LeaveStandbyMode = 107,
    VREvent_TrackedDeviceRoleChanged = 108,
    VREvent_WatchdogWakeUpRequested = 109,
    VREvent_LensDistortionChanged = 110,
    VREvent_PropertyChanged = 111,
    VREvent_WirelessDisconnect = 112,
    VREvent_WirelessReconnect = 113,

    VREvent_ButtonPress = 200,   // data is controller
    VREvent_ButtonUnpress = 201, // data is controller
    VREvent_ButtonTouch = 202,   // data is controller
    VREvent_ButtonUntouch = 203, // data is controller

    // VREvent_DualAnalog_Press			= 250, // No longer sent
    // VREvent_DualAnalog_Unpress		= 251, // No longer sent
    // VREvent_DualAnalog_Touch			= 252, // No longer sent
    // VREvent_DualAnalog_Untouch		= 253, // No longer sent
    // VREvent_DualAnalog_Move			= 254, // No longer sent
    // VREvent_DualAnalog_ModeSwitch1	= 255, // No longer sent
    // VREvent_DualAnalog_ModeSwitch2	= 256, // No longer sent
    VREvent_Modal_Cancel = 257, // Sent to overlays with the

    VREvent_MouseMove = 300,           // data is mouse
    VREvent_MouseButtonDown = 301,     // data is mouse
    VREvent_MouseButtonUp = 302,       // data is mouse
    VREvent_FocusEnter = 303,          // data is overlay
    VREvent_FocusLeave = 304,          // data is overlay
    VREvent_ScrollDiscrete = 305,      // data is scroll
    VREvent_TouchPadMove = 306,        // data is mouse
    VREvent_OverlayFocusChanged = 307, // data is overlay, global event
    VREvent_ReloadOverlays = 308,
    VREvent_ScrollSmooth = 309, // data is scroll
    VREvent_LockMousePosition = 310,
    VREvent_UnlockMousePosition = 311,

    VREvent_InputFocusCaptured = 400, // data is process DEPRECATED
    VREvent_InputFocusReleased = 401, // data is process DEPRECATED
    // VREvent_SceneFocusLost			= 402, // data is process
    // VREvent_SceneFocusGained			= 403, // data is process
    VREvent_SceneApplicationChanged = 404, // data is process - The App actually drawing the scene changed (usually to or from the compositor)
    VREvent_SceneFocusChanged = 405,       // data is process - New app got access to draw the scene
    VREvent_InputFocusChanged = 406,       // data is process
    // VREvent_SceneApplicationSecondaryRenderingStarted = 407,
    VREvent_SceneApplicationUsingWrongGraphicsAdapter = 408, // data is process
    VREvent_ActionBindingReloaded = 409, // data is process - The App that action binds reloaded for

    VREvent_HideRenderModels = 410, // Sent to the scene application to request hiding render models temporarily
    VREvent_ShowRenderModels = 411, // Sent to the scene application to request restoring render model visibility

    VREvent_SceneApplicationStateChanged = 412, // No data; but query VRApplications()->GetSceneApplicationState();

    VREvent_ConsoleOpened = 420,
    VREvent_ConsoleClosed = 421,

    VREvent_OverlayShown = 500,
    VREvent_OverlayHidden = 501,
    VREvent_DashboardActivated = 502,
    VREvent_DashboardDeactivated = 503,
    //VREvent_DashboardThumbSelected		= 504, // Sent to the overlay manager - data is overlay - No longer sent
    VREvent_DashboardRequested = 505, // Sent to the overlay manager - data is overlay
    VREvent_ResetDashboard = 506,     // Send to the overlay manager
    //VREvent_RenderToast					= 507, // Send to the dashboard to render a toast - data is the notification ID -- no longer sent
    VREvent_ImageLoaded = 508, // Sent to overlays when a SetOverlayRaw or SetOverlayFromFile call finishes loading
    VREvent_ShowKeyboard = 509, // Sent to keyboard renderer in the dashboard to invoke it
    VREvent_HideKeyboard = 510, // Sent to keyboard renderer in the dashboard to hide it
    VREvent_OverlayGamepadFocusGained = 511, // Sent to an overlay when IVROverlay::SetFocusOverlay is called on it
    VREvent_OverlayGamepadFocusLost = 512, // Send to an overlay when it previously had focus and IVROverlay::SetFocusOverlay is called on something else
    VREvent_OverlaySharedTextureChanged = 513,
    //VREvent_DashboardGuideButtonDown	= 514, // These are no longer sent
    //VREvent_DashboardGuideButtonUp		= 515,
    VREvent_ScreenshotTriggered = 516, // Screenshot button combo was pressed, Dashboard should request a screenshot
    VREvent_ImageFailed = 517, // Sent to overlays when a SetOverlayRaw or SetOverlayfromFail fails to load
    VREvent_DashboardOverlayCreated = 518,
    VREvent_SwitchGamepadFocus = 519,

    // Screenshot API
    VREvent_RequestScreenshot = 520, // Sent by vrclient application to compositor to take a screenshot
    VREvent_ScreenshotTaken = 521, // Sent by compositor to the application that the screenshot has been taken
    VREvent_ScreenshotFailed = 522, // Sent by compositor to the application that the screenshot failed to be taken
    VREvent_SubmitScreenshotToDashboard = 523, // Sent by compositor to the dashboard that a completed screenshot was submitted
    VREvent_ScreenshotProgressToDashboard = 524, // Sent by compositor to the dashboard that a completed screenshot was submitted

    VREvent_PrimaryDashboardDeviceChanged = 525,
    VREvent_RoomViewShown = 526, // Sent by compositor whenever room-view is enabled
    VREvent_RoomViewHidden = 527, // Sent by compositor whenever room-view is disabled
    VREvent_ShowUI = 528,        // data is showUi
    VREvent_ShowDevTools = 529,  // data is showDevTools
    VREvent_DesktopViewUpdating = 530,
    VREvent_DesktopViewReady = 531,

    VREvent_Notification_Shown = 600,
    VREvent_Notification_Hidden = 601,
    VREvent_Notification_BeginInteraction = 602,
    VREvent_Notification_Destroyed = 603,

    VREvent_Quit = 700,        // data is process
    VREvent_ProcessQuit = 701, // data is process
    //VREvent_QuitAborted_UserPrompt			= 702, // data is process
    VREvent_QuitAcknowledged = 703,    // data is process
    VREvent_DriverRequestedQuit = 704, // The driver has requested that SteamVR shut down
    VREvent_RestartRequested = 705, // A driver or other component wants the user to restart SteamVR

    VREvent_ChaperoneDataHasChanged = 800, // this will never happen with the new chaperone system
    VREvent_ChaperoneUniverseHasChanged = 801,
    VREvent_ChaperoneTempDataHasChanged = 802, // this will never happen with the new chaperone system
    VREvent_ChaperoneSettingsHaveChanged = 803,
    VREvent_SeatedZeroPoseReset = 804,
    VREvent_ChaperoneFlushCache = 805, // Sent when the process needs to reload any cached data it retrieved from VRChaperone()
    VREvent_ChaperoneRoomSetupStarting = 806, // Triggered by CVRChaperoneClient::RoomSetupStarting
    VREvent_ChaperoneRoomSetupFinished = 807, // Triggered by CVRChaperoneClient::CommitWorkingCopy
    VREvent_StandingZeroPoseReset = 808,

    VREvent_AudioSettingsHaveChanged = 820,

    VREvent_BackgroundSettingHasChanged = 850,
    VREvent_CameraSettingsHaveChanged = 851,
    VREvent_ReprojectionSettingHasChanged = 852,
    VREvent_ModelSkinSettingsHaveChanged = 853,
    VREvent_EnvironmentSettingsHaveChanged = 854,
    VREvent_PowerSettingsHaveChanged = 855,
    VREvent_EnableHomeAppSettingsHaveChanged = 856,
    VREvent_SteamVRSectionSettingChanged = 857,
    VREvent_LighthouseSectionSettingChanged = 858,
    VREvent_NullSectionSettingChanged = 859,
    VREvent_UserInterfaceSectionSettingChanged = 860,
    VREvent_NotificationsSectionSettingChanged = 861,
    VREvent_KeyboardSectionSettingChanged = 862,
    VREvent_PerfSectionSettingChanged = 863,
    VREvent_DashboardSectionSettingChanged = 864,
    VREvent_WebInterfaceSectionSettingChanged = 865,
    VREvent_TrackersSectionSettingChanged = 866,
    VREvent_LastKnownSectionSettingChanged = 867,
    VREvent_DismissedWarningsSectionSettingChanged = 868,
    VREvent_GpuSpeedSectionSettingChanged = 869,
    VREvent_WindowsMRSectionSettingChanged = 870,
    VREvent_OtherSectionSettingChanged = 871,

    VREvent_StatusUpdate = 900,

    VREvent_WebInterface_InstallDriverCompleted = 950,

    VREvent_MCImageUpdated = 1000,

    VREvent_FirmwareUpdateStarted = 1100,
    VREvent_FirmwareUpdateFinished = 1101,

    VREvent_KeyboardClosed = 1200,
    VREvent_KeyboardCharInput = 1201,
    VREvent_KeyboardDone = 1202, // Sent when DONE button clicked on keyboard

    //VREvent_ApplicationTransitionStarted		= 1300,
    //VREvent_ApplicationTransitionAborted		= 1301,
    //VREvent_ApplicationTransitionNewAppStarted	= 1302,
    VREvent_ApplicationListUpdated = 1303,
    VREvent_ApplicationMimeTypeLoad = 1304,
    // VREvent_ApplicationTransitionNewAppLaunchComplete = 1305,
    VREvent_ProcessConnected = 1306,
    VREvent_ProcessDisconnected = 1307,

    //VREvent_Compositor_MirrorWindowShown		= 1400, // DEPRECATED
    //VREvent_Compositor_MirrorWindowHidden		= 1401, // DEPRECATED
    VREvent_Compositor_ChaperoneBoundsShown = 1410,
    VREvent_Compositor_ChaperoneBoundsHidden = 1411,
    VREvent_Compositor_DisplayDisconnected = 1412,
    VREvent_Compositor_DisplayReconnected = 1413,
    VREvent_Compositor_HDCPError = 1414, // data is hdcpError
    VREvent_Compositor_ApplicationNotResponding = 1415,
    VREvent_Compositor_ApplicationResumed = 1416,
    VREvent_Compositor_OutOfVideoMemory = 1417,
    VREvent_Compositor_DisplayModeNotSupported = 1418, // k_pch_SteamVR_PreferredRefreshRate
    VREvent_Compositor_StageOverrideReady = 1419,

    VREvent_TrackedCamera_StartVideoStream = 1500,
    VREvent_TrackedCamera_StopVideoStream = 1501,
    VREvent_TrackedCamera_PauseVideoStream = 1502,
    VREvent_TrackedCamera_ResumeVideoStream = 1503,
    VREvent_TrackedCamera_EditingSurface = 1550,

    VREvent_PerformanceTest_EnableCapture = 1600,
    VREvent_PerformanceTest_DisableCapture = 1601,
    VREvent_PerformanceTest_FidelityLevel = 1602,

    VREvent_MessageOverlay_Closed = 1650,
    VREvent_MessageOverlayCloseRequested = 1651,

    VREvent_Input_HapticVibration = 1700, // data is hapticVibration
    VREvent_Input_BindingLoadFailed = 1701, // data is inputBinding
    VREvent_Input_BindingLoadSuccessful = 1702, // data is inputBinding
    VREvent_Input_ActionManifestReloaded = 1703, // no data
    VREvent_Input_ActionManifestLoadFailed = 1704, // data is actionManifest
    VREvent_Input_ProgressUpdate = 1705,  // data is progressUpdate
    VREvent_Input_TrackerActivated = 1706,
    VREvent_Input_BindingsUpdated = 1707,
    VREvent_Input_BindingSubscriptionChanged = 1708,

    VREvent_SpatialAnchors_PoseUpdated = 1800, // data is spatialAnchor. broadcast
    VREvent_SpatialAnchors_DescriptorUpdated = 1801, // data is spatialAnchor. broadcast
    VREvent_SpatialAnchors_RequestPoseUpdate = 1802, // data is spatialAnchor. sent to specific driver
    VREvent_SpatialAnchors_RequestDescriptorUpdate = 1803, // data is spatialAnchor. sent to specific driver

    VREvent_SystemReport_Started = 1900, // user or system initiated generation of a system report. broadcast

    VREvent_Monitor_ShowHeadsetView = 2000, // data is process
    VREvent_Monitor_HideHeadsetView = 2001, // data is process

    // Vendors are free to expose private events in this reserved region
    VREvent_VendorSpecific_Reserved_Start = 10000,
    VREvent_VendorSpecific_Reserved_End = 19999,
}

#[repr(C)]
pub union VREvent_Data {
    pub reserved: VREvent_Reserved,
    pub controller: VREvent_Controller,
    pub mouse: VREvent_Mouse,
    pub scroll: VREvent_Scroll,
    pub process: VREvent_Process,
    pub notification: VREvent_Notification,
    pub overlay: VREvent_Overlay,
    pub status: VREvent_Status,
    pub keyboard: VREvent_Keyboard,
    pub ipd: VREvent_Ipd,
    pub chaperone: VREvent_Chaperone,
    pub performance_test: VREvent_PerformanceTest,
    pub touch_pad_move: VREvent_TouchPadMove,
    pub seated_zero_pose_reset: VREvent_SeatedZeroPoseReset,
    pub screenshot: VREvent_Screenshot,
    pub screenshot_progress: VREvent_ScreenshotProgress,
    pub application_launch: VREvent_ApplicationLaunch,
    pub camera_surface: VREvent_EditingCameraSurface,
    pub message_overlay: VREvent_MessageOverlay,
    pub property: VREvent_Property,
    pub haptic_vibration: VREvent_HapticVibration,
    pub web_console: VREvent_WebConsole,
    pub input_binding: VREvent_InputBindingLoad,
    pub action_manifest: VREvent_InputActionManifestLoad,
    pub spatial_anchor: VREvent_SpatialAnchor,
    pub progress_update: VREvent_ProgressUpdate,
    pub show_ui: VREvent_ShowUI,
    pub show_dev_tools: VREvent_ShowDevTools,
    pub hdcp_error: VREvent_HDCPError,
}

pub struct VREvent {
    pub event_type: u32,
    pub tracked_device_index: TrackedDeviceIndex,
    pub event_age_seconds: f32,
    pub data: VREvent_Data,
}

pub type VRComponentProperties = u32;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRComponentProperty {
    VRComponentProperty_IsStatic = (1 << 0),
    VRComponentProperty_IsVisible = (1 << 1),
    VRComponentProperty_IsTouched = (1 << 2),
    VRComponentProperty_IsPressed = (1 << 3),
    VRComponentProperty_IsScrolled = (1 << 4),
    VRComponentProperty_IsHighlighted = (1 << 5),
}

#[allow(non_camel_case_types)]
pub struct RenderModel_ComponentState {
    pub tracking_to_component_render_model: HmdMatrix34,
    pub tracking_to_component_local: HmdMatrix34,
    pub properties: VRComponentProperties,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRInputError {
    VRInputError_None = 0,
    VRInputError_NameNotFound = 1,
    VRInputError_WrongType = 2,
    VRInputError_InvalidHandle = 3,
    VRInputError_InvalidParam = 4,
    VRInputError_NoSteam = 5,
    VRInputError_MaxCapacityReached = 6,
    VRInputError_IPCError = 7,
    VRInputError_NoActiveActionSet = 8,
    VRInputError_InvalidDevice = 9,
    VRInputError_InvalidSkeleton = 10,
    VRInputError_InvalidBoneCount = 11,
    VRInputError_InvalidCompressedData = 12,
    VRInputError_NoData = 13,
    VRInputError_BufferTooSmall = 14,
    VRInputError_MismatchedActionManifest = 15,
    VRInputError_MissingSkeletonData = 16,
    VRInputError_InvalidBoneIndex = 17,
    VRInputError_InvalidPriority = 18,
    VRInputError_PermissionDenied = 19,
    VRInputError_InvalidRenderModel = 20,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRSpatialAnchorError {
    VRSpatialAnchorError_Success = 0,
    VRSpatialAnchorError_Internal = 1,
    VRSpatialAnchorError_UnknownHandle = 2,
    VRSpatialAnchorError_ArrayTooSmall = 3,
    VRSpatialAnchorError_InvalidDescriptorChar = 4,
    VRSpatialAnchorError_NotYetAvailable = 5,
    VRSpatialAnchorError_NotAvailableInThisUniverse = 6,
    VRSpatialAnchorError_PermanentlyUnavailable = 7,
    VRSpatialAnchorError_WrongDriver = 8,
    VRSpatialAnchorError_DescriptorTooLong = 9,
    VRSpatialAnchorError_Unknown = 10,
    VRSpatialAnchorError_NoRoomCalibration = 11,
    VRSpatialAnchorError_InvalidArgument = 12,
    VRSpatialAnchorError_UnknownDriver = 13,
}

pub struct HiddenAreaMesh {
    pub vertex_data: *const HmdVector2,
    pub triangle_count: u32,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EHiddenAreaMeshType {
    k_eHiddenAreaMesh_Standard = 0,
    k_eHiddenAreaMesh_Inverse = 1,
    k_eHiddenAreaMesh_LineLoop = 2,

    k_eHiddenAreaMesh_Max = 3,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRControllerAxisType {
    k_eControllerAxis_None = 0,
    k_eControllerAxis_TrackPad = 1,
    k_eControllerAxis_Joystick = 2,
    k_eControllerAxis_Trigger = 3, // Analog trigger data is in the X axis
}

pub struct VRControllerAxis {
    pub x: f32, // Ranges from -1.0 to 1.0 for joysticks and track pads. Ranges from 0.0 to 1.0 for triggers were 0 is fully released.
    pub y: f32, // Ranges from -1.0 to 1.0 for joysticks and track pads. Is always 0.0 for triggers.
}

#[allow(non_upper_case_globals)]
pub const k_unControllerStateAxisCount: usize = 5;

pub struct VRControllerState001 {
    // If packet num matches that on your prior call, then the controller state hasn't been changed since
    // your last call and there is no need to process it
    pub packet_num: u32,

    // bit flags for each of the buttons. Use ButtonMaskFromId to turn an ID into a mask
    pub button_pressed: u64,
    pub button_touched: u64,

    // Axis data for the controller's analog inputs
    pub axis: [VRControllerAxis; k_unControllerStateAxisCount],
}

pub type VRControllerState = VRControllerState001;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRControllerEventOutputType {
    ControllerEventOutput_OSEvents = 0,
    ControllerEventOutput_VREvents = 1,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum ECollisionBoundsStyle {
    COLLISION_BOUNDS_STYLE_BEGINNER = 0,
    COLLISION_BOUNDS_STYLE_INTERMEDIATE,
    COLLISION_BOUNDS_STYLE_SQUARES,
    COLLISION_BOUNDS_STYLE_ADVANCED,
    COLLISION_BOUNDS_STYLE_NONE,

    COLLISION_BOUNDS_STYLE_COUNT,
}

pub type VROverlayHandle = u64;
pub const OVERLAY_HANDLE_INVALID: VROverlayHandle = 0;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVROverlayError {
    VROverlayError_None = 0,

    VROverlayError_UnknownOverlay = 10,
    VROverlayError_InvalidHandle = 11,
    VROverlayError_PermissionDenied = 12,
    VROverlayError_OverlayLimitExceeded = 13, // No more overlays could be created because the maximum number already exist
    VROverlayError_WrongVisibilityType = 14,
    VROverlayError_KeyTooLong = 15,
    VROverlayError_NameTooLong = 16,
    VROverlayError_KeyInUse = 17,
    VROverlayError_WrongTransformType = 18,
    VROverlayError_InvalidTrackedDevice = 19,
    VROverlayError_InvalidParameter = 20,
    VROverlayError_ThumbnailCantBeDestroyed = 21,
    VROverlayError_ArrayTooSmall = 22,
    VROverlayError_RequestFailed = 23,
    VROverlayError_InvalidTexture = 24,
    VROverlayError_UnableToLoadFile = 25,
    VROverlayError_KeyboardAlreadyInUse = 26,
    VROverlayError_NoNeighbor = 27,
    VROverlayError_TooManyMaskPrimitives = 29,
    VROverlayError_BadMaskPrimitive = 30,
    VROverlayError_TextureAlreadyLocked = 31,
    VROverlayError_TextureLockCapacityReached = 32,
    VROverlayError_TextureNotLocked = 33,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRApplicationType {
    VRApplication_Other = 0, // Some other kind of application that isn't covered by the other entries
    VRApplication_Scene = 1, // Application will submit 3D frames
    VRApplication_Overlay = 2, // Application only interacts with overlays
    VRApplication_Background = 3, // Application should not start SteamVR if it's not already running, and should not
    // keep it running if everything else quits.
    VRApplication_Utility = 4, // Init should not try to load any drivers. The application needs access to utility
    // interfaces (like IVRSettings and IVRApplications) but not hardware.
    VRApplication_VRMonitor = 5,      // Reserved for vrmonitor
    VRApplication_SteamWatchdog = 6,  // Reserved for Steam
    VRApplication_Bootstrapper = 7,   // reserved for vrstartup
    VRApplication_WebHelper = 8,      // reserved for vrwebhelper
    VRApplication_OpenXRInstance = 9, // reserved for openxr (created instance, but not session yet)
    VRApplication_OpenXRScene = 10,   // reserved for openxr (started session)
    VRApplication_OpenXROverlay = 11, // reserved for openxr (started overlay session)
    VRApplication_Prism = 12,         // reserved for the vrprismhost process

    VRApplication_Max,
}

pub fn is_openxr_app_type(type_: EVRApplicationType) -> bool {
    match type_ {
        EVRApplicationType::VRApplication_OpenXRInstance
        | EVRApplicationType::VRApplication_OpenXROverlay
        | EVRApplicationType::VRApplication_OpenXRScene => true,
        _ => false,
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRFirmwareError {
    VRFirmwareError_None = 0,
    VRFirmwareError_Success = 1,
    VRFirmwareError_Fail = 2,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRNotificationError {
    VRNotificationError_OK = 0,
    VRNotificationError_InvalidNotificationId = 100,
    VRNotificationError_NotificationQueueFull = 101,
    VRNotificationError_InvalidOverlayHandle = 102,
    VRNotificationError_SystemWithUserValueAlreadyExists = 103,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRSkeletalMotionRange {
    // The range of motion of the skeleton takes into account any physical limits imposed by
    // the controller itself.  This will tend to be the most accurate pose compared to the user's
    // actual hand pose, but might not allow a closed fist for example
    VRSkeletalMotionRange_WithController = 0,

    // Retarget the range of motion provided by the input device to make the hand appear to move
    // as if it was not holding a controller.  eg: map "hand grasping controller" to "closed fist"
    VRSkeletalMotionRange_WithoutController = 1,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRSkeletalTrackingLevel {
    // body part location can't be directly determined by the device. Any skeletal pose provided by
    // the device is estimated by assuming the position required to active buttons, triggers, joysticks,
    // or other input sensors.
    // E.g. Vive Controller, Gamepad
    VRSkeletalTracking_Estimated = 0,

    // body part location can be measured directly but with fewer degrees of freedom than the actual body
    // part. Certain body part positions may be unmeasured by the device and estimated from other input data.
    // E.g. Index Controllers, gloves that only measure finger curl
    VRSkeletalTracking_Partial = 1,

    // Body part location can be measured directly throughout the entire range of motion of the body part.
    // E.g. Mocap suit for the full body, gloves that measure rotation of each finger segment
    VRSkeletalTracking_Full = 2,
    /* TODO: invalid syntax in rust */
    // VRSkeletalTrackingLevel_Count,
    // VRSkeletalTrackingLevel_Max = VRSkeletalTrackingLevel_Count - 1
}

pub type BoneIndex = i32;
pub const INVALID_BONE_INDEX: BoneIndex = -1;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum EVRInitError {
    VRInitError_None = 0,
    VRInitError_Unknown = 1,

    VRInitError_Init_InstallationNotFound = 100,
    VRInitError_Init_InstallationCorrupt = 101,
    VRInitError_Init_VRClientDLLNotFound = 102,
    VRInitError_Init_FileNotFound = 103,
    VRInitError_Init_FactoryNotFound = 104,
    VRInitError_Init_InterfaceNotFound = 105,
    VRInitError_Init_InvalidInterface = 106,
    VRInitError_Init_UserConfigDirectoryInvalid = 107,
    VRInitError_Init_HmdNotFound = 108,
    VRInitError_Init_NotInitialized = 109,
    VRInitError_Init_PathRegistryNotFound = 110,
    VRInitError_Init_NoConfigPath = 111,
    VRInitError_Init_NoLogPath = 112,
    VRInitError_Init_PathRegistryNotWritable = 113,
    VRInitError_Init_AppInfoInitFailed = 114,
    VRInitError_Init_Retry = 115, // Used internally to cause retries to vrserver
    VRInitError_Init_InitCanceledByUser = 116, // The calling application should silently exit. The user canceled app startup
    VRInitError_Init_AnotherAppLaunching = 117,
    VRInitError_Init_SettingsInitFailed = 118,
    VRInitError_Init_ShuttingDown = 119,
    VRInitError_Init_TooManyObjects = 120,
    VRInitError_Init_NoServerForBackgroundApp = 121,
    VRInitError_Init_NotSupportedWithCompositor = 122,
    VRInitError_Init_NotAvailableToUtilityApps = 123,
    VRInitError_Init_Internal = 124,
    VRInitError_Init_HmdDriverIdIsNone = 125,
    VRInitError_Init_HmdNotFoundPresenceFailed = 126,
    VRInitError_Init_VRMonitorNotFound = 127,
    VRInitError_Init_VRMonitorStartupFailed = 128,
    VRInitError_Init_LowPowerWatchdogNotSupported = 129,
    VRInitError_Init_InvalidApplicationType = 130,
    VRInitError_Init_NotAvailableToWatchdogApps = 131,
    VRInitError_Init_WatchdogDisabledInSettings = 132,
    VRInitError_Init_VRDashboardNotFound = 133,
    VRInitError_Init_VRDashboardStartupFailed = 134,
    VRInitError_Init_VRHomeNotFound = 135,
    VRInitError_Init_VRHomeStartupFailed = 136,
    VRInitError_Init_RebootingBusy = 137,
    VRInitError_Init_FirmwareUpdateBusy = 138,
    VRInitError_Init_FirmwareRecoveryBusy = 139,
    VRInitError_Init_USBServiceBusy = 140,
    VRInitError_Init_VRWebHelperStartupFailed = 141,
    VRInitError_Init_TrackerManagerInitFailed = 142,
    VRInitError_Init_AlreadyRunning = 143,
    VRInitError_Init_FailedForVrMonitor = 144,
    VRInitError_Init_PropertyManagerInitFailed = 145,
    VRInitError_Init_WebServerFailed = 146,
    VRInitError_Init_IllegalTypeTransition = 147,
    VRInitError_Init_MismatchedRuntimes = 148,
    VRInitError_Init_InvalidProcessId = 149,
    VRInitError_Init_VRServiceStartupFailed = 150,
    VRInitError_Init_PrismNeedsNewDrivers = 151,
    VRInitError_Init_PrismStartupTimedOut = 152,
    VRInitError_Init_CouldNotStartPrism = 153,
    VRInitError_Init_CreateDriverDirectDeviceFailed = 154,
    VRInitError_Init_PrismExitedUnexpectedly = 155,

    VRInitError_Driver_Failed = 200,
    VRInitError_Driver_Unknown = 201,
    VRInitError_Driver_HmdUnknown = 202,
    VRInitError_Driver_NotLoaded = 203,
    VRInitError_Driver_RuntimeOutOfDate = 204,
    VRInitError_Driver_HmdInUse = 205,
    VRInitError_Driver_NotCalibrated = 206,
    VRInitError_Driver_CalibrationInvalid = 207,
    VRInitError_Driver_HmdDisplayNotFound = 208,
    VRInitError_Driver_TrackedDeviceInterfaceUnknown = 209,
    // VRInitError_Driver_HmdDisplayNotFoundAfterFix = 210, // not needed: here for historic reasons
    VRInitError_Driver_HmdDriverIdOutOfBounds = 211,
    VRInitError_Driver_HmdDisplayMirrored = 212,
    VRInitError_Driver_HmdDisplayNotFoundLaptop = 213,
    // Never make error 259 because we return it from main and it would conflict with STILL_ACTIVE
    VRInitError_IPC_ServerInitFailed = 300,
    VRInitError_IPC_ConnectFailed = 301,
    VRInitError_IPC_SharedStateInitFailed = 302,
    VRInitError_IPC_CompositorInitFailed = 303,
    VRInitError_IPC_MutexInitFailed = 304,
    VRInitError_IPC_Failed = 305,
    VRInitError_IPC_CompositorConnectFailed = 306,
    VRInitError_IPC_CompositorInvalidConnectResponse = 307,
    VRInitError_IPC_ConnectFailedAfterMultipleAttempts = 308,
    VRInitError_IPC_ConnectFailedAfterTargetExited = 309,
    VRInitError_IPC_NamespaceUnavailable = 310,

    VRInitError_Compositor_Failed = 400,
    VRInitError_Compositor_D3D11HardwareRequired = 401,
    VRInitError_Compositor_FirmwareRequiresUpdate = 402,
    VRInitError_Compositor_OverlayInitFailed = 403,
    VRInitError_Compositor_ScreenshotsInitFailed = 404,
    VRInitError_Compositor_UnableToCreateDevice = 405,
    VRInitError_Compositor_SharedStateIsNull = 406,
    VRInitError_Compositor_NotificationManagerIsNull = 407,
    VRInitError_Compositor_ResourceManagerClientIsNull = 408,
    VRInitError_Compositor_MessageOverlaySharedStateInitFailure = 409,
    VRInitError_Compositor_PropertiesInterfaceIsNull = 410,
    VRInitError_Compositor_CreateFullscreenWindowFailed = 411,
    VRInitError_Compositor_SettingsInterfaceIsNull = 412,
    VRInitError_Compositor_FailedToShowWindow = 413,
    VRInitError_Compositor_DistortInterfaceIsNull = 414,
    VRInitError_Compositor_DisplayFrequencyFailure = 415,
    VRInitError_Compositor_RendererInitializationFailed = 416,
    VRInitError_Compositor_DXGIFactoryInterfaceIsNull = 417,
    VRInitError_Compositor_DXGIFactoryCreateFailed = 418,
    VRInitError_Compositor_DXGIFactoryQueryFailed = 419,
    VRInitError_Compositor_InvalidAdapterDesktop = 420,
    VRInitError_Compositor_InvalidHmdAttachment = 421,
    VRInitError_Compositor_InvalidOutputDesktop = 422,
    VRInitError_Compositor_InvalidDeviceProvided = 423,
    VRInitError_Compositor_D3D11RendererInitializationFailed = 424,
    VRInitError_Compositor_FailedToFindDisplayMode = 425,
    VRInitError_Compositor_FailedToCreateSwapChain = 426,
    VRInitError_Compositor_FailedToGetBackBuffer = 427,
    VRInitError_Compositor_FailedToCreateRenderTarget = 428,
    VRInitError_Compositor_FailedToCreateDXGI2SwapChain = 429,
    VRInitError_Compositor_FailedtoGetDXGI2BackBuffer = 430,
    VRInitError_Compositor_FailedToCreateDXGI2RenderTarget = 431,
    VRInitError_Compositor_FailedToGetDXGIDeviceInterface = 432,
    VRInitError_Compositor_SelectDisplayMode = 433,
    VRInitError_Compositor_FailedToCreateNvAPIRenderTargets = 434,
    VRInitError_Compositor_NvAPISetDisplayMode = 435,
    VRInitError_Compositor_FailedToCreateDirectModeDisplay = 436,
    VRInitError_Compositor_InvalidHmdPropertyContainer = 437,
    VRInitError_Compositor_UpdateDisplayFrequency = 438,
    VRInitError_Compositor_CreateRasterizerState = 439,
    VRInitError_Compositor_CreateWireframeRasterizerState = 440,
    VRInitError_Compositor_CreateSamplerState = 441,
    VRInitError_Compositor_CreateClampToBorderSamplerState = 442,
    VRInitError_Compositor_CreateAnisoSamplerState = 443,
    VRInitError_Compositor_CreateOverlaySamplerState = 444,
    VRInitError_Compositor_CreatePanoramaSamplerState = 445,
    VRInitError_Compositor_CreateFontSamplerState = 446,
    VRInitError_Compositor_CreateNoBlendState = 447,
    VRInitError_Compositor_CreateBlendState = 448,
    VRInitError_Compositor_CreateAlphaBlendState = 449,
    VRInitError_Compositor_CreateBlendStateMaskR = 450,
    VRInitError_Compositor_CreateBlendStateMaskG = 451,
    VRInitError_Compositor_CreateBlendStateMaskB = 452,
    VRInitError_Compositor_CreateDepthStencilState = 453,
    VRInitError_Compositor_CreateDepthStencilStateNoWrite = 454,
    VRInitError_Compositor_CreateDepthStencilStateNoDepth = 455,
    VRInitError_Compositor_CreateFlushTexture = 456,
    VRInitError_Compositor_CreateDistortionSurfaces = 457,
    VRInitError_Compositor_CreateConstantBuffer = 458,
    VRInitError_Compositor_CreateHmdPoseConstantBuffer = 459,
    VRInitError_Compositor_CreateHmdPoseStagingConstantBuffer = 460,
    VRInitError_Compositor_CreateSharedFrameInfoConstantBuffer = 461,
    VRInitError_Compositor_CreateOverlayConstantBuffer = 462,
    VRInitError_Compositor_CreateSceneTextureIndexConstantBuffer = 463,
    VRInitError_Compositor_CreateReadableSceneTextureIndexConstantBuffer = 464,
    VRInitError_Compositor_CreateLayerGraphicsTextureIndexConstantBuffer = 465,
    VRInitError_Compositor_CreateLayerComputeTextureIndexConstantBuffer = 466,
    VRInitError_Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer = 467,
    VRInitError_Compositor_CreateComputeHmdPoseConstantBuffer = 468,
    VRInitError_Compositor_CreateGeomConstantBuffer = 469,
    VRInitError_Compositor_CreatePanelMaskConstantBuffer = 470,
    VRInitError_Compositor_CreatePixelSimUBO = 471,
    VRInitError_Compositor_CreateMSAARenderTextures = 472,
    VRInitError_Compositor_CreateResolveRenderTextures = 473,
    VRInitError_Compositor_CreateComputeResolveRenderTextures = 474,
    VRInitError_Compositor_CreateDriverDirectModeResolveTextures = 475,
    VRInitError_Compositor_OpenDriverDirectModeResolveTextures = 476,
    VRInitError_Compositor_CreateFallbackSyncTexture = 477,
    VRInitError_Compositor_ShareFallbackSyncTexture = 478,
    VRInitError_Compositor_CreateOverlayIndexBuffer = 479,
    VRInitError_Compositor_CreateOverlayVertexBuffer = 480,
    VRInitError_Compositor_CreateTextVertexBuffer = 481,
    VRInitError_Compositor_CreateTextIndexBuffer = 482,
    VRInitError_Compositor_CreateMirrorTextures = 483,
    VRInitError_Compositor_CreateLastFrameRenderTexture = 484,
    VRInitError_Compositor_CreateMirrorOverlay = 485,
    VRInitError_Compositor_FailedToCreateVirtualDisplayBackbuffer = 486,
    VRInitError_Compositor_DisplayModeNotSupported = 487,
    VRInitError_Compositor_CreateOverlayInvalidCall = 488,
    VRInitError_Compositor_CreateOverlayAlreadyInitialized = 489,
    VRInitError_Compositor_FailedToCreateMailbox = 490,
    VRInitError_Compositor_WindowInterfaceIsNull = 491,
    VRInitError_Compositor_SystemLayerCreateInstance = 492,
    VRInitError_Compositor_SystemLayerCreateSession = 493,

    VRInitError_VendorSpecific_UnableToConnectToOculusRuntime = 1000,
    VRInitError_VendorSpecific_WindowsNotInDevMode = 1001,

    VRInitError_VendorSpecific_HmdFound_CantOpenDevice = 1101,
    VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart = 1102,
    VRInitError_VendorSpecific_HmdFound_NoStoredConfig = 1103,
    VRInitError_VendorSpecific_HmdFound_ConfigTooBig = 1104,
    VRInitError_VendorSpecific_HmdFound_ConfigTooSmall = 1105,
    VRInitError_VendorSpecific_HmdFound_UnableToInitZLib = 1106,
    VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion = 1107,
    VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart = 1108,
    VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart = 1109,
    VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext = 1110,
    VRInitError_VendorSpecific_HmdFound_UserDataAddressRange = 1111,
    VRInitError_VendorSpecific_HmdFound_UserDataError = 1112,
    VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck = 1113,
    VRInitError_VendorSpecific_OculusRuntimeBadInstall = 1114,

    VRInitError_Steam_SteamInstallationNotFound = 2000,

    // Strictly a placeholder
    VRInitError_LastError,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRScreenshotType {
    VRScreenshotType_None = 0,
    VRScreenshotType_Mono = 1, // left eye only
    VRScreenshotType_Stereo = 2,
    VRScreenshotType_Cubemap = 3,
    VRScreenshotType_MonoPanorama = 4,
    VRScreenshotType_StereoPanorama = 5,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRScreenshotPropertyFilenames {
    VRScreenshotPropertyFilenames_Preview = 0,
    VRScreenshotPropertyFilenames_VR = 1,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRTrackedCameraError {
    VRTrackedCameraError_None = 0,
    VRTrackedCameraError_OperationFailed = 100,
    VRTrackedCameraError_InvalidHandle = 101,
    VRTrackedCameraError_InvalidFrameHeaderVersion = 102,
    VRTrackedCameraError_OutOfHandles = 103,
    VRTrackedCameraError_IPCFailure = 104,
    VRTrackedCameraError_NotSupportedForThisDevice = 105,
    VRTrackedCameraError_SharedMemoryFailure = 106,
    VRTrackedCameraError_FrameBufferingFailure = 107,
    VRTrackedCameraError_StreamSetupFailure = 108,
    VRTrackedCameraError_InvalidGLTextureId = 109,
    VRTrackedCameraError_InvalidSharedTextureHandle = 110,
    VRTrackedCameraError_FailedToGetGLTextureId = 111,
    VRTrackedCameraError_SharedTextureFailure = 112,
    VRTrackedCameraError_NoFrameAvailable = 113,
    VRTrackedCameraError_InvalidArgument = 114,
    VRTrackedCameraError_InvalidFrameBufferSize = 115,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRTrackedCameraFrameLayout {
    EVRTrackedCameraFrameLayout_Mono = 0x0001,
    EVRTrackedCameraFrameLayout_Stereo = 0x0002,
    EVRTrackedCameraFrameLayout_VerticalLayout = 0x0010, // Stereo frames are Top/Bottom (left/right)
    EVRTrackedCameraFrameLayout_HorizontalLayout = 0x0020, // Stereo frames are Left/Right
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRTrackedCameraFrameType {
    VRTrackedCameraFrameType_Distorted = 0, // This is the camera video frame size in pixels, still distorted.
    VRTrackedCameraFrameType_Undistorted, // In pixels, an undistorted inscribed rectangle region without invalid regions. This size is subject to changes shortly.
    VRTrackedCameraFrameType_MaximumUndistorted, // In pixels, maximum undistorted with invalid regions. Non zero alpha component identifies valid regions.
    MAX_CAMERA_FRAME_TYPES,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRDistortionFunctionType {
    VRDistortionFunctionType_None,
    VRDistortionFunctionType_FTheta,
    VRDistortionFunctionType_Extended_FTheta,
    MAX_DISTORTION_FUNCTION_TYPES,
}

pub const MAX_DISTORTION_FUNCTION_PARAMETERS: u32 = 8;

pub type TrackedCameraHandle = u64;
pub const INVALID_TRACKED_CAMERA_HANDLE: TrackedCameraHandle = 0;

pub struct CameraVideoStreamFrameHeader {
    pub frame_type: EVRTrackedCameraFrameType,

    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: u32,

    pub frame_sequence: u32,

    pub tracked_device_pose: TrackedDevicePose,

    pub frame_exposure_time: u64, // mid-point of the exposure of the image in host system ticks
}

pub type ScreenshotHandle = u32;
pub const SCREENSHOT_HANDLE_INVALID: ScreenshotHandle = 0;

/** Compositor frame timing reprojection flags. */
#[allow(non_upper_case_globals)]
pub const VRCompositor_ReprojectionReason_Cpu: u32 = 0x01;
#[allow(non_upper_case_globals)]
pub const VRCompositor_ReprojectionReason_Gpu: u32 = 0x02;
#[allow(non_upper_case_globals)]
pub const VRCompositor_ReprojectionAsync: u32 = 0x04; // This flag indicates the async reprojection mode is active,
                                                      // but does not indicate if reprojection actually happened or not.
                                                      // Use the ReprojectionReason flags above to check if reprojection
                                                      // was actually applied (i.e. scene texture was reused).
                                                      // NumFramePresents > 1 also indicates the scene texture was reused,
                                                      // and also the number of times that it was presented in total.
#[allow(non_upper_case_globals)]
pub const VRCompositor_ReprojectionMotion: u32 = 0x08; // This flag indicates whether or not motion smoothing was triggered for this frame
#[allow(non_upper_case_globals)]
pub const VRCompositor_PredictionMask: u32 = 0xF0; // The runtime may predict more than one frame (up to four) ahead if
                                                   // it detects the application is taking too long to render. These two
                                                   // bits will contain the count of additional frames (normally zero).
                                                   // Use the VR_COMPOSITOR_ADDITIONAL_PREDICTED_FRAMES macro to read from
                                                   // the latest frame timing entry.
#[allow(non_upper_case_globals)]
pub const VRCompositor_ThrottleMask: u32 = 0xF00; // Number of frames the compositor is throttling the application.
                                                  // Use the VR_COMPOSITOR_NUMBER_OF_THROTTLED_FRAMES macro to read from
                                                  // the latest frame timing entry.

#[macro_export]
macro_rules! VR_COMPOSITOR_ADDITIONAL_PREDICTED_FRAMES {
    (timing) => {
        (((timing).m_nReprojectionFlags & VRCompositor_PredictionMask) >> 4)
    };
}

#[macro_export]
macro_rules! VR_COMPOSITOR_NUMBER_OF_THROTTLED_FRAMES {
    (timing) => {
        (((timing).m_nReprojectionFlags & vr::VRCompositor_ThrottleMask) >> 8)
    };
}

#[allow(non_camel_case_types)]
/** Provides a single frame's timing information to the app */
pub struct Compositor_FrameTiming {
    pub size: u32, // Set to sizeof( Compositor_FrameTiming )
    pub frame_index: u32,
    pub num_frame_presents: u32, // number of times this frame was presented
    pub num_mis_presented: u32, // number of times this frame was presented on a vsync other than it was originally predicted to
    pub num_dropped_frames: u32, // number of additional times previous frame was scanned out
    pub reprojection_flags: u32,

    /** Absolute time reference for comparing frames.  This aligns with the vsync that running start is relative to. */
    pub system_time_in_seconds: f64,

    /** These times may include work from other processes due to OS scheduling.
    	* The fewer packets of work these are broken up into, the less likely this will happen.
    	* GPU work can be broken up by calling Flush.  This can sometimes be useful to get the GPU started
    	* processing that work earlier in the frame. */
    pub pre_submit_gpu_ms: f32, // time spent rendering the scene (gpu work submitted between WaitGetPoses and second Submit)
    pub post_submit_gpu_ms: f32, // additional time spent rendering by application (e.g. companion window)
    pub total_render_gpu_ms: f32, // time between work submitted immediately after present (ideally vsync) until the end of compositor submitted work
    pub compositor_render_gpu_ms: f32, // time spend performing distortion correction, rendering chaperone, overlays, etc.
    pub compositor_render_cpu_ms: f32, // time spent on cpu submitting the above work for this frame
    pub compositor_idle_cpu_ms: f32, // time spent waiting for running start (application could have used this much more time)

    /** Miscellaneous measured intervals. */
    pub client_frame_interval_ms: f32, // time between calls to WaitGetPoses
    pub present_call_cpu_ms: f32, // time blocked on call to present (usually 0.0, but can go long)
    pub wait_for_present_cpu_ms: f32, // time spent spin-waiting for frame index to change (not near-zero indicates wait object failure)
    pub submit_frame_ms: f32, // time spent in IVRCompositor::Submit (not near-zero indicates driver issue)

    /** The following are all relative to this frame's SystemTimeInSeconds */
    pub wait_get_poses_called_ms: f32,
    pub new_poses_ready_ms: f32,
    pub new_frame_ready_ms: f32, // second call to IVRCompositor::Submit
    pub compositor_update_start_ms: f32,
    pub compositor_update_end_ms: f32,
    pub compositor_render_start_ms: f32,

    pub hmd_pose: TrackedDevicePose, // pose used by app to render this frame

    pub num_vsyncs_ready_for_use: u32,
    pub num_vsyncs_to_first_view: u32,
}

#[allow(non_camel_case_types)]
pub struct Compositor_BenchmarkResults {
    pub mega_pixels_per_second: f32,
    pub hmd_recommended_mega_pixels_per_second: f32,
}

/** Frame timing data provided by direct mode drivers. */
#[allow(non_camel_case_types)]
pub struct DriverDirectMode_FrameTiming {
    pub size: u32,               // Set to sizeof( DriverDirectMode_FrameTiming )
    pub num_frame_presents: u32, // number of times frame was presented
    pub num_mis_presented: u32, // number of times frame was presented on a vsync other than it was originally predicted to
    pub num_dropped_frames: u32, // number of additional times previous frame was scanned out (i.e. compositor missed vsync)
    pub reprojection_flags: u32,
}

pub const VRCOMPOSITOR_REPROJECTION_MOTION_ENABLED: u32 = 0x100;
pub const VRCOMPOSITOR_REPROJECTION_MOTION_FORCED_ON: u32 = 0x200;
pub const VRCOMPOSITOR_REPROJECTION_MOTION_APP_THROTTLED: u32 = 0x400;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVSync
{
	VSync_None,
	VSync_WaitRender,	// block following render work until vsync
	VSync_NoWaitRender,	// do not block following render work (allow to get started early)
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
pub enum EVRMuraCorrectionMode
{
	EVRMuraCorrectionMode_Default = 0,
	EVRMuraCorrectionMode_NoCorrection
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(C)]
/** raw IMU data provided by IVRIOBuffer from paths to tracked devices with IMUs */
pub enum Imu_OffScaleFlags
{
	OffScale_AccelX	= 0x01,
	OffScale_AccelY	= 0x02,
	OffScale_AccelZ	= 0x04,
	OffScale_GyroX	= 0x08,
	OffScale_GyroY	= 0x10,
	OffScale_GyroZ	= 0x20,
}

pub struct ImuSample
{
	pub sample_time: u64,
	pub v_accel: HmdVector3,
	pub v_gyro: HmdVector3,
	pub un_off_scale_flags: u32,
}
