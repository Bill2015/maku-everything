
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
                self.0.clone()
            }

            fn to_str(&self) -> &str {
                self.0.as_str()
            }

            fn new() -> Self {
                Self(String::from(""))
            }
        }

        impl PartialEq for $id_type {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl From<String> for $id_type {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&String> for $id_type {
            fn from(s: &String) -> Self {
                Self(s.to_string())
            }
        }
    };
}