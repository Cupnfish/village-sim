use bevy::{
    math::{Vec2, Vec3},
    prelude::*,
};

use crate::{
    loading::Materials,
    sprite_helpers::spawn_sprite_bundles_,
    village::{Building, LivingSpace, LivingSpaceAvailableEvent},
    world_gen::SimParams,
};

pub fn spawn_house(
    commands: &mut Commands,
    materials: &Res<Materials>,
    pos: Vec2,
    sim_params: &Res<SimParams>,
    max_people: u32,
    ev_living_space_available: &mut EventWriter<LivingSpaceAvailableEvent>,
) {
    let bounding_box = Vec3::new(40.0, 30.0, 40.0);
    let residence_id = spawn_sprite_bundles_(
        commands,
        Vec3::ONE,
        pos,
        bounding_box,
        materials.house.clone(),
        materials.shadow.clone(),
        sim_params.world_rect.size,
        Vec2::new(0.0, -10.0),
    )
    .insert(Building)
    .insert(LivingSpace {
        current_people: 0,
        max_people,
    })
    .id();

    ev_living_space_available.send(LivingSpaceAvailableEvent { residence_id })
}
