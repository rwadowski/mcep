use std::time::Instant;

pub mod pool;
pub mod source;
pub mod sink;
mod util;

use source::SourceId;

struct InstanceId(pub String);

impl From<SourceId> for InstanceId {
    fn from(value: SourceId) -> Self {
        InstanceId(value.0)
    }
}

struct Origin {
    instance_id: InstanceId
}

impl Origin {
    fn new(instance_id: InstanceId) -> Origin {
        Origin {
            instance_id
        }
    }
}

pub struct DataFrame {
    origin: Origin,
    ts: Instant,
    payload: String
}

impl DataFrame {
    fn new(origin: Origin, ts: Instant, payload: String) -> DataFrame {
        DataFrame {
            origin,
            ts,
            payload
        }
    }
}