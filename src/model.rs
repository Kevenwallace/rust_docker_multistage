use serde::{Serialize, Deserialize};
use sqlx::FromRow;

// Defina o struct User para representar um usuário
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}