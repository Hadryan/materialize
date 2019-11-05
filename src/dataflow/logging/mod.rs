// Copyright 2019 Materialize, Inc. All rights reserved.
//
// This file is part of Materialize. Materialize may not be used or
// distributed without the express permission of Materialize, Inc.

pub mod differential;
pub mod materialized;
pub mod timely;

use ::timely::dataflow::operators::capture::{Event, EventPusher};
use dataflow_types::logging::{DifferentialLog, LogVariant, MaterializedLog, TimelyLog};
use dataflow_types::Timestamp;
use std::time::Duration;

/// Logs events as a timely stream, with progress statements.
pub struct BatchLogger<T, E, P>
where
    P: EventPusher<Timestamp, (Duration, E, T)>,
{
    // None when the logging stream is closed
    time: Duration,
    event_pusher: P,
    _phantom: ::std::marker::PhantomData<(E, T)>,
}

impl<T, E, P> BatchLogger<T, E, P>
where
    P: EventPusher<Timestamp, (Duration, E, T)>,
{
    /// Creates a new batch logger.
    pub fn new(event_pusher: P) -> Self {
        BatchLogger {
            time: Default::default(),
            event_pusher,
            _phantom: ::std::marker::PhantomData,
        }
    }
    /// Publishes a batch of logged events and advances the capability.
    #[allow(clippy::clone_on_copy)]
    pub fn publish_batch(&mut self, time: &Duration, data: &mut Vec<(Duration, E, T)>) {
        let new_frontier = time.as_millis() as Timestamp;
        let old_frontier = self.time.as_millis() as Timestamp;
        if !data.is_empty() {
            self.event_pusher.push(Event::Messages(
                self.time.as_millis() as Timestamp,
                // In earlier versions of the code this was a
                // swap, but without any allocations to return
                // it resulted in sizeable logger allocations.
                data.drain(..).collect(),
            ));
        }
        if old_frontier < new_frontier {
            // In principle we can buffer up until this point, if that is appealing to us.
            // We could buffer more aggressively if the logging granularity were exposed
            // here, as the forward ticks would be that much less frequent.
            self.event_pusher
                .push(Event::Progress(vec![(new_frontier, 1), (old_frontier, -1)]));
        }
        self.time = time.clone();
    }
}
impl<T, E, P> Drop for BatchLogger<T, E, P>
where
    P: EventPusher<Timestamp, (Duration, E, T)>,
{
    fn drop(&mut self) {
        self.event_pusher.push(Event::Progress(vec![(
            self.time.as_millis() as Timestamp,
            -1,
        )]));
    }
}
