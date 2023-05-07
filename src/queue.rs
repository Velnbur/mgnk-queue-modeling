use std::collections::{BinaryHeap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::request::Request;

///! Represents model of the queue in **Queuing System**.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queue {
    pub requests: VecDeque<Request>,
}

impl Queue {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            requests: Vec::with_capacity(capacity),
        }
    }

    ///! Make new interation of queue.
    ///!
    ///! Increases ticks amount of being inside [`Queue`] for each request.
    pub(crate) fn tick(&mut self) {
        for request in self.requests.iter_mut() {
            request.tick();
        }
    }

    pub(crate) fn push(&mut self, request: Request) {
        self.requests.push(request);
    }

    pub(crate) fn pop(&mut self) -> Option<Request> {
        self.requests.pop()
    }

    pub fn has_space(&self) -> bool {
        self.requests.len() < self.requests.capacity()
    }
}

///! Represenst different types of events in **Queuing System**.
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub enum Event {
    ///! Represents event of request arrival.
    Arrival(Request),
    ///! Represents event of request departure.
    Departure(u64),
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Arrival(a), Self::Arrival(b)) => a.arrival_time.cmp(&b.arrival_time),
            (Self::Departure(a), Self::Departure(b)) => a.cmp(&b),
            (Self::Arrival(a), Self::Departure(b)) => a.arrival_time.cmp(&b),
            (Self::Departure(a), Self::Arrival(b)) => a.cmp(&b.arrival_time),
        }
    }
}

///! A queue of all events and their next time of execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQueue {
    pub(crate) events: BinaryHeap<Event>,
}
