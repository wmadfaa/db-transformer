use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Product {
    pub id: i32,
    pub category: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Sale {
    pub id: String,
    pub product_id: i32,
    pub date: i64,
    pub quantity: f64,
    pub unit: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SalesAndProducts {
    pub products: Vec<Product>,
    pub sales: Vec<Sale>,
}

pub fn read_json_file() -> SalesAndProducts {
    serde_json::from_str::<SalesAndProducts>(&std::fs::read_to_string("./data/sales.json").unwrap())
        .unwrap()
}
