use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::apicalls::headermap::headermap;
use rusqlite::{Connection, Result};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    account_id: String,
    total_spending: f64
}

pub async fn main_leaderboard(conn: &Connection) {

    let mut stmt = conn.prepare("
            SELECT users.firstname, SUM(transactions.product_price * transactions.product_amount) AS sum
            FROM transactions, products_and_groups, accounts, users
            WHERE 
            transactions.product_id = products_and_groups.productid AND
            accounts.accountid = transactions.account_id AND
            accounts.userid = users.userid
            AND
            products_and_groups.groupid IN 
                (23504, 32502, 32501, 32498, 32497, 32496, 32465, 32404, 32267, 32266, 32265, 30759, 30762, 30776, 30785, 32060)
            GROUP BY transactions.account_id
            ORDER BY sum ASC") 
            .unwrap();
    let leaderboard_iter = stmt.query_map([], |row| {
        Ok(LeaderboardEntry {
            account_id: row.get(0).unwrap(),
            total_spending: row.get(1).unwrap(),
        })
    }).unwrap();

    for entry in leaderboard_iter {
        println!("{:?}", entry.unwrap());
    }
}