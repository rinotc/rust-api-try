/// メールアドレス
#[derive(Debug, Clone)]
pub struct Email(pub String);

impl Email {
    pub fn new(email: String) -> Self {
        Self(email)
    }
}

impl TryFrom<String> for Email {
    type Error = String;
    fn try_from(email: String) -> Result<Self, Self::Error> {
        if email.is_empty() {
            return Err("Email is empty.".to_string());
        }
        Ok(Email(email))
    }
}
