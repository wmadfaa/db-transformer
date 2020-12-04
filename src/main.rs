#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{NewProduct, NewSale, Product, Sale};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

fn establish_connections() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

fn create_product<'a>(conn: &PgConnection, id: i32, category: &'a str, name: &'a str) -> Product {
    use schema::products;

    let new_product = NewProduct { id, category, name };

    diesel::insert_into(products::table)
        .values(new_product)
        .get_result(conn)
        .expect(&format!("Error saving new product with name '{}'", name))
}

fn delete_products(conn: &PgConnection) {
    use schema::products;

    diesel::delete(products::table)
        .execute(conn)
        .expect("Error deleting products");
}

fn main() {
    let conn = establish_connections();
    delete_products(&conn);
    create_product(&conn, 2, "apple", "iphone_");
}
