use actix_web::{web, HttpResponse};

use crate::{
    errors::AppError,
    product::model::{
        ProductPayload, 
        Product
    },
    types::AppState,
};

/// Get All Products Endpoint
/// 
/// Retrieves a list of all products from the database.
/// This endpoint utilizes the GET method to fetch and return a list of products.
/// 
/// If successful, it responds with a status code 200 (Success) along with a JSON array object
/// containing all product entries. In case of a server error, it responds with a status code 500 (Server Error).
#[utoipa::path(
    get,
    tag = "product",
    path = "/api/products",
    responses(
        (status = 200, description = "Success"),
        (status = 500, description = "Server Error")
    )
)]
pub async fn get_all_products(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let db_pool = &app_state.get_ref().db_pool;

    let sql_query = sqlx::query_as::<_, Product>("SELECT * FROM product ORDER BY name");
    let query_result = sql_query.fetch_all(db_pool).await?;

    Ok(HttpResponse::Ok().json(query_result))
}

/// Add Product Endpoint
///
/// Stores a new product into the database.
/// This endpoint uses the POST method to add a new product entry to the databaase.
/// 
/// Excepts a JSON payload containing product details (name, quantity, price, description).
/// Upon succesful insertion, it returns a status code 201 (Success) and an empty body.
#[utoipa::path(
    post,
    tag = "product",
    path = "/api/products",
    responses(
        (status = 201, description = "Success"),
        (status = 500, description = "Server Error")
    ),
    request_body = ProductPayload
)]
pub async fn insert_product(
    app_state: web::Data<AppState>,
    payload: web::Json<ProductPayload>,
) -> Result<HttpResponse, AppError> {
    let db_pool = &app_state.get_ref().db_pool;
    let payload = payload.into_inner();

    let sql_query =
        sqlx::query("INSERT INTO product(name, qty, price, description) VALUES(?, ?, ?, ?);");
    let _ = sql_query
        .bind(&payload.name)
        .bind(&payload.qty)
        .bind(&payload.price)
        .bind(&payload.description)
        .execute(db_pool)
        .await?;

    Ok(HttpResponse::Created().finish())
}

/// Get Single Product Endpoint
/// 
/// Retrieves detailed information for a single based on the provided product ID.
/// This endpoint uses the GET method to fetch and return specific info one product.
/// If successful, it responds with a status code 200 (Success) along with a JSON object
/// containing detailed information about the requested product. In case of a server error,
/// it responds with a status code 500 (Server Error).
#[utoipa::path(
    get,
    tag = "product",
    path = "/api/products/{id_product}",
    responses(
        (status = 200, description = "Success"),
        (status = 500, description = "Server Error")
    )
)]
pub async fn get_single_product(
    app_state: web::Data<AppState>,
    path: web::Path<u32>
) -> Result<HttpResponse, AppError> 
{
    let id_product = path.into_inner();
    
    let db_pool = &app_state.get_ref().db_pool;

    let sql_query = sqlx::query_as::<_, Product>("SELECT * FROM product WHERE id_product = ?");
    let query_result = sql_query
        .bind(id_product)
        .fetch_one(db_pool)
        .await?;
    
    Ok(HttpResponse::Ok().json(query_result))
}

/// Delete Product Endpoint
/// 
/// Deletes a single product from the database based on the provided ID.
/// This endpoint utilizes the DELETE method to remove a product entry from the database.
#[utoipa::path(
    delete,
    tag = "product",
    path = "/api/products/{id_product}",
    responses(
        (status = 204, description = "Success"),
        (status = 500, description = "Server Error")
    )
)]
pub async fn delete_product(
    app_state: web::Data<AppState>,
    path: web::Path<u32>
) -> Result<HttpResponse, AppError>
{
    let id_product = path.into_inner();
    let db_pool = &app_state.get_ref().db_pool;

    let sql_query = sqlx::query("DELETE FROM product WHERE id_product = ?");
    let _ = sql_query
        .bind(id_product)
        .execute(db_pool)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Update Product Endpoint
/// 
/// Update a product with the provided ID.
/// 
/// # Parameters
/// - `id_product`: The ID of the product to be updated.
/// - `ProductPayload`: The Payload containing product details to be updated.
/// 
/// # Returns
/// If successful, returns an `Ok` response with status code 200. Otherwise, returns a `Server Error` response with status code 500.
#[utoipa::path(
    put,
    tag = "product",
    path = "/api/products/{id_product}",
    responses(
        (status = 200, description = "Success"),
        (status = 500, description = "Server Error")
    ),
    request_body = ProductPayload
)]
pub async fn update_product(
    app_state: web::Data<AppState>,
    path: web::Path<u32>,
    payload: web::Json<ProductPayload>
) -> Result<HttpResponse, AppError> {
    let id_product = path.into_inner();
    let payload = payload.into_inner();
    let db_pool = &app_state.get_ref().db_pool;

    let sql_query = sqlx::query("UPDATE product SET name = ?, qty = ?, price = ?, description = ? WHERE id_product = ?");
    let _ = sql_query
        .bind(&payload.name)
        .bind(&payload.qty)
        .bind(&payload.price)
        .bind(&payload.description)
        .bind(id_product)
        .execute(db_pool)
        .await?;

    Ok(HttpResponse::Ok().finish())
}