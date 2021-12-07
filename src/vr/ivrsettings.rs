#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum EVRSettingsError
{
    VRSettingsError_None = 0,
    VRSettingsError_IPCFailed = 1,
    VRSettingsError_WriteFailed = 2,
    VRSettingsError_ReadFailed = 3,
    VRSettingsError_JsonParseFailed = 4,
    VRSettingsError_UnsetSettingHasNoDefault = 5, // This will be returned if the setting does not appear in the appropriate default file and has not been set
}

pub const MAX_SETTINGS_KEY_LENGTH: usize = 128;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IVRSettings
{
    pub GetSettingsErrorNameFromEnum: unsafe extern "stdcall" fn(eError: EVRSettingsError) -> *const i8,

    pub SetBool: unsafe extern "stdcall" fn(pchSection: *const i8, pchSettingsKey: *const i8, bValue: bool, peError: *mut EVRSettingsError),
    pub SetInt32: unsafe extern "stdcall" fn(pchSection: *const i8, pchSettingsKey: *const i8, nValue: i32, peError: *mut EVRSettingsError),
    pub SetFloat: unsafe extern "stdcall" fn(pchSection: *const i8, pchSettingsKey: *const i8, flValue: f32, peError: *mut EVRSettingsError),
    pub SetString: unsafe extern "stdcall" fn(pchSection: *const i8, pchSettingsKey: *const i8, pchValue: *const i8, peError: *mut EVRSettingsError),

    pub GetBool: unsafe extern "stdcall" fn(pchSection: *const i8, pchSettingsKey: *const i8, pError: *mut EVRSettingsError) -> bool,
    pub GetInt32: unsafe extern "stdcall" fn(pchSection: *const i8, pchSettingsKey: *const i8, pError: *mut EVRSettingsError) -> i32,
    pub GetFloat: unsafe extern "stdcall" fn(pchSection: *const i8, pchSettingsKey: *const i8, pError: *mut EVRSettingsError) -> f32,
    pub GetString: unsafe extern "stdcall" fn(pchSection: *const i8, pchSettingsKey: *const i8, pchValue: *mut i8, unValueLen: usize, pError: *mut EVRSettingsError),

    pub RemoveSection: unsafe extern "stdcall" fn(pchSection: *const i8, peError: *mut EVRSettingsError),
    pub RemoveKeyInSection: unsafe extern "stdcall" fn(pchSection: *const i8, pchSettingsKey: *const i8, peError: *mut EVRSettingsError),
}

pub const IVRSETTINGS_VERSION: &'static str = "IVRSettings_003";
//-----------------------------------------------------------------------------
// steamvr keys
pub const SteamVR_Section: &'static str = "steamvr";
pub const SteamVR_RequireHmd_String: &'static str = "requireHmd";
pub const SteamVR_ForcedDriverKey_String: &'static str = "forcedDriver";
pub const SteamVR_ForcedHmdKey_String: &'static str = "forcedHmd";
pub const SteamVR_DisplayDebug_Bool: &'static str = "displayDebug";
pub const SteamVR_DebugProcessPipe_String: &'static str = "debugProcessPipe";
pub const SteamVR_DisplayDebugX_Int32: &'static str = "displayDebugX";
pub const SteamVR_DisplayDebugY_Int32: &'static str = "displayDebugY";
pub const SteamVR_SendSystemButtonToAllApps_Bool: &'static str= "sendSystemButtonToAllApps";
pub const SteamVR_LogLevel_Int32: &'static str = "loglevel";
pub const SteamVR_IPD_Float: &'static str = "ipd";
pub const SteamVR_Background_String: &'static str = "background";
pub const SteamVR_BackgroundUseDomeProjection_Bool: &'static str = "backgroundUseDomeProjection";
pub const SteamVR_BackgroundCameraHeight_Float: &'static str = "backgroundCameraHeight";
pub const SteamVR_BackgroundDomeRadius_Float: &'static str = "backgroundDomeRadius";
pub const SteamVR_GridColor_String: &'static str = "gridColor";
pub const SteamVR_PlayAreaColor_String: &'static str = "playAreaColor";
pub const SteamVR_TrackingLossColor_String: &'static str = "trackingLossColor";
pub const SteamVR_ShowStage_Bool: &'static str = "showStage";
pub const SteamVR_DrawTrackingReferences_Bool: &'static str = "drawTrackingReferences";
pub const SteamVR_ActivateMultipleDrivers_Bool: &'static str = "activateMultipleDrivers";
pub const SteamVR_UsingSpeakers_Bool: &'static str = "usingSpeakers";
pub const SteamVR_SpeakersForwardYawOffsetDegrees_Float: &'static str = "speakersForwardYawOffsetDegrees";
pub const SteamVR_BaseStationPowerManagement_Int32: &'static str = "basestationPowerManagement";
pub const SteamVR_ShowBaseStationPowerManagementTip_Int32: &'static str = "ShowBaseStationPowerManagementTip";
pub const SteamVR_NeverKillProcesses_Bool: &'static str = "neverKillProcesses";
pub const SteamVR_SupersampleScale_Float: &'static str = "supersampleScale";
pub const SteamVR_MaxRecommendedResolution_Int32: &'static str = "maxRecommendedResolution";
pub const SteamVR_MotionSmoothing_Bool: &'static str = "motionSmoothing";
pub const SteamVR_MotionSmoothingOverride_Int32: &'static str = "motionSmoothingOverride";
pub const SteamVR_FramesToThrottle_Int32: &'static str = "framesToThrottle";
pub const SteamVR_AdditionalFramesToPredict_Int32: &'static str = "additionalFramesToPredict";
pub const SteamVR_DisableAsyncReprojection_Bool: &'static str = "disableAsync";
pub const SteamVR_ForceFadeOnBadTracking_Bool: &'static str = "forceFadeOnBadTracking";
pub const SteamVR_DefaultMirrorView_Int32: &'static str = "mirrorView";
pub const SteamVR_ShowLegacyMirrorView_Bool: &'static str = "showLegacyMirrorView";
pub const SteamVR_MirrorViewVisibility_Bool: &'static str = "showMirrorView";
pub const SteamVR_MirrorViewDisplayMode_Int32: &'static str = "mirrorViewDisplayMode";
pub const SteamVR_MirrorViewEye_Int32: &'static str = "mirrorViewEye";
pub const SteamVR_MirrorViewGeometry_String: &'static str = "mirrorViewGeometry";
pub const SteamVR_MirrorViewGeometryMaximized_String: &'static str = "mirrorViewGeometryMaximized";
pub const SteamVR_PerfGraphVisibility_Bool: &'static str = "showPerfGraph";
pub const SteamVR_StartMonitorFromAppLaunch: &'static str = "startMonitorFromAppLaunch";
pub const SteamVR_StartCompositorFromAppLaunch_Bool: &'static str = "startCompositorFromAppLaunch";
pub const SteamVR_StartDashboardFromAppLaunch_Bool: &'static str = "startDashboardFromAppLaunch";
pub const SteamVR_StartOverlayAppsFromDashboard_Bool: &'static str = "startOverlayAppsFromDashboard";
pub const SteamVR_EnableHomeApp: &'static str = "enableHomeApp";
pub const SteamVR_CycleBackgroundImageTimeSec_Int32: &'static str = "CycleBackgroundImageTimeSec";
pub const SteamVR_RetailDemo_Bool: &'static str = "retailDemo";
pub const SteamVR_IpdOffset_Float: &'static str = "ipdOffset";
pub const SteamVR_AllowSupersampleFiltering_Bool: &'static str = "allowSupersampleFiltering";
pub const SteamVR_SupersampleManualOverride_Bool: &'static str = "supersampleManualOverride";
pub const SteamVR_EnableLinuxVulkanAsync_Bool: &'static str = "enableLinuxVulkanAsync";
pub const SteamVR_AllowDisplayLockedMode_Bool: &'static str = "allowDisplayLockedMode";
pub const SteamVR_HaveStartedTutorialForNativeChaperoneDriver_Bool: &'static str = "haveStartedTutorialForNativeChaperoneDriver";
pub const SteamVR_ForceWindows32bitVRMonitor: &'static str = "forceWindows32BitVRMonitor";
pub const SteamVR_DebugInputBinding: &'static str = "debugInputBinding";
pub const SteamVR_DoNotFadeToGrid: &'static str = "doNotFadeToGrid";
pub const SteamVR_RenderCameraMode: &'static str = "renderCameraMode";
pub const SteamVR_EnableSharedResourceJournaling: &'static str = "enableSharedResourceJournaling";
pub const SteamVR_EnableSafeMode: &'static str = "enableSafeMode";
pub const SteamVR_PreferredRefreshRate: &'static str = "preferredRefreshRate";
pub const SteamVR_LastVersionNotice: &'static str = "lastVersionNotice";
pub const SteamVR_LastVersionNoticeDate: &'static str = "lastVersionNoticeDate";
pub const SteamVR_HmdDisplayColorGainR_Float: &'static str = "hmdDisplayColorGainR";
pub const SteamVR_HmdDisplayColorGainG_Float: &'static str = "hmdDisplayColorGainG";
pub const SteamVR_HmdDisplayColorGainB_Float: &'static str = "hmdDisplayColorGainB";
pub const SteamVR_CustomIconStyle_String: &'static str = "customIconStyle";
pub const SteamVR_CustomOffIconStyle_String: &'static str = "customOffIconStyle";
pub const SteamVR_CustomIconForceUpdate_String: &'static str = "customIconForceUpdate";
pub const SteamVR_AllowGlobalActionSetPriority: &'static str = "globalActionSetPriority";
pub const SteamVR_OverlayRenderQuality: &'static str = "overlayRenderQuality_2";
pub const SteamVR_BlockOculusSDKOnOpenVRLaunchOption_Bool: &'static str = "blockOculusSDKOnOpenVRLaunchOption";
pub const SteamVR_BlockOculusSDKOnAllLaunches_Bool: &'static str = "blockOculusSDKOnAllLaunches";
pub const SteamVR_HDCPLegacyCompatibility_Bool: &'static str = "hdcp14legacyCompatibility";
pub const SteamVR_UsePrism_Bool: &'static str = "usePrism";

//-----------------------------------------------------------------------------
// direct mode keys
pub const DirectMode_Section: &'static str = "direct_mode";
pub const DirectMode_Enable_Bool: &'static str = "enable";
pub const DirectMode_Count_Int32: &'static str = "count";
pub const DirectMode_EdidVid_Int32: &'static str = "edidVid";
pub const DirectMode_EdidPid_Int32: &'static str = "edidPid";

//-----------------------------------------------------------------------------
// lighthouse keys
pub const Lighthouse_Section: &'static str = "driver_lighthouse";
pub const Lighthouse_DisableIMU_Bool: &'static str = "disableimu";
pub const Lighthouse_DisableIMUExceptHMD_Bool: &'static str = "disableimuexcepthmd";
pub const Lighthouse_UseDisambiguation_String: &'static str = "usedisambiguation";
pub const Lighthouse_DisambiguationDebug_Int32: &'static str = "disambiguationdebug";
pub const Lighthouse_PrimaryBasestation_Int32: &'static str = "primarybasestation";
pub const Lighthouse_DBHistory_Bool: &'static str = "dbhistory";
pub const Lighthouse_EnableBluetooth_Bool: &'static str = "enableBluetooth";
pub const Lighthouse_PowerManagedBaseStations_String: &'static str = "PowerManagedBaseStations";
pub const Lighthouse_PowerManagedBaseStations2_String: &'static str = "PowerManagedBaseStations2";
pub const Lighthouse_InactivityTimeoutForBaseStations_Int32: &'static str = "InactivityTimeoutForBaseStations";
pub const Lighthouse_EnableImuFallback_Bool: &'static str = "enableImuFallback";

//-----------------------------------------------------------------------------
// null keys
pub const Null_Section: &'static str = "driver_null";
pub const Null_SerialNumber_String: &'static str = "serialNumber";
pub const Null_ModelNumber_String: &'static str = "modelNumber";
pub const Null_WindowX_Int32: &'static str = "windowX";
pub const Null_WindowY_Int32: &'static str = "windowY";
pub const Null_WindowWidth_Int32: &'static str = "windowWidth";
pub const Null_WindowHeight_Int32: &'static str = "windowHeight";
pub const Null_RenderWidth_Int32: &'static str = "renderWidth";
pub const Null_RenderHeight_Int32: &'static str = "renderHeight";
pub const Null_SecondsFromVsyncToPhotons_Float: &'static str = "secondsFromVsyncToPhotons";
pub const Null_DisplayFrequency_Float: &'static str = "displayFrequency";

//-----------------------------------------------------------------------------
// Windows MR keys
pub const WindowsMR_Section: &'static str = "driver_holographic";

//-----------------------------------------------------------------------------
// user interface keys
pub const UserInterface_Section: &'static str = "userinterface";
pub const UserInterface_StatusAlwaysOnTop_Bool: &'static str = "StatusAlwaysOnTop";
pub const UserInterface_MinimizeToTray_Bool: &'static str = "MinimizeToTray";
pub const UserInterface_HidePopupsWhenStatusMinimized_Bool: &'static str = "HidePopupsWhenStatusMinimized";
pub const UserInterface_Screenshots_Bool: &'static str = "screenshots";
pub const UserInterface_ScreenshotType_Int: &'static str = "screenshotType";

//-----------------------------------------------------------------------------
// notification keys
pub const Notifications_Section: &'static str = "notifications";
pub const Notifications_DoNotDisturb_Bool: &'static str = "DoNotDisturb";

//-----------------------------------------------------------------------------
// keyboard keys
pub const Keyboard_Section: &'static str = "keyboard";
pub const Keyboard_TutorialCompletions: &'static str = "TutorialCompletions";
pub const Keyboard_ScaleX: &'static str = "ScaleX";
pub const Keyboard_ScaleY: &'static str = "ScaleY";
pub const Keyboard_OffsetLeftX: &'static str = "OffsetLeftX";
pub const Keyboard_OffsetRightX: &'static str = "OffsetRightX";
pub const Keyboard_OffsetY: &'static str = "OffsetY";
pub const Keyboard_Smoothing: &'static str = "Smoothing";

//-----------------------------------------------------------------------------
// perf keys
pub const Perf_Section: &'static str = "perfcheck";
pub const Perf_PerfGraphInHMD_Bool: &'static str = "perfGraphInHMD";
pub const Perf_AllowTimingStore_Bool: &'static str = "allowTimingStore";
pub const Perf_SaveTimingsOnExit_Bool: &'static str = "saveTimingsOnExit";
pub const Perf_TestData_Float: &'static str = "perfTestData";
pub const Perf_GPUProfiling_Bool: &'static str = "GPUProfiling";

//-----------------------------------------------------------------------------
// collision bounds keys
pub const CollisionBounds_Section: &'static str = "collisionBounds";
pub const CollisionBounds_Style_Int32: &'static str = "CollisionBoundsStyle";
pub const CollisionBounds_GroundPerimeterOn_Bool: &'static str = "CollisionBoundsGroundPerimeterOn";
pub const CollisionBounds_CenterMarkerOn_Bool: &'static str = "CollisionBoundsCenterMarkerOn";
pub const CollisionBounds_PlaySpaceOn_Bool: &'static str = "CollisionBoundsPlaySpaceOn";
pub const CollisionBounds_FadeDistance_Float: &'static str = "CollisionBoundsFadeDistance";
pub const CollisionBounds_WallHeight_Float: &'static str = "CollisionBoundsWallHeight";
pub const CollisionBounds_ColorGammaR_Int32: &'static str = "CollisionBoundsColorGammaR";
pub const CollisionBounds_ColorGammaG_Int32: &'static str = "CollisionBoundsColorGammaG";
pub const CollisionBounds_ColorGammaB_Int32: &'static str = "CollisionBoundsColorGammaB";
pub const CollisionBounds_ColorGammaA_Int32: &'static str = "CollisionBoundsColorGammaA";
pub const CollisionBounds_EnableDriverImport: &'static str = "enableDriverBoundsImport";

//-----------------------------------------------------------------------------
// camera keys
pub const Camera_Section: &'static str = "camera";
pub const Camera_EnableCamera_Bool: &'static str = "enableCamera";
pub const Camera_ShowOnController_Bool: &'static str = "showOnController";
pub const Camera_EnableCameraForCollisionBounds_Bool: &'static str = "enableCameraForCollisionBounds";
pub const Camera_RoomView_Int32: &'static str = "roomView";
pub const Camera_BoundsColorGammaR_Int32: &'static str = "cameraBoundsColorGammaR";
pub const Camera_BoundsColorGammaG_Int32: &'static str = "cameraBoundsColorGammaG";
pub const Camera_BoundsColorGammaB_Int32: &'static str = "cameraBoundsColorGammaB";
pub const Camera_BoundsColorGammaA_Int32: &'static str = "cameraBoundsColorGammaA";
pub const Camera_BoundsStrength_Int32: &'static str = "cameraBoundsStrength";
pub const Camera_RoomViewStyle_Int32: &'static str = "roomViewStyle";

//-----------------------------------------------------------------------------
// audio keys
pub const audio_Section: &'static str = "audio";
pub const audio_SetOsDefaultPlaybackDevice_Bool: &'static str = "setOsDefaultPlaybackDevice";
pub const audio_EnablePlaybackDeviceOverride_Bool: &'static str = "enablePlaybackDeviceOverride";
pub const audio_PlaybackDeviceOverride_String: &'static str = "playbackDeviceOverride";
pub const audio_PlaybackDeviceOverrideName_String: &'static str = "playbackDeviceOverrideName";
pub const audio_SetOsDefaultRecordingDevice_Bool: &'static str = "setOsDefaultRecordingDevice";
pub const audio_EnableRecordingDeviceOverride_Bool: &'static str = "enableRecordingDeviceOverride";
pub const audio_RecordingDeviceOverride_String: &'static str = "recordingDeviceOverride";
pub const audio_RecordingDeviceOverrideName_String: &'static str = "recordingDeviceOverrideName";
pub const audio_EnablePlaybackMirror_Bool: &'static str = "enablePlaybackMirror";
pub const audio_PlaybackMirrorDevice_String: &'static str = "playbackMirrorDevice";
pub const audio_PlaybackMirrorDeviceName_String: &'static str = "playbackMirrorDeviceName";
pub const audio_OldPlaybackMirrorDevice_String: &'static str = "onPlaybackMirrorDevice";
pub const audio_ActiveMirrorDevice_String: &'static str = "activePlaybackMirrorDevice";
pub const audio_EnablePlaybackMirrorIndependentVolume_Bool: &'static str = "enablePlaybackMirrorIndependentVolume";
pub const audio_LastHmdPlaybackDeviceId_String: &'static str = "lastHmdPlaybackDeviceId";
pub const audio_VIVEHDMIGain: &'static str = "viveHDMIGain";
pub const audio_DualSpeakerAndJackOutput_Bool: &'static str = "dualSpeakerAndJackOutput";
pub const audio_MuteMicMonitor_Bool: &'static str = "muteMicMonitor";

//-----------------------------------------------------------------------------
// power management keys
pub const Power_Section: &'static str = "power";
pub const Power_PowerOffOnExit_Bool: &'static str = "powerOffOnExit";
pub const Power_TurnOffScreensTimeout_Float: &'static str = "turnOffScreensTimeout";
pub const Power_TurnOffControllersTimeout_Float: &'static str = "turnOffControllersTimeout";
pub const Power_ReturnToWatchdogTimeout_Float: &'static str = "returnToWatchdogTimeout";
pub const Power_AutoLaunchSteamVROnButtonPress: &'static str = "autoLaunchSteamVROnButtonPress";
pub const Power_PauseCompositorOnStandby_Bool: &'static str = "pauseCompositorOnStandby";

//-----------------------------------------------------------------------------
// dashboard keys
pub const Dashboard_Section: &'static str = "dashboard";
pub const Dashboard_EnableDashboard_Bool: &'static str = "enableDashboard";
pub const Dashboard_ArcadeMode_Bool: &'static str = "arcadeMode";
pub const Dashboard_Position: &'static str = "position";
pub const Dashboard_DesktopScale: &'static str = "desktopScale";
pub const Dashboard_DashboardScale: &'static str = "dashboardScale";
pub const Dashboard_UseStandaloneSystemLayer: &'static str = "standaloneSystemLayer";

//-----------------------------------------------------------------------------
// model skin keys
pub const modelskin_Section: &'static str = "modelskins";

//-----------------------------------------------------------------------------
// driver keys - These could be checked in any driver_<name> section
pub const Driver_Enable_Bool: &'static str = "enable";
pub const Driver_BlockedBySafemode_Bool: &'static str = "blocked_by_safe_mode";
pub const Driver_LoadPriority_Int32: &'static str = "loadPriority";

//-----------------------------------------------------------------------------
// web interface keys
pub const WebInterface_Section: &'static str = "WebInterface";

//-----------------------------------------------------------------------------
// vrwebhelper keys
pub const VRWebHelper_Section: &'static str = "VRWebHelper";
pub const VRWebHelper_DebuggerEnabled_Bool: &'static str = "DebuggerEnabled";
pub const VRWebHelper_DebuggerPort_Int32: &'static str = "DebuggerPort";

//-----------------------------------------------------------------------------
// tracking overrides - keys are device paths, values are the device paths their
//  tracking/pose information overrides
pub const TrackingOverride_Section: &'static str = "TrackingOverrides";

//-----------------------------------------------------------------------------
// per-app keys - the section name for these is the app key itself. Some of these are prefixed by the controller type
pub const App_BindingAutosaveURLSuffix_String: &'static str = "AutosaveURL";
pub const App_BindingLegacyAPISuffix_String: &'static str = "_legacy";
pub const App_BindingSteamVRInputAPISuffix_String: &'static str = "_steamvrinput";
pub const App_BindingCurrentURLSuffix_String: &'static str = "CurrentURL";
pub const App_BindingPreviousURLSuffix_String: &'static str = "PreviousURL";
pub const App_NeedToUpdateAutosaveSuffix_Bool: &'static str = "NeedToUpdateAutosave";
pub const App_DominantHand_Int32: &'static str = "DominantHand";
pub const App_BlockOculusSDK_Bool: &'static str = "blockOculusSDK";

//-----------------------------------------------------------------------------
// configuration for trackers
pub const Trackers_Section: &'static str = "trackers";

//-----------------------------------------------------------------------------
// configuration for desktop UI windows
pub const DesktopUI_Section: &'static str = "DesktopUI";

//-----------------------------------------------------------------------------
// Last known keys for righting recovery
pub const LastKnown_Section: &'static str = "LastKnown";
pub const LastKnown_HMDManufacturer_String: &'static str = "HMDManufacturer";
pub const LastKnown_HMDModel_String: &'static str = "HMDModel";

//-----------------------------------------------------------------------------
// Dismissed warnings
pub const DismissedWarnings_Section: &'static str = "DismissedWarnings";

//-----------------------------------------------------------------------------
// Input Settings
pub const Input_Section: &'static str = "input";
pub const Input_LeftThumbstickRotation_Float: &'static str = "leftThumbstickRotation";
pub const Input_RightThumbstickRotation_Float: &'static str = "rightThumbstickRotation";
pub const Input_ThumbstickDeadzone_Float: &'static str = "thumbstickDeadzone";

//-----------------------------------------------------------------------------
// Log of GPU performance
pub const GpuSpeed_Section: &'static str = "GpuSpeed";