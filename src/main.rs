use std::env;
use std::sync::Arc;
use uuid::Uuid;

//internal
mod pkg;
use crate::pkg::utils::log;
use crate::pkg::utils::config::config::Config;
use crate::pkg::database::postgres::postgres;
use crate::pkg::database::repository::books;

#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        log::logger("fatal", "main", "argument not found");
    }
    log::logger("info", "main", " ============== [START] ============== ");

    let path = "./.config/.config.yaml";
    let methode: String = args[1..2].join(" ");
    log::logger("info", "main", &format!("methode  : {}", methode));
    

    let conf = Arc::new(Config::load(&path).expect("Failed to read config file"));
    log::load(&conf.general.log);
    log::logger("info", "main", &format!("log      : {}", conf.general.log));
    log::logger("trace", "main", " ===== [ CONFIG ] ===== ");
    log::logger("trace", "main", &format!("tz       : {}", conf.general.tz));
    log::logger("trace", "main", " ===== [DATABASE] ===== ");
    log::logger("trace", "main", &format!("host     : {}", conf.database.host));
    log::logger("info",  "main", " ============== [RUN] ============== ");

    let db = postgres::connect(&conf.database.host).await;
    log::logger("info", "main", "Successfully connected to the database");

    match methode.as_str() {
        "create" => {
            let uuid = Uuid::new_v4().to_string(); 
            let book = books::crud::create(&db, uuid.as_str(), "is GOD a MATEMATICIAN", "Marion Livio", 3).await;
            match book {
                Ok(book) => log::logger("info", "main", &format!("Created book : {:?}", book)),
                Err(err) => log::logger("error", "main", &format!("Failed creating book : {:?}", err)),
            }
        }

        "readAll" => {
            match books::crud::read_all(&db).await {
                Ok(books) => {
                    let mut counter = 1; 
                    for book in books {
                        log::logger("info", "main", &format!("[{}] : {:?}",counter, book));
                        counter += 1;
                    }
                }
                Err(e) => {
                    log::logger("info", "main", &format!("Failed readAll : {:?}",e));
                }
            }
        }

        "readUUID" => {
            let uuid = args[2..3].join(" ");
            log::logger("info", "main", &format!("uuid : {:?}", uuid));
            let book = books::crud::read_by_uuid(&db, &uuid).await;
            match book {
                Ok(Some(book)) => log::logger("info", "main", &format!("Found book : {:?}", book)),
                Ok(None) => log::logger("warning", "main", &format!("Book not Found: {:?}", book)),
                Err(err) => log::logger("error", "main", &format!("Failed found book: {:?}", err)),
            }
        }

        "update" => {
            let uuid = args[2..3].join(" ");
            log::logger("info", "main", &format!("uuid : {:?}", uuid));
            let updated_book = books::crud::update(&db,  &uuid, Some("Updated Title"), None, Some(5)).await;
            match updated_book {
                Ok(book) => log::logger("info", "main", &format!("Updated book : {:?}", book)),
                Err(err) => log::logger("error", "main", &format!("Failed updating book: {:?}", err)),
            }
        }

        "delete_soft" => {
            let uuid = args[2..3].join(" ");
            log::logger("info", "main", &format!("uuid : {:?}", uuid));
            let book = books::crud::delete_soft(&db,  &uuid).await;
            match book {
                Ok(result) => log::logger("info", "main", &format!("deleted_soft book : {:?}", result)),
                Err(err) => log::logger("error", "main", &format!("Failed deleted_soft book: {:?}", err)),
            }
        }

        "delete_hard" => {
            let uuid = args[2..3].join(" ");
            log::logger("info", "main", &format!("uuid : {:?}", uuid));
            let delete_result = books::crud::delete(&db,  &uuid).await;
            match delete_result {
                Ok(result) => log::logger("info", "main", &format!("delete_hard book : {:?}", result)),
                Err(err) => log::logger("error", "main", &format!("Failed delete_hard book: {:?}", err)),
            }
        }

        _ =>  log::logger("error", "main", "methode not found"),
    }

}