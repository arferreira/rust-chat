use std::io::Error;

use uuid::Uuid;

use crate::internal::domain::entity::message::Message;
use crate::internal::domain::entity::model::Model;

#[derive(PartialEq)]
pub struct ChatConfig {
    pub model: Model,
    pub temperature: f32,
    pub top_p: f32,
    pub n: u32,
    pub stop: Vec<String>,
    pub max_tokens: u32,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
}

pub struct Chat<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub initial_system_message: Message<'a>,
    pub messages: Vec<Message<'a>>,
    pub erased_messages: Vec<Message<'a>>,
    pub status: String,
    pub token_usage: u32,
    pub config: ChatConfig,
}

impl<'a> Chat<'a> {
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        initial_system_message: Message<'a>,
        messages: Vec<Message<'a>>,
        erased_messages: Vec<Message<'a>>,
        status: String,
        token_usage: u32,
        config: ChatConfig,
    ) -> Self {
        Self {
            id,
            user_id,
            initial_system_message,
            messages,
            erased_messages,
            status,
            token_usage,
            config,
        }
    }

    pub fn add_message(&mut self, message: Message<'a>) -> Result<(), Error> {
        if self.status == "ended" {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Chat has already ended",
            ));
        }
        self.messages.push(message);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let model = Model::new("gpt-3.5-turbo".to_string(), 4096);
        let initial_system_message = Message::new(
            Uuid::new_v4(),
            "system",
            "Hello, I'm the system. How can I help you?",
            0,
            &model,
            chrono::Utc::now(),
        );
        let messages = vec![];
        let erased_messages = vec![];
        let status = "active";
        let token_usage = 0;
        let config = ChatConfig {
            model: Model::new("gpt-3.5-turbo".to_string(), 4096),
            temperature: 0.0,
            top_p: 0.0,
            n: 0,
            stop: vec![],
            max_tokens: 0,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
        };
        let chat = Chat::new(
            id,
            user_id,
            initial_system_message,
            messages,
            erased_messages,
            status.to_string(),
            token_usage,
            config,
        );

        assert_eq!(chat.id, id);
        assert_eq!(chat.user_id, user_id);
        assert_eq!(chat.status, status);
        assert_eq!(chat.token_usage, token_usage);
    }
}
