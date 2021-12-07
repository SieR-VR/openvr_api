use std::ffi::c_void;

use super::vrtypes::*;
use super::public_vrtypes::*;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct VR_IVRSystem_FnTable {
    // ------------------------------------
    // Display Methods
    // ------------------------------------

    /* Suggested size for the intermediate render target that the distortion pulls from. */
    pub GetRecommendedRenderTargetSize: unsafe extern "stdcall" fn(pnWidth: *mut u32, pnHeight: *mut u32),

    /* The projection matrix for the specified eye */
    pub GetProjectionMatrix: unsafe extern "stdcall" fn(eEye: EVREye, fNearZ: f32, fFarZ: f32) -> HmdMatrix44,

    /* The components necessary to build your own projection matrix in case your
    * application is doing something fancy like infinite Z */
    pub GetProjectionRaw: unsafe extern "stdcall" fn(eEye: EVREye, pfLeft: *mut f32, pfRight: *mut f32, pfTop: *mut f32, pfBottom: *mut f32 ),

    /* Gets the result of the distortion function for the specified eye and input UVs. UVs go from 0,0 in
    * the upper left of that eye's viewport and 1,1 in the lower right of that eye's viewport.
    * Returns true for success. Otherwise, returns false, and distortion coordinates are not suitable. */
    pub ComputeDistortion: unsafe extern "stdcall" fn(eEye: EVREye, fU: f32, fV: f32, pDistortionCoordinates: *mut DistortionCoordinates) -> bool,

    /* Returns the transform from eye space to the head space. Eye space is the per-eye flavor of head
    * space that provides stereo disparity. Instead of Model * View * Projection the sequence is Model * View * Eye^-1 * Projection.
    * Normally View and Eye^-1 will be multiplied together and treated as View in your application.
    */
    pub GetEyeToHeadTransform: unsafe extern "stdcall" fn(eEye: EVREye) -> HmdMatrix34,

    /* Returns the number of elapsed seconds since the last recorded vsync event. This
    *	will come from a vsync timer event in the timer if possible or from the application-reported
    *   time if that is not available. If no vsync times are available the function will
    *   return zero for vsync time and frame counter and return false from the method. */
    pub GetTimeSinceLastVsync: unsafe extern "stdcall" fn(pfSecondsSinceLastVsync: *mut f32, pulFrameCounter: *mut u64) -> bool,

    /* [D3D9 Only]
    * Returns the adapter index that the user should pass into CreateDevice to set up D3D9 in such
    * a way that it can go full screen exclusive on the HMD. Returns -1 if there was an error.
    */
    pub GetD3D9AdapterIndex: unsafe extern "stdcall" fn() -> i32,

    /* [D3D10/11 Only]
    * Returns the adapter index that the user should pass into EnumAdapters to create the device
    * and swap chain in DX10 and DX11. If an error occurs the index will be set to -1.
    */
    pub GetDXGIOutputInfo: unsafe extern "stdcall" fn(pnAdapterIndex: *mut i32),

    /*
     * Returns platform- and texture-type specific adapter identification so that applications and the
    * compositor are creating textures and swap chains on the same GPU. If an error occurs the device
    * will be set to 0.
    * pInstance is an optional parameter that is required only when textureType is TextureType_Vulkan.
    * [D3D10/11/12 Only : unsafe extern "stdcall" fn(D3D9 Not Supported)]
    *  Returns the adapter LUID that identifies the GPU attached to the HMD. The user should
    *  enumerate all adapters using IDXGIFactory::EnumAdapters and IDXGIAdapter::GetDesc to find
    *  the adapter with the matching LUID, or use IDXGIFactory4::EnumAdapterByLuid.
    *  The discovered IDXGIAdapter should be used to create the device and swap chain.
    * [Vulkan Only]
    *  Returns the VkPhysicalDevice that should be used by the application.
    *  pInstance must be the instance the application will use to query for the VkPhysicalDevice.  The application
    *  must create the VkInstance with extensions returned by IVRCompositor::GetVulkanInstanceExtensionsRequired enabled.
    * [macOS Only]
    *  For TextureType_IOSurface returns the id<MTLDevice> that should be used by the application.
    *  On 10.13+ for TextureType_OpenGL returns the 'registryId' of the renderer which should be used
    *   by the application. See Apple Technical Q&A QA1168 for information on enumerating GL Renderers, and the
    *   new kCGLRPRegistryIDLow and kCGLRPRegistryIDHigh CGLRendererProperty values in the 10.13 SDK.
    *  Pre 10.13 for TextureType_OpenGL returns 0, as there is no dependable way to correlate the HMDs MTLDevice
    *   with a GL Renderer.
    */
    pub GetOutputDevice: unsafe extern "stdcall" fn(pnDevice: *mut u64, textureType: ETextureType, pInstance: VkInstance ),

    // ------------------------------------
    // Display Mode methods
    // ------------------------------------

    /* Use to determine if the headset display is part of the desktop : unsafe extern "stdcall" fn(i.e. extended) or hidden : unsafe extern "stdcall" fn(i.e. direct mode). */
    pub IsDisplayOnDesktop: unsafe extern "stdcall" fn() -> bool,

    /* Set the display visibility : unsafe extern "stdcall" fn(true = extended, false = direct mode).  Return value of true indicates that the change was successful. */
    pub SetDisplayVisibility: unsafe extern "stdcall" fn( bIsVisibleOnDesktop: bool ) -> bool,

    // ------------------------------------
    // Tracking Methods
    // ------------------------------------

    /* The pose that the tracker thinks that the HMD will be in at the specified number of seconds into the
    * future. Pass 0 to get the state at the instant the method is called. Most of the time the application should
    * calculate the time until the photons will be emitted from the display and pass that time into the method.
    *
    * This is roughly analogous to the inverse of the view matrix in most applications, though
    * many games will need to do some additional rotation or translation on top of the rotation
    * and translation provided by the head pose.
    *
    * For devices where bPoseIsValid is true the application can use the pose to position the device
    * in question. The provided array can be any size up to k_unMaxTrackedDeviceCount.
    *
    * Seated experiences should call this method with TrackingUniverseSeated and receive poses relative
    * to the seated zero pose. Standing experiences should call this method with TrackingUniverseStanding
    * and receive poses relative to the Chaperone Play Area. TrackingUniverseRawAndUncalibrated should
    * probably not be used unless the application is the Chaperone calibration tool itself, but will provide
    * poses relative to the hardware-specific coordinate system in the driver.
    */
    pub GetDeviceToAbsoluteTrackingPose: unsafe extern "stdcall" fn(
        eOrigin: ETrackingUniverseOrigin, 
        fPredictedSecondsToPhotonsFromNow: f32,  
        pTrackedDevicePoseArray: *mut TrackedDevicePose, 
        unTrackedDevicePoseArrayCount: u32 
    ),

    /* Returns the transform from the seated zero pose to the standing absolute tracking system. This allows
    * applications to represent the seated origin to used or transform object positions from one coordinate
    * system to the other.
    *
    * The seated origin may or may not be inside the Play Area or Collision Bounds returned by IVRChaperone. Its position
    * depends on what the user has set from the Dashboard settings and previous calls to ResetSeatedZeroPose. */
    pub GetSeatedZeroPoseToStandingAbsoluteTrackingPose: unsafe extern "stdcall" fn() -> HmdMatrix34,

    /* Returns the transform from the tracking origin to the standing absolute tracking system. This allows
    * applications to convert from raw tracking space to the calibrated standing coordinate system. */
    pub GetRawZeroPoseToStandingAbsoluteTrackingPose: unsafe extern "stdcall" fn() -> HmdMatrix34,

    /* Get a sorted array of device indices of a given class of tracked devices : unsafe extern "stdcall" fn(e.g. controllers).  Devices are sorted right to left
    * relative to the specified tracked device : unsafe extern "stdcall" fn(default: hmd -- pass in -1 for absolute tracking space).  Returns the number of devices
    * in the list, or the size of the array needed if not large enough. */
    pub GetSortedTrackedDeviceIndicesOfClass: unsafe extern "stdcall" fn( 
        eTrackedDeviceClass: ETrackedDeviceClass, 
        punTrackedDeviceIndexArray: *mut TrackedDeviceIndex, 
        unTrackedDeviceIndexArrayCount: u32, 
        unRelativeToTrackedDeviceIndex: TrackedDeviceIndex,
    ) -> u32,

    /* Returns the level of activity on the device. */
    pub GetTrackedDeviceActivityLevel: unsafe extern "stdcall" fn(unDeviceId: TrackedDeviceIndex) -> EDeviceActivityLevel,

    /* Convenience utility to apply the specified transform to the specified pose.
    *   This properly transforms all pose components, including velocity and angular velocity
    */
    pub ApplyTransform: unsafe extern "stdcall" fn( 
        pOutputPose: *mut TrackedDevicePose, 
        pTrackedDevicePose: *const TrackedDevicePose, 
        pTransform: *const HmdMatrix34
    ),

    /* Returns the device index associated with a specific role, for example the left hand or the right hand. This function is deprecated in favor of the new IVRInput system. */
    pub GetTrackedDeviceIndexForControllerRole: unsafe extern "stdcall" fn(unDeviceType: ETrackedControllerRole) -> TrackedDeviceIndex,

    /* Returns the controller type associated with a device index. This function is deprecated in favor of the new IVRInput system. */
    pub GetControllerRoleForTrackedDeviceIndex: unsafe extern "stdcall" fn( unDeviceIndex: TrackedDeviceIndex ) -> ETrackedControllerRole,

    // ------------------------------------
    // Property methods
    // ------------------------------------

    /* Returns the device class of a tracked device. If there has not been a device connected in this slot
    * since the application started this function will return TrackedDevice_Invalid. For previous detected
    * devices the function will return the previously observed device class.
    *
    * To determine which devices exist on the system, just loop from 0 to k_unMaxTrackedDeviceCount and check
    * the device class. Every device with something other than TrackedDevice_Invalid is associated with an
    * actual tracked device. */
    pub GetTrackedDeviceClass: unsafe extern "stdcall" fn( unDeviceIndex: TrackedDeviceIndex ) -> ETrackedDeviceClass,

    /* Returns true if there is a device connected in this slot. */
    pub IsTrackedDeviceConnected: unsafe extern "stdcall" fn( unDeviceIndex: TrackedDeviceIndex ) -> bool,

    /* Returns a bool property. If the device index is not valid or the property is not a bool type this function will return false. */
    pub GetBoolTrackedDeviceProperty: unsafe extern "stdcall" fn( 
        unDeviceIndex: TrackedDeviceIndex, 
        prop: ETrackedDeviceProperty, 
        pError: *mut ETrackedPropertyError
    ) -> bool,

    /* Returns a float property. If the device index is not valid or the property is not a float type this function will return 0. */
    pub GetFloatTrackedDeviceProperty: unsafe extern "stdcall" fn( 
        unDeviceIndex: TrackedDeviceIndex, 
        prop: ETrackedDeviceProperty, 
        pError: *mut ETrackedPropertyError
    ) -> f32,

    /* Returns an int property. If the device index is not valid or the property is not a int type this function will return 0. */
    pub GetInt32TrackedDeviceProperty: unsafe extern "stdcall" fn( 
        unDeviceIndex: TrackedDeviceIndex, 
        prop: ETrackedDeviceProperty, 
        pError: *mut ETrackedPropertyError
    ) -> i32,

    /* Returns a uint64 property. If the device index is not valid or the property is not a uint64 type this function will return 0. */
    pub GetUint64TrackedDeviceProperty: unsafe extern "stdcall" fn( 
        unDeviceIndex: TrackedDeviceIndex, 
        prop: ETrackedDeviceProperty, 
        pError: *mut ETrackedPropertyError
    ) -> u64,

    /* Returns a matrix property. If the device index is not valid or the property is not a matrix type, this function will return identity. */
    pub GetMatrix34TrackedDeviceProperty: unsafe extern "stdcall" fn( 
        unDeviceIndex: TrackedDeviceIndex, 
        prop: ETrackedDeviceProperty, 
        pError: *mut ETrackedPropertyError
    ) -> HmdMatrix34,

    /* Returns an array of one type of property. If the device index is not valid or the property is not a single value or an array of the specified type,
    * this function will return 0. Otherwise it returns the number of bytes necessary to hold the array of properties. If unBufferSize is
    * greater than the returned size and pBuffer is non-NULL, pBuffer is filled with the contents of array of properties. */
    pub GetArrayTrackedDeviceProperty: unsafe extern "stdcall" fn( 
        unDeviceIndex: TrackedDeviceIndex, 
        prop: ETrackedDeviceProperty, 
        propType: PropertyTypeTag, 
        pBuffer: *mut c_void, 
        unBufferSize: u32,
        pError: *mut ETrackedPropertyError
    ) -> u32,

    /* Returns a string property. If the device index is not valid or the property is not a string type this function will
    * return 0. Otherwise it returns the length of the number of bytes necessary to hold this string including the trailing
    * null. Strings will always fit in buffers of k_unMaxPropertyStringSize characters. */
    pub GetStringTrackedDeviceProperty: unsafe extern "stdcall" fn( 
        unDeviceIndex: TrackedDeviceIndex, 
        prop: ETrackedDeviceProperty, 
        pchValue: *mut i8, 
        unBufferSize: u32,
        pError: *mut ETrackedPropertyError
    ) -> u32,

    /* returns a string that corresponds with the specified property error. The string will be the name
    * of the error enum value for all valid error codes */
    pub GetPropErrorNameFromEnum: unsafe extern "stdcall" fn(error: ETrackedPropertyError) -> *const i8,

    // ------------------------------------
    // Event methods
    // ------------------------------------

    /* Returns true and fills the event with the next event on the queue if there is one. If there are no events
    * this method returns false. uncbVREvent should be the size in bytes of the VREvent_t struct */
    pub PollNextEvent: unsafe extern "stdcall" fn(pEvent: *mut VREvent, uncbVREvent: u32) -> bool,

    /* Returns true and fills the event with the next event on the queue if there is one. If there are no events
    * this method returns false. Fills in the pose of the associated tracked device in the provided pose struct.
    * This pose will always be older than the call to this function and should not be used to render the device.
    uncbVREvent should be the size in bytes of the VREvent_t struct */
    pub PollNextEventWithPose: unsafe extern "stdcall" fn( 
        eOrigin: ETrackingUniverseOrigin, 
        pEvent: *mut VREvent, 
        uncbVREvent: u32, 
        pTrackedDevicePose: *mut TrackedDevicePose
    ) -> bool,

    /* returns the name of an EVREvent enum value */
    pub GetEventTypeNameFromEnum: unsafe extern "stdcall" fn(eType: EVREventType) -> *const i8,

    // ------------------------------------
    // Rendering helper methods
    // ------------------------------------

    /* Returns the hidden area mesh for the current HMD. The pixels covered by this mesh will never be seen by the user after the lens distortion is
    * applied based on visibility to the panels. If this HMD does not have a hidden area mesh, the vertex data and count will be NULL and 0 respectively.
    * This mesh is meant to be rendered into the stencil buffer : unsafe extern "stdcall" fn(or into the depth buffer setting nearz) before rendering each eye's view.
    * This will improve performance by letting the GPU early-reject pixels the user will never see before running the pixel shader.
    * NOTE: Render this mesh with backface culling disabled since the winding order of the vertices can be different per-HMD or per-eye.
    * Setting the bInverse argument to true will produce the visible area mesh that is commonly used in place of full-screen quads. The visible area mesh covers all of the pixels the hidden area mesh does not cover.
    * Setting the bLineLoop argument will return a line loop of vertices in HiddenAreaMesh_t->pVertexData with HiddenAreaMesh_t->unTriangleCount set to the number of vertices.
    */
    pub GetHiddenAreaMesh: unsafe extern "stdcall" fn( 
        Eye: EVREye, 
        type_: EHiddenAreaMeshType 
    ) -> HiddenAreaMesh,

    // ------------------------------------
    // Controller methods
    // ------------------------------------

    /* Fills the supplied struct with the current state of the controller. Returns false if the controller index
    * is invalid. This function is deprecated in favor of the new IVRInput system. */
    pub GetControllerState: unsafe extern "stdcall" fn( 
        unControllerDeviceIndex: TrackedDeviceIndex, 
        pControllerState: *mut VRControllerState, 
        unControllerStateSize: u32 
    ) -> bool,

    /* fills the supplied struct with the current state of the controller and the provided pose with the pose of
    * the controller when the controller state was updated most recently. Use this form if you need a precise controller
    * pose as input to your application when the user presses or releases a button. This function is deprecated in favor of the new IVRInput system. */
    pub GetControllerStateWithPose: unsafe extern "stdcall" fn( 
        eOrigin: ETrackingUniverseOrigin, 
        unControllerDeviceIndex: TrackedDeviceIndex, 
        pControllerState: *mut VRControllerState, 
        unControllerStateSize: u32, 
        pTrackedDevicePose: *mut TrackedDevicePose
    ) -> bool,

    /* Trigger a single haptic pulse on a controller. After this call the application may not trigger another haptic pulse on this controller
    * and axis combination for 5ms. This function is deprecated in favor of the new IVRInput system. */
    pub TriggerHapticPulse: unsafe extern "stdcall" fn( 
        unControllerDeviceIndex: TrackedDeviceIndex, 
        unAxisId: u32, 
        usDurationMicroSec: u16 
    ),

    /* returns the name of an EVRButtonId enum value. This function is deprecated in favor of the new IVRInput system.  */
    pub GetButtonIdNameFromEnum: unsafe extern "stdcall" fn(eButtonId: EVRButtonId) -> *const i8,

    /* returns the name of an EVRControllerAxisType enum value. This function is deprecated in favor of the new IVRInput system. */
    pub GetControllerAxisTypeNameFromEnum: unsafe extern "stdcall" fn(eAxisType: EVRControllerAxisType) -> *const i8,

    /* Returns true if this application is receiving input from the system. This would return false if
    * system-related functionality is consuming the input stream. */
    pub IsInputAvailable: unsafe extern "stdcall" fn() -> bool,

    /* Returns true SteamVR is drawing controllers on top of the application. Applications should consider
    * not drawing anything attached to the user's hands in this case. */
    pub IsSteamVRDrawingControllers: unsafe extern "stdcall" fn() -> bool,

    /* Returns true if the user has put SteamVR into a mode that is distracting them from the application.
    * For applications where this is appropriate, the application should pause ongoing activity. */
    pub ShouldApplicationPause: unsafe extern "stdcall" fn() -> bool,

    /* Returns true if SteamVR is doing significant rendering work and the game should do what it can to reduce
    * its own workload. One common way to do this is to reduce the size of the render target provided for each eye. */
    pub ShouldApplicationReduceRenderingWork: unsafe extern "stdcall" fn() -> bool,

    // ------------------------------------
    // Firmware methods
    // ------------------------------------

    /* Performs the actual firmware update if applicable.
     * The following events will be sent, if VRFirmwareError_None was returned: VREvent_FirmwareUpdateStarted, VREvent_FirmwareUpdateFinished
    * Use the properties Prop_Firmware_UpdateAvailable_Bool, Prop_Firmware_ManualUpdate_Bool, and Prop_Firmware_ManualUpdateURL_String
    * to figure our whether a firmware update is available, and to figure out whether its a manual update
    * Prop_Firmware_ManualUpdateURL_String should point to an URL describing the manual update process */
    pub PerformFirmwareUpdate: unsafe extern "stdcall" fn(unDeviceIndex: TrackedDeviceIndex) -> EVRFirmwareError,

    // ------------------------------------
    // Application life cycle methods
    // ------------------------------------

    /* Call this to acknowledge to the system that VREvent_Quit has been received and that the process is exiting.
    * This extends the timeout until the process is killed. */
    pub AcknowledgeQuit_Exiting: unsafe extern "stdcall" fn(),

    // -------------------------------------
    // App container sandbox methods
    // -------------------------------------

    /* Retrieves a null-terminated, semicolon-delimited list of UTF8 file paths that an application
    * must have read access to when running inside of an app container. Returns the number of bytes
    * needed to hold the list. */
    pub GetAppContainerFilePaths: unsafe extern "stdcall" fn(pchBuffer: *mut i8, unBufferSize: u32) -> u32,

    // -------------------------------------
    // System methods
    // -------------------------------------

    /* Returns the current version of the SteamVR runtime. The returned string will remain valid until VR_Shutdown is called.
    *
    * NOTE: Is it not appropriate to use this version to test for the presence of any SteamVR feature. Only use this version
    * number for logging or showing to a user, and not to try to detect anything at runtime. When appropriate, feature-specific
    * presence information is provided by other APIs. */
    pub GetRuntimeVersion: unsafe extern "stdcall" fn() -> *const i8,
}

pub const IVRSYSTEM_VERSION: &'static str = "IVRSystem_022";