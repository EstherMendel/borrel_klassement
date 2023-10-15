use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::apicalls::headermap::headermap;
use rusqlite::{Connection, Result};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub lst_product_groups: Vec<LstProductGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LstProductGroup {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub lst_product_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub product_id: i64,
    pub productgr_id: i64,
    pub productgr_name: String
}

pub async fn getproductgroups(conn: &Connection){
    let path = "/api/v1/ProductGroups";

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
        "CREATE TABLE products_and_groups (
            productid INTEGER PRIMARY KEY,
            groupid INTEGER,
            groupname STRING
        )", 
        (),
    ).unwrap();

    'databaseentry: for productgroup in apicallformat.lst_product_groups {
        'databasesubentry: for productid in productgroup.lst_product_ids {
            // database entry
            let database_entry = Product {
                product_id: productid,
                productgr_id: productgroup.id.clone(),
                productgr_name: productgroup.name.clone()
            };

            conn.execute(
                "INSERT OR REPLACE INTO products_and_groups (productid, groupid, groupname) VALUES (?1, ?2, ?3)", 
                (&database_entry.product_id, &database_entry.productgr_id, &database_entry.productgr_name)
            ).unwrap();
        }
    }
}
