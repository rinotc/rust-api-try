/// ユーザー名
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(pub String);
const MAX_LENGTH: usize = 100;

impl UserName {
    pub fn new(value: impl Into<String>) -> Self {
        Self::try_from(value.into())
            .unwrap_or_else(|err| panic!("Failed to create user name because: {}", err))
    }
}

impl TryFrom<&str> for UserName {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl TryFrom<String> for UserName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("User name must no be empty".to_string());
        }
        if value.len() > MAX_LENGTH {
            return Err(format!("User name is too long: {}", value));
        }
        Ok(Self(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = UserName::new("test".to_string());
        assert_eq!(name.0, "test".to_string());
    }

    #[test]
    fn test_try_from() {
        let valid_name = UserName::try_from("valid".to_string());
        assert_eq!(valid_name.unwrap().0, "valid".to_string());

        let empty_name = UserName::try_from("".to_string());
        assert_eq!(empty_name.err().unwrap(), "User name must no be empty");

        let long_name = UserName::try_from("a".repeat(MAX_LENGTH + 1));
        assert_eq!(long_name.err().unwrap(), format!("User name is too long: {}", "a".repeat(MAX_LENGTH + 1)));
    }
}