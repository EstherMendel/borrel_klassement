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
    #[serde(rename = "id")]
    pub transaction_id: i64,
    // #[serde(default)]
    // pub date: String,
    #[serde(rename = "paymentLines")]
    pub payment_lines: Vec<PaymentLines>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentLines {
    #[serde(rename = "amountPaid")]
    pub amount_paid: f64,
    #[serde(rename = "paymentMethodDetails")]
    pub payment_method_details: PaymentMethodDetails,
    #[serde(rename ="referenceProductInfo")]
    pub reference_product_info: ReferenceProductInfo
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodDetails {
    #[serde(rename = "paidWithAccountId")]
    pub account_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceProductInfo {
    #[serde(rename = "product")]
    pub product: Product
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "id")]
    pub product_id: i64,
    #[serde(rename = "price")]
    pub product_price: f64
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: i64,
    pub account_id: i64,
    pub product_id: i64,
    pub amount_paid: f64,
}

/// Get all Users and enter them in the database. We only save the UserId and a Hash of the email address.
/// This is done to preserve privacy
pub async fn gettransactions(conn: &Connection){
    let path = "/api/v1/RevenueTransactions";
    let date = "?filterDateStart=2023-10-10T17%3A33%3A57&filterDateEnd=2023-11-14T17%3A33%3A57";
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

    conn.execute(
        "CREATE TABLE payments (
            id INTEGER PRIMARY KEY,
            userid INTEGER,
            amountpaid REAL
        )", 
        (),
    ).unwrap();

    'databaseentry: for transaction in apicallformat.lst_revenue_transactions {
        // database entry
        let database_entry = Transaction {
            transaction_id: transaction.transaction_id,
            product_id: 1,
            amount_paid: transaction.payment_lines[0].amount_paid,
            account_id: transaction.payment_lines[0].payment_method_details.account_id,
        };

        conn.execute(
            "INSERT INTO payments (id, userid, amountpaid) VALUES (?1, ?2, ?3)", (&database_entry.transaction_id, &database_entry.account_id, &database_entry.amount_paid)
        ).unwrap();
    }

    // let mut stmt = conn.prepare("SELECT id, userid, amountpaid FROM payments").unwrap();
    // let transaction_iter = stmt.query_map([], |row| {
    //     Ok(Transaction {
    //         transaction_id: row.get(0).unwrap(),
    //         amount_paid: row.get(2).unwrap(),
    //         account_id: row.get(1).unwrap(),
    //     })
    // }).unwrap();

    // for transaction in transaction_iter {
    //     println!("Found transaction {:?}", transaction.unwrap());
    // }
}
