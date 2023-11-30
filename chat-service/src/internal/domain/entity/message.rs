use tiktoken_rs::get_completion_max_tokens;
use uuid::Uuid;

use crate::internal::domain::entity::model::Model;

#[derive(Debug)]
pub struct Message<'a> {
    pub id: Uuid,
    pub role: String,
    pub content: String,
    pub tokens: usize,
    pub model: &'a Model,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl<'a> Message<'a> {
    // implementation of trait clone
    pub fn clone(&self) -> Self {
        Self {
            id: self.id,
            role: self.role.clone(),
            content: self.content.clone(),
            tokens: self.tokens,
            model: self.model,
            created_at: self.created_at,
        }
    }

    pub fn new(
        id: Uuid,
        role: &'a str,
        content: &'a str,
        tokens: usize,
        model: &'a Model,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let total_tokens = get_completion_max_tokens(&model.name, content);

        match total_tokens {
            Ok(total_tokens) => Self {
                id,
                role: role.to_string(),
                content: content.to_string(),
                tokens: total_tokens,
                model,
                created_at,
            },
            Err(_) => Self {
                id,
                role: role.to_string(),
                content: content.to_string(),
                tokens,
                model,
                created_at,
            },
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn tokens(&self) -> usize {
        self.tokens
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }

    pub fn validate(&self) -> Result<(), String> {
        let valid_role = self.role == "user" || self.role == "system" || self.role == "assistant";

        if !valid_role {
            return Err("role is invalid".to_string());
        }

        if self.content.is_empty() {
            return Err("content is empty".to_string());
        }

        if self.created_at > chrono::Utc::now() {
            return Err("created_at is invalid".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = Uuid::new_v4();
        let role = "user";
        let content = "Hello, world!";
        let tokens = 4092;
        let model = Model::new("gpt-3.5-turbo".to_string(), 4096);
        let created_at = chrono::Utc::now();
        let message = Message::new(id, role, content, tokens, &model, created_at);

        assert_eq!(message.id, id);
        assert_eq!(message.role, role);
        assert_eq!(message.content, content);
        assert_eq!(message.tokens, tokens);
        assert_eq!(message.model, &model);
        assert_eq!(message.created_at, created_at);
    }

    #[test]
    fn test_validate() {
        let id = Uuid::new_v4();
        let role = "user";
        let content = "Hello, world!";
        let tokens = 4092;
        let model = Model::new("gpt-3.5-turbo".to_string(), 4096);
        let created_at = chrono::Utc::now();
        let message = Message::new(id, role, content, tokens, &model, created_at);

        assert_eq!(message.validate(), Ok(()));
    }

    #[test]
    fn test_invalid_role() {
        let id = Uuid::new_v4();
        let role = "invalid";
        let content = "Hello, world!";
        let tokens = 4092;
        let model = Model::new("gpt-3.5-turbo".to_string(), 4096);
        let created_at = chrono::Utc::now();
        let message = Message::new(id, role, content, tokens, &model, created_at);

        assert_eq!(message.validate(), Err("role is invalid".to_string()));
    }

    #[test]
    fn test_empty_content() {
        let id = Uuid::new_v4();
        let role = "user";
        let content = "";
        let tokens = 4092;
        let model = Model::new("gpt-3.5-turbo".to_string(), 4096);
        let created_at = chrono::Utc::now();
        let message = Message::new(id, role, content, tokens, &model, created_at);

        assert_eq!(message.validate(), Err("content is empty".to_string()));
    }

    #[test]
    fn test_role_is_empty() {
        let id = Uuid::new_v4();
        let role = "";
        let content = "Hello, world!";
        let tokens = 4092;
        let model = Model::new("gpt-3.5-turbo".to_string(), 4096);
        let created_at = chrono::Utc::now();
        let message = Message::new(id, role, content, tokens, &model, created_at);

        assert_eq!(message.validate(), Err("role is invalid".to_string()));
    }

    #[test]
    fn test_created_at_is_invalid() {
        let id = Uuid::new_v4();
        let role = "user";
        let content = "Hello, world!";
        let tokens = 4092;
        let model = Model::new("gpt-3.5-turbo".to_string(), 4096);
        let created_at = chrono::Utc::now() + chrono::Duration::days(1);
        let message = Message::new(id, role, content, tokens, &model, created_at);

        assert_eq!(message.validate(), Err("created_at is invalid".to_string()));
    }
}
