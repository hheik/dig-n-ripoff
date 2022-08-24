use super::Vector2;

pub type Vector2I = Vector2<i32>;

impl Vector2<f32> {
    pub fn rounded(&self) -> Vector2I {
        Vector2I {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }
}
