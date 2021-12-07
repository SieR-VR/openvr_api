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
pub struct VR_IVRSettings_FnTable
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
pub const STEAMVR_SECTION: &'static str = "steamvr";
pub const STEAMVR_REQUIRE_HMD_STRING: &'static str = "requireHmd";
pub const STEAMVR_FORCED_DRIVER_KEY_STRING: &'static str = "forcedDriver";
pub const STEAMVR_FORCED_HMD_KEY_STRING: &'static str = "forcedHmd";
pub const STEAMVR_DISPLAY_DEBUG_BOOL: &'static str = "displayDebug";
pub const STEAMVR_DEBUG_PROCESS_PIPE_STRING: &'static str = "debugProcessPipe";
pub const STEAMVR_DISPLAY_DEBUG_X_INT32: &'static str = "displayDebugX";
pub const STEAMVR_DISPLAY_DEBUG_Y_INT32: &'static str = "displayDebugY";
pub const STEAMVR_SEND_SYSTEM_BUTTON_TO_ALL_APPS_BOOL: &'static str= "sendSystemButtonToAllApps";
pub const STEAMVR_LOG_LEVEL_INT32: &'static str = "loglevel";
pub const STEAMVR_IPD_FLOAT: &'static str = "ipd";
pub const STEAMVR_BACKGROUND_STRING: &'static str = "background";
pub const STEAMVR_BACKGROUND_USE_DOME_PROJECTION_BOOL: &'static str = "backgroundUseDomeProjection";
pub const STEAMVR_BACKGROUND_CAMERA_HEIGHT_FLOAT: &'static str = "backgroundCameraHeight";
pub const STEAMVR_BACKGROUND_DOME_RADIUS_FLOAT: &'static str = "backgroundDomeRadius";
pub const STEAMVR_GRID_COLOR_STRING: &'static str = "gridColor";
pub const STEAMVR_PLAY_AREA_COLOR_STRING: &'static str = "playAreaColor";
pub const STEAMVR_TRACKING_LOSS_COLOR_STRING: &'static str = "trackingLossColor";
pub const STEAMVR_SHOW_STAGE_BOOL: &'static str = "showStage";
pub const STEAMVR_DRAW_TRACKING_REFERENCES_BOOL: &'static str = "drawTrackingReferences";
pub const STEAMVR_ACTIVATE_MULTIPLE_DRIVERS_BOOL: &'static str = "activateMultipleDrivers";
pub const STEAMVR_USING_SPEAKERS_BOOL: &'static str = "usingSpeakers";
pub const STEAMVR_SPEAKERS_FORWARD_YAW_OFFSET_DEGREES_FLOAT: &'static str = "speakersForwardYawOffsetDegrees";
pub const STEAMVR_BASE_STATION_POWER_MANAGEMENT_INT32: &'static str = "basestationPowerManagement";
pub const STEAMVR_SHOW_BASE_STATION_POWER_MANAGEMENT_TIP_INT32: &'static str = "ShowBaseStationPowerManagementTip";
pub const STEAMVR_NEVER_KILL_PROCESSES_BOOL: &'static str = "neverKillProcesses";
pub const STEAMVR_SUPERSAMPLE_SCALE_FLOAT: &'static str = "supersampleScale";
pub const STEAMVR_MAX_RECOMMENDED_RESOLUTION_INT32: &'static str = "maxRecommendedResolution";
pub const STEAMVR_MOTION_SMOOTHING_BOOL: &'static str = "motionSmoothing";
pub const STEAMVR_MOTION_SMOOTHING_OVERRIDE_INT32: &'static str = "motionSmoothingOverride";
pub const STEAMVR_FRAMES_TO_THROTTLE_INT32: &'static str = "framesToThrottle";
pub const STEAMVR_ADDITIONAL_FRAMES_TO_PREDICT_INT32: &'static str = "additionalFramesToPredict";
pub const STEAMVR_DISABLE_ASYNC_REPROJECTION_BOOL: &'static str = "disableAsync";
pub const STEAMVR_FORCE_FADE_ON_BAD_TRACKING_BOOL: &'static str = "forceFadeOnBadTracking";
pub const STEAMVR_DEFAULT_MIRROR_VIEW_INT32: &'static str = "mirrorView";
pub const STEAMVR_SHOW_LEGACY_MIRROR_VIEW_BOOL: &'static str = "showLegacyMirrorView";
pub const STEAMVR_MIRROR_VIEW_VISIBILITY_BOOL: &'static str = "showMirrorView";
pub const STEAMVR_MIRROR_VIEW_DISPLAY_MODE_INT32: &'static str = "mirrorViewDisplayMode";
pub const STEAMVR_MIRROR_VIEW_EYE_INT32: &'static str = "mirrorViewEye";
pub const STEAMVR_MIRROR_VIEW_GEOMETRY_STRING: &'static str = "mirrorViewGeometry";
pub const STEAMVR_MIRROR_VIEW_GEOMETRY_MAXIMIZED_STRING: &'static str = "mirrorViewGeometryMaximized";
pub const STEAMVR_PERF_GRAPH_VISIBILITY_BOOL: &'static str = "showPerfGraph";
pub const STEAMVR_START_MONITOR_FROM_APP_LAUNCH: &'static str = "startMonitorFromAppLaunch";
pub const STEAMVR_START_COMPOSITOR_FROM_APP_LAUNCH_BOOL: &'static str = "startCompositorFromAppLaunch";
pub const STEAMVR_START_DASHBOARD_FROM_APP_LAUNCH_BOOL: &'static str = "startDashboardFromAppLaunch";
pub const STEAMVR_START_OVERLAY_APPS_FROM_DASHBOARD_BOOL: &'static str = "startOverlayAppsFromDashboard";
pub const STEAMVR_ENABLE_HOME_APP: &'static str = "enableHomeApp";
pub const STEAMVR_CYCLE_BACKGROUND_IMAGE_TIME_SEC_INT32: &'static str = "CycleBackgroundImageTimeSec";
pub const STEAMVR_RETAIL_DEMO_BOOL: &'static str = "retailDemo";
pub const STEAMVR_IPD_OFFSET_FLOAT: &'static str = "ipdOffset";
pub const STEAMVR_ALLOW_SUPERSAMPLE_FILTERING_BOOL: &'static str = "allowSupersampleFiltering";
pub const STEAMVR_SUPERSAMPLE_MANUAL_OVERRIDE_BOOL: &'static str = "supersampleManualOverride";
pub const STEAMVR_ENABLE_LINUX_VULKAN_ASYNC_BOOL: &'static str = "enableLinuxVulkanAsync";
pub const STEAMVR_ALLOW_DISPLAY_LOCKED_MODE_BOOL: &'static str = "allowDisplayLockedMode";
pub const STEAMVR_HAVE_STARTED_TUTORIAL_FOR_NATIVE_CHAPERONE_DRIVER_BOOL: &'static str = "haveStartedTutorialForNativeChaperoneDriver";
pub const STEAMVR_FORCE_WINDOWS32BIT_VRMONITOR: &'static str = "forceWindows32BitVRMonitor";
pub const STEAMVR_DEBUG_INPUT_BINDING: &'static str = "debugInputBinding";
pub const STEAMVR_DO_NOT_FADE_TO_GRID: &'static str = "doNotFadeToGrid";
pub const STEAMVR_RENDER_CAMERA_MODE: &'static str = "renderCameraMode";
pub const STEAMVR_ENABLE_SHARED_RESOURCE_JOURNALING: &'static str = "enableSharedResourceJournaling";
pub const STEAMVR_ENABLE_SAFE_MODE: &'static str = "enableSafeMode";
pub const STEAMVR_PREFERRED_REFRESH_RATE: &'static str = "preferredRefreshRate";
pub const STEAMVR_LAST_VERSION_NOTICE: &'static str = "lastVersionNotice";
pub const STEAMVR_LAST_VERSION_NOTICE_DATE: &'static str = "lastVersionNoticeDate";
pub const STEAMVR_HMD_DISPLAY_COLOR_GAIN_R_FLOAT: &'static str = "hmdDisplayColorGainR";
pub const STEAMVR_HMD_DISPLAY_COLOR_GAIN_G_FLOAT: &'static str = "hmdDisplayColorGainG";
pub const STEAMVR_HMD_DISPLAY_COLOR_GAIN_B_FLOAT: &'static str = "hmdDisplayColorGainB";
pub const STEAMVR_CUSTOM_ICON_STYLE_STRING: &'static str = "customIconStyle";
pub const STEAMVR_CUSTOM_OFF_ICON_STYLE_STRING: &'static str = "customOffIconStyle";
pub const STEAMVR_CUSTOM_ICON_FORCE_UPDATE_STRING: &'static str = "customIconForceUpdate";
pub const STEAMVR_ALLOW_GLOBAL_ACTION_SET_PRIORITY: &'static str = "globalActionSetPriority";
pub const STEAMVR_OVERLAY_RENDER_QUALITY: &'static str = "overlayRenderQuality_2";
pub const STEAMVR_BLOCK_OCULUS_SDKON_OPEN_VRLAUNCH_OPTION_BOOL: &'static str = "blockOculusSDKOnOpenVRLaunchOption";
pub const STEAMVR_BLOCK_OCULUS_SDKON_ALL_LAUNCHES_BOOL: &'static str = "blockOculusSDKOnAllLaunches";
pub const STEAMVR_HDCPLEGACY_COMPATIBILITY_BOOL: &'static str = "hdcp14legacyCompatibility";
pub const STEAMVR_USE_PRISM_BOOL: &'static str = "usePrism";

//-----------------------------------------------------------------------------
// direct mode keys
pub const DIRECT_MODE_SECTION: &'static str = "direct_mode";
pub const DIRECT_MODE_ENABLE_BOOL: &'static str = "enable";
pub const DIRECT_MODE_COUNT_INT32: &'static str = "count";
pub const DIRECT_MODE_EDID_VID_INT32: &'static str = "edidVid";
pub const DIRECT_MODE_EDID_PID_INT32: &'static str = "edidPid";

//-----------------------------------------------------------------------------
// lighthouse keys
pub const LIGHTHOUSE_SECTION: &'static str = "driver_lighthouse";
pub const LIGHTHOUSE_DISABLE_IMU_BOOL: &'static str = "disableimu";
pub const LIGHTHOUSE_DISABLE_IMUEXCEPT_HMD_BOOL: &'static str = "disableimuexcepthmd";
pub const LIGHTHOUSE_USE_DISAMBIGUATION_STRING: &'static str = "usedisambiguation";
pub const LIGHTHOUSE_DISAMBIGUATION_DEBUG_INT32: &'static str = "disambiguationdebug";
pub const LIGHTHOUSE_PRIMARY_BASESTATION_INT32: &'static str = "primarybasestation";
pub const LIGHTHOUSE_DBHISTORY_BOOL: &'static str = "dbhistory";
pub const LIGHTHOUSE_ENABLE_BLUETOOTH_BOOL: &'static str = "enableBluetooth";
pub const LIGHTHOUSE_POWER_MANAGED_BASE_STATIONS_STRING: &'static str = "PowerManagedBaseStations";
pub const LIGHTHOUSE_POWER_MANAGED_BASE_STATIONS2_STRING: &'static str = "PowerManagedBaseStations2";
pub const LIGHTHOUSE_INACTIVITY_TIMEOUT_FOR_BASE_STATIONS_INT32: &'static str = "InactivityTimeoutForBaseStations";
pub const LIGHTHOUSE_ENABLE_IMU_FALLBACK_BOOL: &'static str = "enableImuFallback";

//-----------------------------------------------------------------------------
// null keys
pub const NULL_SECTION: &'static str = "driver_null";
pub const NULL_SERIAL_NUMBER_STRING: &'static str = "serialNumber";
pub const NULL_MODEL_NUMBER_STRING: &'static str = "modelNumber";
pub const NULL_WINDOW_X_INT32: &'static str = "windowX";
pub const NULL_WINDOW_Y_INT32: &'static str = "windowY";
pub const NULL_WINDOW_WIDTH_INT32: &'static str = "windowWidth";
pub const NULL_WINDOW_HEIGHT_INT32: &'static str = "windowHeight";
pub const NULL_RENDER_WIDTH_INT32: &'static str = "renderWidth";
pub const NULL_RENDER_HEIGHT_INT32: &'static str = "renderHeight";
pub const NULL_SECONDS_FROM_VSYNC_TO_PHOTONS_FLOAT: &'static str = "secondsFromVsyncToPhotons";
pub const NULL_DISPLAY_FREQUENCY_FLOAT: &'static str = "displayFrequency";

//-----------------------------------------------------------------------------
// Windows MR keys
pub const WINDOWS_MR_SECTION: &'static str = "driver_holographic";

//-----------------------------------------------------------------------------
// user interface keys
pub const USER_INTERFACE_SECTION: &'static str = "userinterface";
pub const USER_INTERFACE_STATUS_ALWAYS_ON_TOP_BOOL: &'static str = "StatusAlwaysOnTop";
pub const USER_INTERFACE_MINIMIZE_TO_TRAY_BOOL: &'static str = "MinimizeToTray";
pub const USER_INTERFACE_HIDE_POPUPS_WHEN_STATUS_MINIMIZED_BOOL: &'static str = "HidePopupsWhenStatusMinimized";
pub const USER_INTERFACE_SCREENSHOTS_BOOL: &'static str = "screenshots";
pub const USER_INTERFACE_SCREENSHOT_TYPE_INT: &'static str = "screenshotType";

//-----------------------------------------------------------------------------
// notification keys
pub const NOTIFICATIONS_SECTION: &'static str = "notifications";
pub const NOTIFICATIONS_DO_NOT_DISTURB_BOOL: &'static str = "DoNotDisturb";

//-----------------------------------------------------------------------------
// keyboard keys
pub const KEYBOARD_SECTION: &'static str = "keyboard";
pub const KEYBOARD_TUTORIAL_COMPLETIONS: &'static str = "TutorialCompletions";
pub const KEYBOARD_SCALE_X: &'static str = "ScaleX";
pub const KEYBOARD_SCALE_Y: &'static str = "ScaleY";
pub const KEYBOARD_OFFSET_LEFT_X: &'static str = "OffsetLeftX";
pub const KEYBOARD_OFFSET_RIGHT_X: &'static str = "OffsetRightX";
pub const KEYBOARD_OFFSET_Y: &'static str = "OffsetY";
pub const KEYBOARD_SMOOTHING: &'static str = "Smoothing";

//-----------------------------------------------------------------------------
// perf keys
pub const PERF_SECTION: &'static str = "perfcheck";
pub const PERF_PERF_GRAPH_IN_HMD_BOOL: &'static str = "perfGraphInHMD";
pub const PERF_ALLOW_TIMING_STORE_BOOL: &'static str = "allowTimingStore";
pub const PERF_SAVE_TIMINGS_ON_EXIT_BOOL: &'static str = "saveTimingsOnExit";
pub const PERF_TEST_DATA_FLOAT: &'static str = "perfTestData";
pub const PERF_GPUPROFILING_BOOL: &'static str = "GPUProfiling";

//-----------------------------------------------------------------------------
// collision bounds keys
pub const COLLISION_BOUNDS_SECTION: &'static str = "collisionBounds";
pub const COLLISION_BOUNDS_STYLE_INT32: &'static str = "CollisionBoundsStyle";
pub const COLLISION_BOUNDS_GROUND_PERIMETER_ON_BOOL: &'static str = "CollisionBoundsGroundPerimeterOn";
pub const COLLISION_BOUNDS_CENTER_MARKER_ON_BOOL: &'static str = "CollisionBoundsCenterMarkerOn";
pub const COLLISION_BOUNDS_PLAY_SPACE_ON_BOOL: &'static str = "CollisionBoundsPlaySpaceOn";
pub const COLLISION_BOUNDS_FADE_DISTANCE_FLOAT: &'static str = "CollisionBoundsFadeDistance";
pub const COLLISION_BOUNDS_WALL_HEIGHT_FLOAT: &'static str = "CollisionBoundsWallHeight";
pub const COLLISION_BOUNDS_COLOR_GAMMA_R_INT32: &'static str = "CollisionBoundsColorGammaR";
pub const COLLISION_BOUNDS_COLOR_GAMMA_G_INT32: &'static str = "CollisionBoundsColorGammaG";
pub const COLLISION_BOUNDS_COLOR_GAMMA_B_INT32: &'static str = "CollisionBoundsColorGammaB";
pub const COLLISION_BOUNDS_COLOR_GAMMA_A_INT32: &'static str = "CollisionBoundsColorGammaA";
pub const COLLISION_BOUNDS_ENABLE_DRIVER_IMPORT: &'static str = "enableDriverBoundsImport";

//-----------------------------------------------------------------------------
// camera keys
pub const CAMERA_SECTION: &'static str = "camera";
pub const CAMERA_ENABLE_CAMERA_BOOL: &'static str = "enableCamera";
pub const CAMERA_SHOW_ON_CONTROLLER_BOOL: &'static str = "showOnController";
pub const CAMERA_ENABLE_CAMERA_FOR_COLLISION_BOUNDS_BOOL: &'static str = "enableCameraForCollisionBounds";
pub const CAMERA_ROOM_VIEW_INT32: &'static str = "roomView";
pub const CAMERA_BOUNDS_COLOR_GAMMA_R_INT32: &'static str = "cameraBoundsColorGammaR";
pub const CAMERA_BOUNDS_COLOR_GAMMA_G_INT32: &'static str = "cameraBoundsColorGammaG";
pub const CAMERA_BOUNDS_COLOR_GAMMA_B_INT32: &'static str = "cameraBoundsColorGammaB";
pub const CAMERA_BOUNDS_COLOR_GAMMA_A_INT32: &'static str = "cameraBoundsColorGammaA";
pub const CAMERA_BOUNDS_STRENGTH_INT32: &'static str = "cameraBoundsStrength";
pub const CAMERA_ROOM_VIEW_STYLE_INT32: &'static str = "roomViewStyle";

//-----------------------------------------------------------------------------
// audio keys
pub const AUDIO_SECTION: &'static str = "audio";
pub const AUDIO_SET_OS_DEFAULT_PLAYBACK_DEVICE_BOOL: &'static str = "setOsDefaultPlaybackDevice";
pub const AUDIO_ENABLE_PLAYBACK_DEVICE_OVERRIDE_BOOL: &'static str = "enablePlaybackDeviceOverride";
pub const AUDIO_PLAYBACK_DEVICE_OVERRIDE_STRING: &'static str = "playbackDeviceOverride";
pub const AUDIO_PLAYBACK_DEVICE_OVERRIDE_NAME_STRING: &'static str = "playbackDeviceOverrideName";
pub const AUDIO_SET_OS_DEFAULT_RECORDING_DEVICE_BOOL: &'static str = "setOsDefaultRecordingDevice";
pub const AUDIO_ENABLE_RECORDING_DEVICE_OVERRIDE_BOOL: &'static str = "enableRecordingDeviceOverride";
pub const AUDIO_RECORDING_DEVICE_OVERRIDE_STRING: &'static str = "recordingDeviceOverride";
pub const AUDIO_RECORDING_DEVICE_OVERRIDE_NAME_STRING: &'static str = "recordingDeviceOverrideName";
pub const AUDIO_ENABLE_PLAYBACK_MIRROR_BOOL: &'static str = "enablePlaybackMirror";
pub const AUDIO_PLAYBACK_MIRROR_DEVICE_STRING: &'static str = "playbackMirrorDevice";
pub const AUDIO_PLAYBACK_MIRROR_DEVICE_NAME_STRING: &'static str = "playbackMirrorDeviceName";
pub const AUDIO_OLD_PLAYBACK_MIRROR_DEVICE_STRING: &'static str = "onPlaybackMirrorDevice";
pub const AUDIO_ACTIVE_MIRROR_DEVICE_STRING: &'static str = "activePlaybackMirrorDevice";
pub const AUDIO_ENABLE_PLAYBACK_MIRROR_INDEPENDENT_VOLUME_BOOL: &'static str = "enablePlaybackMirrorIndependentVolume";
pub const AUDIO_LAST_HMD_PLAYBACK_DEVICE_ID_STRING: &'static str = "lastHmdPlaybackDeviceId";
pub const AUDIO_VIVEHDMIGAIN: &'static str = "viveHDMIGain";
pub const AUDIO_DUAL_SPEAKER_AND_JACK_OUTPUT_BOOL: &'static str = "dualSpeakerAndJackOutput";
pub const AUDIO_MUTE_MIC_MONITOR_BOOL: &'static str = "muteMicMonitor";

//-----------------------------------------------------------------------------
// power management keys
pub const POWER_SECTION: &'static str = "power";
pub const POWER_POWER_OFF_ON_EXIT_BOOL: &'static str = "powerOffOnExit";
pub const POWER_TURN_OFF_SCREENS_TIMEOUT_FLOAT: &'static str = "turnOffScreensTimeout";
pub const POWER_TURN_OFF_CONTROLLERS_TIMEOUT_FLOAT: &'static str = "turnOffControllersTimeout";
pub const POWER_RETURN_TO_WATCHDOG_TIMEOUT_FLOAT: &'static str = "returnToWatchdogTimeout";
pub const POWER_AUTO_LAUNCH_STEAMVRON_BUTTON_PRESS: &'static str = "autoLaunchSteamVROnButtonPress";
pub const POWER_PAUSE_COMPOSITOR_ON_STANDBY_BOOL: &'static str = "pauseCompositorOnStandby";

//-----------------------------------------------------------------------------
// dashboard keys
pub const DASHBOARD_SECTION: &'static str = "dashboard";
pub const DASHBOARD_ENABLE_DASHBOARD_BOOL: &'static str = "enableDashboard";
pub const DASHBOARD_ARCADE_MODE_BOOL: &'static str = "arcadeMode";
pub const DASHBOARD_POSITION: &'static str = "position";
pub const DASHBOARD_DESKTOP_SCALE: &'static str = "desktopScale";
pub const DASHBOARD_DASHBOARD_SCALE: &'static str = "dashboardScale";
pub const DASHBOARD_USE_STANDALONE_SYSTEM_LAYER: &'static str = "standaloneSystemLayer";

//-----------------------------------------------------------------------------
// model skin keys
pub const MODELSKIN_SECTION: &'static str = "modelskins";

//-----------------------------------------------------------------------------
// driver keys - These could be checked in any driver_<name> section
pub const DRIVER_ENABLE_BOOL: &'static str = "enable";
pub const DRIVER_BLOCKED_BY_SAFEMODE_BOOL: &'static str = "blocked_by_safe_mode";
pub const DRIVER_LOAD_PRIORITY_INT32: &'static str = "loadPriority";

//-----------------------------------------------------------------------------
// web interface keys
pub const WEB_INTERFACE_SECTION: &'static str = "WebInterface";

//-----------------------------------------------------------------------------
// vrwebhelper keys
pub const VRWEB_HELPER_SECTION: &'static str = "VRWebHelper";
pub const VRWEB_HELPER_DEBUGGER_ENABLED_BOOL: &'static str = "DebuggerEnabled";
pub const VRWEB_HELPER_DEBUGGER_PORT_INT32: &'static str = "DebuggerPort";

//-----------------------------------------------------------------------------
// tracking overrides - keys are device paths, values are the device paths their
//  tracking/pose information overrides
pub const TRACKING_OVERRIDE_SECTION: &'static str = "TrackingOverrides";

//-----------------------------------------------------------------------------
// per-app keys - the section name for these is the app key itself. Some of these are prefixed by the controller type
pub const APP_BINDING_AUTOSAVE_URLSUFFIX_STRING: &'static str = "AutosaveURL";
pub const APP_BINDING_LEGACY_APISUFFIX_STRING: &'static str = "_legacy";
pub const APP_BINDING_STEAMVRINPUT_APISUFFIX_STRING: &'static str = "_steamvrinput";
pub const APP_BINDING_CURRENT_URLSUFFIX_STRING: &'static str = "CurrentURL";
pub const APP_BINDING_PREVIOUS_URLSUFFIX_STRING: &'static str = "PreviousURL";
pub const APP_NEED_TO_UPDATE_AUTOSAVE_SUFFIX_BOOL: &'static str = "NeedToUpdateAutosave";
pub const APP_DOMINANT_HAND_INT32: &'static str = "DominantHand";
pub const APP_BLOCK_OCULUS_SDK_BOOL: &'static str = "blockOculusSDK";

//-----------------------------------------------------------------------------
// configuration for trackers
pub const TRACKERS_SECTION: &'static str = "trackers";

//-----------------------------------------------------------------------------
// configuration for desktop UI windows
pub const DESKTOP_UI_SECTION: &'static str = "DesktopUI";

//-----------------------------------------------------------------------------
// Last known keys for righting recovery
pub const LAST_KNOWN_SECTION: &'static str = "LastKnown";
pub const LAST_KNOWN_HMDMANUFACTURER_STRING: &'static str = "HMDManufacturer";
pub const LAST_KNOWN_HMDMODEL_STRING: &'static str = "HMDModel";

//-----------------------------------------------------------------------------
// Dismissed warnings
pub const DISMISSED_WARNINGS_SECTION: &'static str = "DismissedWarnings";

//-----------------------------------------------------------------------------
// Input Settings
pub const INPUT_SECTION: &'static str = "input";
pub const INPUT_LEFT_THUMBSTICK_ROTATION_FLOAT: &'static str = "leftThumbstickRotation";
pub const INPUT_RIGHT_THUMBSTICK_ROTATION_FLOAT: &'static str = "rightThumbstickRotation";
pub const INPUT_THUMBSTICK_DEADZONE_FLOAT: &'static str = "thumbstickDeadzone";

//-----------------------------------------------------------------------------
// Log of GPU performance
pub const GPU_SPEED_SECTION: &'static str = "GpuSpeed";