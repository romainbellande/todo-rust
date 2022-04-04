use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Todo {
    pub id: String,
    pub name: String,
}
