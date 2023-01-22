use crate::body::RigidBody;
use vecmath::{Vector2, vec2_add, vec2_mul};


#[derive(Debug)]
pub struct PhysicsSimulation {
    pub gravity: Vector2<f64>,
} 

impl PhysicsSimulation {

    pub fn init_sim(gravity: [f64; 2]) -> Self {
        Self {
            gravity,
        }
    }
    
    pub fn step(&self, rigid_bodies: &mut Vec<&mut RigidBody>, dt: f64) {

        for rb in rigid_bodies {

            // F = m*g; v = v_0 + g*t; s = v_0*t + (g*t^2)/2

            // Calculate the new force
            rb.force.force = vec2_add(rb.force.force, self.gravity);

            // Calculate the new velocity
            rb.velocity.current_velocity = vec2_add(
                                                rb.velocity.current_velocity, 
                                                vec2_mul(
                                                    [dt, dt],
                                                    rb.force.force));

            // Calculate the new position
            rb.position.current_position = vec2_add(
                                            rb.position.current_position, 
                                            vec2_mul(
                                                rb.velocity.current_velocity,
                                                [dt/2.0, dt/2.0]));
            
            // Set the force back to zero; 
            rb.force.force = [0.0, 0.0];

        }
    }
}