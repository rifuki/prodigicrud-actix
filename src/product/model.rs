use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(FromRow, Serialize, ToSchema)]
pub struct Product {
    pub id_product: i32,
    pub name: String,
    pub qty: i32,
    pub price: f64,
    pub description: Option<String>
}

#[derive(Deserialize, ToSchema)]
pub struct ProductPayload {
    pub name: String,
    pub qty: i32,
    pub price: f64,
    pub description: Option<String>
}
