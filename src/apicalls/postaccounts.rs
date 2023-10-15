use crate::apicalls::headermap::headermap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use chrono::Utc;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Account {
    number: String,
    description: String,
    credits: u64,
    isNoSale: bool,
    statusId: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ReturnAccount {
    id: u64,
}

/// Post an account to the api to create it
/// The scheme used for this is as followed
/// {
///    "number": "string",
///    "description": "string",
///    "credits": 0,
///    "limit": 0,
///    "discountPercentage": 0,
///    "isNoSale": true,
///    "statusId": 0
///  }
pub async fn postaccounts(username: String, credits: u64, userid: u64) {
    let date = Utc::now().format("%Y%d%m%H%M%S%f");

    let accountnumber = format!("{date}");

    let path = "/api/v1/Accounts";
    let url = format!("https://clientapi.twelve.eu{}", path);

    let headers = match headermap(path.to_string()).await {
        Ok(val) => val,
        Err(err) => {
            println!("Couldn't create a headermap\n{}", err);
            return;
        }
    };

    let account = Account {
        isNoSale: false,
        description: username,
        credits,
        number: accountnumber,
        statusId: 2,
    };

    let client = reqwest::Client::new();
    let apicall = client
        .post(url)
        .headers(headers)
        .json(&account)
        .send()
        .await
        .unwrap();

    let apicallformat = match apicall.json::<ReturnAccount>().await {
        Ok(val) => val,
        Err(err) => {
            println!("There was an error while parsing the apicall into the Root struct\n{err}");
            return;
        }
    };
    let linkpath = format!(
        "/api/v1/Users/{}/Accounts/{}/LinkToAccount",
        userid, apicallformat.id
    );
    let linkurl = format!("https://clientapi.twelve.eu{}", linkpath);

    let headerslink = match headermap(linkpath.to_string()).await {
        Ok(val) => val,
        Err(err) => {
            println!("Couldn't create a headermap\n{}", err);
            return;
        }
    };

    let returncall = client
        .post(linkurl)
        .headers(headerslink)
        .send()
        .await
        .unwrap();

    if returncall.status().is_success() {
        println!("User created succesfully");
    } else {
        println!("Something went wrong");
    }
}

pub fn input(print: &str) -> String {
    println!("{print}");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string().to_lowercase();
    input
}
