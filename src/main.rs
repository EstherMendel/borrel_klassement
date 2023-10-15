#![doc = include_str!("../README.md")]

pub mod apicalls;
pub mod leaderboards;

use rusqlite::{Connection, Result};

pub const CONTENT: &str = include_str!("../.env");


#[tokio::main]
pub async fn main() {
    
    let conn = Connection::open_in_memory().unwrap();
    apicalls::gettransactions::gettransactions(&conn).await;
    apicalls::getproductgroups::getproductgroups(&conn).await;
    apicalls::getusers::getusers(&conn).await;
    apicalls::getsingleaccounts_v2::getaccounts(&conn).await;
    leaderboards::main_leaderboard::main_leaderboard(&conn).await;

}
