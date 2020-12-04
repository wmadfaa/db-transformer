table! {
    products (id) {
        id -> Int4,
        category -> Text,
        name -> Text,
    }
}

table! {
    sales (id) {
        id -> Text,
        product_id -> Int4,
        sale_date -> Int8,
        quantity -> Float8,
        unit -> Text,
    }
}

joinable!(sales -> products (product_id));

allow_tables_to_appear_in_same_query!(
    products,
    sales,
);
