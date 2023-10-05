use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomParticipationStatus {
    Joined,
    Left,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomParticipationEvent {
    #[serde(rename = "r")]
    pub room: String,
    #[serde(rename = "u")]
    pub username: String,
    #[serde(rename = "s")]
    pub status: RoomParticipationStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMessageEvent {
    #[serde(rename = "r")]
    pub room: String,
    #[serde(rename = "u")]
    pub username: String,
    #[serde(rename = "c")]
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum Event {
    RoomParticipation(RoomParticipationEvent),
    UserMessage(UserMessageEvent),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_event_serialization(event: &Event, expected: &str) {
        let serialized = serde_json::to_string(&event).unwrap();
        assert_eq!(serialized, expected);
        let deserialized: Event = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, *event);
    }

    #[test]
    fn test_room_participation_join_event() {
        let event = Event::RoomParticipation(RoomParticipationEvent {
            room: "test".to_string(),
            username: "test".to_string(),
            status: RoomParticipationStatus::Joined,
        });

        assert_event_serialization(
            &event,
            r#"{"t":"room_participation","r":"test","u":"test","s":"joined"}"#,
        );
    }

    #[test]
    fn test_room_participation_leave_event() {
        let event = Event::RoomParticipation(RoomParticipationEvent {
            room: "test".to_string(),
            username: "test".to_string(),
            status: RoomParticipationStatus::Left,
        });

        assert_event_serialization(
            &event,
            r#"{"t":"room_participation","r":"test","u":"test","s":"left"}"#,
        );
    }

    #[test]
    fn test_user_message_event() {
        let event = Event::UserMessage(UserMessageEvent {
            room: "test".to_string(),
            username: "test".to_string(),
            content: "test".to_string(),
        });

        assert_event_serialization(
            &event,
            r#"{"t":"user_message","r":"test","u":"test","c":"test"}"#,
        );
    }
}