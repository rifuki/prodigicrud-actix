use actix_web::{
    HttpServer,
    App, 
    web, 
    middleware
};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use actix_cors::Cors;

use backend_prodigicrud::{
    product::{
        model::{
            Product,
            ProductPayload 
        },
        handler::{
            __path_get_all_products, 
            __path_insert_product,
            __path_get_single_product,
            __path_delete_product,
            __path_update_product
        },
        routes::scoped_product
    },
    ping::{
        __path_ping, 
        ping
    }, 
    types::AppState, 
    db::establish_connection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_port = std::env::var("APP_PORT")
        .unwrap_or(String::from("80"))
        .parse::<u16>()
        .unwrap_or_else(|e| {
            eprintln!("Invalid APP_PORT [{}]", e);
            std::process::exit(1);
        });

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            eprintln!("DATABASE_URL Must be Set First.");
            std::process::exit(1);
        });
    let db_pool = establish_connection(&db_url).await;
    let app_state = web::Data::new(AppState {
        db_pool
    });


    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        
        #[derive(OpenApi)]
        #[openapi(
            paths(
                ping, 
                get_all_products, 
                insert_product, 
                get_single_product, 
                delete_product, 
                update_product
            ),
            info(
                title = "ProdigiCrud API",
                version = "0.1.0",
            ),
            components(
                schemas(
                    Product, ProductPayload
                )
            )
        )]
        struct ApiDoc;

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
            .service(
                web::scope("/api")
                    .wrap(middleware::NormalizePath::trim())
                    .configure(scoped_product)
                    .service(ping)
            )
    })
    .bind(("0.0.0.0", app_port))?
    .run()
    .await
}
