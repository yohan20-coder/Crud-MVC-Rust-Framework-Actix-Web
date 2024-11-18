use actix_web::{web, App, HttpServer};
use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use tera::Tera;

mod controllers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL harus diset");
    let pool = MySqlPool::connect(&database_url).await.expect("Gagal koneksi database");

    let tera = Tera::new("src/views/**/*").expect("Error membuat Tera template");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(controllers::barang_controller::index))
            .route("/barang/create", web::get().to(controllers::barang_controller::create_form))
            .route("/barang", web::post().to(controllers::barang_controller::create))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
