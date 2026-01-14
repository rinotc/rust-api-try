use serde::Deserialize;
use account_domain::user::email::Email;
use account_domain::user::name::UserName;
use crate::handlers::request_validator::{ValidatableRequest, ValidationError};

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    name: String,
    email: String
}

pub struct ValidatedRegisterUserRequest {
    pub name: UserName,
    pub email: Email
}

impl ValidatableRequest<ValidatedRegisterUserRequest> for RegisterUserRequest {
    fn validate(self) -> Result<ValidatedRegisterUserRequest, ValidationError> {
        let n = UserName::try_from(self.name).map_err(|err| ValidationError { error_reason: err.to_string() })?;
        let e = Email::try_from(self.email).map_err(|err| ValidationError { error_reason: err.to_string() })?;
        Ok(ValidatedRegisterUserRequest { name: n, email: e })
    }
}