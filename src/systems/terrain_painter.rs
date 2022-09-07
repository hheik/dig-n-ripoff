use crate::{
    mst::texel::TexelID,
    resources::{Camera, Input, MouseButton, Terrain},
    util::Vector2I,
};
use specs::{Read, System, Write};

pub struct TerrainPainter {
    radius: i32,
}

impl TerrainPainter {
    pub fn new() -> TerrainPainter {
        TerrainPainter { radius: 6 }
    }
}

impl<'a> TerrainPainter {
    fn paint_circle(
        &self,
        terrain: &mut Write<'a, Terrain>,
        origin: Vector2I,
        radius: i32,
        id: TexelID,
    ) {
        for y in origin.y - (radius - 1)..origin.y + radius {
            for x in origin.x - (radius - 1)..origin.x + radius {
                let dx = (x - origin.x).abs();
                let dy = (y - origin.y).abs();
                if dx * dx + dy * dy <= (radius - 1) * (radius - 1) {
                    terrain.set_texel(&Vector2I { x, y }, id)
                }
            }
        }
    }

    fn mouse_to_world_pos(camera: &Camera, mouse_position: Vector2I) -> Vector2I {
        (camera.transform.get_position().rounded() + mouse_position) / 4 // FIXME: harcoded value
    }
}

impl<'a> System<'a> for TerrainPainter {
    type SystemData = (Read<'a, Input>, Read<'a, Camera>, Write<'a, Terrain>);

    fn run(&mut self, (input, camera, mut terrain): Self::SystemData) {
        let mut updates: Vec<(Vector2I, TexelID)> = Vec::new();

        self.radius = (self.radius + input.get_mouse_scroll().y).clamp(1, 128);

        // TODO: Fix scaled transforms, remove hardcoded values
        let brush_pos = Self::mouse_to_world_pos(&camera, input.get_mouse_position());
        if input.mouse_held(MouseButton::Left) {
            self.paint_circle(&mut terrain, brush_pos, self.radius, 1)
        }

        if input.mouse_held(MouseButton::Right) {
            self.paint_circle(&mut terrain, brush_pos, self.radius, 0)
        }

        loop {
            match updates.pop() {
                Some((global, id)) => terrain.set_texel(&global, id),
                None => break,
            }
        }
    }
}
