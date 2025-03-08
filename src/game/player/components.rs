use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub target_position: Option<Vec3>,
}
