
pub trait ID : PartialEq {
    fn new() -> Self;

    fn to_str(&self) -> &str;
}

#[macro_export]
macro_rules! impl_domain_id {
    ($id_type:ty, $namespace: expr) => {
        impl ID for $id_type {
            fn to_str(&self) -> &str {
                self.0.as_str()
            }

            fn new() -> Self {
                Self(format!("{}:{}", $namespace, Id::rand().to_string()))
            }
        }

        impl ToString for $id_type {
            fn to_string(&self) -> String {
                self.0.to_string()
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

        impl Into<String> for $id_type {
            fn into(self) -> String {
                self.to_string()
            }
        }
    };
}