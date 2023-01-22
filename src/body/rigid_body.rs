use vecmath::Vector2;

use crate::body::{RigidBodyPosition, RigidBodyRotation, RigidBodyVelocity, RigidBodyMass, RigidBodyShape, Shape, 
    RigidBodyForce, CIRC_DRAG_COEFF, RECT_DRAG_COEFF};

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub name: String,
    pub position: RigidBodyPosition,
    pub rotation: RigidBodyRotation,
    pub velocity: RigidBodyVelocity,
    pub mass: RigidBodyMass,
    pub shape: RigidBodyShape,
    pub force: RigidBodyForce,
}

impl RigidBody {

    pub fn new() -> Self {
        println!("New rigid body has been created.");
        
        Self {
            name: ToString::to_string("RigidBody"),
            position: RigidBodyPosition::default(),
            rotation: RigidBodyRotation::default(),
            velocity: RigidBodyVelocity::default(),
            mass: RigidBodyMass::default(),
            shape: RigidBodyShape::default(),
            force: RigidBodyForce::default(),
        }
    }

    pub fn new_circle(name: &str, position: Vector2<f64>, rotation: f64, velocity: Vector2<f64>, mass: Vector2<f64>, radius: f64) -> Self {
        println!("New rigid body '{}' has been created.", name);
        println!("Position: {:?}, Rotation: {}, Velocity: {:?}, Mass: {:?}, Radius: {}", position, rotation, velocity, mass, radius);
        
        Self {
            name: ToString::to_string(name),
            position: RigidBodyPosition { initial_position: (position), current_position: (position) },
            rotation: RigidBodyRotation { initial_rotation: (rotation), current_rotation: (rotation) },
            velocity: RigidBodyVelocity { initial_velocity: (velocity), current_velocity: (velocity) },
            mass: RigidBodyMass { mass: (mass) },
            shape: RigidBodyShape { shape: (Shape::Circle(radius)), drag_coefficient: CIRC_DRAG_COEFF },
            force: RigidBodyForce::default(),
        }
    }

    pub fn new_rectangle(name: &str, position: Vector2<f64>, rotation: f64, velocity: Vector2<f64>, mass: Vector2<f64>, width: f64, height: f64) -> Self {
        println!("New rigid body '{}' has been created.", name);
        println!("Position: {:?}, Rotation: {}, Velocity: {:?}, Mass: {:?}, Width: {}, Heigth: {}", position, rotation, velocity, mass, width, height);
        
        Self {
            name: ToString::to_string(name),
            position: RigidBodyPosition { initial_position: (position), current_position: (position) },
            rotation: RigidBodyRotation { initial_rotation: (rotation), current_rotation: (rotation) },
            velocity: RigidBodyVelocity { initial_velocity: (velocity), current_velocity: (velocity) },
            mass: RigidBodyMass { mass: (mass) },
            shape: RigidBodyShape { shape: (Shape::Rectangle(width, height)), drag_coefficient: RECT_DRAG_COEFF },
            force: RigidBodyForce::default(),
        }
    }
}