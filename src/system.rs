use std::collections::VecDeque;

use crate::{
    distributions::{ConsumingDistribution, ProducingDistribution},
    events::{Event, EventType, EventsQueue},
    request::Request,
};

/// Repsenets imitating model if **Queueing System**.
#[derive(Debug)]
pub struct System {
    current_tick: f64,
    nodes_number: usize,
    nodes_busy: usize,

    events_queue: EventsQueue,
    queue: VecDeque<Request>,

    request_finish_dsrt: ConsumingDistribution,
    request_arrival_dsrt: ProducingDistribution,

    finished_requests: Vec<Request>,
}

#[derive(Debug, Default)]
pub struct Stats {
    /// Current tick
    pub current_tick: f64,
    /// Sum of requests in queue + requests that are being processed.
    pub requests_in_system: usize,
    /// Finished requests:
    pub finished_requests: Vec<Request>,
}

impl System {
    /// Creates new [`System`] instance.
    pub fn new(
        nodes_number: usize,
        queue_capacity: usize,
        request_finish_dsrt: ConsumingDistribution,
        request_arrival_dsrt: ProducingDistribution,
    ) -> Self {
        Self {
            current_tick: 0.0,
            nodes_busy: 0,
            events_queue: EventsQueue::new(),
            queue: VecDeque::with_capacity(queue_capacity),
            finished_requests: Vec::new(),
            nodes_number,
            request_finish_dsrt,
            request_arrival_dsrt,
        }
    }

    pub fn next(&mut self) -> Stats {
        if self.events_queue.is_empty() {
            self.produce_arrival();
        }

        let event = self
            .events_queue
            .pop()
            .expect("Events queue should not be empty");

        self.handle_event(event);

        if self.nodes_busy < self.nodes_number {
            if let Some(mut request) = self.queue.pop_front() {
                self.nodes_busy += 1;
                request.started_at = Some(self.current_tick);
                self.produce_departure(request);
            }
        }

        log::debug!("Events: {:?}", self.events_queue);
        log::debug!("Queue: {:?}", self.queue);

        let stats = Stats {
            current_tick: self.current_tick,
            requests_in_system: self.queue.len() + self.nodes_busy,
            finished_requests: self.finished_requests.clone(),
        };

        log::debug!("Stats: {:?}", stats);

        stats
    }

    fn handle_event(&mut self, event: Event) {
        let Event {
            time,
            mut request,
            r#type,
        } = event;

        match r#type {
            EventType::Arrival => {
                self.current_tick = time;

                self.produce_arrival();

                if self.queue.capacity() == self.queue.len() {
                    return;
                }
                request.created_at = Some(self.current_tick);
                self.queue.push_back(request);
            }
            EventType::Departure => {
                self.current_tick = time;

                self.finished_requests.push(request);
                self.nodes_busy -= 1;

                let Some(mut request) = self.queue.pop_front() else {
                    return; // Skip if queue is empty
                };
                request.started_at = Some(self.current_tick);

                self.nodes_busy += 1;
                self.produce_departure(request);
            }
        }
    }

    fn produce_arrival(&mut self) {
        let request_arrival =
            self.current_tick + self.request_arrival_dsrt.sample(&mut rand::thread_rng());

        let request = self.new_request();

        self.events_queue.push(Event {
            time: request_arrival,
            request,
            r#type: EventType::Arrival,
        });
    }

    fn new_request(&mut self) -> Request {
        let time_to_finish = self.request_finish_dsrt.sample(&mut rand::thread_rng());

        Request::new(time_to_finish)
    }

    fn produce_departure(&mut self, request: Request) {
        self.events_queue.push(Event {
            time: self.current_tick + request.time_to_finish,
            request,
            r#type: EventType::Departure,
        });
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     /// The producing is one tick, and consuming is two ticks.
//     #[test]
//     fn test_system_next() {
//         let mut system = System::new(
//             2,
//             10,
//             ConsumingDistribution::Degenerate { μ: 500.0 }, // for each it will take 2 ticks
//             ProducingDistribution::Degenerate { value: 1 },
//         );
//         system.next();
//         assert_eq!(system.current_tick, 1.0);
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 1);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             2,
//             "first departure, and second arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );
//         assert_eq!(system.finished_requests.len(), 0);

//         system.next();
//         assert_eq!(system.current_tick, 2.0);
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 2);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             3,
//             "two departures, and one arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );
//         assert_eq!(system.finished_requests.len(), 0);

//         system.next();
//         assert_eq!(system.current_tick, 3.0, "first departure processed");
//         assert_eq!(system.nodes_busy, 1, "first deprature must be finished");
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.finished_requests.len(), 1);

//         system.next();
//         assert_eq!(system.current_tick, 3.0, "third arrival processed");
//         assert_eq!(system.nodes_busy, 2, "arrival -> departure");
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.finished_requests.len(), 1);
//     }

//     /// The producing is one tick, and consuming is 5 ticks.
//     #[test]
//     fn test_sysmtem_queue_filled() {
//         let mut system = System::new(
//             2,                                               // 2 nodes
//             3,                                               // queue length is 3
//             ConsumingDistribution::Degenerate { μ: 100.0 }, // for each it will take 10 ticks
//             ProducingDistribution::Degenerate { value: 1 },
//         );

//         system.next();
//         assert_eq!(system.current_tick, 1.0);
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 1);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             2,
//             "first departure, and second arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );

//         system.next();
//         assert_eq!(system.current_tick, 2.0);
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 2);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             3,
//             "two departures, and one arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );

//         system.next();
//         assert_eq!(system.current_tick, 3.0);
//         assert_eq!(system.queue.len(), 1);
//         assert_eq!(system.nodes_busy, 2);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             3,
//             "two departures, and two arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );

//         system.next();
//         assert_eq!(system.current_tick, 4.0);
//         assert_eq!(system.queue.len(), 2);
//         assert_eq!(system.nodes_busy, 2);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             3,
//             "two departures, and three arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );

//         system.next();
//         assert_eq!(system.current_tick, 5.0);
//         assert_eq!(system.queue.len(), 3);
//         assert_eq!(system.nodes_busy, 2);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             3,
//             "two departures, and four arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );
//     }

//     /// The producing is one tick, and consuming is 5 ticks.
//     #[test]
//     fn test_system_all_busy_nodes() {
//         let mut system = System::new(
//             5, // 5 nodes
//             10,
//             ConsumingDistribution::Degenerate { μ: 200.0 }, // for each it will take 5 ticks
//             ProducingDistribution::Degenerate { value: 1 },
//         );

//         system.next();
//         assert_eq!(system.current_tick, 1);
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 1);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             2,
//             "first departure, and second arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );

//         system.next();
//         assert_eq!(system.current_tick, 2);
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 2);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             3,
//             "two departures, and one arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );

//         system.next();
//         assert_eq!(system.current_tick, 3);
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 3);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             4,
//             "three departures, and one arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );

//         system.next();
//         assert_eq!(system.current_tick, 4);
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 4);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             5,
//             "four departures, and one arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );

//         system.next();
//         assert_eq!(system.current_tick, 5);
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 5);
//         assert_eq!(
//             system.events_queue.heap.len(),
//             6,
//             "five departures, and one arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );

//         system.next();
//         assert_eq!(system.current_tick, 6, "first departure processed");
//         assert_eq!(system.queue.len(), 0);
//         assert_eq!(system.nodes_busy, 4, "first deprature must be finished");
//         assert_eq!(
//             system.events_queue.heap.len(),
//             5,
//             "five departures, and one arrival event MUST be inserted, events: {:?}",
//             system.events_queue.heap
//         );
//         assert_eq!(system.finished_requests.len(), 1);
//     }
// }
