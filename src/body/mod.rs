pub use self::rigid_body::RigidBody;
pub use self::rigid_body_properties::{RigidBodyPosition, RigidBodyRotation, RigidBodyVelocity, RigidBodyMass, RigidBodyShape, Shape, RigidBodyForce};

pub mod rigid_body;
pub mod rigid_body_properties;

const RECT_DRAG_COEFF: f64 = 2.05;
const CIRC_DRAG_COEFF: f64 = 1.17;