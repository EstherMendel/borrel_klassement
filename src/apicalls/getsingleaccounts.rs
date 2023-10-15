use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::apicalls::headermap::headermap;
use rusqlite::{Connection, Result};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub lst_accounts: Vec<LstAccount>,
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

pub async fn getsingleaccounts(conn: &Connection){
    let path = "/api/v1/Accounts";

    let url = format!("https://clientapi.twelve.eu{}", path);

    let headers = match headermap(path.to_string()).await {
        Ok(val) => val,
        Err(err) => {
            println!("Couldn't create a headermap\n{}", err);
            return;
        }
    };

    let client = reqwest::Client::new();
    let apicall = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .expect("Couldn't request api");

    let apicallformat = match apicall.json::<Root>().await {
        Ok(val) => val,
        Err(err) => {
            println!("There was an error while parsing the apicall into the Root struct\n{err}");
            return;
        }
    };
    
    conn.execute(
        "CREATE TABLE accounts (
            accountid INTEGER PRIMARY KEY,
            userid INTEGER
        )", 
        (),
    ).unwrap();

    'databaseentry: for account in apicallformat.lst_accounts {
        if account.lst_user_ids.len() > 1{
            continue;
        }
        // database entry
        let database_entry = Account {
            account_id: account.id,
            user_id: account.lst_user_ids[0]
        };

        conn.execute(
            "INSERT INTO accounts (accountid, userid) VALUES (?1, ?2)", 
            (&database_entry.account_id, &database_entry.user_id)
        ).unwrap();
    }
}
