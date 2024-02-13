use serde::{de::DeserializeOwned, Serialize};


pub trait IAggregateRoot<T, V> 
where V: Clone + Sized {
    
    fn to_properties(self) -> V;

    fn reconstitute(properties: V) -> T;
}

pub trait ToPlainObject<P>
where P: Clone + Sized + DeserializeOwned + Serialize {
    fn to_plain(self) -> P;
}

#[macro_export]
macro_rules! base_aggregate {
    ($name: ident { $($field: ident: $t: ty,) * }) => {
        #[derive(Debug, Serialize)]
        pub struct $name {
            $( $field: $t),*
        }

        ::paste::paste! {
            #[derive(Debug, Serialize, Clone)]
            pub struct [<$name Props>] {
                $(pub $field: $t ),*
            }

            impl $name {
                pub fn new(properties: [<$name Props>]) -> Self {
                    Self { 
                        $($field: properties.$field),*
                    }
                }
                
                pub fn to_properties(self) -> [<$name Props>] {
                    [<$name Props>] { 
                        $($field: self.$field),*
                    }
                }

                $(
                    pub fn [<get_ $field>](&self) -> &$t {
                        &self.$field
                    }

                    pub fn [<take_ $field>](self) -> $t {
                        self.$field
                    }
                )*
            }
        }
    };
}
