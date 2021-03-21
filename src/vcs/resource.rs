use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct VcsRepository {
    pub name: String,
    pub ssh_clone_url: String,
}