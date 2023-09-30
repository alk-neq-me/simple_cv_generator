use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::base::{Experence, Qualifacation};


#[derive(Deserialize, Serialize)]
pub enum MarrieStatus {
    Single,
    Married,
}

#[derive(Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female
}

#[derive(Deserialize, Serialize)]
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
    pub experences: HashMap<String, Experence>,
    pub address: String,
    pub other_qualifacations: HashMap<String, Qualifacation>
}
