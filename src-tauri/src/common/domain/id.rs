
pub trait ID : PartialEq {
    fn new() -> Self;

    fn to_string(&self) -> String;

    fn to_str(&self) -> &str;
}

#[macro_export]
macro_rules! impl_domain_id {
    ($id_type:ty) => {
        impl ID for $id_type {
            fn to_string(&self) -> String {
                format!("{}", self.id)
            }

            fn to_str(&self) -> &str {
                self.id.as_str()
            }

            fn new() -> Self {
                Self { id: String::from("") }
            }
        }

        impl PartialEq for $id_type {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl From<String> for $id_type {
            fn from(s: String) -> Self {
                Self { id: s.to_string() }
            }
        }

        impl From<&String> for $id_type {
            fn from(s: &String) -> Self {
                Self { id: s.to_string() }
            }
        }
    };
}