use super::chunk::Chunk;
use crate::util::{Segment2I, Vector2F, Vector2I};
use lazy_static::lazy_static;
use std::collections::VecDeque;

type Island = VecDeque<Segment2I>;

lazy_static! {
    /// Marching Square case dictionary.
    ///
    /// Key is a bitmask of neighbouring tiles (up, right, down, left - least significant bit first).
    /// Bit set to 1 means that the neighbour has collision. Only the 4 least significant bits are currently used.
    ///
    /// Value is an array of segments that the tile should have. The segments are configured to go clockwise.
    ///
    /// Note: This dictionary should only be used for empty tiles.
    static ref MST_CASE_MAP: [Vec<Segment2I>; 16] = [
        /* 0b0000 */ vec![],
        /* 0b0001 */ vec![ Segment2I { from: Vector2I::RIGHT, to: Vector2I::ZERO } ],
        /* 0b0010 */ vec![ Segment2I { from: Vector2I::ONE, to: Vector2I::RIGHT } ],
        /* 0b0011 */ vec![ Segment2I { from: Vector2I::ONE, to: Vector2I::ZERO } ],
        /* 0b0100 */ vec![ Segment2I { from: Vector2I::DOWN, to: Vector2I::ONE } ],
        /* 0b0101 */ vec![ Segment2I { from: Vector2I::RIGHT, to: Vector2I::ZERO }, Segment2I { from: Vector2I::DOWN, to: Vector2I::ONE } ],
        /* 0b0110 */ vec![ Segment2I { from: Vector2I::DOWN, to: Vector2I::RIGHT } ],
        /* 0b0111 */ vec![ Segment2I { from: Vector2I::DOWN, to: Vector2I::ZERO } ],
        /* 0b1000 */ vec![ Segment2I { from: Vector2I::ZERO, to: Vector2I::DOWN } ],
        /* 0b1001 */ vec![ Segment2I { from: Vector2I::RIGHT, to: Vector2I::DOWN } ],
        /* 0b1010 */ vec![ Segment2I { from: Vector2I::ONE, to: Vector2I::RIGHT }, Segment2I { from: Vector2I::ZERO, to: Vector2I::DOWN } ],
        /* 0b1011 */ vec![ Segment2I { from: Vector2I::ONE, to: Vector2I::DOWN } ],
        /* 0b1100 */ vec![ Segment2I { from: Vector2I::ZERO, to: Vector2I::ONE } ],
        /* 0b1101 */ vec![ Segment2I { from: Vector2I::RIGHT, to: Vector2I::ONE } ],
        /* 0b1110 */ vec![ Segment2I { from: Vector2I::ZERO, to: Vector2I::RIGHT } ],
        /* 0b1111 */ vec![],
    ];

    /// Version of the MS case dictionary that is used by the solid tiles at the edge of the chunk
    static ref MST_EDGE_CASE_MAP: [Segment2I; 4] = [
        /* up    */ Segment2I { from: Vector2I::ZERO, to: Vector2I::RIGHT },
        /* right */ Segment2I { from: Vector2I::RIGHT, to: Vector2I::ONE },
        /* down  */ Segment2I { from: Vector2I::ONE, to: Vector2I::DOWN },
        /* left  */ Segment2I { from: Vector2I::DOWN, to: Vector2I::ZERO },
    ];
}

pub fn calculate_collisions(chunk: &Chunk) -> Vec<Vec<Vector2F>> {
    let mut islands: Vec<Island> = Vec::new();
    for i in 0..chunk.texels.len() {
        let local = Vector2I {
            x: i as i32 % Chunk::SIZE.x,
            y: i as i32 / Chunk::SIZE.y,
        };

        let edge_mask: u8 = if local.y == 0 { 1 << 0 } else { 0 }
            | if local.x == Chunk::SIZE.x { 1 << 1 } else { 0 }
            | if local.y == Chunk::SIZE.y { 1 << 2 } else { 0 }
            | if local.x == 0 { 1 << 3 } else { 0 };

        let mut sides: Vec<Segment2I>;
        if chunk.texels[i].is_empty() {
            sides = MST_CASE_MAP[chunk.texels[i].neighbour_mask as usize]
                .iter()
                .clone()
                .map(|side| Segment2I {
                    from: side.from + local,
                    to: side.to + local,
                })
                .collect();
        } else if chunk.texels[i].is_empty() && edge_mask != 0 {
            sides = Vec::with_capacity(Chunk::SIZE_X * 2 + Chunk::SIZE_Y * 2);
            for i in 0..MST_EDGE_CASE_MAP.len() {
                if edge_mask & (1 << i) != 0 {
                    let edge = MST_EDGE_CASE_MAP[i];
                    sides.push(Segment2I {
                        from: edge.from + local,
                        to: edge.to + local,
                    })
                }
            }
        } else {
            continue;
        }

        for side in sides {
            // Check if the side can be attached to any island
            // The naming of front and back are kind of misleading, and come from the VecDeque type.
            // You can think of the front as the beginning of the island loop, and back the end.

            // Connect to an island if possible, otherwise create a new island
            {
                let mut connected_to: Option<&mut Island> = None;
                for island in islands.iter_mut() {
                    if island.back().is_some() && island.back().unwrap().to == side.from {
                        connected_to = Some(island);
                    }
                }

                match connected_to {
                    Some(back) => {
                        back.push_back(side);
                    }
                    None => {
                        let mut island: Island = Island::new();
                        island.push_back(side);
                        islands.push(island);
                    }
                }
            }

            // Find connected islands
            let mut merge_index: Option<usize> = None;
            'outer: for i in 0..islands.len() {
                for j in 0..islands.len() {
                    if i == j {
                        continue;
                    }
                    if islands[i].back().is_some()
                        && islands[j].front().is_some()
                        && islands[i].back().unwrap().to == islands[j].front().unwrap().from
                    {
                        merge_index = Some(i);
                        break 'outer;
                    }
                }
            }

            // Merge connected islands
            match merge_index {
                Some(index) => {
                    let mut merge_from = islands.swap_remove(index);
                    match islands.iter_mut().find(|island| match island.front() {
                        Some(front) => front.from == merge_from.back().unwrap().to,
                        None => false,
                    }) {
                        Some(merge_to) => loop {
                            match merge_from.pop_front() {
                                Some(segment) => merge_to.push_front(segment),
                                None => break,
                            }
                        },
                        None => (),
                    };
                }
                None => (),
            }
        }
    }

    let mut result: Vec<Vec<Vector2F>> = Vec::with_capacity(islands.len());
    for island in islands {
        if island.len() < 4 {
            continue;
        }
        let mut points: Vec<Vector2F> = Vec::with_capacity(island.len() + 1);
        points.push(Vector2F::from(island.front().unwrap().from));
        let mut current_angle: Option<f32> = None;
        for side in island {
            if current_angle.is_some() && (current_angle.unwrap() - side.angle()).abs() < 0.1 {
                let len = points.len();
                points[len - 1] = Vector2F::from(side.to)
            } else {
                current_angle = Some(side.angle());
                points.push(Vector2F::from(side.to));
            }
        }
        result.push(points);
    }
    result
}
