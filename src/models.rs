use super::schema::{products, sales};

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub category: String,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(Product)]
#[table_name = "sales"]
pub struct Sale {
    pub id: String,
    pub product_id: i32,
    pub sale_date: i64,
    pub quantity: f64,
    pub unit: String,
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub id: i32,
    pub category: &'a str,
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name = "sales"]
pub struct NewSale<'a> {
    pub id: String,
    pub product_id: &'a i32,
    pub sale_date: &'a i64,
    pub quantity: &'a f64,
    pub unit: &'a str,
}
