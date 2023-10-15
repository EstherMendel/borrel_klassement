use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::apicalls::headermap::headermap;
use rusqlite::{Connection, Result};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub lst_users: Vec<LstUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LstUser {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub user_id: i64,
    pub user_first_name: String,
    pub user_last_name: String
}

pub async fn getusers(conn: &Connection){
    let path = "/api/v1/Users";

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
        "CREATE TABLE users (
            userid INTEGER PRIMARY KEY,
            firstname STRING,
            lastname STRING
        )", 
        (),
    ).unwrap();

    'databaseentry: for user in apicallformat.lst_users {
        // database entry
        let database_entry = User {
            user_id: user.id,
            user_first_name: user.first_name,
            user_last_name: user.last_name
        };

        conn.execute(
            "INSERT OR REPLACE INTO users (userid, firstname, lastname) VALUES (?1, ?2, ?3)", 
            (&database_entry.user_id, &database_entry.user_first_name, &database_entry.user_last_name)
        ).unwrap();
    }
}
