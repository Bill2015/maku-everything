use std::str::FromStr;

use super::OrderBy;

#[derive(Clone, Debug)]
pub struct QueryBuilderResult(String);

impl ToString for QueryBuilderResult {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
impl From<String> for QueryBuilderResult {
    fn from(value: String) -> Self {
        QueryBuilderResult(value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BaseQueryBuilderError {
    #[error("Invalid order format")]
    InvalidOrderByFormat(),

    #[error("Start value cannot be negative")]
    StartValueCannotBeNegative(),

    #[error("Limit value cannot be negative")]
    LimitValueCannotBeNegative(),
}

pub trait QueryBuilder {
    fn set_order_by<S: Into<String>>(self, field_name: S) -> Self;

    fn set_start(self, start: i64) -> Self;

    fn set_limit(self, limit: i64) -> Self;

    fn build(&self) -> Result<QueryBuilderResult, BaseQueryBuilderError>;
}

pub struct BaseQueryBuilder {
    order_by: Option<String>,

    limit: Option<i64>,

    start: Option<i64>,
}
impl BaseQueryBuilder {
    pub fn new() -> Self {
        Self {
            order_by: None,
            limit: None,
            start: None,
        }
    }
    pub fn from_value(order_by: Option<String>, limit: Option<i64>, start: Option<i64>) -> Self {
        Self {
            order_by,
            limit,
            start,
        }
    }

    fn build_string(&self) -> Result<String, BaseQueryBuilderError> {
        // build order by string
        let orderby = self.order_by
            .as_ref()
            .map(|x| OrderBy::from_str(&x).map_err(|_| BaseQueryBuilderError::InvalidOrderByFormat()))
            .transpose()?
            .and_then(|val| format!("ORDER BY {} {}", val.get_field(), val.get_order_type().to_string()).into() );

        // build limit string
        let limit = match self.limit {
            Some(val) if val > 0 => Some(format!("LIMIT {}", val)),
            None => None,
            _ => Err(BaseQueryBuilderError::LimitValueCannotBeNegative())?
        };

        // build start string
        let start = match self.start {
            Some(val) if val > 0 => Some(format!("START {}", val)),
            None => None,
            _ => Err(BaseQueryBuilderError::StartValueCannotBeNegative())?
        };
        
        let qdata: Vec<&Option<String>> = vec![
            &orderby,
            &limit,
            &start,
        ];
  
        return Ok(qdata
            .into_iter()
            .filter(|x| x.is_some())
            .map(|val| val.to_owned().unwrap())
            .collect::<Vec<String>>()
            .join(" ")
        );
    }
}

impl QueryBuilder for BaseQueryBuilder {
    fn set_order_by<S: Into<String>>(mut self, field_name: S) -> Self {
        self.order_by = Some(field_name.into());
        self
    }

    fn set_start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    fn set_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    fn build(&self) -> Result<QueryBuilderResult, BaseQueryBuilderError> {
        Ok(QueryBuilderResult::from(self.build_string()?))
    }
}

#[macro_export]
macro_rules! impl_query_builder {
    ($id_type: ty, { $($field: ident),* }) => {
        impl QueryBuilder for $id_type {
            fn set_order_by<S: Into<String>>(mut self, field_name: S) -> $id_type {
                self.base_builder = self.base_builder.set_order_by(field_name);
                self
            }
        
            fn set_start(mut self, start: i64) -> $id_type {
                self.base_builder = self.base_builder.set_start(start);
                self
            }
        
            fn set_limit(mut self, limit: i64) -> $id_type {
                self.base_builder = self.base_builder.set_limit(limit);
                self
            }

            fn build(&self) -> Result<QueryBuilderResult, BaseQueryBuilderError> {
                Ok(QueryBuilderResult::from(self.build_string()?))
            }
        }

        ::paste::paste! {
            impl $id_type {
                $(
                    pub fn [<set_ $field>]<S: Into<String>>(mut self, value: S) -> $id_type {
                        let value: String = value.into();
                        if !value.is_empty() {
                            self.$field = Some(value);
                        }
                        self
                    }
                )*
            }
        }
    };
}
