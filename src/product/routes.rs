use actix_web::web;

use crate::product::handler::{
    get_all_products,
    insert_product,
    get_single_product,
    delete_product,
    update_product
};

pub fn scoped_product(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(get_all_products))            
            .route("", web::post().to(insert_product))
            .route("/{id_product}", web::get().to(get_single_product))
            .route("/{id_product}", web::delete().to(delete_product))
            .route("/{id_product}", web::put().to(update_product))
    );
}