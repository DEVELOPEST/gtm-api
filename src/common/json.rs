use serde::Deserialize;

#[derive(Deserialize)]
pub struct Value<T> {
    pub value: T,
}