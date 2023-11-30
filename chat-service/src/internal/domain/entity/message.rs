use uuid::Uuid;

use crate::internal::domain::entity::model::Model;

#[derive(Debug)]
pub struct Message<'a> {
    pub id: Uuid,
    pub role: String,
    pub content: String,
    pub tokes: u32,
    pub model: &'a Model,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl<'a> Message<'a> {
    pub fn new(
        id: Uuid,
        role: &'a str,
        content: &'a str,
        tokes: u32,
        model: &'a Model,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            role: role.to_string(),
            content: content.to_string(),
            tokes,
            model,
            created_at,
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

    pub fn tokes(&self) -> u32 {
        self.tokes
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
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
        let tokes = 128;
        let model = Model::new("gpt-3.5-turbo".to_string(), 4096);
        let created_at = chrono::Utc::now();
        let message = Message::new(id, role, content, tokes, &model, created_at);

        assert_eq!(message.id, id);
        assert_eq!(message.role, role);
        assert_eq!(message.content, content);
        assert_eq!(message.tokes, tokes);
        assert_eq!(message.model, &model);
        assert_eq!(message.created_at, created_at);
    }
}
