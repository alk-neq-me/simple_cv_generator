use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize)]
pub enum MarrieStatus {
    Single,
    Married,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Basic {
    pub name: String,
    pub photo: String,
    pub phone: String,
    pub website: Option<String>,
    pub mail: Option<String>,

    pub father: String,
    pub education: String,
    pub gender: Gender,
    pub marrie_status: MarrieStatus,
    pub nrc: String,
    pub birth: String,
    pub experiecnce: String,
    pub address: String,
    pub other_qualifacation: Option<String>,
}
