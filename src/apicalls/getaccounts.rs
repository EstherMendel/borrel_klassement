use crate::DB;
use chrono::Utc;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::io::Error;

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
struct LstAccount {
    id: i64,
    #[serde(rename = "lstUserIds")]
    #[serde(default)]
    // Sometimes you have accounts without user id's connected to them. This prevents the whole thing from crashing
    lst_user_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    accountid: i64,
    userid: i64,
}

/// Get all Accounts (Rekeningen) and enter them in the database. It is only important to store the AccountId and the UserId
/// If there are multiple users associated with the same Account (Rekening), it uses the first user.
/// It pushes these accounts to a database with the following struct / table scheme:
/// ```rs
/// pub struct Account {
/// accountid: i64,
/// userid: i64,
/// }
/// ```
pub async fn getaccounts() {
    // We are only interested at first in how many pages there are. Note that changing pagesize influences this as well.
    let pagecount = match getpagecount().await {
        Ok(val) => val,
        Err(err) => {
            println!("Couldn't get page count\n{}", err);
            return;
        }
    };

    // Benchmarking the succesful database entries
    let mut succesentries = 0;
    let mut errorentries = 0;
    let mut nouserid = 0;

    let path = "/api/v1/Accounts";
    // create time in yyyymmdd (i.e. 20230808), this does not change in the loop normally (unless you enter the loop at exactly 1 second before midnight)
    let _date = Utc::now().format("%Y%d%m");
    // We don't have to create a new Client everytime either
    let client = reqwest::Client::new();

    // Iterating over all pages
    for page in 0..pagecount {
        // Update the URL according to at which page we are
        let url = format!("https://clientapi.twelve.eu{path}?pageNumber={page}");

        let headers = match headermap(path.to_string()).await {
            Ok(val) => val,
            Err(err) => {
                println!("Couldn't create a headermap\n{}", err);
                return;
            }
        };

        let apicall = match client.get(url).headers(headers).send().await {
            Ok(val) => val,
            Err(err) => {
                println!("There was an error making the api request\n{}", err);
                return;
            }
        };

        let apicallformat = match apicall.json::<Root>().await {
            Ok(val) => val,
            Err(err) => {
                println!(
                    "There was an error parsing the api call into the Root struct {}",
                    err
                );
                return;
            }
        };

        'databaseentry: for account in apicallformat.lst_accounts {
            // Check if an user is attached to the account
            if account.lst_user_ids.is_empty() {
                nouserid += 1;
                continue 'databaseentry;
            }

            // database entry
            let _querycreation: Vec<Account> = match DB
                .create("account")
                .content(Account {
                    accountid: account.id,
                    // Here we specify that we only want the first user in the list to be attached to an account.
                    userid: account.lst_user_ids[0],
                })
                .await
            {
                Ok(val) => {
                    succesentries += 1;
                    val
                }
                Err(error) => {
                    println!("error {:#?}", error);
                    errorentries += 1;
                    break 'databaseentry;
                }
            };
        }
    }

    println!(
        "{} were added succefully\n{} were not added succesfully,\nand {} didn't have an userid",
        succesentries, errorentries, nouserid
    );
}

/// Get the number of pages there are when requesting accounts. It uses pages of size 100.
async fn getpagecount() -> Result<i64, Error> {
    let path = "/api/v1/Accounts";
    let url = format!("https://clientapi.twelve.eu{path}?pageNumber=0");

    let headers = match headermap(path.to_string()).await {
        Ok(val) => val,
        Err(err) => {
            println!("There was an error while creating headers\n{err}");
            return Err(Error::new(std::io::ErrorKind::InvalidData, err));
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
            return Err(Error::new(std::io::ErrorKind::InvalidData, err));
        }
    };
    Ok(apicallformat.pagination.page_count)
}
