use crate::apicalls::headermap::headermap;
use crate::DB;
use chrono::Utc;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::io::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResAccount {
    accountid: i64,
    userid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResUser {
    email: String,
    userid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    id: i64,
    number: String,
    description: String,
    credits: f64,
    #[serde(rename = "lstAccountHolders")]
    lst_account_holders: Vec<LstAccountHolder>,
    #[serde(rename = "lstUsers")]
    lst_users: Vec<LstUser>,
    #[serde(rename = "lstTokens")]
    lst_tokens: Vec<LstToken>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LstAccountHolder {
    id: i64,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "prefixName")]
    prefix_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LstUser {
    id: i64,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "prefixName")]
    prefix_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LstToken {
    id: i64,
    number: String,
    description: String,
    #[serde(rename = "statusId")]
    status_id: i64,
}

// This creates a new token in Twelve. The token is associated with the NFC card token, the UserId and the AccountId
// A token is created by sending a post request with a json body of the following content. On top of this, it uses the default Headers.
// With an additional header called "autoAddAccount" which should parse the False value as far as I know.
// {
// "number": "06730....", /// The number always has to start with 06730 followed by an unique ID.
// "description": "Test", /// This is the description of the Card.
// "serialNumber": "04B80....", /// This is the token from your NFC card.
// "cvc": "10000070", /// This is the card number on the top left corner of your campus card
// "allowLogin": false, /// Should be false. Turn this to true if users are allowed to login into the terminal with this.
// "tokenStatusId": 2, /// 1=Created, 2=Active, 3=Blocked, 4=Ordered, 5=Produced, 6=Lost, 7=Defect, 8=Expired
// "paymentAccountId": 4221647, /// The AccoundId
// "ownerUserId": 1151823 /// The UserIds
//  }
pub async fn posttokens() {
    let (userid, accountid) = match checkemail().await {
        Ok(val) => val,
        Err(err) => {
            println!("There was an error checking the mail\n{}", err);
            return;
        }
    };

    let path = "/api/v1/Tokens";
    let url = format!("https://clientapi.twelve.eu{path}");

    // Adding the cardtoken
    println!("Put the card on the NFC reader");
    let mut cardtoken = String::new();
    std::io::stdin().read_line(&mut cardtoken).unwrap();
    cardtoken = cardtoken.trim().to_string();
    println!("{cardtoken}");

    // Adding the CVC
    println!("Enter the CVC, on campus card this is the number in the top left corner");
    let mut cvctoken = String::new();
    std::io::stdin().read_line(&mut cvctoken).unwrap();
    cvctoken = cvctoken.trim().to_string();
    println!("{cvctoken}");

    // create time in yyyymmdd (i.e. 20230808)
    let date = Utc::now().format("%Y%d%m");

    // Number to attach to the card
    let number = format!("06730{}", Utc::now().format("%Y%d%m%H%M%S"));

    // Create a Headermap
    let mut headers = match headermap(path.to_string()).await {
        Ok(val) => val,
        Err(err) => {
            println!("Couldn't create a headermap\n{}", err);
            return;
        }
    };
    headers.insert("autoAddAccount", "false".parse().unwrap());

    let mut map = HashMap::new();
    map.insert("number", number);
    map.insert("description", format!("Rusty Card Creator {}", date));
    map.insert("serialNumber", cardtoken);
    map.insert("cvc", cvctoken);
    // map.insert("allowLogin", false);
    map.insert("tokenStatusId", "2".to_string());
    map.insert("paymentAccountId", accountid.to_string());
    map.insert("ownerUserId", userid.to_string());

    println!("Please check if all information is correct:\n{:#?}", map);

    println!("Is this correct? [y/n]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string().to_lowercase();

    if input != "y" && input != "yes" {
        println!("canceling adding this card...");
        return;
    }

    let client = reqwest::Client::new();
    let apicall = client
        .post(url)
        .headers(headers)
        .json(&map)
        .send()
        .await
        .unwrap();

    print!("{}", apicall.text().await.unwrap());
}

/// This function checks if the user email exists in the Twelve database.
/// Followed by finding the userid and accountid
pub async fn checkemail() -> Result<(i64, i64), Error> {
    println!("What is your email address used for Twelve?");
    let mut email = String::new();
    std::io::stdin().read_line(&mut email).unwrap();
    email = email.trim().to_string();
    let mailhash = sha256::digest(email.to_lowercase());

    println!("Fetching {email} from the user table in the database");

    let mut resuserid = DB
        .query("SELECT * FROM user WHERE email = $email;")
        .bind(("email", mailhash))
        .await
        .unwrap();
    let user: Vec<ResUser> = resuserid.take(0).unwrap();

    if user.is_empty() {
        println!("The email address does not exist in Twelve");
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "This email address does not exist in Twelve",
        ));
    }

    let mut resaccountid = DB
        .query("SELECT * FROM account WHERE userid = $userid")
        .bind(("userid", user[0].userid))
        .await
        .unwrap();
    let account: Vec<ResAccount> = resaccountid.take(0).unwrap();
    println!("There are {} account(s) / rekening(en) found related to your userid\nPress ENTER to show results", account.len());

    if account.is_empty() {
        println!("There are no accounts in Twelve associated with this email address");
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "There are no accounts in Twelve associated with this email address",
        ));
    }

    {
        let _enterpress = std::io::stdin().read_line(&mut String::new());
    }

    let mut itercount = 0;
    for i in &account {
        itercount += 1;
        getaccountdetails(i.accountid, itercount).await;
    }

    println!("Type the Number of the account you wish to use");
    let mut accountnumber = String::new();
    std::io::stdin().read_line(&mut accountnumber).unwrap();
    accountnumber = accountnumber.trim().to_string();
    let accountnumber = match accountnumber.parse::<usize>() {
        Ok(val) => val - 1,
        Err(err) => {
            println!("This was not a valid option");
            return Err(Error::new(std::io::ErrorKind::InvalidData, err));
        }
    };

    let userid = account[accountnumber].userid;
    let accountid = account[accountnumber].accountid;

    Ok((userid, accountid))
}

/// This fetches the account details for the related userid.
/// This is done as some userid's have multiple accounts (Rekeningen).
async fn getaccountdetails(accountid: i64, count: i32) {
    let path = format!("/api/v1/Accounts/{}", accountid);
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
    println!("=========================================");
    println!("ACCOUNT {}", count);
    println!("=========================================");

    println!(
        "accountnumber / rekeningnummer : {}\ndescription : {}\ncredits : {}
    ",
        apicallformat.number, apicallformat.description, apicallformat.credits
    );

    println!("=========================================");

    for holders in apicallformat.lst_account_holders {
        println!(
            "Accountholder : {} {} {}\naccountid : {}",
            holders.first_name, holders.prefix_name, holders.last_name, apicallformat.id
        );
    }

    println!("=========================================");

    for users in apicallformat.lst_users {
        println!(
            "User : {} {} {}\nuserid : {}",
            users.first_name, users.prefix_name, users.last_name, users.id
        );
    }

    println!("=========================================");

    for token in apicallformat.lst_tokens {
        println!("Token : {} {}", token.number, token.description)
    }
}