use url::Url;
use serde::Serialize;

use crate::modules::resource::domain::ResourceGenericError;

#[derive(Debug, Serialize, Clone)]
pub struct ResourceUrlVO {
    pub host: String,
    pub full: String,
}
impl ResourceUrlVO {
    pub fn new(url: String) -> Result<Self, ResourceGenericError> {
        let url_obj = Url::parse(url.as_str())
            .or(Err(ResourceGenericError::UrlParseFailed()))?;

        if url_obj.host().is_none() {
            return Err(ResourceGenericError::UrlEmptyHost());
        }

        Ok(
            ResourceUrlVO {
                host: url_obj.host().unwrap().to_string(),
                full: url,
            }
        )
    }
}
