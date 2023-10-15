use rusqlite::Connection;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::headermap::headermap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Root {
    pagination: Pagination,
    #[serde(rename = "lstAccounts")]
    lst_accounts: Vec<LstAccount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Pagination {
    #[serde(rename = "pageNumber")]
    page_number: i64,
    #[serde(rename = "pageSize")]
    page_size: i64,
    #[serde(rename = "recordCount")]
    record_count: i64,
    #[serde(rename = "pageCount")]
    page_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LstAccount {
    pub id: i64,
    pub lst_user_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub account_id: i64,
    pub user_id: i64,
}


pub async fn getaccounts(conn: &Connection) {
    // We are only interested at first in how many pages there are. Note that changing pagesize influences this as well.
    let pagecount = getpagecount().await;

    let path = "/api/v1/Accounts";
    // We don't have to create a new Client everytime either
    let client = reqwest::Client::new();

    conn.execute(
        "CREATE TABLE accounts (
            accountid INTEGER PRIMARY KEY,
            userid INTEGER
        )", 
        (),
    ).unwrap();

    // Iterating over all pages
    for page in 0..10 {
        // println!("{:#?}", pagecount);

        // Update the URL according to at which page we are
        let url = format!("https://clientapi.twelve.eu{path}?pageNumber={page}");

        let headers = headermap(path.to_string()).await.unwrap();

        let apicall = client.get(url).headers(headers).send().await.unwrap();

        let apicallformat = apicall.json::<Root>().await.unwrap();

        
        
        'databaseentry: for account in apicallformat.lst_accounts {
            // Check if an user is attached to the account
            if account.lst_user_ids.is_empty() {
                continue;
            }
            if account.lst_user_ids.len() > 1{
                continue;
            }
            // database entry
            let database_entry = Account {
                account_id: account.id,
                user_id: account.lst_user_ids[0]
            };
    
            conn.execute(
                "INSERT OR REPLACE INTO accounts (accountid, userid) VALUES (?1, ?2)", 
                (&database_entry.account_id, &database_entry.user_id)
            ).unwrap();
        }
    }
}

/// Get the number of pages there are when requesting accounts. It uses pages of size 100.
async fn getpagecount() -> i64{
    let path = "/api/v1/Accounts";
    let url = format!("https://clientapi.twelve.eu{path}?pageNumber=0");

    let headers = headermap(path.to_string()).await.unwrap();

    let client = reqwest::Client::new();
    let apicall = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .expect("Couldn't request api");

    let apicallformat = apicall.json::<Root>().await.unwrap();
    return apicallformat.pagination.page_count;
}
