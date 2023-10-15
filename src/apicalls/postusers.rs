use crate::apicalls::headermap::headermap;

use crate::apicalls::postaccounts::postaccounts;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ReturnAccount {
    id: u64,
}

/// Post an user to the api to create it
/// The scheme used for this is as followed
/// {
///  "NameFirst": "string", // Needed
///  "NamePrefix": "string", // Needed
///  "NameLast": "string", // Needed
///  "EmailAddress": "string", // Needed
///  "Phone": "string",
///  "MobilePhone": "string",
///  "AddressStreet": "string",
///  "AddressNumber": "string",
///  "AddressNumber2": "string",
///  "AddressZipCode": "string",
///  "AddressCity": "string",
///  "DebtorNumber": "string",
///  "ExternalId": "string",
///  "MembershipNumber": "string",
///  "birthDate": "2023-08-29T15:01:17.224Z"
/// }
pub async fn postuser() {
    let firstname = input("Firstname: ");
    let prefix = input("Prefix: ");
    let lastname = input("Lastname: ");
    let email = input("EmailAddress: ");

    let path = "/api/v1/Users";
    let url = format!("https://clientapi.twelve.eu{}", path);

    let headers = match headermap(path.to_string()).await {
        Ok(val) => val,
        Err(err) => {
            println!("Couldn't create a headermap\n{}", err);
            return;
        }
    };
    let mut map = HashMap::new();
    map.insert("NameFirst", firstname.clone());
    map.insert("NamePrefix", prefix.clone());
    map.insert("NameLast", lastname.clone());
    map.insert("EmailAddress", email);

    let client = reqwest::Client::new();
    let apicall = client
        .post(url)
        .headers(headers)
        .json(&map)
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

    postaccounts(
        format!("{} {} {}", firstname, prefix, lastname),
        0,
        apicallformat.id,
    )
    .await;
}

pub fn input(print: &str) -> String {
    println!("{print}");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}
