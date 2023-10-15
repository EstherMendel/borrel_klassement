use crate::apicalls::headermap::headermap;
use crate::DB;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "lstUsers")]
    pub lst_users: Vec<LstUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LstUser {
    #[serde(rename = "id")]
    pub userid: i64,
    #[serde(default)]
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub userid: i64,
    #[serde(default)]
    pub email: String,
}

/// Get all Users and enter them in the database. We only save the UserId and a Hash of the email address.
/// This is done to preserve privacy
pub async fn getusers() {
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

    let mut succesentries = 0;
    let mut errorentries = 0;
    'databaseentry: for user in apicallformat.lst_users {
        // database entry
        let _querycreation: Vec<User> = match DB
            .create("user")
            .content(User {
                userid: user.userid,
                // Hash the email for privacy purposes
                email: sha256::digest(user.email.to_lowercase()),
            })
            .await
        {
            Ok(val) => {
                succesentries += 1;
                //dbg!(&val);
                val
            }
            Err(error) => {
                println!("error {:#?}", error);
                errorentries += 1;
                break 'databaseentry;
            }
        };
    }
    println!(
        "{} were added succefully\n{} were not added succesfully",
        succesentries, errorentries
    );
}
