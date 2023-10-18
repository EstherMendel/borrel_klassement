use crate::apicalls::headermap::headermap;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use rusqlite::{Connection, Result};


/*
This file makes an API call to twelve and then processes it into the "transactions table"
This table has the following fields:
- product_id
- date
- product_amount
- product_price
- account_id
*/


/**************STRUCTS***************/
// This struct represents the entire API call result, only contains the list of revenue transactions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pagination: Pagination,
    #[serde(rename = "lstRevenueTransactions")]
    pub lst_revenue_transactions: Vec<LstRevenueTransactions>,
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

// This represents a single revenue transaction. It contains the date, product lines, and payment lines
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LstRevenueTransactions {
    #[serde(default)]
    pub date: String,
    #[serde(rename = "productLines")]
    pub product_lines: Vec<ProductLines>,
    #[serde(rename = "paymentLines")]
    pub payment_lines: Vec<PaymentLines>,
}

// This represents the product lines, contains the product id, number of items sold, price of one unit
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductLines {
    #[serde(rename = "productId")]
    pub product_id: i64,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub price: f64,
}

// This represents the paymentlines. It only contains the payment method details
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentLines {
    #[serde(rename = "paymentMethodDetails")]
    pub payment_method_details: PaymentMethodDetails,
}

// This represents the payment method details. It only contains the account id
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodDetails {
    #[serde(rename = "paidWithAccountId")]
    pub account_id: i64,
}

// This struct collects the relevant information from the API call, it has the same fields as the DB table
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub product_id: i64,
    pub date: String,
    pub product_amount: i64,
    pub product_price: f64,
    pub account_id: i64,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Number {
    pub number: i64,
}

// Get all transactions from a Twelve API call and insert it into a table named "transactions"
pub async fn gettransactions(conn: &Connection){
     // Specifies the Twelve data source
     let path = "/api/v1/RevenueTransactions";
     let startdate = "2023-07-01%2000%3A00%3A00";
     let enddate = "2023-10-01%2000%3A00%3A00";
     let pagesize = "10000";

    //let pagecount = 1;
    let pagecount: i64 = getpagecount(path, startdate, enddate, pagesize).await;
  
    // Creates the "transactions" table in the database
    conn.execute(
        "CREATE TABLE transactions (
            product_id INTEGER,
            date STRING,
            product_amount INTEGER,
            product_price REAL,
            account_id INTEGER
        )", 
        (),
    ).unwrap();

    for page in 0..pagecount {

        let url: String = format!("https://clientapi.twelve.eu{path}?filterDateStart={startdate}&filterDateEnd={enddate}&pageSize={pagesize}&pageNumber={page}");

        // Gets the relevant header from the headermap function
        let headers = match headermap(path.to_string()).await {
            Ok(val) => val,
            Err(err) => {
                println!("Couldn't create a headermap\n{}", err);
                return;
            }
        };
        
        // Makes a new client and performs the API call
        // apicall here has the entire output
        let client: reqwest::Client = reqwest::Client::new();

        let apicall = client.get(url).headers(headers).send().await.unwrap();

        let apicallformat = apicall.json::<Root>().await.unwrap();


    // Uncomment this for debug purposes
    // println!("{:#?}", apicallformat);

    

        // Loops through the revenue transactions
        'databaseentry: for transaction in apicallformat.lst_revenue_transactions {
            // Loops through the product lines in a single transaction
            // This is relevant for transactions of multiple products
            'databasesubentry: for subtransaction in transaction.product_lines {
                // This struct will be a single row in our table
                let database_entry = Transaction {
                    product_id: subtransaction.product_id,
                    date: transaction.date.clone(),
                    product_amount: subtransaction.count,
                    product_price : subtransaction.price as f64,
                    account_id: transaction.payment_lines[0].payment_method_details.account_id,
                };

                // Insert the transaction struct into our table
                conn.execute(
                    "INSERT INTO transactions (product_id, date, product_amount, product_price, account_id) VALUES (?1, ?2, ?3, ?4, ?5)",
                    (&database_entry.product_id, &database_entry.date, &database_entry.product_amount, &database_entry.product_price, &database_entry.account_id)
                ).unwrap();
            }
        }
    }

    // let mut stmt = conn.prepare("SELECT COUNT(*) FROM transactions").unwrap();
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(Number {
    //         number: row.get(0).unwrap(),
    //     })
    // }).unwrap();

    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }

}

/// Get the number of pages there are when requesting accounts. It uses pages of size 100.
async fn getpagecount(path: &str, startdate: &str, enddate: &str, pagesize: &str) -> i64{
    let url = format!("https://clientapi.twelve.eu{path}?filterDateStart={startdate}&filterDateEnd={enddate}&pageSize={pagesize}&pageNumber=0");

    let headers = headermap(path.to_string()).await.unwrap();

    let client = reqwest::Client::new();
    let apicall = client.get(url).headers(headers).send().await.expect("Couldn't request api");
    let apicallformat = apicall.json::<Root>().await.unwrap();
    return apicallformat.pagination.page_count;
}