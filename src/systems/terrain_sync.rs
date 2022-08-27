use std::collections::HashMap;

use crate::resources::Terrain;

use specs::{Join, Read, System};

pub struct TerrainSync;
impl<'a> System<'a> for TerrainSync {
    type SystemData = (Read<'a, Terrain>,);

    fn run(&mut self, (terrain): Self::SystemData) {}
}
