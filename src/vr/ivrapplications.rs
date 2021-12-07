#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum EVRApplicationError {
    VRApplicationError_None,

    VRApplicationError_AppKeyAlreadyExists = 100, // Only one application can use any given key
    VRApplicationError_NoManifest = 101, // the running application does not have a manifest
    VRApplicationError_NoApplication = 102, // No application is running
    VRApplicationError_InvalidIndex = 103,
    VRApplicationError_UnknownApplication = 104, // the application could not be found
    VRApplicationError_IPCFailed = 105,          // An IPC failure caused the request to fail
    VRApplicationError_ApplicationAlreadyRunning = 106,
    VRApplicationError_InvalidManifest = 107,
    VRApplicationError_InvalidApplication = 108,
    VRApplicationError_LaunchFailed = 109, // the process didn't start
    VRApplicationError_ApplicationAlreadyStarting = 110, // the system was already starting the same application
    VRApplicationError_LaunchInProgress = 111, // The system was already starting a different application
    VRApplicationError_OldApplicationQuitting = 112,
    VRApplicationError_TransitionAborted = 113,
    VRApplicationError_IsTemplate = 114, // error when you try to call LaunchApplication() on a template type app (use LaunchTemplateApplication)
    VRApplicationError_SteamVRIsExiting = 115,

    VRApplicationError_BufferTooSmall = 200, // The provided buffer was too small to fit the requested data
    VRApplicationError_PropertyNotSet = 201, // The requested property was not set
    VRApplicationError_UnknownProperty = 202,
    VRApplicationError_InvalidParameter = 203,

    VRApplicationError_NotImplemented = 300, // Fcn is not implemented in current interface
}


pub const MAX_APPLICATION_KEY_LENGTH: usize = 128;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum EVRApplicationProperty {
    VRApplicationProperty_Name_String,

    VRApplicationProperty_LaunchType_String = 11,
    VRApplicationProperty_WorkingDirectory_String = 12,
    VRApplicationProperty_BinaryPath_String = 13,
    VRApplicationProperty_Arguments_String = 14,
    VRApplicationProperty_URL_String = 15,

    VRApplicationProperty_Description_String = 50,
    VRApplicationProperty_NewsURL_String = 51,
    VRApplicationProperty_ImagePath_String = 52,
    VRApplicationProperty_Source_String = 53,
    VRApplicationProperty_ActionManifestURL_String = 54,

    VRApplicationProperty_IsDashboardOverlay_Bool = 60,
    VRApplicationProperty_IsTemplate_Bool = 61,
    VRApplicationProperty_IsInstanced_Bool = 62,
    VRApplicationProperty_IsInternal_Bool = 63,
    VRApplicationProperty_WantsCompositorPauseInStandby_Bool = 64,
    VRApplicationProperty_IsHidden_Bool = 65,

    VRApplicationProperty_LastLaunchTime_Uint64 = 70,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum EVRSceneApplicationState
{
    EVRSceneApplicationState_None						     , // Scene Application is not running
    EVRSceneApplicationState_Starting					      = 1, // Scene Application is starting
    EVRSceneApplicationState_Quitting					      = 2, // Scene Application is quitting
    EVRSceneApplicationState_Running						  = 3, // Scene Application is running, and submitting frames, a custom skybox, or a visible overlay
    EVRSceneApplicationState_Waiting						  = 4, // Scene Application is running, but not drawing anything
}

#[allow(non_snake_case)]
pub struct AppOverrideKeys
{
    pub pchKey: *const i8,
    pub pchValue: *const i8,
}

pub const MIME_TYPE_HOMEAPP: &'static str = "vr/home";
pub const MIME_TYPE_GAMETHEATER: &'static str = "vr/game_theater";

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct VR_IVRApplications_FnTable
{
    // ---------------  Application management  --------------- //

	/** Adds an application manifest to the list to load when building the list of installed applications.
	* Temporary manifests are not automatically loaded */
    pub AddApplicationManifest: unsafe extern "stdcall" fn(pchApplicationManifestFullPath: *const i8, bTemporary: bool) -> EVRApplicationError,

    /** Removes an application manifest from the list to load when building the list of installed applications. */
    pub RemoveApplicationManifest: unsafe extern "stdcall" fn(pchApplicationManifestFullPath: *const i8) -> EVRApplicationError,

    /** Returns true if an application is installed */
	pub IsApplicationInstalled: unsafe extern "stdcall" fn(pchAppKey: *const i8) -> bool,

    /** Returns the key of the current application */ 

    /** Returns the number of applications available in the list */
    pub GetApplicationCount: unsafe extern "stdcall" fn() -> u32,

    /** Returns the key of the specified application. The index is at least 0 and is less than the return
    * value of GetApplicationCount(). The buffer should be at least k_unMaxApplicationKeyLength in order to
    * fit the key. */
    pub GetApplicationKeyByIndex: unsafe extern "stdcall" fn(
        unApplicationIndex: u32, 
        pchAppKeyBuffer: *mut i8, 
        unAppKeyBufferLen: u32) -> EVRApplicationError,

    /** Returns the key of the application for the specified Process Id. The buffer should be at least
    * k_unMaxApplicationKeyLength in order to fit the key. */
    pub GetApplicationKeyByProcessId: unsafe extern "stdcall" fn(
        unProcessId: u32, 
        pchAppKeyBuffer: *mut i8, 
        unAppKeyBufferLen: u32) -> EVRApplicationError,

    /** Launches the application. The existing scene application will exit and then the new application will start.
    * This call is not valid for dashboard overlay applications. */
    pub LaunchApplication: unsafe extern "stdcall" fn(pchAppKey: *const i8) -> EVRApplicationError,

    /** Launches an instance of an application of type template, with its app key being pchNewAppKey (which must be unique) and optionally override sections
    * from the manifest file via AppOverrideKeys_t
    */
    pub LaunchTemplateApplication: unsafe extern "stdcall" fn( 
        pchTemplateAppKey: *const i8, 
        pchNewAppKey: *const i8, 
        pKeys: *const AppOverrideKeys, 
        unKeys: u32) -> EVRApplicationError,

    /** launches the application currently associated with this mime type and passes it the option args, typically the filename or object name of the item being launched */
    pub LaunchApplicationFromMimeType: unsafe extern "stdcall" fn(pchMimeType: *const i8, pchArgs: *const i8) -> EVRApplicationError,

    /** Launches the dashboard overlay application if it is not already running. This call is only valid for
    * dashboard overlay applications. */
    pub LaunchDashboardOverlay: unsafe extern "stdcall" fn(pchAppKey: *const i8) -> EVRApplicationError,

    /** Cancel a pending launch for an application */
    pub CancelApplicationLaunch: unsafe extern "stdcall" fn(pchAppKey: *const i8) -> bool,

    /** Identifies a running application. OpenVR can't always tell which process started in response
    * to a URL. This function allows a URL handler (or the process itself) to identify the app key
    * for the now running application. Passing a process ID of 0 identifies the calling process.
    * The application must be one that's known to the system via a call to AddApplicationManifest. */
    pub IdentifyApplication: unsafe extern "stdcall" fn( unProcessId: u32, pchAppKey: *const i8) -> EVRApplicationError,

    /** Returns the process ID for an application. Return 0 if the application was not found or is not running. */
    pub GetApplicationProcessId: unsafe extern "stdcall" fn(pchAppKey: *const i8) -> u32,

    /** Returns a string for an applications error */
    pub GetApplicationsErrorNameFromEnum: unsafe extern "stdcall" fn( error: EVRApplicationError ) -> *const i8,

    // ---------------  Application properties  --------------- //

    /** Returns a value for an application property. The required buffer size to fit this value will be returned. */
    pub GetApplicationPropertyString: unsafe extern "stdcall" fn(
        pchAppKey: *const i8, 
        eProperty: EVRApplicationProperty, 
        pchPropertyValueBuffer: *mut i8, 
        unPropertyValueBufferLen: u32,
        peError: *mut EVRApplicationError) -> u32,

    /** Returns a bool value for an application property. Returns false in all error cases. */
    pub GetApplicationPropertyBool: unsafe extern "stdcall" fn( 
        pchAppKey: *const i8, 
        eProperty: EVRApplicationProperty,
        peError: *mut EVRApplicationError) -> bool,

    /** Returns a uint64 value for an application property. Returns 0 in all error cases. */
    pub GetApplicationPropertyUint64: unsafe extern "stdcall" fn( 
        pchAppKey: *const i8, 
        eProperty: EVRApplicationProperty,
        peError: *mut EVRApplicationError) -> u64,

    /** Sets the application auto-launch flag. This is only valid for applications which return true for VRApplicationProperty_IsDashboardOverlay_Bool. */
    pub SetApplicationAutoLaunch: unsafe extern "stdcall" fn(pchAppKey: *const i8, bAutoLaunch: bool) -> EVRApplicationError,

    /** Gets the application auto-launch flag. This is only valid for applications which return true for VRApplicationProperty_IsDashboardOverlay_Bool. */
    pub GetApplicationAutoLaunch: unsafe extern "stdcall" fn(pchAppKey: *const i8) -> bool,

    /** Adds this mime-type to the list of supported mime types for this application*/
    pub SetDefaultApplicationForMimeType: unsafe extern "stdcall" fn(pchAppKey: *const i8, pchMimeType: *const i8) -> EVRApplicationError,

    /** return the app key that will open this mime type */
    pub GetDefaultApplicationForMimeType: unsafe extern "stdcall" fn(
        pchMimeType: *const i8,
        pchAppKeyBuffer: *mut i8,
        unAppKeyBufferLen: u32) -> bool,

    /** Get the list of supported mime types for this application, comma-delimited */
    pub GetApplicationSupportedMimeTypes: unsafe extern "stdcall" fn(
        pchAppKey: *const i8,
        pchMimeTypesBuffer: *mut i8, 
        unMimeTypesBuffer: u32) -> bool,

    /** Get the list of app-keys that support this mime type, comma-delimited, the return value is number of bytes you need to return the full string */
    pub GetApplicationsThatSupportMimeType: unsafe extern "stdcall" fn( 
        pchMimeType: *const i8, 
        pchAppKeysThatSupportBuffer: *mut i8, 
        unAppKeysThatSupportBuffer: u32) -> u32,

    /** Get the args list from an app launch that had the process already running, you call this when you get a VREvent_ApplicationMimeTypeLoad */
    pub GetApplicationLaunchArguments: unsafe extern "stdcall" fn(
        unHandle: u32,
        pchArgs: *mut i8, 
        unArgs: u32) -> u32,

    // ---------------  Transition methods --------------- //

    /** Returns the app key for the application that is starting up */
    pub GetStartingApplication: unsafe extern "stdcall" fn(pchAppKeyBuffer: *mut i8, unAppKeyBufferLen: u32) -> EVRApplicationError,

    /** Returns the application transition state */
    pub GetSceneApplicationState: unsafe extern "stdcall" fn() -> EVRSceneApplicationState,

    /** Returns errors that would prevent the specified application from launching immediately. Calling this function will
    * cause the current scene application to quit, so only call it when you are actually about to launch something else.
    * What the caller should do about these failures depends on the failure:
    *   VRApplicationError_OldApplicationQuitting - An existing application has been told to quit. Wait for a VREvent_ProcessQuit
    *                                               and try again.
    *   VRApplicationError_ApplicationAlreadyStarting - This application is already starting. This is a permanent failure.
    *   VRApplicationError_LaunchInProgress	      - A different application is already starting. This is a permanent failure.
    *   VRApplicationError_None                   - Go ahead and launch. Everything is clear.
    */
    pub PerformApplicationPrelaunchCheck: unsafe extern "stdcall" fn( pchAppKey: *const i8) -> EVRApplicationError,

    /** Returns a string for an application transition state */
    pub GetSceneApplicationStateNameFromEnum: unsafe extern "stdcall" fn( state: EVRSceneApplicationState ) -> *const i8,

    /** Starts a subprocess within the calling application. This
    * suppresses all application transition UI and automatically identifies the new executable
    * as part of the same application. On success the calling process should exit immediately.
    * If working directory is NULL or "" the directory portion of the binary path will be
    * the working directory. */
    pub LaunchInternalProcess: unsafe extern "stdcall" fn(
        pchBinaryPath: *const i8, 
        pchArguments: *const i8, 
        pchWorkingDirectory: *const i8) -> EVRApplicationError,

    /** Returns the current scene process ID according to the application system. A scene process will get scene
    * focus once it starts rendering, but it will appear here once it calls VR_Init with the Scene application
    * type. */
    pub GetCurrentSceneProcessId: unsafe extern "stdcall" fn() -> u32,
}