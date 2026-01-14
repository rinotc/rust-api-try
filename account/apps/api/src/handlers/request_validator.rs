use axum::http::StatusCode;

pub struct ValidationError {
    pub error_reason: String,
}

pub trait ValidatableRequest<Validated> {
    fn validate(self) -> Result<Validated, ValidationError>;
}
pub async fn validate_request<V, R, F, Fut, T>(request: R, block: F) -> Result<T, StatusCode>
where
    R: ValidatableRequest<V>,
    F: FnOnce(V) -> Fut,
    Fut: Future<Output = Result<T, StatusCode>>,
{
    let validated = request.validate().map_err(|err| StatusCode::BAD_REQUEST)?;
    block(validated).await
}
