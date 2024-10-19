#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
#[sqlx(rename_all = "PascalCase")]
#[serde(rename_all = "camelCase")]
pub struct City {
    #[sqlx(rename = "ID")]
    pub id: Option<i32>,
    pub name: String,
    pub country_code: String,
    pub district: String,
    pub population: i32,
}
