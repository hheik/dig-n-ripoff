use std::collections::LinkedList;

use u64 as PointInTime;

struct ChangeEvent<T> {
    point_in_time: PointInTime,
    event: T,
}

pub struct ChangeBuffer<T> {
    /// Running counter for the point-in-time
    current_pit: PointInTime,
    events: LinkedList<ChangeEvent<T>>,
}

impl<T> ChangeBuffer<T> {
    pub fn new() -> Self {
        ChangeBuffer {
            events: LinkedList::new(),
            current_pit: 0,
        }
    }

    pub fn get_lister(&self) -> PointInTime {
        self.current_pit
    }

    pub fn push_event(&mut self, event: T) {
        self.events.push_back(ChangeEvent {
            point_in_time: self.current_pit,
            event,
        });
        self.current_pit += 1;
    }

    pub fn get_changes(&mut self)
}
