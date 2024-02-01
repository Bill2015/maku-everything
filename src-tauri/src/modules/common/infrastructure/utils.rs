
pub mod dateutils {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use crate::modules::common::infrastructure::date as date_ref;

    pub fn parse<S: Into<String>>(value: S) -> Result<NaiveDateTime, chrono::ParseError> {
        NaiveDateTime::parse_from_str(&value.into(), date_ref::DATE_TIME_FORMAT)
    }

    pub fn format<S: Into<DateTime<Utc>>>(value: S) -> String {
        value.into().format(date_ref::DATE_TIME_FORMAT).to_string()
    }
}



