use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub fn random_string(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect::<String>()
}