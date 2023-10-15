use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::apicalls::headermap::headermap;
use rusqlite::{Connection, Result};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub product_id: i64,
    pub productgr_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: i64,
    pub amount_paid: f64,
    pub account_id: i64,
}

pub async fn main_leaderboard(conn: &Connection) {




    let mut stmt = conn.prepare("
            SELECT 
            FROM payments, products
            WHERE payments.")
            .unwrap();
    let product_iter = stmt.query_map([], |row| {
        Ok(Product {
            product_id: row.get(1).unwrap(),
            productgr_id: row.get(2).unwrap(),
        })
    }).unwrap();

    for product in product_iter {
        println!("Found transaction {:?}", product.unwrap());
    }
}