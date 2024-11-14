use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub tz: bool,
    pub log:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub host : String,
}