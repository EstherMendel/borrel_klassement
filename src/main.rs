#![doc = include_str!("../README.md")]

pub mod apicalls;
use surrealdb::engine::local::Db;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use once_cell::sync::Lazy;


pub const CONTENT: &str = include_str!("../.env");

static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[tokio::main]
pub async fn main() {
    apicalls::getspent::getspent().await;

    panic!("heyyooo");
    // #[cfg(debug_assertions)]
    // #[cfg(target_feature = "crt-static")]
    // println!("the C runtime should be statically linked");
    // #[cfg(debug_assertions)]
    // #[cfg(not(target_feature = "crt-static"))]
    // println!("the C runtime should be dynamically linked");

    // let mut password = rpassword::prompt_password("Password: ").unwrap();
    // // println!("Password: ");
    // // let mut password = String::new();
    // // std::io::stdin().read_line(&mut password).unwrap();
    // password = password.trim().to_string().to_lowercase();
    // password = sha256::digest(password);

    // if password != "a38f7fae24b7a02d643cbfa7900ad32ded130eac257d44b47eccc6a2f42c74b0" {
    //     panic!("Incorrect password");
    // }

    println!("Starting Twelve Card Adder");

    // This finds a place where to store the database and config file.
    #[cfg(unix)]
    let app_data = std::env::var("HOME").expect("No HOME directory");
    #[cfg(windows)]
    let app_data = std::env::var("appdata").unwrap();

    // Yes I am ashamed of this config code. But it works.... and am too lazy to make it fancy.
    // It fetches a value from a file, if it is 0 then it makes a database on disk.
    // If it is 1, it makes a database in memory. Otherwise it will default to in memory.
    let path = format!("{app_data}/twelve/twelveconfig.txt");
    let mut configvar = "1".to_string();
    if std::path::Path::exists(std::path::Path::new(&path)) {
        configvar = std::fs::read_to_string(path.clone()).unwrap();
        println!("Found an existing configuration");
    } else {
        println!("No existing configuration was found");
    }

    // Deciding where the database should be stored.
    match configvar.trim() {
        // "0" => {
        //     println!("Starting the database on disk");

        //     match DB.connect::<File>("twelve.db").await {
        //         Ok(val) => val,
        //         Err(err) => panic!("failed to connect {}", err),
        //     };
        //     match DB.use_ns("twelve").use_db("twelvedb").await {
        //         Ok(val) => val,
        //         Err(_) => panic!("failed to use namescheme"),
        //     };
        // }
        "1" => {
            println!("Starting the database in memory");
            match DB.connect::<Mem>(()).await {
                Ok(val) => val,
                Err(_) => panic!("failed to connect"),
            };
            match DB.use_ns("twelve").use_db("twelvedb").await {
                Ok(val) => val,
                Err(_) => panic!("failed to use namescheme"),
            };
        }
        _ => {
            println!("Bad config found, so defaulting to starting the database in memory");
            match DB.connect::<Mem>(()).await {
                Ok(val) => val,
                Err(_) => panic!("failed to connect"),
            };
            match DB.use_ns("twelve").use_db("twelvedb").await {
                Ok(val) => val,
                Err(_) => panic!("failed to use namescheme"),
            };
        }
    }
    'choice: loop {
        // Decidig what the user wants to do
        println!(
            r#"
    Choose an option:
    [1] Create the Database
    [2] Add a card to an user
    [3] Create an user
    [4] Settings
    [5] Add card through user
    [6] Exit
        "#
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        match input.as_str() {
            "2" => {
                apicalls::posttoken::posttokens().await;
            }
            "1" => {
                apicalls::getusers::getusers().await;
                apicalls::getaccounts::getaccounts().await;
            }
            "4" => {
                println!("I have not yet made this, go to {} and change the value to 1 for in memory databases and 0 for on disk databases", path)
            }
            "6" => {
                break 'choice;
            }
            "3" => {
                apicalls::postusers::postuser().await;
            }
            "5" => {
                apicalls::tokenuserlink::posttokens().await;
            }
            _ => {
                println!("That was not a valid option, try again")
            }
        }
    }
}
