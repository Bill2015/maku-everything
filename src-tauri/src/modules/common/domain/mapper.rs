pub trait DomainModelMapper<V> : Sized {
    fn to_domain(self) -> V;

    fn from_domain(value: V) -> Self;
}

#[macro_export]
macro_rules! domain_model_valueobj_mapper {
    ($do_name: ident, $vo_name: ident) => {
        impl DomainModelMapper<$vo_name> for $do_name {
            fn to_domain(self) -> $vo_name {
                let v = serde_json::to_value(self).unwrap();
                serde_json::from_value::<$vo_name>(v).unwrap()
            }
        
            fn from_domain(value: $vo_name) -> Self {
                let v = serde_json::to_value(value).unwrap();
                serde_json::from_value::<$do_name>(v).unwrap()
            }
        }
    };
}