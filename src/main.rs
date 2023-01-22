pub mod body;
pub mod physics;

use crate::body::RigidBody;
use crate::physics::PhysicsSimulation;

fn main() {

    // ---- Test ----

    let mut rb1: RigidBody = RigidBody::new_rectangle(
                                            "Rectangle_1", [1.2, 3.3], 5.4, 
                                            [0.0, 0.0], [2.0, 3.3], 4.6, 4.5);

    let mut rb2: RigidBody = RigidBody::new_circle(
                                            "Circle_1", [8.2, 6.3], 1.1,
                                            [0.0, 0.0], [2.0, 9.3], 1.6);

    let rigid_bodies: &mut Vec<&mut RigidBody> = &mut Vec::new();

    rigid_bodies.push(&mut rb1);
    rigid_bodies.push(&mut rb2);

    let gravity: [f64; 2] = [0.0, -9.81];

    let ps = &PhysicsSimulation::init_sim(gravity);

    println!("\n\nInitial positions:");
    for j in 0..rigid_bodies.len() {
        println!("\tName: {:?} \n\tPosition {:?}",  &rigid_bodies[j].name, 
                                                &rigid_bodies[j].position.current_position);
    }

    let _ = &ps.step(rigid_bodies, 10.0);

    println!("\n\nAfter 10 seconds of free fall: ");
    for j in 0..rigid_bodies.len() {
        println!("\tName: {:?} \n\tPosition {:?}",  &rigid_bodies[j].name, 
                                                &rigid_bodies[j].position.current_position);
    }

}