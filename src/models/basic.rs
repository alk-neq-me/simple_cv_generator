use serde::{Serialize, Deserialize};
use tera::{Context, Error};

use crate::utils::template::Generate;


#[derive(Serialize, Deserialize, Debug)]
pub struct Basic {
    pub name: String,
    pub nrc: String,
    pub birth: String,
    pub experiecnce: String,
    pub other_qualifacation: Option<String>,
    pub phone: String,
    pub photo: String,
    pub website: Option<String>,
    pub mail: Option<String>,
}

impl Generate for Basic {
    fn get_context(&self) -> Result<Context, Error> {
        Context::from_serialize(&self)
    }
}


