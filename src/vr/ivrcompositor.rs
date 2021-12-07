use std::ffi::c_void;

use super::public_vrtypes::*;
use super::vrtypes::*;

/** Errors that can occur with the VR compositor */
#[allow(non_camel_case_types)]
pub enum EVRCompositorError
{
	VRCompositorError_None						= 0,
	VRCompositorError_RequestFailed				= 1,
	VRCompositorError_IncompatibleVersion		= 100,
	VRCompositorError_DoNotHaveFocus			= 101,
	VRCompositorError_InvalidTexture			= 102,
	VRCompositorError_IsNotSceneApplication		= 103,
	VRCompositorError_TextureIsOnWrongDevice	= 104,
	VRCompositorError_TextureUsesUnsupportedFormat = 105,
	VRCompositorError_SharedTexturesNotSupported = 106,
	VRCompositorError_IndexOutOfRange			= 107,
	VRCompositorError_AlreadySubmitted			= 108,
	VRCompositorError_InvalidBounds				= 109,
	VRCompositorError_AlreadySet				= 110,
}

/** Timing mode passed to SetExplicitTimingMode(); see that function for documentation */
#[allow(non_camel_case_types)]
pub enum EVRCompositorTimingMode
{
	VRCompositorTimingMode_Implicit											= 0,
	VRCompositorTimingMode_Explicit_RuntimePerformsPostPresentHandoff		= 1,
	VRCompositorTimingMode_Explicit_ApplicationPerformsPostPresentHandoff	= 2,
}

#[allow(non_camel_case_types, non_snake_case)]
pub struct Compositor_CumulativeStats {
    pub m_nPid: u32, // Process id associated with these stats (may no longer be running).
	pub m_nNumFramePresents: u32, // total number of times we called present (includes reprojected frames)
	pub m_nNumDroppedFrames: u32, // total number of times an old frame was re-scanned out (without reprojection)
	pub m_nNumReprojectedFrames: u32, // total number of times a frame was scanned out a second time (with reprojection)

	/** Values recorded at startup before application has fully faded in the first time. */
	pub m_nNumFramePresentsOnStartup: u32,
	pub m_nNumDroppedFramesOnStartup: u32,
	pub m_nNumReprojectedFramesOnStartup: u32,

	/** Applications may explicitly fade to the compositor.  This is usually to handle level transitions, and loading often causes
	* system wide hitches.  The following stats are collected during this period.  Does not include values recorded during startup. */
	pub m_nNumLoading: u32,
	pub m_nNumFramePresentsLoading: u32,
	pub m_nNumDroppedFramesLoading: u32,
	pub m_nNumReprojectedFramesLoading: u32,

	/** If we don't get a new frame from the app in less than 2.5 frames, then we assume the app has hung and start
	* fading back to the compositor.  The following stats are a result of this, and are a subset of those recorded above.
	* Does not include values recorded during start up or loading. */
	pub m_nNumTimedOut: u32,
	pub m_nNumFramePresentsTimedOut: u32,
	pub m_nNumDroppedFramesTimedOut: u32,
	pub m_nNumReprojectedFramesTimedOut: u32,
}

#[allow(non_camel_case_types, non_snake_case)]
pub struct Compositor_StageRenderSettings {
    /** Primary color is applied as a tint to (i.e. multiplied with) the model's texture */
	pub m_PrimaryColor: HmdColor,
    pub m_SecondaryColor: HmdColor,

	/** Vignette radius is in meters and is used to fade to the specified secondary solid color over
	* that 3D distance from the origin of the playspace. */
	pub m_flVignetteInnerRadius: f32,
	pub m_flVignetteOuterRadius: f32,

	/** Fades to the secondary color based on view incidence.  This variable controls the linearity
	* of the effect.  It is mutually exclusive with vignette.  Additionally, it treats the mesh as faceted. */
	pub m_flFresnelStrength: f32,

	/** Controls backface culling. */
	pub m_bBackfaceCulling: bool,

	/** Converts the render model's texture to luma and applies to rgb equally.  This is useful to
	* combat compression artifacts that can occur on desaturated source material. */
	pub m_bGreyscale: bool,

	/** Renders mesh as a wireframe. */
	pub m_bWireframe: bool,
}

impl Compositor_StageRenderSettings {
    pub fn new() -> Compositor_StageRenderSettings {
        Compositor_StageRenderSettings {
            m_PrimaryColor: HmdColor { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            m_SecondaryColor: HmdColor { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            m_flVignetteInnerRadius: 0.0,
            m_flVignetteOuterRadius: 0.0,
            m_flFresnelStrength: 0.0,
            m_bBackfaceCulling: false,
            m_bGreyscale: false,
            m_bWireframe: false,
        }
    }
}

#[allow(non_camel_case_types, non_snake_case)]
pub struct VR_IVRCompositor_FnTable {
    /** Sets tracking space returned by WaitGetPoses */
    pub SetTrackingSpace: unsafe extern "stdcall" fn(eOrigin: ETrackingUniverseOrigin),

    /** Gets current tracking space returned by WaitGetPoses */
    pub GetTrackingSpace: unsafe extern "stdcall" fn() -> ETrackingUniverseOrigin,

    /** Scene applications should call this function to get poses to render with (and optionally poses predicted an additional frame out to use for gameplay).
	* This function will block until "running start" milliseconds before the start of the frame, and should be called at the last moment before needing to
	* start rendering.
	*
	* Return codes:
	*	- IsNotSceneApplication (make sure to call VR_Init with VRApplicaiton_Scene)
	*	- DoNotHaveFocus (some other app has taken focus - this will throttle the call to 10hz to reduce the impact on that app)
	*/
    pub WaitGetPoses: unsafe extern "stdcall" fn(
        pRenderPoseArray: *mut TrackedDevicePose, 
        unRenderPoseArrayCount: u32,
        pGamePoseArray: *mut TrackedDevicePose, 
        unGamePoseArrayCount: u32) -> EVRCompositorError,

    /** Get the last set of poses returned by WaitGetPoses. */
    pub GetLastPoses: unsafe extern "stdcall" fn(
        pRenderPoseArray: *mut TrackedDevicePose, 
        unRenderPoseArrayCount: u32,
        pGamePoseArray: *mut TrackedDevicePose, 
        unGamePoseArrayCount: u32) -> EVRCompositorError,

    /** Interface for accessing last set of poses returned by WaitGetPoses one at a time.
	* Returns VRCompositorError_IndexOutOfRange if unDeviceIndex not less than k_unMaxTrackedDeviceCount otherwise VRCompositorError_None.
	* It is okay to pass NULL for either pose if you only want one of the values. */
    pub GetLastPoseForTrackedDeviceIndex: unsafe extern "stdcall" fn(
        unDeviceIndex: u32,
        pOutputPose: *mut TrackedDevicePose,
        pOutputGamePose : *mut TrackedDevicePose) -> EVRCompositorError,
    
    /** Updated scene texture to display. If pBounds is NULL the entire texture will be used.  If called from an OpenGL app, consider adding a glFlush after
	* Submitting both frames to signal the driver to start processing, otherwise it may wait until the command buffer fills up, causing the app to miss frames.
	*
	* OpenGL dirty state:
	*	glBindTexture
	*
	* Return codes:
	*	- IsNotSceneApplication (make sure to call VR_Init with VRApplicaiton_Scene)
	*	- DoNotHaveFocus (some other app has taken focus)
	*	- TextureIsOnWrongDevice (application did not use proper AdapterIndex - see IVRSystem.GetDXGIOutputInfo)
	*	- SharedTexturesNotSupported (application needs to call CreateDXGIFactory1 or later before creating DX device)
	*	- TextureUsesUnsupportedFormat (scene textures must be compatible with DXGI sharing rules - e.g. uncompressed, no mips, etc.)
	*	- InvalidTexture (usually means bad arguments passed in)
	*	- AlreadySubmitted (app has submitted two left textures or two right textures in a single frame - i.e. before calling WaitGetPoses again)
	*/
    pub Submit: unsafe extern "stdcall" fn(
        eEye: EVREye, 
        pTexture: *const Texture, 
        pBounds: *const VRTextureBounds,
        nSubmitFlags: EVRSubmitFlags) -> EVRCompositorError,

    /** Clears the frame that was sent with the last call to Submit. This will cause the
	* compositor to show the grid until Submit is called again. */
    pub ClearLastSubmittedFrame: unsafe extern "stdcall" fn(),

    /** Call immediately after presenting your app's window (i.e. companion window) to unblock the compositor.
	* This is an optional call, which only needs to be used if you can't instead call WaitGetPoses immediately after Present.
	* For example, if your engine's render and game loop are not on separate threads, or blocking the render thread until 3ms before the next vsync would
	* introduce a deadlock of some sort.  This function tells the compositor that you have finished all rendering after having Submitted buffers for both
	* eyes, and it is free to start its rendering work.  This should only be called from the same thread you are rendering on. */
    pub PostPresentHandoff: unsafe extern "stdcall" fn(),

    /** Returns true if timing data is filled it.  Sets oldest timing info if nFramesAgo is larger than the stored history.
	* Be sure to set timing.size = sizeof(Compositor_FrameTiming) on struct passed in before calling this function. */
	pub GetFrameTiming: unsafe extern "stdcall" fn(pTiming: *mut Compositor_FrameTiming, unFramesAgo: u32) -> bool,

	/** Interface for copying a range of timing data.  Frames are returned in ascending order (oldest to newest) with the last being the most recent frame.
	* Only the first entry's m_nSize needs to be set, as the rest will be inferred from that.  Returns total number of entries filled out. */
	pub GetFrameTimings: unsafe extern "stdcall" fn(pTiming: *mut Compositor_FrameTiming, nFrames: u32) -> u32,

	/** Returns the time in seconds left in the current (as identified by FrameTiming's frameIndex) frame.
	* Due to "running start", this value may roll over to the next frame before ever reaching 0.0. */
	pub GetFrameTimeRemaining: unsafe extern "stdcall" fn() -> f32,

	/** Fills out stats accumulated for the last connected application.  Pass in sizeof( Compositor_CumulativeStats ) as second parameter. */
	pub GetCumulativeStats: unsafe extern "stdcall" fn(pStats: *mut Compositor_CumulativeStats, nStatsSizeInBytes: u32),

	/** Fades the view on the HMD to the specified color. The fade will take fSeconds, and the color values are between
	* 0.0 and 1.0. This color is faded on top of the scene based on the alpha parameter. Removing the fade color instantly
	* would be FadeToColor( 0.0, 0.0, 0.0, 0.0, 0.0 ).  Values are in un-premultiplied alpha space. */
	pub FadeToColor: unsafe extern "stdcall" fn(fSeconds: f32, fRed: f32, fGreen: f32, fBlue: f32, fAlpha: f32, bBackground: bool),

	/** Get current fade color value. */
	pub GetCurrentFadeColor: unsafe extern "stdcall" fn(bBackground: bool) -> HmdColor,

	/** Fading the Grid in or out in fSeconds */
	pub FadeGrid: unsafe extern "stdcall" fn(fSeconds: f32, bFadeGridIn: bool),

	/** Get current alpha value of grid. */
	pub GetCurrentGridAlpha: unsafe extern "stdcall" fn() -> f32,

	/** Override the skybox used in the compositor (e.g. for during level loads when the app can't feed scene images fast enough)
	* Order is Front, Back, Left, Right, Top, Bottom.  If only a single texture is passed, it is assumed in lat-long format.
	* If two are passed, it is assumed a lat-long stereo pair. */
	pub SetSkyboxOverride: unsafe extern "stdcall" fn(pTextures: *const Texture, unTextureCount: u32),

	/** Resets compositor skybox back to defaults. */
	pub ClearSkyboxOverride: unsafe extern "stdcall" fn(),

	/** Brings the compositor window to the front. This is useful for covering any other window that may be on the HMD
	* and is obscuring the compositor window. */
	pub CompositorBringToFront: unsafe extern "stdcall" fn(),

	/** Pushes the compositor window to the back. This is useful for allowing other applications to draw directly to the HMD. */
	pub CompositorGoToBack: unsafe extern "stdcall" fn(),

	/** DEPRECATED: Tells the compositor process to clean up and exit. You do not need to call this function at shutdown.
	* Under normal circumstances the compositor will manage its own life cycle based on what applications are running. */
	pub CompositorQuit: unsafe extern "stdcall" fn(),

	/** Return whether the compositor is fullscreen */
	pub IsFullscreen: unsafe extern "stdcall" fn() -> bool,

	/** Returns the process ID of the process that is currently rendering the scene */
	pub GetCurrentSceneFocusProcess: unsafe extern "stdcall" fn() -> u32,

	/** Returns the process ID of the process that rendered the last frame (or 0 if the compositor itself rendered the frame.)
	* Returns 0 when fading out from an app and the app's process Id when fading into an app. */
	pub GetLastFrameRenderer: unsafe extern "stdcall" fn() -> u32,

	/** Returns true if the current process has the scene focus */
	pub CanRenderScene: unsafe extern "stdcall" fn() -> bool,

	/** DEPRECATED: Opens the headset view (as either a window or docked widget depending on user's preferences) that displays what the user
	* sees in the headset. */
	pub ShowMirrorWindow: unsafe extern "stdcall" fn(),

	/** DEPRECATED: Closes the headset view, either as a window or docked widget. */
	pub HideMirrorWindow: unsafe extern "stdcall" fn(),

	/** DEPRECATED: Returns true if the headset view (either as a window or docked widget) is shown. */
	pub IsMirrorWindowVisible: unsafe extern "stdcall" fn() -> bool,

	/** Writes back buffer and stereo left/right pair from the application to a 'screenshots' folder in the SteamVR runtime root. */
	pub CompositorDumpImages: unsafe extern "stdcall" fn(),

	/** Let an app know it should be rendering with low resources. */
	pub ShouldAppRenderWithLowResources: unsafe extern "stdcall" fn() -> bool,

	/** Override interleaved reprojection logic to force on. */
	pub ForceInterleavedReprojectionOn: unsafe extern "stdcall" fn(bOverride: bool),

	/** Force reconnecting to the compositor process. */
	pub ForceReconnectProcess: unsafe extern "stdcall" fn(),

	/** Temporarily suspends rendering (useful for finer control over scene transitions). */
	pub SuspendRendering: unsafe extern "stdcall" fn(bSuspend: bool),

	/** Opens a shared D3D11 texture with the undistorted composited image for each eye.  Use ReleaseMirrorTextureD3D11 when finished
	* instead of calling Release on the resource itself. */
	pub GetMirrorTextureD3D11: unsafe extern "stdcall" fn(
        eEye: EVREye, 
        pD3D11DeviceOrResource: *mut c_void, 
        ppD3D11ShaderResourceView: *mut *mut c_void) -> EVRCompositorError,
	pub ReleaseMirrorTextureD3D11: unsafe extern "stdcall" fn(pD3D11ShaderResourceView: *mut c_void),

	/** Access to mirror textures from OpenGL. */
	pub GetMirrorTextureGL: unsafe extern "stdcall" fn(
        eEye: EVREye, 
        pglTextureId: GLUint,
        pglSharedTextureHandle: *mut GLSharedTextureHandle) -> EVRCompositorError,
	pub ReleaseSharedGLTexture: unsafe extern "stdcall" fn(glTextureId: GLUint, glSharedTextureHandle: GLSharedTextureHandle) -> bool,
	pub LockGLSharedTextureForAccess: unsafe extern "stdcall" fn(glSharedTextureHandle: GLSharedTextureHandle),
	pub UnlockGLSharedTextureForAccess: unsafe extern "stdcall" fn(glSharedTextureHandle: GLSharedTextureHandle),

	/** [Vulkan Only]
	* return 0. Otherwise it returns the length of the number of bytes necessary to hold this string including the trailing
	* null.  The string will be a space separated list of-required instance extensions to enable in VkCreateInstance */
	pub GetVulkanInstanceExtensionsRequired: unsafe extern "stdcall" fn(pchValue: *mut i8, unBufferSize: u32) -> u32,

	/** [Vulkan only]
	* return 0. Otherwise it returns the length of the number of bytes necessary to hold this string including the trailing
	* null.  The string will be a space separated list of required device extensions to enable in VkCreateDevice */
	pub GetVulkanDeviceExtensionsRequired: unsafe extern "stdcall" fn(pPhysicalDevice: VkPhysicalDevice, pchValue: *mut i8, unBufferSize: u32) -> u32,

	/** [ Vulkan/D3D12 Only ]
	* There are two purposes for SetExplicitTimingMode:
	*	1. To get a more accurate GPU timestamp for when the frame begins in Vulkan/D3D12 applications.
	*	2. (Optional) To avoid having WaitGetPoses access the Vulkan queue so that the queue can be accessed from
	*	another thread while WaitGetPoses is executing.
	*
	* More accurate GPU timestamp for the start of the frame is achieved by the application calling
	* SubmitExplicitTimingData immediately before its first submission to the Vulkan/D3D12 queue.
	* This is more accurate because normally this GPU timestamp is recorded during WaitGetPoses.  In D3D11,
	* WaitGetPoses queues a GPU timestamp write, but it does not actually get submitted to the GPU until the
	* application flushes.  By using SubmitExplicitTimingData, the timestamp is recorded at the same place for
	* Vulkan/D3D12 as it is for D3D11, resulting in a more accurate GPU time measurement for the frame.
	*
	* Avoiding WaitGetPoses accessing the Vulkan queue can be achieved using SetExplicitTimingMode as well.  If this is desired,
	* the application should set the timing mode to Explicit_ApplicationPerformsPostPresentHandoff and *MUST* call PostPresentHandoff
	* itself. If these conditions are met, then WaitGetPoses is guaranteed not to access the queue.  Note that PostPresentHandoff
	* and SubmitExplicitTimingData will access the queue, so only WaitGetPoses becomes safe for accessing the queue from another
	* thread. */
	pub SetExplicitTimingMode: unsafe extern "stdcall" fn(eTimingMode: EVRCompositorTimingMode),

	/** [ Vulkan/D3D12 Only ]
	* Submit explicit timing data.  When SetExplicitTimingMode is true, this must be called immediately before
	* the application's first vkQueueSubmit (Vulkan) or ID3D12CommandQueue::ExecuteCommandLists (D3D12) of each frame.
	* This function will insert a GPU timestamp write just before the application starts its rendering.  This function
	* will perform a vkQueueSubmit on Vulkan so must not be done simultaneously with VkQueue operations on another thread.
	* Returns VRCompositorError_RequestFailed if SetExplicitTimingMode is not enabled. */
	pub SubmitExplicitTimingData: unsafe extern "stdcall" fn() -> EVRCompositorError,

	/** Indicates whether or not motion smoothing is enabled by the user settings.
	* If you want to know if motion smoothing actually triggered due to a late frame, check Compositor_FrameTiming
	* m_nReprojectionFlags & VRCompositor_ReprojectionMotion instead. */
	pub IsMotionSmoothingEnabled: unsafe extern "stdcall" fn() -> bool,

	/** Indicates whether or not motion smoothing is supported by the current hardware. */
	pub IsMotionSmoothingSupported: unsafe extern "stdcall" fn() -> bool,

	/** Indicates whether or not the current scene focus app is currently loading.  This is inferred from its use of FadeGrid to
	* explicitly fade to the compositor to cover up the fact that it cannot render at a sustained full framerate during this time. */
	pub IsCurrentSceneFocusAppLoading: unsafe extern "stdcall" fn() -> bool,

	/** Override the stage model used in the compositor to replace the grid.  RenderModelPath is a full path the an OBJ file to load.
	* This file will be loaded asynchronously from disk and uploaded to the gpu by the runtime.  Once ready for rendering, the
	* VREvent StageOverrideReady will be sent.  Use FadeToGrid to reveal.  Call ClearStageOverride to free the associated resources when finished. */
	pub SetStageOverride_Async: unsafe extern "stdcall" fn( 
        pchRenderModelPath: *const i8, 
        pTransform: *const HmdMatrix34,
		pRenderSettings: *const Compositor_StageRenderSettings, 
        nSizeOfRenderSettings: u32) -> EVRCompositorError,

	/** Resets the stage to its default user specified setting. */
	pub ClearStageOverride: unsafe extern "stdcall" fn(),

	/** Returns true if pBenchmarkResults is filled it.  Sets pBenchmarkResults with the result of the compositor benchmark.
	* nSizeOfBenchmarkResults should be set to sizeof(Compositor_BenchmarkResults) */
	pub GetCompositorBenchmarkResults: unsafe extern "stdcall" fn(
        pBenchmarkResults: *mut Compositor_BenchmarkResults,
        nSizeOfBenchmarkResults: u32) -> bool,

	/** Returns the frame id associated with the poses last returned by WaitGetPoses.  Deltas between IDs correspond to number of headset vsync intervals. */
	pub GetLastPosePredictionIDs: unsafe extern "stdcall" fn(pRenderPosePredictionID: *mut u32, pGamePosePredictionID: *mut u32) -> EVRCompositorError,

	/** Get the most up-to-date predicted (or recorded - up to 100ms old) set of poses for a given frame id. */
	pub GetPosesForFrame: unsafe extern "stdcall" fn(
        unPosePredictionID: u32, 
        pPoseArray: *mut TrackedDevicePose, 
        unPoseArrayCount: u32),
}

pub const IVRCOMPOSITOR_VERSION: &'static str = "IVRCompositor_027";
