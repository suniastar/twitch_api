#![doc(alias = "automod.message.hold")]
//! A user is notified if a message is caught by automod for review.

use super::*;
/// [`automod.message.hold`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodMessageHoldV1 {
    /// User ID of the broadcaster (channel).
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// User ID of the moderator.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub moderator_user_id: types::UserId,
}

impl AutomodMessageHoldV1 {
    /// Get automod message hold events for this channel as moderator.
    pub fn new(
        broadcaster_user_id: impl Into<types::UserId>,
        moderator_user_id: impl Into<types::UserId>,
    ) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
            moderator_user_id: moderator_user_id.into(),
        }
    }
}

impl EventSubscription for AutomodMessageHoldV1 {
    type Payload = AutomodMessageHoldV1Payload;

    const EVENT_TYPE: EventType = EventType::AutomodMessageHold;
    #[cfg(feature = "twitch_oauth2")]
    /// Requires a user access token that includes the [moderator:manage:automod][twitch_oauth2::Scope::ModeratorManageAutoMod] scope.
    /// The ID in the moderator_user_id condition parameter must match the user ID in the access token.
    /// If app access token used,
    /// then additionally requires the [moderator:manage:automod][twitch_oauth2::Scope::ModeratorManageAutoMod] scope for the moderator.
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ModeratorManageAutoMod];
    const VERSION: &'static str = "1";
}

/// [`automod.message.hold`](AutomodMessageHoldV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodMessageHoldV1Payload {
    /// The broadcaster user ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::UserName,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The user ID of the user that sent the message.
    pub user_id: types::UserId,
    /// The user name of the user that sent the message.
    pub user_name: types::UserName,
    /// The user login of the user that sent the message.
    pub user_login: types::UserName,
    /// A UUID that identifies the message.
    pub message_id: types::MsgId,
    /// The structured chat Message
    pub message: Message,
    /// The level of severity. Measured between 1 and 4.
    pub level: usize,
    /// The category of the message.
    pub category: String, // TODO enum?,
    /// The timestamp of when automod saved the message.
    pub held_at: types::Timestamp,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    // TODO get real payload since the example in the api doc does not match the promised structure.
    let payload = r##"
    {
      "subscription": {
        "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        "type": "automod.message.hold",
        "version": "1",
        "status": "enabled",
        "cost": 0,
        "condition": {
          "broadcaster_user_id": "1337",
          "moderator_user_id": "9001"
        },
        "transport": {
          "method": "webhook",
          "callback": "https://example.com/webhooks/callback"
        },
        "created_at": "2023-04-11T10:11:12.123Z"
      },
      "event": {
        "broadcaster_user_id": "1337",
        "broadcaster_user_name": "blah",
        "broadcaster_user_login": "blahblah",
        "user_id": "456789012",
        "user_name": "baduser",
        "user_login": "baduserbla",
        "message_id": "bad-message-id",
        "message": "This is a bad message… ",
        "level": 5,
        "category": "aggressive",
        "held_at": "2022-12-02T15:00:00.00Z",
        "fragments": {
          "emotes": [
            {
              "text": "badtextemote1",
              "id": "emote-123",
              "set-id": "set-emote-1"
            },
            {
              "text": "badtextemote2",
              "id": "emote-234",
              "set-id": "set-emote-2"
            }
          ],
          "cheermotes": [
            {
              "text": "badtextcheermote1",
              "amount": 1000,
              "prefix": "prefix",
              "tier": 1
            }
          ]
        }
      }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}
