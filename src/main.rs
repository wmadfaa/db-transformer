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

fn create_sale<'a>(
    conn: &PgConnection,
    id: String,
    product_id: &'a i32,
    sale_date: &'a i64,
    quantity: &'a f64,
    unit: &'a str,
) -> Sale {
    use schema::sales;

    let new_product = NewSale {
        id,
        product_id,
        sale_date,
        quantity,
        unit,
    };

    diesel::insert_into(sales::table)
        .values(&new_product)
        .get_result(conn)
        .expect(&format!(
            "Error saving new sale with id '{}'",
            &new_product.id
        ))
}

fn delete_sales(conn: &PgConnection) {
    use schema::sales;

    diesel::delete(sales::table)
        .execute(conn)
        .expect("Error deleting sales");
}

fn create_product<'a>(conn: &PgConnection, id: i32, category: &'a str, name: &'a str) -> Product {
    use schema::products;

    let new_product = NewProduct { id, category, name };

    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(conn)
        .expect(&format!(
            "Error saving new product with id '{}'",
            &new_product.id
        ))
}

fn delete_products(conn: &PgConnection) {
    use schema::products;

    delete_sales(&conn);

    diesel::delete(products::table)
        .execute(conn)
        .expect("Error deleting products");
}

fn main() {
    let conn = establish_connections();
    delete_products(&conn);
    create_product(&conn, 1, "apple", "iphone");
    delete_sales(&conn);
    create_sale(&conn, "3".to_string(), &1, &1234527890, &2.0, "u.");
}
