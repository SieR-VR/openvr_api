pub type HmdMatrix34 = [[f32; 4]; 3];
pub type HmdMatrix33 = [[f32; 3]; 3];
pub type HmdMatrix44 = [[f32; 4]; 4];

pub type HmdVector2 = [f32; 2];
pub type HmdVector3 = [f32; 3];
pub type HmdVector4 = [f32; 4];
pub type HmdVector3d = [f64; 3];

pub struct HmdQuaternion {
  pub w: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

pub struct HmdQuaternionf {
  pub w: f32,
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

pub struct HmdColor {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

pub type HmdQuad = [HmdVector3; 4];

pub struct HmdRect2 {
  pub top_left: HmdVector2,
  pub bottom_right: HmdVector2,
}

pub struct VRBoneTransform {
  pub position: HmdVector4,
  pub orientation: HmdQuaternionf,
}

