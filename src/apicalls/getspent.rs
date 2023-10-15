use std::ffi::c_float;

use crate::apicalls::headermap::headermap;
use crate::DB;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use surrealdb::sql::Datetime;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "lstRevenueTransactions")]
    pub lst_revenue_transactions: Vec<LstRevenueTransactions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LstRevenueTransactions {
    #[serde(rename = "id")]
    pub transactionid: i64,
    #[serde(default)]
    pub date: String,
    #[serde(rename = "paymentLines")]
    pub payment_lines: Vec<PaymentLines>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentLines {
    #[serde(rename = "amountPaid")]
    pub amount_paid: f64,
    #[serde(rename = "paymentMethodDetails")]
    pub payment_method_details: PaymentMethodDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodDetails {
    #[serde(rename = "paidWithAccountId")]
    pub account_id: i64,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct User {
//     pub userid: i64,
//     #[serde(default)]
//     pub email: String,
// }

/// Get all Users and enter them in the database. We only save the UserId and a Hash of the email address.
/// This is done to preserve privacy
pub async fn getspent() {
    let path = "/api/v1/RevenueTransactions";
    let test = "?filterDateStart=2023-10-10T17%3A33%3A57&filterDateEnd=2023-11-14T17%3A33%3A57";
    let superpath = path.to_owned() + test;

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

    
    println!("{:#?}", apicall);

    let apicallformat = match apicall.json::<Root>().await {
        Ok(val) => val,
        Err(err) => {
            println!("There was an error while parsing the apicall into the Root struct\n{err}");
            return;
        }
    };

    println!("{:#?}", apicallformat);

    // let mut succesentries = 0;
    // let mut errorentries = 0;
    // 'databaseentry: for user in apicallformat.lst_users {
    //     // database entry
    //     let _querycreation: Vec<User> = match DB
    //         .create("user")
    //         .content(User {
    //             userid: user.userid,
    //             // Hash the email for privacy purposes
    //             email: sha256::digest(user.email.to_lowercase()),
    //         })
    //         .await
    //     {
    //         Ok(val) => {
    //             succesentries += 1;
    //             //dbg!(&val);
    //             val
    //         }
    //         Err(error) => {
    //             println!("error {:#?}", error);
    //             errorentries += 1;
    //             break 'databaseentry;
    //         }
    //     };
    // }
    // println!(
    //     "{} were added succefully\n{} were not added succesfully",
    //     succesentries, errorentries
    // );
}
