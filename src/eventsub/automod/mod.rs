#![doc(alias = "automods")]
//! Subscription types regarding the automod.
use super::{EventSubscription, EventType};

pub mod message;

#[doc(inline)]
pub use message::hold::{AutomodMessageHoldV1, AutomodMessageHoldV1Payload};
