#[macro_use]
extern crate diesel;

use actix_identity::CookieIdentityPolicy;
use actix_identity::IdentityService;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod auth_handler;
mod errors;
mod models;
mod register_handler;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable Logger
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(86400) // one day in seconds
                    .secure(false), // this can only be true if you have https
            ))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/auth")
                            .route(web::post().to(auth_handler::login))
                            .route(web::delete().to(auth_handler::logout))
                            .route(web::get().to(auth_handler::get_me)),
                    )
                    .service(
                        web::resource("/register")
                            .route(web::post().to(register_handler::register)),
                    ),
            )
            .route("/", web::get().to(index))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
