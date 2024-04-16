use super::schema::rustaceans;

#[derive(serde::Serialize, serde::Deserialize, diesel::Queryable)]
pub struct Rustacean {
    #[serde(skip_serializing)]
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub created_at: String,
}

#[derive(serde::Deserialize, diesel::Insertable)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
