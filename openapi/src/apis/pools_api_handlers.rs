#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]

use crate::apis::{Body, NoContent};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};

/// Configure handlers for the Pools resource
pub fn configure<T: crate::apis::Pools + 'static, A: FromRequest + 'static>(
    cfg: &mut ServiceConfig,
) {
    cfg.service(
        actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}")
            .name("del_node_pool")
            .guard(actix_web::guard::Delete())
            .route(actix_web::web::delete().to(del_node_pool::<T, A>)),
    )
    .service(
        actix_web::web::resource("/pools/{pool_id}")
            .name("del_pool")
            .guard(actix_web::guard::Delete())
            .route(actix_web::web::delete().to(del_pool::<T, A>)),
    )
    .service(
        actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}")
            .name("get_node_pool")
            .guard(actix_web::guard::Get())
            .route(actix_web::web::get().to(get_node_pool::<T, A>)),
    )
    .service(
        actix_web::web::resource("/nodes/{id}/pools")
            .name("get_node_pools")
            .guard(actix_web::guard::Get())
            .route(actix_web::web::get().to(get_node_pools::<T, A>)),
    )
    .service(
        actix_web::web::resource("/pools/{pool_id}")
            .name("get_pool")
            .guard(actix_web::guard::Get())
            .route(actix_web::web::get().to(get_pool::<T, A>)),
    )
    .service(
        actix_web::web::resource("/pools")
            .name("get_pools")
            .guard(actix_web::guard::Get())
            .route(actix_web::web::get().to(get_pools::<T, A>)),
    )
    .service(
        actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}")
            .name("put_node_pool")
            .guard(actix_web::guard::Put())
            .route(actix_web::web::put().to(put_node_pool::<T, A>)),
    );
}

async fn del_node_pool<T: crate::apis::Pools + 'static, A: FromRequest + 'static>(
    _token: A,
    path: Path<(String, String)>,
) -> Result<NoContent, crate::apis::RestError<crate::models::RestJsonError>> {
    T::del_node_pool(crate::apis::Path(path.into_inner()))
        .await
        .map(Json)
        .map(Into::into)
}

async fn del_pool<T: crate::apis::Pools + 'static, A: FromRequest + 'static>(
    _token: A,
    path: Path<String>,
) -> Result<NoContent, crate::apis::RestError<crate::models::RestJsonError>> {
    T::del_pool(crate::apis::Path(path.into_inner()))
        .await
        .map(Json)
        .map(Into::into)
}

async fn get_node_pool<T: crate::apis::Pools + 'static, A: FromRequest + 'static>(
    _token: A,
    path: Path<(String, String)>,
) -> Result<Json<crate::models::Pool>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_node_pool(crate::apis::Path(path.into_inner()))
        .await
        .map(Json)
}

async fn get_node_pools<T: crate::apis::Pools + 'static, A: FromRequest + 'static>(
    _token: A,
    path: Path<String>,
) -> Result<Json<Vec<crate::models::Pool>>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_node_pools(crate::apis::Path(path.into_inner()))
        .await
        .map(Json)
}

async fn get_pool<T: crate::apis::Pools + 'static, A: FromRequest + 'static>(
    _token: A,
    path: Path<String>,
) -> Result<Json<crate::models::Pool>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_pool(crate::apis::Path(path.into_inner()))
        .await
        .map(Json)
}

async fn get_pools<T: crate::apis::Pools + 'static, A: FromRequest + 'static>(
    _token: A,
) -> Result<Json<Vec<crate::models::Pool>>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_pools().await.map(Json)
}

async fn put_node_pool<T: crate::apis::Pools + 'static, A: FromRequest + 'static>(
    _token: A,
    path: Path<(String, String)>,
    Json(create_pool_body): Json<crate::models::CreatePoolBody>,
) -> Result<Json<crate::models::Pool>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::put_node_pool(crate::apis::Path(path.into_inner()), Body(create_pool_body))
        .await
        .map(Json)
}
