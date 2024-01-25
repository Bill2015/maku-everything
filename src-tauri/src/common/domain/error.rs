#[derive(serde::Serialize)]
pub struct ErrorBody {
    pub message: String,
    pub command: String,
}


#[derive(serde::Serialize)]
pub struct ErrorBodyTest {
    pub message: String,
    pub other: String,
    pub command: String,
}