#[derive(serde::Serialize)]
pub struct ErrorBody {
    pub message: String,
    pub command: String,
}
