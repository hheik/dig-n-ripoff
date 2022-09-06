use std::collections::LinkedList;

pub use u64 as Listener;

struct ChangeEvent<T> {
    point_in_time: Listener,
    event: T,
}

#[derive(Default)]
pub struct ChangeBuffer<T> {
    /// Running counter for the point-in-time
    current_point_in_time: Listener,
    listeners: Vec<Listener>,
    events: LinkedList<ChangeEvent<T>>,
}

impl<T> ChangeBuffer<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        ChangeBuffer {
            events: LinkedList::new(),
            listeners: Vec::new(),
            current_point_in_time: 0,
        }
    }

    pub fn get_listener(&mut self) -> Listener {
        let listener = self.current_point_in_time;
        self.listeners.push(listener);
        self.current_point_in_time += 1;
        listener
    }

    pub fn push_event(&mut self, event: T) {
        if self.listeners.len() == 0 {
            return;
        }
        self.current_point_in_time += 1;
        self.events.push_back(ChangeEvent {
            point_in_time: self.current_point_in_time,
            event,
        });
    }

    pub fn consume_listener(&mut self, listener: Listener) -> Option<Vec<T>> {
        let index = match self.listeners.iter().position(|l| *l == listener) {
            Some(index) => index,
            None => return None,
        };
        self.listeners.swap_remove(index);

        let result = Some(
            self.events
                .iter()
                .clone()
                .filter_map(|event| {
                    if event.point_in_time > listener {
                        Some(event.event.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        );

        self.cleanup();

        result
    }

    /// Remove events that aren't newer than the oldest listener
    fn cleanup(&mut self) {
        let oldest = self.listeners.iter().min();
        match oldest {
            Some(oldest) => loop {
                let first = match self.events.front() {
                    Some(first) => first,
                    None => break,
                };
                if first.point_in_time <= *oldest {
                    self.events.pop_front();
                } else {
                    break;
                }
            },
            None => self.events.clear(),
        }
        if self.listeners.len() == 0 && self.events.len() == 0 {
            self.current_point_in_time = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChangeBuffer;

    #[test]
    fn consume_listener() {
        let mut buffer: ChangeBuffer<u8> = ChangeBuffer::new();

        assert_eq!(buffer.current_point_in_time, 0);
        assert_eq!(buffer.events.len(), 0);
        assert_eq!(buffer.listeners.len(), 0);

        let listener1 = buffer.get_listener();
        buffer.push_event(50);

        assert_eq!(listener1, 0);
        assert_eq!(buffer.current_point_in_time, 1);

        let listener2 = buffer.get_listener();
        buffer.push_event(60);
        buffer.push_event(70);

        assert_eq!(listener2, 1);
        assert_eq!(buffer.current_point_in_time, 3);

        let changes1 = buffer.consume_listener(listener1);

        assert_eq!(buffer.events.len(), 2);

        let changes2 = buffer.consume_listener(listener2);

        assert_eq!(buffer.events.len(), 0);
        assert_eq!(changes1, Some(vec![50, 60, 70]));
        assert_eq!(changes2, Some(vec![60, 70]));

        // Cleanup also resetted current point in time
        assert_eq!(buffer.current_point_in_time, 0);
    }
}
