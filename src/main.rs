/* cSpell: disable */
#[macro_use]
extern crate diesel;
extern crate env_logger;
extern crate log;

use actix_cors::Cors;
use actix_identity::CookieIdentityPolicy;
use actix_identity::IdentityService;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use auth_handler::LoggedUser;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod activity_handler;
mod auth_handler;
mod entry_handler;
mod errors;
mod models;
mod mood_handler;
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
    let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .allowed_origin(&frontend_url)
            .supports_credentials();
        App::new()
            .wrap(cors)
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
                    )
                    .service(
                        web::resource("/activity")
                            .route(web::post().to(activity_handler::create_activity))
                            .route(web::get().to(activity_handler::get_activities)),
                    )
                    .service(
                        web::resource("/mood")
                            .route(web::post().to(mood_handler::create_mood))
                            .route(web::get().to(mood_handler::get_moods)),
                    )
                    .service(
                        web::resource("/entry")
                            .route(web::get().to(entry_handler::get_entrys))
                            .route(web::post().to(entry_handler::create_entry)),
                    )
                    .service(
                        web::resource("/entry/{id}")
                            .route(web::get().to(entry_handler::get_entry_by_id))
                    ),
            )
            .route("/", web::get().to(index))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

async fn index(logged_user: Option<LoggedUser>) -> impl Responder {
    if let Some(user) = logged_user {
        return HttpResponse::Ok().body(format!("Hello {}", &user.email));
    }
    HttpResponse::Ok().body("Hello World")
}
