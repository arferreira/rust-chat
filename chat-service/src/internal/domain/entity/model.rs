pub struct Model {
    pub name: String,
    pub max_tokens: u32,
}

impl Model {
    pub fn new(name: String, max_tokens: u32) -> Self {
        Self { name, max_tokens }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn max_tokens(&self) -> u32 {
        self.max_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = "gpt-3.5-turbo".to_string();
        let max_tokens = 4096;
        let model = Model::new(name.clone(), max_tokens);

        assert_eq!(model.name, name);
        assert_eq!(model.max_tokens, max_tokens);
    }

    #[test]
    fn test_name() {
        let name = "gpt-4-1106-preview".to_string();
        let max_tokens = 128000;
        let model = Model::new(name.clone(), max_tokens);

        assert_eq!(model.name(), name);
    }

    #[test]
    fn test_max_tokens() {
        let name = "gpt-4-vision-preview".to_string();
        let max_tokens = 128000;
        let model = Model::new(name, max_tokens);

        assert_eq!(model.max_tokens(), max_tokens);
    }
}
