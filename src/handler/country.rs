use crate::repository::country::City;

use axum::{
    extract::rejection::JsonRejection,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::mysql::MySqlPool;

pub async fn get_city_handler(
    State(pool): State<MySqlPool>,
    Path(city_name): Path<String>,
) -> Result<Json<City>, StatusCode> {
    let city = sqlx::query_as::<_, City>("SELECT * FROM city WHERE Name = ?")
        .bind(&city_name)
        .fetch_one(&pool)
        .await;

    match city {
        Ok(city) => Ok(Json(city)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn post_city_handler(
    State(pool): State<MySqlPool>,
    query: Result<Json<City>, JsonRejection>,
) -> Result<Json<City>, StatusCode> {
    match query {
        Ok(Json(mut city)) => {
            let result = sqlx::query(
                "INSERT INTO city (Name, CountryCode, District, Population) VALUES (?, ?, ?, ?)",
            )
            .bind(&city.name)
            .bind(&city.country_code)
            .bind(&city.district)
            .bind(city.population)
            .execute(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            city.id = Some(result.last_insert_id() as i32);
            Ok(Json(city))
        }
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
