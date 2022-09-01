use super::chunk::Chunk;
use crate::util::{Segment2I, Vector2I};
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
}

pub fn calculate_collisions(chunk: &Chunk) -> Vec<Island> {
    let mut islands: Vec<Island> = Vec::new();
    for i in 0..chunk.texels.len() {
        if !chunk.texels[i].is_empty() {
            continue; // Only process empty tiles
        }
        let local = Vector2I {
            x: i as i32 % Chunk::SIZE.x,
            y: i as i32 / Chunk::SIZE.y,
        };

        let sides = MST_CASE_MAP[chunk.texels[i].neighbour_mask as usize]
            .iter()
            .clone()
            .map(|side| Segment2I {
                from: side.from + local,
                to: side.to + local,
            });
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

    let mut result: Vec<Island> = Vec::new();
    // NOTIMP
    result
}
