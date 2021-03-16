use serde::Deserialize;

#[derive(Deserialize)]
pub struct Value<T> {
    pub value: T,
}

#[derive(Deserialize)]
pub struct Values<T> {
    pub values: Vec<T>,
}