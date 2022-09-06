use crate::{
    mst::{
        texel::TexelID,
        utils::{index_to_global, texel_index_to_local},
    },
    resources::{Terrain, Time, Input, MouseButton, Camera},
    util::{Vector2I, Vector2F},
};
use specs::{Read, System, Write};

pub struct TerrainPainter;

impl<'a> TerrainPainter {
    fn paint_circle(&self, terrain: &mut Write<'a, Terrain>, origin: Vector2I, radius: u8, id: TexelID) {
        println!("painting! {}", origin);
        for y in origin.y - (radius as i32 - 1)..origin.y + radius as i32 {
            for x in origin.x - (radius as i32 - 1)..origin.x + radius as i32 {
                terrain.set_texel(&Vector2I {x, y}, id)
            }
        }
    }
}

impl<'a> System<'a> for TerrainPainter {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, Input>,
        Read<'a, Camera>,
        Write<'a, Terrain>
    );

    fn run(&mut self, (time, input, camera, mut terrain): Self::SystemData) {
        let mut updates: Vec<(Vector2I, TexelID)> = Vec::new();

        if input.mouse_pressed(MouseButton::Left) || input.mouse_held(MouseButton::Left) && input.get_mouse_velocity() != Vector2I::ZERO {
            // self.paint_circle(&mut terrain, (camera.transform.xform_inverse(Vector2F::from(input.get_mouse_position())) / camera.transform.get_scale()).rounded(), 6, 1);
            // self.paint_circle(&mut terrain, camera.transform.xform(Vector2F::from(input.get_mouse_position())).rounded(), 6, 1);
            // self.paint_circle(&mut terrain, camera.transform.xform_inverse(Vector2F::from(input.get_mouse_position())).rounded(), 6, 1);
            self.paint_circle(&mut terrain, input.get_mouse_position() / 4, 6, 1);
            // TODO: Fix scaled transforms, remove hardcoded values
        }

        // for (index, chunk) in terrain.chunk_iter() {
        //     for i in 0..chunk.texels.len() {
        //         let local = texel_index_to_local(i);
        //         let global = index_to_global(index) + local;
        //         // rng gen from crate rand was super slow, but even this is quite slow
        //         let rng = (time.lifetime.elapsed().unwrap().as_millis()
        //             % (time.frame + 1000) as u128)
        //             + (global.x * global.x + (global.y * 387)) as u128;
        //         if rng % 1000 < 5
        //             // && *index == (Vector2I { x: 1, y: 2 })
        //             && match terrain.global_to_texel(&global) {
        //                 Some(texel) => texel.is_empty(),
        //                 None => false,
        //             }
        //             && (match terrain.global_to_texel(&(global + Vector2I::UP)) {
        //                 Some(texel) => texel.id == 2,
        //                 None => false,
        //             } || match terrain.global_to_texel(&(global + Vector2I::DOWN)) {
        //                 Some(texel) => texel.id == 2,
        //                 None => false,
        //             } || match terrain.global_to_texel(&(global + Vector2I::LEFT)) {
        //                 Some(texel) => texel.id == 2,
        //                 None => false,
        //             } || match terrain.global_to_texel(&(global + Vector2I::RIGHT)) {
        //                 Some(texel) => texel.id == 2,
        //                 None => false,
        //             })
        //         {
        //             updates.push((global.to_owned(), 2));
        //         }
        //     }
        // }
        loop {
            match updates.pop() {
                Some((global, id)) => terrain.set_texel(&global, id),
                None => break,
            }
        }
    }
}
