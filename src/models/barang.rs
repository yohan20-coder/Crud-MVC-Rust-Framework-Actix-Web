use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Barang {
    pub id: i32,
    pub nama: String,
    pub deskripsi: Option<String>,
    pub harga: f64,
    pub stok: i32,
}

#[derive(Deserialize)]
pub struct NewBarang {
    pub nama: String,
    pub deskripsi: Option<String>,
    pub harga: f64,
    pub stok: i32,
}
