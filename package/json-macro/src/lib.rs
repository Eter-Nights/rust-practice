pub use std::boxed::Box;
pub use std::collections::HashMap;
pub use std::string::ToString;

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(num: $t) -> Json {
                    Json::Number(num as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize u128 i128 f32 f64);

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        $crate::Json::Array(vec![ $( json!($element) ),* ])
    };
    ({ $( $key:tt : $val:tt ),* }) => {
        {
            let mut fields = $crate::Box::new($crate::HashMap::new());
            $(
                fields.insert($crate::ToString::to_string($key), json!($val));
            )*
            $crate::Json::Object(fields)
        }
    };
    ($other:tt) => {
        $crate::Json::from($other)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(json!(null), Json::Null);

        let marco_generated_value = json!([{"key" : 10}]);

        let hand_code_value = Json::Array(vec![Json::Object(Box::new(
            vec![("key".to_string(), Json::from(10))]
                .into_iter()
                .collect(),
        ))]);
        assert_eq!(marco_generated_value, hand_code_value);
    }
}
