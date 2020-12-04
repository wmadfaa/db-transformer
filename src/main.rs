#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{NewProduct, NewSale, Product, Sale};
use crate::json::read_json_file;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod json;
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

fn delete_products_and_sales(conn: &PgConnection) {
    use schema::products;

    delete_sales(&conn);

    diesel::delete(products::table)
        .execute(conn)
        .expect("Error deleting products");
}

fn main() {
    use schema::products::dsl::*;
    use schema::sales::dsl::*;
    let conn = establish_connections();
    let res = read_json_file();

    delete_products_and_sales(&conn);
    for _product in &res.products {
        create_product(&conn, _product.id, &_product.category, &_product.name);
    }

    for _sale in &res.sales {
        create_sale(
            &conn,
            _sale.id.to_string(),
            &_sale.product_id,
            &_sale.date,
            &_sale.quantity,
            &_sale.unit,
        );
    }

    let products_from_db = products
        .load::<Product>(&conn)
        .expect("Error loading products");

    for product_from_db in products_from_db {
        let product_sales = sales
            .filter(product_id.eq(product_from_db.id))
            .load::<Sale>(&conn)
            .expect("Error loading sales");

        println!("{:#?}", vec![(product_from_db, product_sales)]);
    }
}
