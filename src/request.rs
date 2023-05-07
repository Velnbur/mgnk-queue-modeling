use serde::{Deserialize, Serialize};

///! A model that represents request accepted by queueing system.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Request {
    ///! Unique identifier of the request.
    pub id: u64,

    ///! The time needed for node to process the request.
    pub ticks_to_finish: u64,

    ///! Amount of ticks request has been waiting in the queue.
    pub ticks_in_queue: u64,
}

impl Request {
    ///! Creates new request with given ticks to finish.
    pub fn new(id: u64, ticks_to_finish: u64) -> Self {
        Self {
            id,
            ticks_to_finish,
            ticks_in_queue: 0,
        }
    }

    ///! Increment ticks in queue.
    pub(crate) fn tick(&mut self) {
        self.ticks_in_queue += 1;
    }
}
