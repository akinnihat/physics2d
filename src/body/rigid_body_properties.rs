use vecmath::Vector2;


#[derive(Debug, Clone, Copy)]
pub struct RigidBodyPosition { 
    // Current position of the 2d rigid body
    pub initial_position: Vector2<f64>,       // x and y-axis in cartesian coordinate system
    pub current_position: Vector2<f64>,
}

impl Default for RigidBodyPosition {
    fn default() -> Self {
        RigidBodyPosition { initial_position: ([0.0, 0.0]), current_position: ([0.0, 0.0]) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RigidBodyRotation { 
    // Current rotation of the 2d rigid body
    pub initial_rotation: f64,              // Angle in radians
    pub current_rotation: f64, 
}

impl Default for RigidBodyRotation {
    fn default() -> Self {
        RigidBodyRotation { initial_rotation: (0.0), current_rotation: (0.0) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RigidBodyVelocity {
    pub initial_velocity: Vector2<f64>,       // x and y speeds in cartesian coordinate system
    pub current_velocity: Vector2<f64>,
} 

impl Default for RigidBodyVelocity {
    fn default() -> Self {
        RigidBodyVelocity { initial_velocity: ([0.0, 0.0]), current_velocity: ([0.0, 0.0]) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RigidBodyMass {
    pub mass: Vector2<f64>,
}

impl Default for RigidBodyMass {
    fn default() -> Self {
        RigidBodyMass { mass: ([1.0, 0.0]) }
    }
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

#[derive(Debug, Clone, Copy)]
pub struct RigidBodyShape {
    pub shape: Shape,
    pub drag_coefficient: f64,
}

impl Default for RigidBodyShape {
    fn default() -> Self {
        RigidBodyShape { shape: (Shape::Rectangle(10.0, 10.0)), drag_coefficient: super::RECT_DRAG_COEFF }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RigidBodyForce {
    pub force: Vector2<f64>,
}

impl Default for RigidBodyForce {
    fn default() -> Self {
        RigidBodyForce { force: ([0.0, 0.0]) }
    }
}