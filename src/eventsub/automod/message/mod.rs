#![doc(alias = "automod.message")]
//! Subscription types regarding automod messages.
use super::{EventSubscription, EventType};
use crate::types;
use crate::eventsub::channel::chat::Message;
use serde_derive::{Deserialize, Serialize};

pub mod hold;
