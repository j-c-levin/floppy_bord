use bevy_xpbd_2d::prelude::*;

#[derive(PhysicsLayer)]
pub enum Layer {
    Bird,
    Rock,
    Score
}