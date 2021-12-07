use super::public_vrtypes::*;

#[allow(non_camel_case_types)]
pub enum EChaperoneConfigFile
{
	EChaperoneConfigFile_Live = 1,		// The live chaperone config, used by most applications and games
	EChaperoneConfigFile_Temp = 2,		// The temporary chaperone config, used to live-preview collision bounds in room setup
}

#[allow(non_camel_case_types)]
pub enum EChaperoneImportFlags
{
	EChaperoneImport_BoundsOnly = 0x0001,
}

/** Manages the working copy of the chaperone info. By default this will be the same as the
* live copy. Any changes made with this interface will stay in the working copy until
* CommitWorkingCopy() is called, at which point the working copy and the live copy will be
* the same again. */
#[allow(non_camel_case_types, non_snake_case)]
pub struct VR_IVRCHaperoneSetup_FnTable
{
    /** Saves the current working copy to disk */
    pub CommitWorkingCopy: unsafe extern "stdcall" fn(eChaperoneConfigFile: EChaperoneConfigFile) -> bool,
    
    /** Reverts the working copy to match the live chaperone calibration.
	* To modify existing data this MUST be do WHILE getting a non-error ChaperoneCalibrationStatus.
	* Only after this should you do gets and sets on the existing data. */
	pub RevertWorkingCopy: unsafe extern "stdcall" fn(),

	/** Returns the width and depth of the Play Area (formerly named Soft Bounds) in X and Z from the working copy.
	* Tracking space center (0,0,0) is the center of the Play Area. */
	pub GetWorkingPlayAreaSize: unsafe extern "stdcall" fn(pSizeX: *mut f32, pSizeZ: *mut f32) -> bool,

	/** Returns the 4 corner positions of the Play Area (formerly named Soft Bounds) from the working copy.
	* Corners are in clockwise order.
	* Tracking space center (0,0,0) is the center of the Play Area.
	* It's a rectangle.
	* 2 sides are parallel to the X axis and 2 sides are parallel to the Z axis.
	* Height of every corner is 0Y (on the floor). **/
	pub GetWorkingPlayAreaRect: unsafe extern "stdcall" fn(rect: *mut HmdQuad) -> bool,

	/** Returns the number of Quads if the buffer points to null. Otherwise it returns Quads
	* into the buffer up to the max specified from the working copy. */
	pub GetWorkingCollisionBoundsInfo: unsafe extern "stdcall" fn(pQuadsBuffer: *mut HmdQuad, punQuadsCount: *mut u32) -> bool,

	/** Returns the number of Quads if the buffer points to null. Otherwise it returns Quads
	* into the buffer up to the max specified. */
	pub GetLiveCollisionBoundsInfo: unsafe extern "stdcall" fn(pQuadsBuffer: *mut HmdQuad, punQuadsCount: *mut u32) -> bool,

	/** Returns the preferred seated position from the working copy. */
	pub GetWorkingSeatedZeroPoseToRawTrackingPose: unsafe extern "stdcall" fn(pmatSeatedZeroPoseToRawTrackingPose: *mut HmdMatrix34) -> bool,

	/** Returns the standing origin from the working copy. */
	pub GetWorkingStandingZeroPoseToRawTrackingPose: unsafe extern "stdcall" fn(pmatStandingZeroPoseToRawTrackingPose: *mut HmdMatrix34) -> bool,

	/** Sets the Play Area in the working copy. */
	pub SetWorkingPlayAreaSize: unsafe extern "stdcall" fn(sizeX: f32, sizeZ: f32),

	/** Sets the Collision Bounds in the working copy. Note: ceiling height is ignored. */
	pub SetWorkingCollisionBoundsInfo: unsafe extern "stdcall" fn(pQuadsBuffer: *mut HmdQuad, unQuadsCount: u32),

	/** Sets the Collision Bounds in the working copy. */
	pub SetWorkingPerimeter: unsafe extern "stdcall" fn(pPointBuffer: *mut HmdVector2, unPointCount: u32),

	/** Sets the preferred seated position in the working copy. */
	pub SetWorkingSeatedZeroPoseToRawTrackingPose: unsafe extern "stdcall" fn(pMatSeatedZeroPoseToRawTrackingPose: *const HmdMatrix34),

	/** Sets the preferred standing position in the working copy. */
	pub SetWorkingStandingZeroPoseToRawTrackingPose: unsafe extern "stdcall" fn(pMatStandingZeroPoseToRawTrackingPose: *const HmdMatrix34),

	/** Tear everything down and reload it from the file on disk */
	pub ReloadFromDisk: unsafe extern "stdcall" fn(configFile: EChaperoneConfigFile),

	/** Returns the preferred seated position. */
	pub GetLiveSeatedZeroPoseToRawTrackingPose: unsafe extern "stdcall" fn(pmatSeatedZeroPoseToRawTrackingPose: *mut HmdMatrix34) -> bool,

	pub ExportLiveToBuffer: unsafe extern "stdcall" fn(pBuffer: *mut i8, pnBufferLength: *mut u32) -> bool,
	pub ImportFromBufferToWorking: unsafe extern "stdcall" fn(pBuffer: *const i8, nImportFlags: u32) -> bool,

	/** Shows the chaperone data in the working set to preview in the compositor.*/
	pub ShowWorkingSetPreview: unsafe extern "stdcall" fn(),

	/** Hides the chaperone data in the working set to preview in the compositor (if it was visible).*/
	pub HideWorkingSetPreview: unsafe extern "stdcall" fn(),

	/** Fire an event that the tracking system can use to know room setup is about to begin. This lets the tracking
	 * system make any last minute adjustments that should be incorporated into the new setup.  If the user is adjusting
	 * live in HMD using a tweak tool, keep in mind that calling this might cause the user to see the room jump. */
	pub RoomSetupStarting: unsafe extern "stdcall" fn(),
}

pub const IVRCHAPERONE_SETUP_VERSION: &'static str = "IVRChaperoneSetup_006";