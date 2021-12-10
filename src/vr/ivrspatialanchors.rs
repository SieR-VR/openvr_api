use super::{vrtypes::*, public_vrtypes:: *};

pub const INVALID_SPATIAL_ANCHOR_HANDLE: SpatialAnchorHandle = 0;

pub struct SpatialAnchorPose
{
    pub anchor_to_absolute_tracking: HmdMatrix34,
}

#[allow(non_camel_case_types, non_snake_case)]
pub struct VR_IVRSpatialAnchors_FnTable
{
    /** Returns a handle for an spatial anchor described by "descriptor".  On success, pHandle
    * will contain a handle valid for this session.  Caller can wait for an event or occasionally
    * poll GetSpatialAnchorPose() to find the virtual coordinate associated with this anchor. */
    pub CreateSpatialAnchorFromDescriptor: unsafe extern "stdcall" fn( 
        pchDescriptor: *const i8, 
        pHandleOut: *mut SpatialAnchorHandle) -> EVRSpatialAnchorError,

    /** Returns a handle for an new spatial anchor at pPose.  On success, pHandle
    * will contain a handle valid for this session.  Caller can wait for an event or occasionally
    * poll GetSpatialAnchorDescriptor() to find the permanent descriptor for this pose.
    * The result of GetSpatialAnchorPose() may evolve from this initial position if the driver chooses
    * to update it.
    * The anchor will be associated with the driver that provides unDeviceIndex, and the driver may use that specific
    * device as a hint for how to best create the anchor.
    * The eOrigin must match whatever tracking origin you are working in (seated/standing/raw).
    * This should be called when the user is close to (and ideally looking at/interacting with) the target physical
    * location.  At that moment, the driver will have the most information about how to recover that physical point
    * in the future, and the quality of the anchor (when the descriptor is re-used) will be highest.
    * The caller may decide to apply offsets from this initial pose, but is advised to stay relatively close to the
    * original pose location for highest fidelity. */
    pub CreateSpatialAnchorFromPose: unsafe extern "stdcall" fn( 
        unDeviceIndex: TrackedDeviceIndex, 
        eOrigin: ETrackingUniverseOrigin, 
        pPose: *mut SpatialAnchorPose, 
        pHandleOut: *mut SpatialAnchorHandle) -> EVRSpatialAnchorError,

    /** Get the pose for a given handle.  This is intended to be cheap enough to call every frame (or fairly often)
    * so that the driver can refine this position when it has more information available. */
    pub GetSpatialAnchorPose: unsafe extern "stdcall" fn( 
        unHandle: SpatialAnchorHandle, 
        eOrigin: ETrackingUniverseOrigin, 
        pPoseOut: *mut SpatialAnchorPose) -> EVRSpatialAnchorError,

    /** Get the descriptor for a given handle.  This will be empty for handles where the driver has not
    * yet built a descriptor.  It will be the application-supplied descriptor for previously saved anchors
    * that the application is requesting poses for.  If the driver has called UpdateSpatialAnchorDescriptor()
    * already in this session, it will be the descriptor provided by the driver.
    * Returns true if the descriptor fits into the buffer, else false.  Buffer size should be at least
    * k_unMaxSpatialAnchorDescriptorSize. */
    pub GetSpatialAnchorDescriptor: unsafe extern "stdcall" fn(
        unHandle: SpatialAnchorHandle, 
        pchDescriptorOut: *mut i8,
        punDescriptorBufferLenInOut: u32) -> EVRSpatialAnchorError,

}

pub const IVRSPATIALANCHORS_VERSION: &'static str = "IVRSpatialAnchors_001";