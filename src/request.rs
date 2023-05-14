use std::sync::atomic::{AtomicU64, Ordering};

/// Represents request that is processed by the system.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Request {
    /// Unique identifier of the request.
    pub id: u64,

    /// Time required to process [`Requset`].
    pub time_to_finish: f64,

    /// Time that request was created.
    pub created_at: f64,

    /// Time when request was processed
    pub started_at: Option<f64>,
}

impl Eq for Request {}

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

impl Request {
    pub fn new(time_to_finish: f64, created_at: f64) -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            time_to_finish,
            created_at,
            started_at: None,
        }
    }
}
