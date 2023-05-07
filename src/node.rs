use serde::{Deserialize, Serialize};

use crate::request::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeState {
    Busy(Request),
    Idle,
}

impl Default for NodeState {
    fn default() -> Self {
        Self::Idle
    }
}

///! The model that accepts [`Request`] and consumes them untile they are done.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub(crate) state: NodeState,
}

impl Node {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    ///! Make another iteration of system, and return true if node is ready to
    ///! accept another request.
    pub(crate) fn tick(&mut self) -> bool {
        if let NodeState::Busy(request) = &mut self.state {
            request.ticks_to_finish = request.ticks_to_finish.saturating_sub(1);

            if request.ticks_to_finish == 0 {
                self.state = NodeState::Idle;
                return true;
            }
            return false;
        }
        true
    }

    ///! Consume request, and start processing it.
    pub(crate) fn consume(&mut self, request: Request) {
        self.state = NodeState::Busy(request);
    }
}
