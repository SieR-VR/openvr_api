use super::{public_vrtypes::*, vrtypes::ETrackingUniverseOrigin};

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum ChaperoneCalibrationState
{
	// OK!
	ChaperoneCalibrationState_OK = 1,									// Chaperone is fully calibrated and working correctly

	// Warnings
	ChaperoneCalibrationState_Warning = 100,
	ChaperoneCalibrationState_Warning_BaseStationMayHaveMoved = 101,	// A base station thinks that it might have moved
	ChaperoneCalibrationState_Warning_BaseStationRemoved = 102,			// There are less base stations than when calibrated
	ChaperoneCalibrationState_Warning_SeatedBoundsInvalid = 103,		// Seated bounds haven't been calibrated for the current tracking center

	// Errors
	ChaperoneCalibrationState_Error = 200,								// The UniverseID is invalid
	ChaperoneCalibrationState_Error_BaseStationUninitialized = 201,		// Tracking center hasn't be calibrated for at least one of the base stations
	ChaperoneCalibrationState_Error_BaseStationConflict = 202,			// Tracking center is calibrated, but base stations disagree on the tracking space
	ChaperoneCalibrationState_Error_PlayAreaInvalid = 203,				// Play Area hasn't been calibrated for the current tracking center
	ChaperoneCalibrationState_Error_CollisionBoundsInvalid = 204,		// Collision Bounds haven't been calibrated for the current tracking center
}

/** HIGH LEVEL TRACKING SPACE ASSUMPTIONS:
* 0,0,0 is the preferred standing area center.
* 0Y is the floor height.
* -Z is the preferred forward facing direction. */
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct VR_IVRChaperone_FnTable
{
    /** Get the current state of Chaperone calibration. This state can change at any time during a session due to physical base station changes. **/
    pub GetCalibrationState: unsafe extern "stdcall" fn() -> ChaperoneCalibrationState,

    /** Returns the width and depth of the Play Area (formerly named Soft Bounds) in X and Z.
	* Tracking space center (0,0,0) is the center of the Play Area. **/
    pub GetPlayAreaSize: unsafe extern "stdcall" fn(pSizex: *mut f32, pSizeZ: *mut f32) -> bool,

    /** Returns the 4 corner positions of the Play Area (formerly named Soft Bounds).
	* Corners are in counter-clockwise order.
	* Standing center (0,0,0) is the center of the Play Area.
	* It's a rectangle.
	* 2 sides are parallel to the X axis and 2 sides are parallel to the Z axis.
	* Height of every corner is 0Y (on the floor). **/
    pub GetPlayAreaRect: unsafe extern "stdcall" fn(rect: *mut HmdQuad) -> bool,

    /** Reload Chaperone data from the .vrchap file on disk. */
    pub ReloadInfo: unsafe extern "stdcall" fn(),

    /** Optionally give the chaperone system a hit about the color and brightness in the scene **/
	pub SetSceneColor: unsafe extern "stdcall" fn(color: HmdColor),

	/** Get the current chaperone bounds draw color and brightness **/
	pub GetBoundsColor: unsafe extern "stdcall" fn(
        pOutputColorArray: *mut HmdColor, 
        nNumOutputColors: i32, 
        flCollisionBoundsFadeDistance: f32, 
        pOutputCameraColor: *mut HmdColor),

	/** Determine whether the bounds are showing right now **/
	pub AreBoundsVisible: unsafe extern "stdcall" fn() -> bool,

	/** Force the bounds to show, mostly for utilities **/
	pub ForceBoundsVisible: unsafe extern "stdcall" fn(bForce: bool),

	/** Sets the zero pose for the given tracker coordinate system to the current position and yaw of the HMD. After
	* ResetZeroPose all GetDeviceToAbsoluteTrackingPose calls as the origin will be relative to this new zero pose.
	* The new zero coordinate system will not change the fact that the Y axis is up in the real world, so the next
	* pose returned from GetDeviceToAbsoluteTrackingPose after a call to ResetZeroPose may not be exactly an
	* identity matrix.
	*
	* NOTE: This function overrides the user's previously saved zero pose and should only be called as the result of a user action.
	* Users are also able to set their zero pose via the OpenVR Dashboard.
	**/
	pub ResetZeroPose: unsafe extern "stdcall" fn(eTrackingUniverseOrigin: ETrackingUniverseOrigin),
}

pub const IVRCHAPERONE_VERSION: &'static str = "IVRChaperone_004";
