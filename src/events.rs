use std::cmp::Reverse;

use crate::request::Request;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum EventType {
    Arrival,
    Departure,
}

///! Represents event in the system.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Event {
    ///! Time to which event is scheduled.
    ///!
    ///! If event is `Arrival` then it is time of arrival.
    ///! If event is `Departure` then it is time of departure.
    pub time: f64,
    ///! Request to which event is related.
    ///!
    ///! If event is `Arrival` then it is request which is arriving.
    ///! If event is `Departure` then it is request which is departing.
    pub request: Request,
    ///! Type of the event.
    pub r#type: EventType,
}

impl Eq for Event {}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.time > other.time {
            std::cmp::Ordering::Less
        } else if self.time < other.time {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

/// Represents queue of events in the system.
///
/// Yields nearest event to the current time.
#[derive(Debug, Default)]
pub struct EventsQueue {
    pub(crate) heap: std::collections::BinaryHeap<Reverse<Event>>,
}

impl EventsQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: Event) {
        self.heap.push(Reverse(event));
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.heap.pop().map(|Reverse(event)| event)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

// #[cfg(test)]
// mod tests {
//     use once_cell::sync::Lazy;

//     use super::*;

//     static REQUESTS: Lazy<[Request; 4]> = Lazy::new(|| {
//         [
//             Request::new(5.0, 10.0),
//             Request::new(5.0, 15.0),
//             Request::new(10.0, 25.0),
//             Request::new(10.0, 30.0),
//         ]
//     });

//     static EVENTS: Lazy<[Event; 4]> = Lazy::new(|| {
//         [
//             Event {
//                 time: 5,
//                 request: REQUESTS[0].clone(),
//                 r#type: EventType::Arrival,
//             },
//             Event {
//                 time: 6,
//                 request: REQUESTS[1].clone(),
//                 r#type: EventType::Departure,
//             },
//             Event {
//                 time: 10,
//                 request: REQUESTS[2].clone(),
//                 r#type: EventType::Departure,
//             },
//             Event {
//                 time: 11,
//                 request: REQUESTS[3].clone(),
//                 r#type: EventType::Arrival,
//             },
//         ]
//     });

//     #[test]
//     fn test_events_queue_push_pop() {
//         let mut events = EventsQueue::new();
//         assert_eq!(events.pop(), None);

//         let event1 = EVENTS[0].clone();
//         let event2 = EVENTS[1].clone();
//         let event3 = EVENTS[2].clone();
//         let event4 = EVENTS[3].clone();

//         events.push(event1.clone());
//         events.push(event2.clone());
//         events.push(event3.clone());
//         events.push(event4.clone());

//         assert_eq!(events.is_empty(), false);
//         assert_eq!(events.pop(), Some(event1), "heap: {:?}", events.heap);
//         assert_eq!(events.pop(), Some(event2), "heap: {:?}", events.heap);
//         assert_eq!(events.pop(), Some(event3));
//         assert_eq!(events.pop(), Some(event4));
//         assert_eq!(events.pop(), None);
//     }

//     #[test]
//     fn test_events_queue_ordering() {
//         let event1 = EVENTS[0].clone();
//         let event2 = EVENTS[1].clone();
//         let event3 = EVENTS[2].clone();
//         let event4 = EVENTS[3].clone();

//         let mut events = vec![
//             event1.clone(),
//             event2.clone(),
//             event3.clone(),
//             event4.clone(),
//         ];
//         events.sort();

//         assert_eq!(events[0], event1);
//         assert_eq!(events[1], event2);
//         assert_eq!(events[2], event3);
//         assert_eq!(events[3], event4);
//     }
// }
