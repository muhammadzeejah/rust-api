use actix_web::{web, App, HttpServer};
mod db;
mod handlers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .route("/users", web::post().to(handlers::create_user))
                    .route("/users", web::get().to(handlers::get_users))
                    .route("/users/{id}", web::get().to(handlers::get_user))
                    .route("/users/{id}", web::put().to(handlers::update_user))
                    .route("/users/{id}", web::delete().to(handlers::delete_user))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}