use serde::Serializer;
use surrealdb::sql::Thing;

pub fn thing_serialize<S>(x: &Thing, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.to_string().as_str())
}
