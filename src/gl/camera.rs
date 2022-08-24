use crate::{util::Vector2};

#[derive(Default)]
pub struct Camera {
    pub position: Vector2<f32>,
    pub scale: f32,
}
