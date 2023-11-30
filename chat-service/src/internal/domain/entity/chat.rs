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
    pub max_tokens: usize,
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
    pub token_usage: usize,
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
        token_usage: usize,
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

    // validate checks if the chat is valid
    pub fn validate(&self) -> Result<(), Error> {
        if self.status != "active" && self.status != "ended" {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Chat status is invalid",
            ));
        }

        if self.token_usage > self.config.max_tokens {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Chat token usage is invalid",
            ));
        }

        if self.status != "ended" && self.status != "active" {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Chat status is invalid",
            ));
        }

        Ok(())
    }

    // add_message adds a message to the chat
    pub fn add_message(&mut self, message: Message<'a>) -> Result<(), Error> {
        if self.status == "ended" {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Chat has already ended",
            ));
        }

        if self.config.max_tokens >= message.tokens + self.token_usage {
            self.messages.push(message.clone());
            self.refresh_token_usage();
        } else {
            self.erased_messages.push(message.clone());
        }

        Ok(())
    }

    // refresh_token_usage is called after a message is added to the chat to update the token_usage
    pub fn refresh_token_usage(&mut self) {
        self.token_usage = self
            .messages
            .iter()
            .fold(0, |acc, message| acc + message.tokens);
    }

    // get_messages returns a copy of the messages
    pub fn get_messages(&self) -> Vec<Message<'a>> {
        self.messages.iter().map(|msg| msg.clone()).collect()
    }

    pub fn count_messages(&self) -> usize {
        self.messages.len()
    }

    // end sets the status of the chat to "ended"
    pub fn end(&mut self) {
        self.status = "ended".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_chat() {
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
        let status = "invalid";
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

        assert_eq!(
            chat.validate().unwrap_err().kind(),
            std::io::ErrorKind::Other
        );
    }

    #[test]
    fn test_validate_chat() {
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

        assert_eq!(chat.validate().unwrap(), ());
    }

    #[test]
    fn test_ended_chat() {
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
        let status = "ended";
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
        let mut chat = Chat::new(
            id,
            user_id,
            initial_system_message,
            messages,
            erased_messages,
            status.to_string(),
            token_usage,
            config,
        );

        let message = Message::new(
            Uuid::new_v4(),
            "user",
            "Hello, I'm the user. How can I help you?",
            0,
            &model,
            chrono::Utc::now(),
        );

        assert_eq!(
            chat.add_message(message.clone()).unwrap_err().kind(),
            std::io::ErrorKind::Other
        );
    }

    #[test]
    fn test_add_message() {
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
            max_tokens: 5000,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
        };
        let mut chat = Chat::new(
            id,
            user_id,
            initial_system_message,
            messages,
            erased_messages,
            status.to_string(),
            token_usage,
            config,
        );

        let message = Message::new(
            Uuid::new_v4(),
            "user",
            "Hello, I'm the user. How can I help you?",
            0,
            &model,
            chrono::Utc::now(),
        );

        // check number of tokens on message
        assert_eq!(message.tokens, 4083);

        chat.add_message(message.clone()).unwrap();
        assert_eq!(chat.messages.len(), 1);
        assert_eq!(chat.erased_messages.len(), 0);
        assert_eq!(chat.token_usage, 4083);

        chat.add_message(message.clone()).unwrap();
        assert_eq!(chat.messages.len(), 1);
        assert_eq!(chat.erased_messages.len(), 1);
        assert_eq!(chat.token_usage, 4083);
    }

    #[test]
    fn test_refresh_token_usage() {
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
        let mut chat = Chat::new(
            id,
            user_id,
            initial_system_message,
            messages,
            erased_messages,
            status.to_string(),
            token_usage,
            config,
        );

        let message = Message::new(
            Uuid::new_v4(),
            "user",
            "Hello, I'm the user. How can I help you?",
            0,
            &model,
            chrono::Utc::now(),
        );

        chat.add_message(message.clone()).unwrap();
        assert_eq!(chat.token_usage, 0);

        chat.refresh_token_usage();
        assert_eq!(chat.token_usage, 0);

        chat.add_message(message.clone()).unwrap();
        assert_eq!(chat.token_usage, 0);

        chat.refresh_token_usage();
        assert_eq!(chat.token_usage, 0);

        chat.add_message(message.clone()).unwrap();
        assert_eq!(chat.token_usage, 0);

        chat.refresh_token_usage();
        assert_eq!(chat.token_usage, 0);
    }

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
