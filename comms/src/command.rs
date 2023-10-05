use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginCommand {
    #[serde(rename = "u")]
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoinRoomCommand {
    #[serde(rename = "r")]
    pub room: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaveRoomCommand {
    #[serde(rename = "r")]
    pub room: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendMessageCommand {
    #[serde(rename = "r")]
    pub room: String,
    #[serde(rename = "c")]
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuitCommand;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum UserCommand {
    Login(LoginCommand),
    JoinRoom(JoinRoomCommand),
    LeaveRoom(LeaveRoomCommand),
    SendMessage(SendMessageCommand),
    Quit(QuitCommand),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_command_serialization(command: &UserCommand, expected: &str) {
        let serialized = serde_json::to_string(&command).unwrap();
        assert_eq!(serialized, expected);
        let deserialized: UserCommand = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, *command);
    }

    #[test]
    fn test_login_command() {
        let command = UserCommand::Login(LoginCommand {
            username: "test".to_string(),
        });

        assert_command_serialization(&command, r#"{"t":"login","u":"test"}"#);
    }

    #[test]
    fn test_join_command() {
        let command = UserCommand::JoinRoom(JoinRoomCommand {
            room: "test".to_string(),
        });

        assert_command_serialization(&command, r#"{"t":"join_room","r":"test"}"#);
    }

    #[test]
    fn test_leave_command() {
        let command = UserCommand::LeaveRoom(LeaveRoomCommand {
            room: "test".to_string(),
        });

        assert_command_serialization(&command, r#"{"t":"leave_room","r":"test"}"#);
    }

    #[test]
    fn test_message_command() {
        let command = UserCommand::SendMessage(SendMessageCommand {
            room: "test".to_string(),
            content: "test".to_string(),
        });

        assert_command_serialization(&command, r#"{"t":"send_message","r":"test","c":"test"}"#);
    }

    #[test]
    fn test_quit_command() {
        let command = UserCommand::Quit(QuitCommand);

        assert_command_serialization(&command, r#"{"t":"quit"}"#);
    }
}