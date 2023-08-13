
pub trait ID : PartialEq {
    fn parse(str: String) -> Self;

    fn to_string(&self) -> String;

    fn to_str(&self) -> &str;
}

#[macro_export]
macro_rules! impl_domain_id {
    ($id_type:ty) => {
        impl ID for $id_type {
            fn parse(str_id: String) -> Self {
                <$id_type>::new(str_id)
            }

            fn to_string(&self) -> String {
                format!("{}", self.id)
            }

            fn to_str(&self) -> &str {
                self.id.as_str()
            }
        }

        impl PartialEq for $id_type {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
    };
}