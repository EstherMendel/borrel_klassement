use crate::apicalls::headermap::headermap;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use rusqlite::{Connection, Result};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "lstRevenueTransactions")]
    pub lst_revenue_transactions: Vec<LstRevenueTransactions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LstRevenueTransactions {
    // #[serde(rename = "id")]
    // pub transaction_id: i64,
    // #[serde(default)]
    // pub date: String,
    #[serde(rename = "productLines")]
    pub product_lines: Vec<ProductLines>,
    #[serde(rename = "paymentLines")]
    pub payment_lines: Vec<PaymentLines>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductLines {
    #[serde(rename = "productId")]
    pub product_id: i64,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentLines {
    #[serde(rename = "paymentMethodDetails")]
    pub payment_method_details: PaymentMethodDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodDetails {
    #[serde(rename = "paidWithAccountId")]
    pub account_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub product_id: i64,
    pub product_amount: i64,
    pub product_price: f64,
    pub account_id: i64,
}

/// Get all Users and enter them in the database. We only save the UserId and a Hash of the email address.
/// This is done to preserve privacy
pub async fn gettransactions(conn: &Connection){
    let path = "/api/v1/RevenueTransactions";
    let date = "?filterDateStart=2023-10-15%2016%3A00%3A00&filterDateEnd=2023-10-15%2019%3A08%3A00";
    let superpath = path.to_owned() + date;

    let url = format!("https://clientapi.twelve.eu{}", superpath);

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

    println!("{:#?}", apicallformat);
    panic!("hello");

    // conn.execute(
    //     "CREATE TABLE payments (
    //         id INTEGER PRIMARY KEY,
    //         userid INTEGER,
    //         amountpaid REAL
    //     )", 
    //     (),
    // ).unwrap();

    // 'databaseentry: for transaction in apicallformat.lst_revenue_transactions {
    //     // database entry
    //     let database_entry = Transaction {
    //         transaction_id: transaction.transaction_id,
    //         product_id: 1,
    //         amount_paid: transaction.payment_lines[0].amount_paid,
    //         account_id: transaction.payment_lines[0].payment_method_details.account_id,
    //     };

    //     conn.execute(
    //         "INSERT INTO payments (id, userid, amountpaid) VALUES (?1, ?2, ?3)", (&database_entry.transaction_id, &database_entry.account_id, &database_entry.amount_paid)
    //     ).unwrap();
    // }

    // // let mut stmt = conn.prepare("SELECT id, userid, amountpaid FROM payments").unwrap();
    // // let transaction_iter = stmt.query_map([], |row| {
    // //     Ok(Transaction {
    // //         transaction_id: row.get(0).unwrap(),
    // //         amount_paid: row.get(2).unwrap(),
    // //         account_id: row.get(1).unwrap(),
    // //     })
    // // }).unwrap();

    // // for transaction in transaction_iter {
    // //     println!("Found transaction {:?}", transaction.unwrap());
    // }
}
