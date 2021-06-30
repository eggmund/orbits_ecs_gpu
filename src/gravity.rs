use glsl_layout::{Std140, float, vec2, uint};

use bevy::prelude::*;

use crate::{ResultantForce, Mass};



#[derive(Clone, Copy, Default)]
struct GravityJob {
    position: vec2,
    output_force: vec2,
    mass: float,
    entity_id: uint,
}

pub fn gravity_job_sender_system(
    grav_query: Query<(
        &Entity,
        &Mass,
        &GlobalTransform,
    )>,
) {
    // Plan: Produce jobs for the GPU to complete. These jobs contain only the information required to calculate the gravitational pull, an index and an output variable (in this case `force`).
    for (entity, mass, transform) in grav_query.iter() {
        let job = GravityJob {
            position: [transform.translation.x, transform.translation.y].into(),
            output_force: vec2::default(),
            mass: mass.0,
            entity_id: entity.id(),
        };

        
    }
}

pub fn gravity_job_receiver_system(
    force_query: Query<(
        &mut ResultantForce,
        &Mass,
    )>,
) {

}