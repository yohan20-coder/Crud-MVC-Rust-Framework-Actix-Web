use actix_web::{web, HttpResponse, Responder};
use sqlx::mysql::MySqlPool;
use tera::Tera;
use crate::models::barang::{Barang, NewBarang};

pub async fn index(
    pool: web::Data<MySqlPool>,
    tmpl: web::Data<Tera>,
) -> impl Responder {
    let barang_list = sqlx::query_as::<_, Barang>("SELECT * FROM barang")
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_else(|_| vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("barang_list", &barang_list);

    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn create_form(tmpl: web::Data<Tera>) -> impl Responder {
    let rendered = tmpl.render("create.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn create(
    pool: web::Data<MySqlPool>,
    form: web::Form<NewBarang>,
) -> impl Responder {
    let query = "INSERT INTO barang (nama, deskripsi, harga, stok) VALUES (?, ?, ?, ?)";
    sqlx::query(query)
        .bind(&form.nama)
        .bind(&form.deskripsi)
        .bind(form.harga)
        .bind(form.stok)
        .execute(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish()
}
